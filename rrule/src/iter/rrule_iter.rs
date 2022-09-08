use super::{
    build_pos_list, increment_counter_date, make_timeset, remove_filtered_days,
    utils::from_ordinal, IterInfo, MAX_ITER_LOOP,
};
use crate::core::utils::collect_or_error;
use crate::{core::Time, Frequency, RRule, RRuleError, WithError};
use chrono::{Datelike, Timelike};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub(crate) struct RRuleIter<'a, TZ: chrono::TimeZone> {
    /// Date the iterator is currently at.
    pub(crate) counter_date: chrono::DateTime<TZ>,
    pub(crate) ii: IterInfo<'a, TZ>,
    pub(crate) timeset: Vec<Time>,
    pub(crate) dt_start: chrono::DateTime<TZ>,
    /// Buffer of datetimes not yet yielded
    pub(crate) buffer: VecDeque<chrono::DateTime<TZ>>,
    /// Indicate of iterator should not return more items.
    /// Once set `true` is will always return `None`.
    pub(crate) finished: bool,
    /// Number of events that should still be generated before the end.
    /// Counter always goes down after each iteration.
    pub(crate) count: Option<u32>,
    /// Store the last error, so it can be handled by the user.
    pub(crate) error: Option<RRuleError>,
}

impl<'a, TZ: chrono::TimeZone> RRuleIter<'a, TZ> {
    pub(crate) fn new(
        rrule: &'a RRule<TZ>,
        dt_start: &chrono::DateTime<TZ>,
    ) -> Result<Self, RRuleError> {
        let ii = IterInfo::new(rrule, dt_start)?;

        let timeset = make_timeset(&ii, dt_start, rrule)?;
        let count = ii.get_rrule().count;

        Ok(RRuleIter {
            counter_date: dt_start.clone(),
            ii,
            timeset,
            dt_start: dt_start.clone(),
            buffer: VecDeque::new(),
            finished: false,
            count,
            error: None,
        })
    }

    /// Generates a list of dates that will be added to the buffer.
    /// Returns true if finished, no more items should/can be returned.
    #[warn(clippy::too_many_lines)]
    // TODO: This function is too long.
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
        match self.count {
            Some(count) if count == 0 => return Ok(true),
            _ => (),
        };

        // Get general info about recurrence rules
        let rrule = self.ii.get_rrule();

        if rrule.interval == 0 {
            return Ok(true);
        }

