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

pub fn eval_exdate(
    after: &DateTime<Tz>,
    before: &DateTime<Tz>,
    exrule: &mut Vec<RRule>,
    exdate_hash: &mut HashMap<i64, bool>,
) {
    for rrule in exrule.iter_mut() {
        for date in rrule.between(after, before, true) {
            exdate_hash.insert(date.timestamp(), true);
        }
    }
}

fn accept_1(
    date: DateTime<Tz>,
    exdate_hash: &mut HashMap<i64, bool>,
    exrule: &mut Vec<RRule>,
    iter_res: &mut IterResult,
) -> bool {
    println!("Accept 1");
    println!("Exdates: {:?}", exdate_hash);
    let dt = date.timestamp();
    if !exdate_hash.contains_key(&dt) {
        eval_exdate(
            &UTC.timestamp(dt - 1, 0),
            &UTC.timestamp(dt + 1, 0),
            exrule,
            exdate_hash,
        );
        if !exdate_hash.contains_key(&dt) {
            exdate_hash.insert(dt, true);
            return iter_res.accept(date.clone());
        }
    }

    true
}

fn accept_2(
    date: DateTime<Tz>,
    exdate_hash: &mut HashMap<i64, bool>,
    iter_res: &mut IterResult,
) -> bool {
    let dt = date.timestamp();
    if !exdate_hash.contains_key(&dt) {
        if !exdate_hash.contains_key(&dt) {
            exdate_hash.insert(dt, true);
            return iter_res.accept(date.clone());
        }
    }

    true
}

pub fn iter_set(
    iter_res: &mut IterResult,
    mut rrule: Vec<RRule>,
    mut exrule: Vec<RRule>,
    rdate: Vec<DateTime<Utc>>,
    exdate: Vec<DateTime<Utc>>,
    tzid: Option<String>,
) -> Vec<DateTime<Tz>> {
    let tzid: Tz = tzid.unwrap_or(String::from("UTC")).parse().unwrap_or(UTC);

    let mut exdate_hash = HashMap::new();

    for date in &exdate {
        let zoned_date = date.with_timezone(&tzid);
        exdate_hash.insert(zoned_date.timestamp(), true);
    }

    match iter_res.method {
        QueryMethodTypes::BETWEEN => {
            eval_exdate(
                &iter_res.args.after,
                &iter_res.args.before,
                &mut exrule,
                &mut exdate_hash,
            );
        }
        _ => (),
    };

    for date in &rdate {
        let zoned_date = date.with_timezone(&tzid);
        println!("Zoned date: {:?}", zoned_date);

        match iter_res.method {
            QueryMethodTypes::BETWEEN => {
                if !accept_2(zoned_date, &mut exdate_hash, iter_res) {
                    break;
                }
            }
            _ => {
                if !accept_1(zoned_date, &mut exdate_hash, &mut exrule, iter_res) {
                    break;
                }
            }
        };
    }

    for rule in rrule.iter_mut() {
        iter_v2(iter_res, &mut rule.options, &mut exdate_hash, &mut exrule);
    }

    let mut res = iter_res._result.clone();
    res.sort();
    res
}

