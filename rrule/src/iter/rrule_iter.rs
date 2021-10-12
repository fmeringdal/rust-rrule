use super::{
    build_pos_list, increment_counter_date, make_timeset, remove_filtered_days,
    utils::from_ordinal, IterInfo, MAX_ITER_LOOP,
};
use crate::{
    core::{DateTime, Time},
    Frequency, RRule, RRuleError,
};
use chrono::{Datelike, TimeZone, Timelike};
use chrono_tz::UTC;
use std::collections::VecDeque;

pub struct RRuleIter<'a> {
    /// Date the iterator is currently at.
    counter_date: DateTime,
    ii: IterInfo<'a>,
    timeset: Vec<Time>,
    // Buffer of datetimes not yet yielded
    buffer: VecDeque<DateTime>,
    /// Indicate of iterator should not return more items.
    /// Once set set `true` is will always return `None`.
    finished: bool,
    /// Number of events that should still be generated before the end.
    /// Counter always goes down after each iteration.
    count: Option<u32>,
    /// Store the last error so it can be handled by the user.
    error: Option<RRuleError>,
}

impl<'a> RRuleIter<'a> {
    /// Return `true` if an error has occurred.
    pub fn has_err(&self) -> bool {
        self.error.is_some()
    }
    /// Return an the last error while iterating.
    pub fn get_err(&self) -> Option<&RRuleError> {
        self.error.as_ref()
    }

    fn new(rrule: &'a RRule) -> Result<Self, RRuleError> {
        let ii = IterInfo::new(rrule.get_options())?;
        let ii_options = ii.get_options();
        let counter_date = ii_options.dt_start;

        let timeset = make_timeset(&ii, &counter_date, ii_options)?;
        let count = ii_options.count;

        Ok(RRuleIter {
            counter_date,
            ii,
            timeset,
            buffer: VecDeque::new(),
            finished: false,
            count,
            error: None,
        })
    }
    /// Generate a list of dates that will be added to the buffer.
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
        match self.count {
            Some(count) if count == 0 => return Ok(true),
            _ => (),
        };

        // Get general info about recurrence rules
        let options = self.ii.get_options().clone();

        if options.interval == 0 {
            return Ok(true);
        }

        // If `counter_date` is later then `until` date, we can stop
        if let Some(until) = options.until {
            if self.counter_date > until {
                return Ok(false);
            }
        }
        let mut loop_counter: u32 = 0;

        // Loop until there is at least 1 item in the buffer.
        while self.buffer.is_empty() {
            // Prevent infinite loops
            loop_counter += 1;
            if loop_counter >= MAX_ITER_LOOP {
                return Err(RRuleError::new_iter_err(format!(
                    "Reached max loop counter (`{}`). \
                    See 'validator limits' in docs for more info.",
                    MAX_ITER_LOOP
                )));
            }

            let (dayset, start, end) = self.ii.get_dayset(
                &self.ii.get_options().freq,
                self.counter_date.year(),
                self.counter_date.month() as u8,
                self.counter_date.day() as u8,
            )?;

            // If `counter_date` is later then `until` date, we can stop
            if let Some(until) = options.until {
                if self.counter_date > until {
                    return Ok(false);
                }
            }

            // Change `Vec<u64>` to `Vec<Option<u64>>`
            let mut dayset = dayset.into_iter().map(Some).collect::<Vec<Option<u64>>>();

            let filtered = remove_filtered_days(&mut dayset, start, end, &self.ii)?;

            // Change `Vec<Option<u64>>` to `Vec<Option<i32>>`
            let dayset = dayset
                .into_iter()
                .map(|day| day.map(|day| day as i32))
                .collect::<Vec<Option<i32>>>();

            if !options.by_set_pos.is_empty() {
                let pos_list = build_pos_list(
                    &options.by_set_pos,
                    &self.timeset,
                    start,
                    end,
                    &self.ii,
                    &dayset,
                    &options.tz,
                )?;

                for res in pos_list {
                    if options.until.is_some() && res > options.until.unwrap() {
                        continue; // or break ?
                    }

                    if res >= options.dt_start {
                        self.buffer.push_back(res);

                        if let Some(count) = self.count {
                            if count > 0 {
                                self.count = Some(count - 1);
                                // self.ii.options.count = Some(count - 1);
                            }
                            // This means that the real count is 0, because of the decrement above
                            if count == 1 {
                                return Ok(true);
                            }
                        }
                    }
                }
            } else {
                // Loop over `start..end`
                for current_day in dayset.iter().take(end as usize).skip(start as usize) {
                    if current_day.is_none() {
                        continue;
                    }

                    let current_day = current_day.unwrap();
                    let year_ordinal = self.ii.year_ordinal().unwrap();
                    // Ordinal conversion uses UTC: if we apply local-TZ here, then
                    // just below we'll end up double-applying.
                    let date = from_ordinal(year_ordinal + current_day as i64, &UTC);
                    // We apply the local-TZ here,
                    let date = options.tz.ymd(date.year(), date.month(), date.day());
                    for timeset in &self.timeset {
                        let res = date
                            .and_hms(0, 0, 0)
                            .checked_add_signed(timeset.duration_from_midnight())
                            .ok_or_else(|| RRuleError::new_iter_err("Invalid datetime."))?;

                        if options.until.is_some() && res > options.until.unwrap() {
                            return Ok(true);
                        }
                        if res >= options.dt_start {
                            self.buffer.push_back(res);

                            if let Some(count) = self.count {
                                if count > 0 {
                                    self.count = Some(count - 1);
                                    // self.ii.options.count = Some(count - 1);
                                }
                                // This means that the real count is 0, because of the decrement above
                                if count == 1 {
                                    return Ok(true);
                                }
                            }
                        }
                    }
                }
            }

            // Handle frequency and interval
            self.counter_date = increment_counter_date(self.counter_date, &options, filtered)?;

            if options.freq == Frequency::Hourly
                || options.freq == Frequency::Minutely
                || options.freq == Frequency::Secondly
            {
                self.timeset = self.ii.get_timeset(
                    &options.freq,
                    self.counter_date.hour() as u8,
                    self.counter_date.minute() as u8,
                    self.counter_date.second() as u8,
                    0,
                )?;
            }

            let year = self.counter_date.year();
            let month = self.counter_date.month() as u8;

            self.ii.rebuild(year, month)?;
        }
        // Indicate that there might be more items on the next iteration.
        Ok(false)
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

        self.finished = match self.generate() {
            Ok(finished) => finished,
            Err(err) => {
                log::error!("{}", err);
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

impl<'a> IntoIterator for &'a RRule {
    type Item = DateTime;

    type IntoIter = RRuleIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        match RRuleIter::new(self) {
            Ok(iter) => iter,
            Err(err) => {
                // Print error and create iterator that will ways return the error if used.
                log::error!("{}", err);
                let error = Some(err);
                // This is mainly a dummy object, as it will ways return the error when called.
                RRuleIter {
                    counter_date: self.get_options().dt_start,
                    ii: IterInfo::new_no_rebuild(self.get_options()),
                    timeset: vec![],
                    buffer: VecDeque::new(),
                    finished: false,
                    count: None,
                    error,
                }
            }
        }
    }
}
