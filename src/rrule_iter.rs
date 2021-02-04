use crate::iter::{
    build_poslist, increment_counter_date, make_timeset, remove_filtered_days, IterInfo,
};
use crate::{datetime::from_ordinal, RRule};
use crate::{datetime::Time, Frequenzy};
use chrono::prelude::*;
use chrono_tz::Tz;
use std::collections::VecDeque;

const MAX_YEAR: i32 = 9999;

pub struct RRuleIter {
    pub counter_date: DateTime<Tz>,
    pub ii: IterInfo,
    pub timeset: Vec<Time>,
    // Buffer of datetimes not yet yielded
    pub buffer: VecDeque<DateTime<Tz>>,
    pub finished: bool,
}

impl RRuleIter {
    pub fn generate(&mut self) {
        let options = self.ii.options.clone();

        match options.count {
            Some(count) if count == 0 => return,
            _ => (),
        };

        if self.counter_date.year() > MAX_YEAR {
            return;
        }

        while self.buffer.is_empty() {
            let (dayset, start, end) = self.ii.getdayset(
                &self.ii.options.freq,
                self.counter_date.year() as isize,
                self.counter_date.month() as usize,
                self.counter_date.day() as usize,
            );

            let mut dayset = dayset
                .into_iter()
                .map(|s| Some(s as isize))
                .collect::<Vec<Option<isize>>>();

            let filtered = remove_filtered_days(&mut dayset, start, end, &self.ii);

            if options.bysetpos.len() > 0 {
                let poslist = build_poslist(
                    &options.bysetpos,
                    &self.timeset,
                    start,
                    end,
                    &self.ii,
                    &dayset,
                    &options.tzid,
                );

                for j in 0..poslist.len() {
                    let res = poslist[j];
                    if options.until.is_some() && res > options.until.unwrap() {
                        // return iter_result.get_value();
                        continue; // or break ?
                    }

                    if res >= options.dtstart {
                        self.buffer.push_back(res);

                        if let Some(count) = self.ii.options.count {
                            if count > 0 {
                                self.ii.options.count = Some(count - 1);
                            }
                            // This means that the real count is 0, because of the decrement above
                            if count == 1 {
                                return;
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
                    let date =
                        from_ordinal(self.ii.yearordinal().unwrap() + current_day, &options.tzid);
                    for k in 0..self.timeset.len() {
                        let res = options
                            .tzid
                            .ymd(date.year(), date.month(), date.day())
                            .and_hms(
                                self.timeset[k].hour as u32,
                                self.timeset[k].minute as u32,
                                self.timeset[k].second as u32,
                            );
                        if options.until.is_some() && res > options.until.unwrap() {
                            return;
                        }
                        if res >= options.dtstart {
                            self.buffer.push_back(res);

                            if let Some(count) = self.ii.options.count {
                                if count > 0 {
                                    self.ii.options.count = Some(count - 1);
                                }
                                // This means that the real count is 0, because of the decrement above
                                if count == 1 {
                                    return;
                                }
                            }
                        }
                    }
                }
            }

            if options.interval == 0 {
                return;
            }

            // Handle frequency and interval
            self.counter_date = increment_counter_date(self.counter_date, &options, filtered);

            if self.counter_date.year() > MAX_YEAR {
                return;
            }

            if options.freq == Frequenzy::Hourly
                || options.freq == Frequenzy::Minutely
                || options.freq == Frequenzy::Secondly
            {
                self.timeset = self.ii.gettimeset(
                    &options.freq,
                    self.counter_date.hour() as usize,
                    self.counter_date.minute() as usize,
                    self.counter_date.second() as usize,
                    0,
                );
            }

            let year = self.counter_date.year();
            let month = self.counter_date.month();

            self.ii.rebuild(year as isize, month as usize);
        }
    }
}

impl Iterator for RRuleIter {
    type Item = DateTime<Tz>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        if !self.buffer.is_empty() {
            return self.buffer.pop_front();
        }

        self.generate();

        if self.buffer.is_empty() {
            self.finished = true;
        }
        self.buffer.pop_front()
    }
}

impl IntoIterator for RRule {
    type Item = DateTime<Tz>;

    type IntoIter = RRuleIter;

    fn into_iter(self) -> Self::IntoIter {
        let mut ii = IterInfo::new(self.options);
        let counter_date = ii.options.dtstart;
        ii.rebuild(counter_date.year() as isize, counter_date.month() as usize);

        let timeset = make_timeset(&ii, &counter_date, &ii.options);

        RRuleIter {
            counter_date,
            ii,
            timeset,
            buffer: VecDeque::new(),
            finished: false,
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