pub fn iter_v2(
    iter_result: &mut IterResult,
    options: &mut ParsedOptions,
    exdate_hash: &mut HashMap<i64, bool>,
    exrule: &mut Vec<RRule>,
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

                    let accepted = match iter_result.method {
                        QueryMethodTypes::BETWEEN => {
                            accept_2(rezoned_date, exdate_hash, iter_result)
                        }
                        _ => accept_1(rezoned_date, exdate_hash, exrule, iter_result),
                    };

                    if !accepted {
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

                        if rezoned_date.day() == 2 {
                            println!("yoooooooooooooooooooooooo");
                            println!("iter date ts: {}", rezoned_date.timestamp());
                            println!("Ex dates: {:?}", exdate_hash);
                        }

                        let accepted = match iter_result.method {
                            QueryMethodTypes::BETWEEN => {
                                accept_2(rezoned_date, exdate_hash, iter_result)
                            }
                            _ => accept_1(rezoned_date, exdate_hash, exrule, iter_result),
                        };
                        if !accepted {
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

#[cfg(test)]
mod test_iter_set {
    use super::*;

    fn ymd_hms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> DateTime<Utc> {
        Utc.ymd(year, month, day).and_hms(hour, minute, second)
    }

    fn ymd_hms_2(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> DateTime<Tz> {
        UTC.ymd(year, month, day).and_hms(hour, minute, second)
    }

    fn test_recurring(actual_dates: Vec<DateTime<Tz>>, expected_dates: Vec<DateTime<Tz>>) {
        assert_eq!(
            actual_dates.len(),
            expected_dates.len(),
            "Expected number of returned dates to be equal to the expected"
        );

        println!("Acutal: {:?}", actual_dates);
        for (actual, exptected) in actual_dates.into_iter().zip(expected_dates) {
            assert_eq!(actual, exptected);
        }
    }

    #[test]
    fn rrule_and_exrule() {
        let iter_args = IterArgs {
            inc: true,
            before: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            after: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            dt: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            _value: Some(vec![]),
        };
        let mut iter_res = IterResult::new(QueryMethodTypes::ALL, iter_args);

        let mut options1 = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(6),
            bymonth: vec![],
            dtstart: Utc.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
            byeaster: None,
        };
        let rrule1 = RRule::new(options1);
        let mut options2 = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: Utc.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
            byeaster: None,
        };
        let rrule2 = RRule::new(options2);

        let res = iter_set(
            &mut iter_res,
            vec![rrule1],
            vec![rrule2],
            vec![],
            vec![],
            None,
        );
        test_recurring(
            res,
            vec![
                ymd_hms_2(1997, 9, 2, 9, 0, 0),
                ymd_hms_2(1997, 9, 9, 9, 0, 0),
                ymd_hms_2(1997, 9, 16, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn setdate_and_exdate() {
        let iter_args = IterArgs {
            inc: true,
            before: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            after: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            dt: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            _value: Some(vec![]),
        };
        let mut iter_res = IterResult::new(QueryMethodTypes::ALL, iter_args);

        let res = iter_set(
            &mut iter_res,
            vec![],
            vec![],
            vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 9, 4, 9, 0, 0),
                ymd_hms(1997, 9, 9, 9, 0, 0),
                ymd_hms(1997, 9, 11, 9, 0, 0),
                ymd_hms(1997, 9, 16, 9, 0, 0),
                ymd_hms(1997, 9, 18, 9, 0, 0),
            ],
            vec![
                ymd_hms(1997, 9, 4, 9, 0, 0),
                ymd_hms(1997, 9, 11, 9, 0, 0),
                ymd_hms(1997, 9, 18, 9, 0, 0),
            ],
            None,
        );
        test_recurring(
            res,
            vec![
                ymd_hms_2(1997, 9, 2, 9, 0, 0),
                ymd_hms_2(1997, 9, 9, 9, 0, 0),
                ymd_hms_2(1997, 9, 16, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn setdate_and_exrule() {
        let iter_args = IterArgs {
            inc: true,
            before: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            after: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            dt: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            _value: Some(vec![]),
        };
        let mut iter_res = IterResult::new(QueryMethodTypes::ALL, iter_args);

        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: Utc.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
            byeaster: None,
        };
        let exrrule = RRule::new(options);
        let res = iter_set(
            &mut iter_res,
            vec![],
            vec![exrrule],
            vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 9, 4, 9, 0, 0),
                ymd_hms(1997, 9, 9, 9, 0, 0),
                ymd_hms(1997, 9, 11, 9, 0, 0),
                ymd_hms(1997, 9, 16, 9, 0, 0),
                ymd_hms(1997, 9, 18, 9, 0, 0),
            ],
            vec![],
            None,
        );
        test_recurring(
            res,
            vec![
                ymd_hms_2(1997, 9, 2, 9, 0, 0),
                ymd_hms_2(1997, 9, 9, 9, 0, 0),
                ymd_hms_2(1997, 9, 16, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn rrule_and_exdate() {
        let iter_args = IterArgs {
            inc: true,
            before: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            after: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            dt: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            _value: Some(vec![]),
        };
        let mut iter_res = IterResult::new(QueryMethodTypes::ALL, iter_args);

        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(6),
            bymonth: vec![],
            dtstart: Utc.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
            byeaster: None,
        };
        let rrule = RRule::new(options);
        let res = iter_set(
            &mut iter_res,
            vec![rrule],
            vec![],
            vec![],
            vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 9, 4, 9, 0, 0),
                ymd_hms(1997, 9, 9, 9, 0, 0),
            ],
            None,
        );
        test_recurring(
            res,
            vec![
                ymd_hms_2(1997, 9, 11, 9, 0, 0),
                ymd_hms_2(1997, 9, 16, 9, 0, 0),
                ymd_hms_2(1997, 9, 18, 9, 0, 0),
            ],
        );
    }
}
