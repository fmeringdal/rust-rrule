use crate::datetime::DateTime;
use crate::iter::{
    build_poslist, increment_counter_date, make_timeset, remove_filtered_days, IterInfo,
    RRuleIterError,
};
use crate::{datetime::from_ordinal, RRule};
use crate::{datetime::Time, Frequency};
use chrono::{Datelike, TimeZone, Timelike};
use chrono_tz::UTC;
use std::collections::VecDeque;

const MAX_YEAR: i32 = 9999;

pub struct RRuleIter<'a> {
    /// Date the iterator is currently at.
    pub counter_date: DateTime,
    pub ii: IterInfo<'a>,
    pub timeset: Vec<Time>,
    // Buffer of datetimes not yet yielded
    pub buffer: VecDeque<DateTime>,
    /// Indicate of iterator should not return more items.
    /// Once set set `true` is will always return `None`.
    pub finished: bool,
    /// Number of events that should still be generated before the end.
    /// Counter always goes down after each iteration.
    pub count: Option<u32>,
    /// Store the last error so it can be handled by the user.
    pub error: Option<RRuleIterError>,
}

impl<'a> RRuleIter<'a> {
    /// Return `true` if an error has occurred.
    pub fn has_err(&self) -> bool {
        self.error.is_some()
    }
    /// Return an the last error while iterating.
    pub fn get_err(&self) -> &Option<RRuleIterError> {
        &self.error
    }

    fn new(rrule: &'a RRule) -> Result<Self, RRuleIterError> {
        let ii = IterInfo::new(&rrule.options)?;
        let counter_date = ii.options.dtstart;

        let timeset = make_timeset(&ii, &counter_date, &ii.options)?;
        let count = ii.options.count.clone();

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
    pub fn generate(&mut self) -> Result<bool, RRuleIterError> {
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
        // Check if `MAX_YEAR` is reached.
        if self.counter_date.year() > MAX_YEAR {
            return Err(RRuleIterError(format!(
                "Iterator reached maximum year `{}`.",
                MAX_YEAR
            )));
        }

        // Get general info about recurrence rules
        let options = self.ii.options;

        if options.interval == 0 {
            return Ok(true);
        }

        // Loop until there is at least 1 item in the buffer.
        while self.buffer.is_empty() {
            let (dayset, start, end) = self.ii.getdayset(
                &self.ii.options.freq,
                self.counter_date.year() as isize,
                self.counter_date.month() as usize,
                self.counter_date.day() as usize,
            )?;

            let mut dayset = dayset
                .into_iter()
                .map(|s| Some(s as isize))
                .collect::<Vec<Option<isize>>>();

            let filtered = remove_filtered_days(&mut dayset, start, end, &self.ii)?;

            if options.bysetpos.len() > 0 {
                let poslist = build_poslist(
                    &options.bysetpos,
                    &self.timeset,
                    start,
                    end,
                    &self.ii,
                    &dayset,
                    &options.tzid,
                )?;

                for j in 0..poslist.len() {
                    let res = poslist[j];
                    if options.until.is_some() && res > options.until.unwrap() {
                        continue; // or break ?
                    }

                    if res >= options.dtstart {
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
                for j in start..end {
                    let current_day = dayset[j];
                    if current_day.is_none() {
                        continue;
                    }

                    let current_day = current_day.unwrap();
                    let year_ordinal = self.ii.yearordinal().unwrap();
                    // Ordinal conversion uses UTC: if we apply local-TZ here, then
                    // just below we'll end up double-applying.
                    let date = from_ordinal(year_ordinal + current_day as i64, &UTC);
                    // We apply the local-TZ here,
                    let date = options.tzid.ymd(date.year(), date.month(), date.day());
                    for timeset in &self.timeset {
                        let res = date
                            .and_hms(0, 0, 0)
                            .checked_add_signed(timeset.duration_from_midnight())
                            .ok_or_else(|| RRuleIterError("Invalid datetime.".to_owned()))?;

                        if options.until.is_some() && res > options.until.unwrap() {
                            return Ok(true);
                        }
                        if res >= options.dtstart {
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

            if self.counter_date.year() > MAX_YEAR {
                return Ok(true);
            }

            if options.freq == Frequency::Hourly
                || options.freq == Frequency::Minutely
                || options.freq == Frequency::Secondly
            {
                self.timeset = self.ii.gettimeset(
                    &options.freq,
                    self.counter_date.hour() as usize,
                    self.counter_date.minute() as usize,
                    self.counter_date.second() as usize,
                    0,
                )?;
            }

            let year = self.counter_date.year();
            let month = self.counter_date.month();

            self.ii.rebuild(year as isize, month as usize)?;
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
                    counter_date: self.options.dtstart,
                    ii: IterInfo {
                        options: &self.options,
                        yearinfo: None,
                        monthinfo: None,
                        eastermask: None,
                    },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Takes too much time, only run when releasing"]
    fn iteration_past_max_year_should_not_panic() {
        let rrule = "DTSTART:20220201T100000Z\nRRULE:FREQ=DAILY"
            .parse::<RRule>()
            .unwrap();
        rrule.clone().into_iter().nth(15000000);
        let rrule = "DTSTART:20220201T100000Z\nRRULE:FREQ=MONTHLY"
            .parse::<RRule>()
            .unwrap();
        rrule.clone().into_iter().nth(15000000);
        let rrule = "DTSTART:20220201T100000Z\nRRULE:FREQ=YEARLY"
            .parse::<RRule>()
            .unwrap();
        rrule.clone().into_iter().nth(15000000);
    }
}
