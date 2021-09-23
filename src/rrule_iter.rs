use crate::iter::{
    build_poslist, increment_counter_date, make_timeset, remove_filtered_days, IterInfo,
};
use crate::{datetime::from_ordinal, RRule};
use crate::{datetime::Time, Frequency};
use chrono::{prelude::*, Duration};
use chrono_tz::{Tz, UTC};
use std::collections::VecDeque;

const MAX_YEAR: i32 = 9999;

pub struct RRuleIter<'a> {
    pub counter_date: DateTime<Tz>,
    pub ii: IterInfo<'a>,
    pub timeset: Vec<Time>,
    // Buffer of datetimes not yet yielded
    pub buffer: VecDeque<DateTime<Tz>>,
    pub finished: bool,
    pub count: Option<u32>,
}

impl<'a> RRuleIter<'a> {
    pub fn generate(&mut self) -> bool {
        let options = self.ii.options;

        if options.interval == 0 {
            return true;
        }

        match self.count {
            Some(count) if count == 0 => return true,
            _ => (),
        };

        if self.counter_date.year() > MAX_YEAR {
            return true;
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
                                return true;
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
                    let date = from_ordinal(year_ordinal + current_day, &UTC);
                    // We apply the local-TZ here,
                    let date = options.tzid.ymd(date.year(), date.month(), date.day());
                    for k in 0..self.timeset.len() {
                        let res = date.and_hms(0, 0, 0)
                            + Duration::hours(self.timeset[k].hour as i64)
                            + Duration::minutes(self.timeset[k].minute as i64)
                            + Duration::seconds(self.timeset[k].second as i64);

                        if options.until.is_some() && res > options.until.unwrap() {
                            return true;
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
                                    return true;
                                }
                            }
                        }
                    }
                }
            }

            // Handle frequency and interval
            self.counter_date = increment_counter_date(self.counter_date, &options, filtered);

            if self.counter_date.year() > MAX_YEAR {
                return true;
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
                );
            }

            let year = self.counter_date.year();
            let month = self.counter_date.month();

            self.ii.rebuild(year as isize, month as usize);
        }

        false
    }
}

impl<'a> Iterator for RRuleIter<'a> {
    type Item = DateTime<Tz>;

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

impl<'a> IntoIterator for &'a RRule {
    type Item = DateTime<Tz>;

    type IntoIter = RRuleIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let ii = IterInfo::new(&self.options);
        let counter_date = ii.options.dtstart;
        // ii.rebuild(counter_date.year() as isize, counter_date.month() as usize);

        let timeset = make_timeset(&ii, &counter_date, &ii.options);
        let count = ii.options.count.clone();

        RRuleIter {
            counter_date,
            ii,
            timeset,
            buffer: VecDeque::new(),
            finished: false,
            count,
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
