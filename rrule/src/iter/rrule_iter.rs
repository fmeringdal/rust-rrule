use super::counter_date::DateTimeIter;
use super::{build_pos_list, utils::from_ordinal, IterInfo, MAX_ITER_LOOP};
use crate::core::{get_hour, get_minute, get_second};
use crate::validator::check_limits;
use crate::{core::DateTime, Frequency, RRule, RRuleError, WithError};
use chrono::NaiveTime;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub(crate) struct RRuleIter<'a> {
    /// Date the iterator is currently at.
    pub(crate) counter_date: DateTimeIter,
    pub(crate) ii: IterInfo<'a>,
    pub(crate) timeset: Vec<NaiveTime>,
    pub(crate) dt_start: DateTime,
    /// Buffer of datetimes not yet yielded
    pub(crate) buffer: VecDeque<DateTime>,
    /// Indicate of iterator should not return more items.
    /// Once set `true` is will always return `None`.
    pub(crate) finished: bool,
    /// Number of events that should still be generated before the end.
    /// Counter always goes down after each iteration.
    pub(crate) count: Option<u32>,
    /// Store the last error, so it can be handled by the user.
    pub(crate) error: Option<RRuleError>,
    pub(crate) limited: bool,
}

impl<'a> RRuleIter<'a> {
    pub(crate) fn new(rrule: &'a RRule, dt_start: &DateTime, limited: bool) -> Self {
        let ii = IterInfo::new(rrule, dt_start);

        let hour = get_hour(dt_start);
        let minute = get_minute(dt_start);
        let second = get_second(dt_start);
        let timeset = ii.get_timeset(hour, minute, second);
        let count = ii.rrule().count;

        let error = if limited {
            // Validate (optional) sanity checks. (arbitrary limits)
            if let Err(e) = check_limits::check_limits(rrule, dt_start) {
                Some(e.into())
            } else {
                None
            }
        } else {
            None
        };

        RRuleIter {
            counter_date: dt_start.into(),
            ii,
            timeset,
            dt_start: *dt_start,
            buffer: VecDeque::new(),
            finished: false,
            count,
            error,
            limited,
        }
    }

    /// Attempts to add a date to the result. Returns `true` if we should
    /// terminate the iteration.
    fn try_add_datetime(
        dt: DateTime,
        rrule: &RRule,
        count: &mut Option<u32>,
        buffer: &mut VecDeque<DateTime>,
        dt_start: &DateTime,
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
    fn generate(&mut self) -> Result<bool, RRuleError> {
        // If there already was an error, return the error again.
        if let Some(err) = self.get_err() {
            return Err(err.clone());
        }

        // Do early check if done (if known)
        if self.finished {
            return Ok(true);
        }
        // Check if the count is set, and if 0
        if matches!(self.count, Some(count) if count == 0) {
            return Ok(true);
        }

        let rrule = self.ii.rrule();

        if rrule.interval == 0 {
            return Ok(true);
        }

        let mut loop_counter: u32 = 0;
        // Loop until there is at least 1 item in the buffer.
        while self.buffer.is_empty() {
            // Prevent infinite loops
            if self.limited {
                loop_counter += 1;
                if loop_counter >= MAX_ITER_LOOP {
                    return Err(RRuleError::new_iter_err(format!(
                        "Reached max loop counter (`{}`). \
                    See 'validator limits' in docs for more info.",
                        MAX_ITER_LOOP
                    )));
                }
            }
            let rrule = self.ii.rrule();

            let dayset = self.ii.get_dayset(
                rrule.freq,
                self.counter_date.year,
                self.counter_date.month,
                self.counter_date.day,
            );

            if rrule.by_set_pos.is_empty() {
                // Loop over `start..end`
                for current_day in &dayset {
                    let current_day = i64::try_from(*current_day).expect(
                        "We control the dayset and we know that it will always fit within an i64",
                    );
                    let year_ordinal = self.ii.year_ordinal();
                    // Ordinal conversion uses UTC: if we apply local-TZ here, then
                    // just below we'll end up double-applying.
                    let date = from_ordinal(year_ordinal + current_day);

                    for time in &self.timeset {
                        let dt = match self.dt_start.timezone().datetime(
                            date.year(),
                            date.month(),
                            date.day(),
                            *time,
                        ) {
                            Some(dt) => dt,
                            None => continue,
                        };
                        if Self::try_add_datetime(
                            dt,
                            rrule,
                            &mut self.count,
                            &mut self.buffer,
                            &self.dt_start,
                        ) {
                            return Ok(true);
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
                )?;
                for dt in pos_list {
                    if Self::try_add_datetime(
                        dt,
                        rrule,
                        &mut self.count,
                        &mut self.buffer,
                        &self.dt_start,
                    ) {
                        return Ok(true);
                    }
                }
            }

            let increment_day = dayset.is_empty();
            self.counter_date.increment(rrule, increment_day)?;

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
        Ok(false)
    }
}

impl<'a> WithError for RRuleIter<'a> {
    fn has_err(&self) -> bool {
        self.error.is_some()
    }

    fn get_err(&self) -> Option<&RRuleError> {
        self.error.as_ref()
    }
}

impl<'a> Iterator for RRuleIter<'a> {
    type Item = DateTime;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buffer.is_empty() {
            return self.buffer.pop_front();
        }

        if self.finished {
            return None;
        }

        self.finished = self.generate().unwrap_or_else(|err| {
            log::error!("{:?}", err);
            self.error = Some(err);
            true
        });

        if self.buffer.is_empty() {
            self.finished = true;
        }
        self.buffer.pop_front()
    }
}
