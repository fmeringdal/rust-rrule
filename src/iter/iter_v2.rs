use std::cmp::{max, min};

use crate::{datetime::Time, Frequenzy};
use chrono::prelude::*;
use chrono_tz::Tz;

use super::{build_poslist, from_ordinal, increment_counter_date, remove_filtered_days, IterInfo};

pub struct RRuleIter {
    pub counter_date: DateTime<Tz>,
    pub ii: IterInfo,
    pub timeset: Vec<Time>,
    pub remain: Vec<DateTime<Tz>>,
    pub finished: bool,
}

impl Iterator for RRuleIter {
    type Item = DateTime<Tz>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        if !self.remain.is_empty() {
            return Some(self.remain.remove(0));
        }

        generate(self);

        // println!("Done generating: {:?}", self.remain);

        if self.remain.is_empty() {
            self.finished = true;
            None
        } else {
            Some(self.remain.remove(0))
        }
    }
}

pub fn generate(iter: &mut RRuleIter) {
    let options = iter.ii.options.clone();

    match options.count {
        Some(count) if count == 0 => return,
        _ => (),
    };

    while iter.remain.is_empty() {
        let (dayset, start, end) = iter.ii.getdayset(
            &iter.ii.options.freq,
            iter.counter_date.year() as isize,
            iter.counter_date.month() as usize,
            iter.counter_date.day() as usize,
        );

        let mut dayset = dayset
            .into_iter()
            .map(|s| Some(s as isize))
            .collect::<Vec<Option<isize>>>();

        let filtered = remove_filtered_days(&mut dayset, start, end, &iter.ii);

        if options.bysetpos.len() > 0 {
            let poslist = build_poslist(
                &options.bysetpos,
                &iter.timeset,
                start,
                end,
                &iter.ii,
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
                    iter.remain.push(res);

                    if let Some(count) = iter.ii.options.count {
                        if count > 0 {
                            iter.ii.options.count = Some(count - 1);
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
                    from_ordinal(iter.ii.yearordinal().unwrap() + current_day, &options.tzid);
                for k in 0..iter.timeset.len() {
                    let res = options
                        .tzid
                        .ymd(date.year(), date.month(), date.day())
                        .and_hms(
                            iter.timeset[k].hour as u32,
                            iter.timeset[k].minute as u32,
                            iter.timeset[k].second as u32,
                        );
                    if options.until.is_some() && res > options.until.unwrap() {
                        return;
                    }
                    if res >= options.dtstart {
                        iter.remain.push(res);

                        if let Some(count) = iter.ii.options.count {
                            if count > 0 {
                                iter.ii.options.count = Some(count - 1);
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
        iter.counter_date = increment_counter_date(iter.counter_date, &options, filtered);

        if iter.counter_date.year() > 2200 {
            return;
        }

        if options.freq == Frequenzy::Hourly
            || options.freq == Frequenzy::Minutely
            || options.freq == Frequenzy::Secondly
        {
            iter.timeset = iter.ii.gettimeset(
                &options.freq,
                iter.counter_date.hour() as usize,
                iter.counter_date.minute() as usize,
                iter.counter_date.second() as usize,
                0,
            );
        }

        let year = iter.counter_date.year();
        let month = iter.counter_date.month();

        iter.ii.rebuild(year as isize, month as usize);
    }
}
