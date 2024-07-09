use super::counter_date::DateTimeIter;
use super::utils::add_time_to_date;
use super::{build_pos_list, utils::date_from_ordinal, IterInfo, MAX_ITER_LOOP};
use crate::core::{get_hour, get_minute, get_second};
use crate::{Frequency, RRule, Tz};
use chrono::NaiveTime;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub(crate) struct RRuleIter {
    /// Date the iterator is currently at.
    pub(crate) counter_date: DateTimeIter,
    pub(crate) ii: IterInfo,
    pub(crate) timeset: Vec<NaiveTime>,
    pub(crate) dt_start: chrono::DateTime<Tz>,
    /// Buffer of datetimes is not yet yielded
    pub(crate) buffer: VecDeque<chrono::DateTime<Tz>>,
    /// Indicate of iterator should not return more items.
    /// Once set `true` is will always return `None`.
    pub(crate) finished: bool,
    /// Number of events that should still be generated before the end.
    /// Counter always goes down after each iteration.
    pub(crate) count: Option<u32>,
    /// If the iterator should be using iterator limits.
    pub(crate) limited: bool,
    /// If the iterator has been stopped by the iterator limits.
    pub(crate) was_limited: bool,
}

impl RRuleIter {
    pub(crate) fn new(rrule: &RRule, dt_start: &chrono::DateTime<Tz>, limited: bool) -> Self {
        let ii = IterInfo::new(rrule, dt_start);

        let hour = get_hour(dt_start);
        let minute = get_minute(dt_start);
        let second = get_second(dt_start);
        let timeset = ii.get_timeset(hour, minute, second);
        let count = ii.rrule().count;

        Self {
            counter_date: dt_start.into(),
            ii,
            timeset,
            dt_start: *dt_start,
            buffer: VecDeque::new(),
            finished: false,
            count,
            limited,
            was_limited: false,
        }
    }

    /// Attempts to add a date to the result. Returns `true` if we should
    /// terminate the iteration.
    fn try_add_datetime(
        dt: chrono::DateTime<Tz>,
        rrule: &RRule,
        count: &mut Option<u32>,
        buffer: &mut VecDeque<chrono::DateTime<Tz>>,
        dt_start: &chrono::DateTime<Tz>,
    ) -> bool {
        if matches!(rrule.until, Some(until) if dt > until) {
            // We can break because `pos_list` is sorted and
            // all the next dates will only be larger than `until`.
            return true;
        }

        if dt >= *dt_start {
            buffer.push_back(dt);

            if let Some(count) = count {
                *count -= 1;

                if *count == 0 {
                    return true;
                }
            }
        }
        false
    }

    /// Generates a list of dates that will be added to the buffer.
    /// Returns true if finished, no more items should/can be returned.
    fn generate(&mut self) -> bool {
        // Do early check if done (if known)
        if self.finished {
            return true;
        }
        // Check if the count is set, and if 0
        if matches!(self.count, Some(count) if count == 0) {
            return true;
        }

        let rrule = self.ii.rrule();

        if rrule.interval == 0 {
            return true;
        }

        let mut loop_counter: u32 = 0;
        // Loop until there is at least 1 item in the buffer.
        while self.buffer.is_empty() {
            // Prevent infinite loops
            if self.limited {
                loop_counter += 1;
                if loop_counter >= MAX_ITER_LOOP {
                    self.finished = true;
                    self.was_limited = true;
                    log::warn!(
                        "Reached max loop counter (`{}`). \
                    See 'validator limits' in docs for more info.",
                        MAX_ITER_LOOP
                    );
                    return true;
                }
            }
            let rrule = self.ii.rrule();

            let dayset = self.ii.get_dayset(
                rrule.freq,
                self.counter_date.year,
                self.counter_date.month,
                self.counter_date.day,
            );

            let tz = self.dt_start.timezone();

            if rrule.by_set_pos.is_empty() {
                // Loop over `start..end`
                for current_day in &dayset {
                    let current_day = i64::try_from(*current_day).expect(
                        "We control the dayset, and we know that it will always fit within an i64",
                    );
                    let year_ordinal = self.ii.year_ordinal();
                    // Ordinal conversion uses UTC: if we apply local-TZ here, then
                    // just below we'll end up double-applying.
                    let date = date_from_ordinal(year_ordinal + current_day);
                    for time in &self.timeset {
                        let Some(dt) = add_time_to_date(tz, date, *time) else {
                            continue;
                        };
                        if Self::try_add_datetime(
                            dt,
                            rrule,
                            &mut self.count,
                            &mut self.buffer,
                            &self.dt_start,
                        ) {
                            return true;
                        }
                    }
                }
            } else {
                let pos_list = build_pos_list(
                    &rrule.by_set_pos,
                    &dayset,
                    &self.timeset,
                    self.ii.year_ordinal(),
                    self.dt_start.timezone(),
                );
                for dt in pos_list {
                    if Self::try_add_datetime(
                        dt,
                        rrule,
                        &mut self.count,
                        &mut self.buffer,
                        &self.dt_start,
                    ) {
                        return true;
                    }
                }
            }

            let increment_day = dayset.is_empty();
            if self.counter_date.increment(rrule, increment_day).is_err() {
                self.finished = true;
                return true;
            }

            if matches!(
                rrule.freq,
                Frequency::Hourly | Frequency::Minutely | Frequency::Secondly
            ) {
                let hour =
                    u8::try_from(self.counter_date.hour).expect("range 0-23 is covered by u8");
                let minute =
                    u8::try_from(self.counter_date.minute).expect("range 0-59 is covered by u8");
                let second =
                    u8::try_from(self.counter_date.second).expect("range 0-59 is covered by u8");
                self.timeset = self.ii.get_timeset_unchecked(hour, minute, second);
            }

            self.ii.rebuild(&self.counter_date);
        }

        // Indicate that there might be more items on the next iteration.
        false
    }
}

impl Iterator for RRuleIter {
    type Item = chrono::DateTime<Tz>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buffer.is_empty() {
            return self.buffer.pop_front();
        }

        if self.finished {
            return None;
        }

        self.finished = self.generate();

        if self.buffer.is_empty() {
            self.finished = true;
        }
        self.buffer.pop_front()
    }
}

pub(crate) trait WasLimited {
    fn was_limited(&self) -> bool;
}

impl WasLimited for RRuleIter {
    fn was_limited(&self) -> bool {
        self.was_limited
    }
}
