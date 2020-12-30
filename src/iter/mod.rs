mod iterinfo;
mod monthinfo;
mod yearinfo;
use iterinfo::IterInfo;
mod poslist;
use poslist::build_poslist;
mod easter;
mod masks;

use crate::datetime::{from_ordinal, get_weekday_val, DTime, Time};
use crate::options::*;
use crate::utils::{includes, not_empty};
use chrono::offset::TimeZone;
use chrono::prelude::*;
use chrono::Duration;

pub trait IterResult {
    fn accept(&mut self, date: DTime) -> bool;
    fn get_value(&self) -> Vec<DTime>;
}

pub fn iter<T: IterResult>(iter_result: &mut T, options: &mut ParsedOptions) -> Vec<DTime> {
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
            let poslist = build_poslist(
                &options.bysetpos,
                &timeset,
                start,
                end,
                &ii,
                &dayset,
                &options.tzid,
            );

            for j in 0..poslist.len() {
                let res = poslist[j];
                if options.until.is_some() && res > options.until.unwrap() {
                    return iter_result.get_value();
                }

                if res >= options.dtstart {
                    if !iter_result.accept(res) {
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
                let date = from_ordinal(ii.yearordinal().unwrap() + current_day, &options.tzid);
                for k in 0..timeset.len() {
                    let res = options
                        .tzid
                        .ymd(date.year(), date.month(), date.day())
                        .and_hms(
                            timeset[k].hour as u32,
                            timeset[k].minute as u32,
                            timeset[k].second as u32,
                        );
                    if options.until.is_some() && res > options.until.unwrap() {
                        return iter_result.get_value();
                    }
                    if res >= options.dtstart {
                        if !iter_result.accept(res) {
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

        if options.freq == Frequenzy::Hourly
            || options.freq == Frequenzy::Minutely
            || options.freq == Frequenzy::Secondly
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

pub fn increment_counter_date(
    counter_date: DTime,
    options: &ParsedOptions,
    filtered: bool,
) -> DTime {
    match options.freq {
        Frequenzy::Yearly => counter_date
            .with_year(counter_date.year() + options.interval as i32)
            .unwrap(),
        Frequenzy::Monthly => {
            let new_month = counter_date.month() + options.interval as u32;
            if new_month > 12 {
                let mut year_div = new_month / 12;
                let mut new_month = new_month % 12;
                if new_month == 0 {
                    new_month = 12;
                    year_div -= 1;
                }
                let new_year = counter_date.year() + year_div as i32;
                counter_date
                    .with_month(new_month)
                    .unwrap()
                    .with_year(new_year)
                    .unwrap()
            } else {
                counter_date.with_month(new_month).unwrap()
            }
        }
        Frequenzy::Weekly => {
            let mut day_delta = 0;
            let weekday = get_weekday_val(&counter_date.weekday());
            if options.wkst > weekday {
                day_delta += -((weekday + 1 + (6 - options.wkst)) as isize)
                    + (options.interval as isize) * 7;
            } else {
                day_delta += -((weekday - options.wkst) as isize) + (options.interval as isize) * 7;
            }
            counter_date + Duration::days(day_delta as i64)
        }
        Frequenzy::Daily => counter_date + Duration::days(options.interval as i64),
        Frequenzy::Hourly => {
            let mut new_hours = counter_date.hour() as usize;
            if filtered {
                new_hours += ((23 - new_hours) as f32 / options.interval as f32).floor() as usize
                    * options.interval;
            }

            loop {
                new_hours += options.interval;
                if options.byhour.is_empty()
                    || options
                        .byhour
                        .iter()
                        .any(|bh| *bh == (new_hours % 24) as usize)
                {
                    break;
                }
            }
            counter_date.with_hour(0).unwrap() + Duration::hours(new_hours as i64)
        }
        Frequenzy::Minutely => {
            let mut minutes_inc = 0;
            let minutes = counter_date.minute() as usize;
            let hours = counter_date.hour() as usize;
            if filtered {
                // Jump to one iteration before next day
                minutes_inc = (1439. - ((hours * 60 + minutes) as f32 / options.interval as f32))
                    .floor() as usize
                    * options.interval;
            }

            let mut counter_date = counter_date + Duration::minutes(minutes_inc as i64);
            loop {
                counter_date = counter_date + Duration::minutes(options.interval as i64);
                let minutes = counter_date.minute() as usize;
                let hours = counter_date.hour() as usize;

                if (options.byhour.is_empty() || includes(&options.byhour, &hours))
                    && (options.byminute.is_empty() || includes(&options.byminute, &minutes))
                {
                    break;
                }
            }

            counter_date
        }
        Frequenzy::Secondly => {
            let mut seconds_inc = 0;
            let seconds = counter_date.second() as usize;
            let minutes = counter_date.minute() as usize;
            let hours = counter_date.hour() as usize;
            if filtered {
                // Jump to one iteration before next day
                seconds_inc = (86399.
                    - ((hours * 3600 + minutes * 60 + seconds) as f32 / options.interval as f32))
                    .floor() as usize
                    * options.interval;
            }

            let mut counter_date = counter_date + Duration::seconds(seconds_inc as i64);
            loop {
                counter_date = counter_date + Duration::seconds(options.interval as i64);
                let seconds = counter_date.second() as usize;
                let minutes = counter_date.minute() as usize;
                let hours = counter_date.hour() as usize;

                if (options.byhour.is_empty() || includes(&options.byhour, &hours))
                    && (options.byminute.is_empty() || includes(&options.byminute, &minutes))
                    && (options.bysecond.is_empty() || includes(&options.bysecond, &seconds))
                {
                    break;
                }
            }

            counter_date
        }
    }
}

pub fn is_filtered(ii: &IterInfo, current_day: usize, options: &ParsedOptions) -> bool {
    return (not_empty(&options.bymonth)
        && !includes(&options.bymonth, &ii.mmask().unwrap()[current_day]))
        || (not_empty(&options.byweekno) && (ii.wnomask().unwrap()[current_day]) == 0)
        || (not_empty(&options.byweekday)
            && !includes(&options.byweekday, &ii.wdaymask().unwrap()[current_day]))
        || (ii.nwdaymask().is_some()
            && not_empty(ii.nwdaymask().unwrap())
            && (ii.nwdaymask().unwrap()[current_day]) == 0)
        || (options.byeaster.is_some()
            && !(includes(ii.eastermask().unwrap(), &(current_day as isize))))
        || ((not_empty(&options.bymonthday) || not_empty(&options.bynmonthday))
            && !includes(&options.bymonthday, &ii.mdaymask().unwrap()[current_day])
            && !includes(&options.bynmonthday, &ii.nmdaymask().unwrap()[current_day]))
        || (not_empty(&options.byyearday)
            && ((current_day < ii.yearlen().unwrap()
                && !includes(&options.byyearday, &(current_day as isize + 1))
                && !includes(
                    &options.byyearday.iter().map(|v| *v as isize).collect(),
                    &(-(ii.yearlen().unwrap() as isize) + current_day as isize),
                ))
                || (current_day >= ii.yearlen().unwrap()
                    && !includes(
                        &options.byyearday,
                        &((current_day + 1 - ii.yearlen().unwrap()) as isize),
                    )
                    && !includes(
                        &options.byyearday.iter().map(|v| *v as isize).collect(),
                        &(-(ii.nextyearlen().unwrap() as isize) + current_day as isize
                            - ii.yearlen().unwrap() as isize),
                    ))));
}

pub fn remove_filtered_days(
    dayset: &mut Vec<Option<isize>>,
    start: usize,
    end: usize,
    ii: &IterInfo,
    options: &ParsedOptions,
) -> bool {
    let mut filtered = false;

    for daycounter in start..end {
        match dayset[daycounter] {
            Some(current_day) => {
                filtered = is_filtered(ii, current_day as usize, options);
                if filtered {
                    dayset[daycounter] = None;
                }
            }
            None => continue,
        }
    }
    filtered
}

pub fn build_timeset(options: &ParsedOptions) -> Vec<Time> {
    let millisecond_mod = (options.dtstart.timestamp_millis() % 1000) as usize;

    if options.freq > Frequenzy::Daily {
        return vec![];
    }

    let mut timeset = vec![];
    for hour in &options.byhour {
        for minute in &options.byminute {
            for second in &options.bysecond {
                timeset.push(Time::new(*hour, *minute, *second, millisecond_mod));
            }
        }
    }

    timeset
}

pub fn make_timeset(ii: &IterInfo, counter_date: &DTime, options: &ParsedOptions) -> Vec<Time> {
    if options.freq < Frequenzy::Hourly {
        return build_timeset(options);
    }

    if (options.freq >= Frequenzy::Hourly
        && !options.byhour.is_empty()
        && !options
            .byhour
            .iter()
            .any(|&h| h == counter_date.hour() as usize))
        || (options.freq >= Frequenzy::Minutely
            && !options.byminute.is_empty()
            && !options
                .byminute
                .iter()
                .any(|&m| m == counter_date.minute() as usize))
        || (options.freq >= Frequenzy::Secondly
            && !options.bysecond.is_empty()
            && !options
                .bysecond
                .iter()
                .any(|&s| s == counter_date.second() as usize))
    {
        return vec![];
    }

    ii.gettimeset(
        &options.freq,
        counter_date.hour() as usize,
        counter_date.minute() as usize,
        counter_date.second() as usize,
        counter_date.timestamp_subsec_millis() as usize,
    )
}
