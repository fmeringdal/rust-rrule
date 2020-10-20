use crate::iter::*;
use crate::iterinfo::*;
use crate::options::*;
use crate::poslist::*;
use crate::rrule::*;
use chrono::offset::TimeZone;
use chrono::prelude::*;
use chrono::{DateTime, Duration, Utc};
use chrono_tz::*;
use std::collections::HashMap;

pub trait TIterResult {
    fn accept(&mut self, date: DateTime<Tz>) -> bool;
    fn get_value(&self) -> Vec<DateTime<Tz>>;
}

pub fn iter_v2<T: TIterResult>(
    iter_result: &mut T,
    options: &mut ParsedOptions,
) -> Vec<DateTime<Tz>> {
    if (options.count.is_some() && options.count.unwrap() == 0) || options.interval == 0 {
        return iter_result.get_value();
    }

    let mut counter_date = options.dtstart;
    let mut ii = IterInfo::new(options);
    ii.rebuild(counter_date.year() as isize, counter_date.month() as usize);

    let mut timeset = make_timeset(&ii, &counter_date, options);
    let mut count = match options.count {
        Some(count) => count,
        _ => 0,
    };

    loop {
        let (dayset, start, end) = ii.getdayset(
            &options.freq,
            counter_date.year() as isize,
            counter_date.month() as usize,
            counter_date.day() as usize,
        );

        let mut dayset = dayset
            .into_iter()
            .map(|s| Some(s as isize))
            .collect::<Vec<Option<isize>>>();

        let filtered = remove_filtered_days(&mut dayset, start, end, &ii, options);

        if not_empty(&options.bysetpos) {
            let poslist = build_poslist(&options.bysetpos, &timeset, start, end, &ii, &dayset);

            for j in 0..poslist.len() {
                let res = poslist[j];
                if options.until.is_some() && res > options.until.unwrap() {
                    return iter_result.get_value();
                }

                if res >= options.dtstart {
                    //let rezoned_date = rezone_if_needed(&res, &options);
                    let rezoned_date = UTC.timestamp(res.timestamp(), 0);

                    if !iter_result.accept(rezoned_date) {
                        return iter_result.get_value();
                    }

                    if count > 0 {
                        count -= 1;
                        if count == 0 {
                            return iter_result.get_value();
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
                let date = from_ordinal(ii.yearordinal().unwrap() + current_day);
                for k in 0..timeset.len() {
                    let res = Utc.ymd(date.year(), date.month(), date.day()).and_hms(
                        timeset[k].hour as u32,
                        timeset[k].minute as u32,
                        timeset[k].second as u32,
                    );
                    if options.until.is_some() && res > options.until.unwrap() {
                        return iter_result.get_value();
                    }
                    if res >= options.dtstart {
                        //let rezoned_date = rezone_if_needed(&res, &options);
                        let rezoned_date = UTC.timestamp(res.timestamp(), 0);

                        if !iter_result.accept(rezoned_date) {
                            return iter_result.get_value();
                        }
                        if count > 0 {
                            count -= 1;
                            if count == 0 {
                                return iter_result.get_value();
                            }
                        }
                    }
                }
            }
        }

        if options.interval == 0 {
            return iter_result.get_value();
        }

        // Handle frequency and interval
        counter_date = increment_counter_date(counter_date, options, filtered);

        if counter_date.year() > 2200 {
            return iter_result.get_value();
        }

        if options.freq == Frequenzy::HOURLY
            || options.freq == Frequenzy::MINUTELY
            || options.freq == Frequenzy::SECONDLY
        {
            timeset = ii.gettimeset(
                &options.freq,
                counter_date.hour() as usize,
                counter_date.minute() as usize,
                counter_date.second() as usize,
                0,
            );
        }

        ii.rebuild(counter_date.year() as isize, counter_date.month() as usize);
    }
}