        // If `counter_date` is later than `until` date, we can stop
        if let Some(until) = &rrule.until {
            if &self.counter_date > until {
                return Ok(false);
            }
        }
        let mut loop_counter: u32 = 0;
        // Loop until there is at least 1 item in the buffer.
        while self.buffer.is_empty() {
            let rrule = self.ii.get_rrule();

            // Prevent infinite loops
            loop_counter += 1;
            if loop_counter >= MAX_ITER_LOOP {
                return Err(RRuleError::new_iter_err(format!(
                    "Reached max loop counter (`{}`). \
                    See 'validator limits' in docs for more info.",
                    MAX_ITER_LOOP
                )));
            }

            #[allow(clippy::cast_possible_truncation)]
            let (dayset, start, end) = self.ii.get_dayset(
                rrule.freq,
                self.counter_date.year(),
                self.counter_date.month() as u8,
                self.counter_date.day() as u8,
            )?;

            // If `counter_date` is later than `until` date, we can stop
            if let Some(until) = &rrule.until {
                if &self.counter_date > until {
                    return Ok(false);
                }
            }

            // Change `Vec<u64>` to `Vec<Option<u64>>`
            let mut dayset = dayset
                .into_iter()
                .map(|x| Some(x as i32))
                .collect::<Vec<_>>();

            let filtered = remove_filtered_days(&mut dayset, start, end, &self.ii);

            #[allow(clippy::cast_possible_truncation)]
            if rrule.by_set_pos.is_empty() {
                // Loop over `start..end`
                for current_day in dayset.iter().take(end as usize).skip(start as usize) {
                    if current_day.is_none() {
                        continue;
                    }

                    let current_day = i64::from(current_day.unwrap());
                    let year_ordinal = self.ii.year_ordinal().unwrap();
                    // Ordinal conversion uses UTC: if we apply local-TZ here, then
                    // just below we'll end up double-applying.
                    let date = from_ordinal(year_ordinal + current_day);
                    // We apply the local-TZ here,
                    let date = self
                        .dt_start
                        .timezone()
                        .ymd(date.year(), date.month(), date.day());
                    for timeset in &self.timeset {
                        let res = date
                            .and_hms_opt(0, 0, 0)
                            .ok_or_else(|| RRuleError::new_iter_err("Invalid datetime."))?
                            .checked_add_signed(timeset.duration_from_midnight())
                            .ok_or_else(|| RRuleError::new_iter_err("Invalid datetime."))?;

                        if rrule.until.is_some() && res > *rrule.until.as_ref().unwrap() {
                            return Ok(true);
                        }
                        if res >= self.dt_start {
                            self.buffer.push_back(res);

                            if let Some(count) = self.count {
                                if count > 0 {
                                    self.count = Some(count - 1);
                                    // self.ii.get_rrule().count = Some(count - 1);
                                }
                                // This means that the real count is 0, because of the decrement above
                                if count == 1 {
                                    return Ok(true);
                                }
                            }
                        }
                    }
                }
            } else {
                let pos_list = build_pos_list(
                    &rrule.by_set_pos,
                    &self.timeset,
                    start,
                    end,
                    &self.ii,
                    &dayset,
                    self.dt_start.timezone(),
                )?;

                for res in pos_list {
                    if rrule.until.is_some() && res > *rrule.until.as_ref().unwrap() {
                        continue; // or break ?
                    }

                    if res >= self.dt_start {
                        self.buffer.push_back(res);

                        if let Some(count) = self.count {
                            if count > 0 {
                                self.count = Some(count - 1);
                                // self.ii.get_rrule().count = Some(count - 1);
                            }
                            // This means that the real count is 0, because of the decrement above
                            if count == 1 {
                                return Ok(true);
                            }
                        }
                    }
                }
            }

            // Handle frequency and interval
            self.counter_date = increment_counter_date(self.counter_date.clone(), rrule, filtered)?;

            #[allow(clippy::cast_possible_truncation)]
            if rrule.freq == Frequency::Hourly
                || rrule.freq == Frequency::Minutely
                || rrule.freq == Frequency::Secondly
            {
                self.timeset = self.ii.get_timeset(
                    rrule.freq,
                    self.counter_date.hour() as u8,
                    self.counter_date.minute() as u8,
                    self.counter_date.second() as u8,
                    0,
                )?;
            }

            let year = self.counter_date.year();
            #[allow(clippy::cast_possible_truncation)]
            let month = self.counter_date.month() as u8;

            self.ii.rebuild(year, month)?;
        }
        // Indicate that there might be more items on the next iteration.
        Ok(false)
    }

    /// Returns all the recurrences of the rrule between after and before.
    ///
    /// The `inclusive` keyword defines what happens if after and/or before are
    /// themselves recurrences. With `inclusive == true`, they will be included in the
    /// list, if they are found in the recurrence set.
    pub(crate) fn all_between(
        self,
        start: chrono::DateTime<TZ>,
        end: chrono::DateTime<TZ>,
        inclusive: bool,
    ) -> Result<Vec<chrono::DateTime<TZ>>, RRuleError> {
        collect_or_error(self, &Some(start), &Some(end), inclusive, u16::MAX)
    }
}

impl<'a, TZ: chrono::TimeZone> WithError for RRuleIter<'a, TZ> {
    fn has_err(&self) -> bool {
        self.error.is_some()
    }

    fn get_err(&self) -> Option<&RRuleError> {
        self.error.as_ref()
    }
}

impl<'a, TZ: chrono::TimeZone> Iterator for RRuleIter<'a, TZ> {
    type Item = chrono::DateTime<TZ>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buffer.is_empty() {
            return self.buffer.pop_front();
        }

        if self.finished {
            return None;
        }

        self.finished = match self.generate() {
            Ok(finished) => finished,
            Err(err) => {
                log::error!("{:?}", err);
                self.error = Some(err);
                true
            }
        };

        if self.buffer.is_empty() {
            self.finished = true;
        }
        self.buffer.pop_front()
    }
}
