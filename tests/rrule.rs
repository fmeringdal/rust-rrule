extern crate chrono;
extern crate rrule;

use chrono::prelude::*;
use rrule::iter::*;
use rrule::options::*;
use rrule::yearinfo::*;

#[cfg(test)]
mod test {
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

    fn test_recurring(options: &mut ParsedOptions, expected_dates: &Vec<DateTime<Utc>>) {
        let iter_args = IterArgs {
            inc: true,
            before: Utc::now(),
            after: Utc::now(),
            dt: Utc::now(),
            _value: Some(vec![]),
        };
        let mut iter_res = IterResult::new(QueryMethodTypes::ALL, iter_args);
        let res = iter(&mut iter_res, options);

        assert_eq!(
            res.len(),
            expected_dates.len(),
            "Expected number of returned dates to be equal to the expected"
        );

        println!("Acutal: {:?}", res);
        for (actual, exptected) in res.iter().zip(expected_dates) {
            assert_eq!(actual, exptected);
        }
    }

    #[ignore = "change options"]
    #[test]
    fn int_works() {
        let mut options = ParsedOptions::new(Frequenzy::WEEKLY, &ymd_hms(2012, 1, 1, 10, 30, 0))
            .interval(5)
            .count(3)
            .byweekday(vec![0, 4])
            .bymonth(vec![6]);
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(2012, 6, 18, 10, 30, 0),
                ymd_hms(2012, 6, 22, 10, 30, 0),
                ymd_hms(2013, 6, 3, 10, 30, 0),
            ],
        );
    }

    #[test]
    fn yearly() {
        let mut options =
            ParsedOptions::new(Frequenzy::YEARLY, &ymd_hms(1997, 9, 2, 9, 0, 0)).count(3);
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1998, 9, 2, 9, 0, 0),
                ymd_hms(1999, 9, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_interval() {
        let mut options = ParsedOptions::new(Frequenzy::YEARLY, &ymd_hms(1997, 9, 2, 9, 0, 0))
            .count(3)
            .interval(2);
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1999, 9, 2, 9, 0, 0),
                ymd_hms(2001, 9, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_interval_large() {
        let mut options = ParsedOptions::new(Frequenzy::YEARLY, &ymd_hms(1997, 9, 2, 9, 0, 0))
            .count(3)
            .interval(40);
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(2037, 9, 2, 9, 0, 0),
                ymd_hms(2077, 9, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_month() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 2, 9, 0, 0),
                ymd_hms(1998, 3, 2, 9, 0, 0),
                ymd_hms(1999, 1, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_monthday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![1, 3],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 3, 9, 0, 0),
                ymd_hms(1997, 10, 1, 9, 0, 0),
                ymd_hms(1997, 10, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_month_and_monthday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![5, 7],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 5, 9, 0, 0),
                ymd_hms(1998, 1, 7, 9, 0, 0),
                ymd_hms(1998, 3, 5, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 9, 4, 9, 0, 0),
                ymd_hms(1997, 9, 9, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_nweekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![vec![1, 1], vec![3, -1]],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 25, 9, 0, 0),
                ymd_hms(1998, 1, 6, 9, 0, 0),
                ymd_hms(1998, 12, 31, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_nweekday_large() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![vec![1, 13], vec![3, -13]],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 10, 2, 9, 0, 0),
                ymd_hms(1998, 3, 31, 9, 0, 0),
                ymd_hms(1998, 10, 8, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_month_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 1, 6, 9, 0, 0),
                ymd_hms(1998, 1, 8, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_month_and_nweekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![vec![1, 1], vec![3, -1]],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 6, 9, 0, 0),
                ymd_hms(1998, 1, 29, 9, 0, 0),
                ymd_hms(1998, 3, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_month_and_nweekday_large() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![vec![1, 3], vec![3, -3]],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 15, 9, 0, 0),
                ymd_hms(1998, 1, 20, 9, 0, 0),
                ymd_hms(1998, 3, 12, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_monthday_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![1, 3],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 2, 3, 9, 0, 0),
                ymd_hms(1998, 3, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_month_and_monthday_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![1, 3],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 3, 3, 9, 0, 0),
                ymd_hms(2001, 3, 1, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_yearday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(4),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![1, 100, 200, 365],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 31, 9, 0, 0),
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 4, 10, 9, 0, 0),
                ymd_hms(1998, 7, 19, 9, 0, 0),
            ],
        );
    }

    #[ignore = "negative yeardays are not supported"]
    #[test]
    fn yearly_by_yeardayneq() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(4),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![365],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 31, 9, 0, 0),
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 4, 10, 9, 0, 0),
                ymd_hms(1998, 7, 19, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_month_and_yearday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(4),
            bymonth: vec![4, 7],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![1, 100, 200, 365],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 4, 10, 9, 0, 0),
                ymd_hms(1998, 7, 19, 9, 0, 0),
                ymd_hms(1999, 4, 10, 9, 0, 0),
                ymd_hms(1999, 7, 19, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_weekno() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![20],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 5, 11, 9, 0, 0),
                ymd_hms(1998, 5, 12, 9, 0, 0),
                ymd_hms(1998, 5, 13, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_weekno_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![0],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![1],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 29, 9, 0, 0),
                ymd_hms(1999, 1, 4, 9, 0, 0),
                ymd_hms(2000, 1, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_weekno_and_weekday_large() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![6],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![52],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 28, 9, 0, 0),
                ymd_hms(1998, 12, 27, 9, 0, 0),
                ymd_hms(2000, 1, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_weekno_and_weekday_last() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![6],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![-1],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 28, 9, 0, 0),
                ymd_hms(1999, 1, 3, 9, 0, 0),
                ymd_hms(2000, 1, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_weekno_and_weekday53_last() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![0],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![53],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 12, 28, 9, 0, 0),
                ymd_hms(2004, 12, 27, 9, 0, 0),
                ymd_hms(2009, 12, 28, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_hour() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![9],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![6, 18],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 0, 0),
                ymd_hms(1998, 9, 2, 6, 0, 0),
                ymd_hms(1998, 9, 2, 18, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_minute() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![9],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![6, 18],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 6, 0),
                ymd_hms(1997, 9, 2, 9, 18, 0),
                ymd_hms(1998, 9, 2, 9, 6, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_second() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![9],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![6, 18],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 6),
                ymd_hms(1997, 9, 2, 9, 0, 18),
                ymd_hms(1998, 9, 2, 9, 0, 6),
            ],
        );
    }

    #[test]
    fn yearly_by_hour_and_minute() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![9],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![6, 18],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![6, 18],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 6, 0),
                ymd_hms(1997, 9, 2, 18, 18, 0),
                ymd_hms(1998, 9, 2, 6, 6, 0),
            ],
        );
    }

    #[test]
    fn yearly_by_hour_and_second() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![9],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![6, 18],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![6, 18],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 0, 6),
                ymd_hms(1997, 9, 2, 18, 0, 18),
                ymd_hms(1998, 9, 2, 6, 0, 6),
            ],
        );
    }

    #[test]
    fn yearly_by_minute_and_second() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![9],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![6, 18],
            bysecond: vec![6, 18],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 6, 6),
                ymd_hms(1997, 9, 2, 9, 6, 18),
                ymd_hms(1997, 9, 2, 9, 18, 6),
            ],
        );
    }

    #[test]
    fn yearly_by_hour_and_minute_and_second() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![9],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![6, 18],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![6, 18],
            bysecond: vec![6, 18],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 6, 6),
                ymd_hms(1997, 9, 2, 18, 6, 18),
                ymd_hms(1997, 9, 2, 18, 18, 6),
            ],
        );
    }

    #[test]
    fn yearly_by_setpos() {
        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![6, 18],
            bysetpos: vec![3, -3],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![15],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 11, 15, 18, 0, 0),
                ymd_hms(1998, 2, 15, 6, 0, 0),
                ymd_hms(1998, 11, 15, 18, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 10, 2, 9, 0, 0),
                ymd_hms(1997, 11, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_interval() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 2,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 11, 2, 9, 0, 0),
                ymd_hms(1998, 1, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_interval_large() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 18,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1999, 3, 2, 9, 0, 0),
                ymd_hms(2000, 9, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_month() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 2, 9, 0, 0),
                ymd_hms(1998, 3, 2, 9, 0, 0),
                ymd_hms(1999, 1, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_monthday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![1, 3],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 3, 9, 0, 0),
                ymd_hms(1997, 10, 1, 9, 0, 0),
                ymd_hms(1997, 10, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_month_and_monthday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![5, 7],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 5, 9, 0, 0),
                ymd_hms(1998, 1, 7, 9, 0, 0),
                ymd_hms(1998, 3, 5, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 9, 4, 9, 0, 0),
                ymd_hms(1997, 9, 9, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_nweekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![vec![1, 1], vec![3, -1]],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 9, 25, 9, 0, 0),
                ymd_hms(1997, 10, 7, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_nweekday_large() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![vec![1, 3], vec![3, -3]],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 11, 9, 0, 0),
                ymd_hms(1997, 9, 16, 9, 0, 0),
                ymd_hms(1997, 10, 16, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_month_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 1, 6, 9, 0, 0),
                ymd_hms(1998, 1, 8, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_month_and_nweekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![vec![1, 1], vec![3, -1]],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 6, 9, 0, 0),
                ymd_hms(1998, 1, 29, 9, 0, 0),
                ymd_hms(1998, 3, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_month_and_nweekday_large() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![vec![1, 3], vec![3, -3]],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 15, 9, 0, 0),
                ymd_hms(1998, 1, 20, 9, 0, 0),
                ymd_hms(1998, 3, 12, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_monthday_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![1, 3],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 2, 3, 9, 0, 0),
                ymd_hms(1998, 3, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_month_and_monthday_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![1, 3],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 3, 3, 9, 0, 0),
                ymd_hms(2001, 3, 1, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_yearday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(4),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![1, 100, 200, 365],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 31, 9, 0, 0),
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 4, 10, 9, 0, 0),
                ymd_hms(1998, 7, 19, 9, 0, 0),
            ],
        );
    }

    #[ignore = "negative year day not supported yet"]
    #[test]
    fn monthly_by_yearday_neg() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(4),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            //byyearday: vec![-365, -266, -166, -1],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 31, 9, 0, 0),
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 4, 10, 9, 0, 0),
                ymd_hms(1998, 7, 19, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_month_and_yearday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(4),
            bymonth: vec![4, 7],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![1, 100, 200, 365],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 4, 10, 9, 0, 0),
                ymd_hms(1998, 7, 19, 9, 0, 0),
                ymd_hms(1999, 4, 10, 9, 0, 0),
                ymd_hms(1999, 7, 19, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_weekno() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![20],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 5, 11, 9, 0, 0),
                ymd_hms(1998, 5, 12, 9, 0, 0),
                ymd_hms(1998, 5, 13, 9, 0, 0),
            ],
        );
    }

    // That's a nice one. The first days of week number one
    // may be in the last year.
    #[test]
    fn monthly_by_weekno_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![0],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![1],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 29, 9, 0, 0),
                ymd_hms(1999, 1, 4, 9, 0, 0),
                ymd_hms(2000, 1, 3, 9, 0, 0),
            ],
        );
    }

    // Another nice test. The last days of week number 52/53
    // may be in the next year.
    #[test]
    fn monthly_by_weekno_and_weekday_large() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![6],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![52],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 28, 9, 0, 0),
                ymd_hms(1998, 12, 27, 9, 0, 0),
                ymd_hms(2000, 1, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_weekno_and_weekday_last() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![6],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![-1],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 28, 9, 0, 0),
                ymd_hms(1999, 1, 3, 9, 0, 0),
                ymd_hms(2000, 1, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_weekno_and_weekday53() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![0],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![53],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 12, 28, 9, 0, 0),
                ymd_hms(2004, 12, 27, 9, 0, 0),
                ymd_hms(2009, 12, 28, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_hour() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![6, 18],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 0, 0),
                ymd_hms(1997, 10, 2, 6, 0, 0),
                ymd_hms(1997, 10, 2, 18, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_minute() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![6, 18],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 6, 0),
                ymd_hms(1997, 9, 2, 9, 18, 0),
                ymd_hms(1997, 10, 2, 9, 6, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_second() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![6, 18],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 6),
                ymd_hms(1997, 9, 2, 9, 0, 18),
                ymd_hms(1997, 10, 2, 9, 0, 6),
            ],
        );
    }

    #[test]
    fn monthly_by_hour_and_minute() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![6, 18],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![6, 18],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 6, 0),
                ymd_hms(1997, 9, 2, 18, 18, 0),
                ymd_hms(1997, 10, 2, 6, 6, 0),
            ],
        );
    }

    #[test]
    fn monthly_by_hour_and_second() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![6, 18],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![6, 18],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 0, 6),
                ymd_hms(1997, 9, 2, 18, 0, 18),
                ymd_hms(1997, 10, 2, 6, 0, 6),
            ],
        );
    }

    #[test]
    fn monthly_by_minute_and_second() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![6, 18],
            bysecond: vec![6, 18],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 6, 6),
                ymd_hms(1997, 9, 2, 9, 6, 18),
                ymd_hms(1997, 9, 2, 9, 18, 6),
            ],
        );
    }

    #[test]
    fn monthly_by_hour_and_minute_and_second() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![6, 18],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![6, 18],
            bysecond: vec![6, 18],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 6, 6),
                ymd_hms(1997, 9, 2, 18, 6, 18),
                ymd_hms(1997, 9, 2, 18, 18, 6),
            ],
        );
    }

    #[test]
    fn monthly_by_setpos() {
        let mut options = ParsedOptions {
            freq: Frequenzy::MONTHLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![6, 18],
            bysetpos: vec![3, -3],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![13, 17],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 13, 18, 0, 0),
                ymd_hms(1997, 9, 17, 6, 0, 0),
                ymd_hms(1997, 10, 13, 18, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 9, 9, 9, 0, 0),
                ymd_hms(1997, 9, 16, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_interval() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1],
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
            interval: 2,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 9, 16, 9, 0, 0),
                ymd_hms(1997, 9, 30, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_interval_large() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1],
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
            interval: 20,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1998, 1, 20, 9, 0, 0),
                ymd_hms(1998, 6, 9, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_month() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(6),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 6, 9, 0, 0),
                ymd_hms(1998, 1, 13, 9, 0, 0),
                ymd_hms(1998, 1, 20, 9, 0, 0),
                ymd_hms(1998, 1, 27, 9, 0, 0),
                ymd_hms(1998, 3, 3, 9, 0, 0),
                ymd_hms(1998, 3, 10, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_monthday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![1, 3],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 3, 9, 0, 0),
                ymd_hms(1997, 10, 1, 9, 0, 0),
                ymd_hms(1997, 10, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_month_and_monthday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![5, 7],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 5, 9, 0, 0),
                ymd_hms(1998, 1, 7, 9, 0, 0),
                ymd_hms(1998, 3, 5, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 9, 4, 9, 0, 0),
                ymd_hms(1997, 9, 9, 9, 0, 0),
            ],
        );
    }

    // ! why isnt this using nweekday ???
    #[test]
    fn weekly_by_nweekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 9, 4, 9, 0, 0),
                ymd_hms(1997, 9, 9, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_month_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 1, 6, 9, 0, 0),
                ymd_hms(1998, 1, 8, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_monthday_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![1, 3],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 2, 3, 9, 0, 0),
                ymd_hms(1998, 3, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_month_monthday_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![1, 3],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 3, 3, 9, 0, 0),
                ymd_hms(2001, 3, 1, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_yearday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(4),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![1, 100, 200, 365],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 31, 9, 0, 0),
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 4, 10, 9, 0, 0),
                ymd_hms(1998, 7, 19, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_month_yearday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(4),
            bymonth: vec![1, 7],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![1, 100, 200, 365],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 7, 19, 9, 0, 0),
                ymd_hms(1999, 1, 1, 9, 0, 0),
                ymd_hms(1999, 7, 19, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_weekno() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![20],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 5, 11, 9, 0, 0),
                ymd_hms(1998, 5, 12, 9, 0, 0),
                ymd_hms(1998, 5, 13, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_weekno_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![0],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![1],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 29, 9, 0, 0),
                ymd_hms(1999, 1, 4, 9, 0, 0),
                ymd_hms(2000, 1, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_weekno_and_weekday_large() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![6],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![52],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 28, 9, 0, 0),
                ymd_hms(1998, 12, 27, 9, 0, 0),
                ymd_hms(2000, 1, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_weekno_and_weekday_last() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![6],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![-1],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 28, 9, 0, 0),
                ymd_hms(1999, 1, 3, 9, 0, 0),
                ymd_hms(2000, 1, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_weekno_and_weekday53() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![0],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![53],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 12, 28, 9, 0, 0),
                ymd_hms(2004, 12, 27, 9, 0, 0),
                ymd_hms(2009, 12, 28, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_hour() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1],
            byhour: vec![6, 18],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 0, 0),
                ymd_hms(1997, 9, 9, 6, 0, 0),
                ymd_hms(1997, 9, 9, 18, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_minute() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![6, 18],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 6, 0),
                ymd_hms(1997, 9, 2, 9, 18, 0),
                ymd_hms(1997, 9, 9, 9, 6, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_second() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![6, 18],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 6),
                ymd_hms(1997, 9, 2, 9, 0, 18),
                ymd_hms(1997, 9, 9, 9, 0, 6),
            ],
        );
    }

    #[test]
    fn weekly_by_hour_and_minute() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1],
            byhour: vec![6, 18],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![6, 18],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 6, 0),
                ymd_hms(1997, 9, 2, 18, 18, 0),
                ymd_hms(1997, 9, 9, 6, 6, 0),
            ],
        );
    }

    #[test]
    fn weekly_by_hour_and_second() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1],
            byhour: vec![6, 18],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![6, 18],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 0, 6),
                ymd_hms(1997, 9, 2, 18, 0, 18),
                ymd_hms(1997, 9, 9, 6, 0, 6),
            ],
        );
    }

    #[test]
    fn weekly_by_minute_and_second() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![6, 18],
            bysecond: vec![6, 18],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 6, 6),
                ymd_hms(1997, 9, 2, 9, 6, 18),
                ymd_hms(1997, 9, 2, 9, 18, 6),
            ],
        );
    }

    #[test]
    fn weekly_by_hour_and_minute_and_second() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(5),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1],
            byhour: vec![6, 18],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![6, 18],
            bysecond: vec![6, 18],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 6, 6),
                ymd_hms(1997, 9, 2, 18, 6, 18),
                ymd_hms(1997, 9, 2, 18, 18, 6),
                ymd_hms(1997, 9, 2, 18, 18, 18),
                ymd_hms(1997, 9, 9, 6, 6, 6),
            ],
        );
    }

    #[test]
    fn weekly_by_setpos() {
        let mut options = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![6, 18],
            bysetpos: vec![3, -3],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 0, 0),
                ymd_hms(1997, 9, 4, 6, 0, 0),
                ymd_hms(1997, 9, 9, 18, 0, 0),
            ],
        );
    }

    #[test]
    fn daily() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 9, 3, 9, 0, 0),
                ymd_hms(1997, 9, 4, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_interval() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
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
            interval: 2,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 9, 4, 9, 0, 0),
                ymd_hms(1997, 9, 6, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_interval_large() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
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
            interval: 92,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 12, 3, 9, 0, 0),
                ymd_hms(1998, 3, 5, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_by_month() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 1, 2, 9, 0, 0),
                ymd_hms(1998, 1, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_by_monthday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![1, 3],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 3, 9, 0, 0),
                ymd_hms(1997, 10, 1, 9, 0, 0),
                ymd_hms(1997, 10, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_by_month_and_monthday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![5, 7],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 5, 9, 0, 0),
                ymd_hms(1998, 1, 7, 9, 0, 0),
                ymd_hms(1998, 3, 5, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_by_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 0),
                ymd_hms(1997, 9, 4, 9, 0, 0),
                ymd_hms(1997, 9, 9, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_by_month_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 1, 6, 9, 0, 0),
                ymd_hms(1998, 1, 8, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_by_monthday_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![1, 3],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 2, 3, 9, 0, 0),
                ymd_hms(1998, 3, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_by_month_and_monthday_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![1, 3],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![1, 3],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 3, 3, 9, 0, 0),
                ymd_hms(2001, 3, 1, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_by_yearday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(4),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![1, 100, 200, 365],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 31, 9, 0, 0),
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 4, 10, 9, 0, 0),
                ymd_hms(1998, 7, 19, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_by_month_and_yearday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(4),
            bymonth: vec![1, 7],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![1, 100, 200, 365],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 1, 1, 9, 0, 0),
                ymd_hms(1998, 7, 19, 9, 0, 0),
                ymd_hms(1999, 1, 1, 9, 0, 0),
                ymd_hms(1999, 7, 19, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_by_weekno() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![20],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 5, 11, 9, 0, 0),
                ymd_hms(1998, 5, 12, 9, 0, 0),
                ymd_hms(1998, 5, 13, 9, 0, 0),
            ],
        );
    }

    // That's a nice one. The first days of week number one
    // may be in the last year.
    #[test]
    fn daily_by_weekno_and_weekday() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![0],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![1],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 29, 9, 0, 0),
                ymd_hms(1999, 1, 4, 9, 0, 0),
                ymd_hms(2000, 1, 3, 9, 0, 0),
            ],
        );
    }

    // Another nice test. The last days of week number 52/53
    // may be in the next year.
    #[test]
    fn daily_by_weekno_and_weekday_large() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![6],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![52],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 28, 9, 0, 0),
                ymd_hms(1998, 12, 27, 9, 0, 0),
                ymd_hms(2000, 1, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_by_weekno_and_weekday_last() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![6],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![-1],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 12, 28, 9, 0, 0),
                ymd_hms(1999, 1, 3, 9, 0, 0),
                ymd_hms(2000, 1, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_by_weekno_and_weekday53() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![0],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![53],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1998, 12, 28, 9, 0, 0),
                ymd_hms(2004, 12, 27, 9, 0, 0),
                ymd_hms(2009, 12, 28, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_by_hour() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![6, 18],
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
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 0, 0),
                ymd_hms(1997, 9, 3, 6, 0, 0),
                ymd_hms(1997, 9, 3, 18, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_by_minute() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![6, 18],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 6, 0),
                ymd_hms(1997, 9, 2, 9, 18, 0),
                ymd_hms(1997, 9, 3, 9, 6, 0),
            ],
        );
    }

    #[test]
    fn daily_by_second() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![6, 18],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 0, 6),
                ymd_hms(1997, 9, 2, 9, 0, 18),
                ymd_hms(1997, 9, 3, 9, 0, 6),
            ],
        );
    }

    #[test]
    fn daily_by_hour_and_minute() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![6, 18],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![6, 18],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 6, 0),
                ymd_hms(1997, 9, 2, 18, 18, 0),
                ymd_hms(1997, 9, 3, 6, 6, 0),
            ],
        );
    }

    #[test]
    fn daily_by_hour_and_second() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![6, 18],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![6, 18],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 0, 6),
                ymd_hms(1997, 9, 2, 18, 0, 18),
                ymd_hms(1997, 9, 3, 6, 0, 6),
            ],
        );
    }

    #[test]
    fn daily_by_minute_and_second() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![6, 18],
            bysecond: vec![6, 18],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 9, 6, 6),
                ymd_hms(1997, 9, 2, 9, 6, 18),
                ymd_hms(1997, 9, 2, 9, 18, 6),
            ],
        );
    }

    #[test]
    fn daily_by_hour_and_minute_and_second() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![6, 18],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![6, 18],
            bysecond: vec![6, 18],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 6, 6),
                ymd_hms(1997, 9, 2, 18, 6, 18),
                ymd_hms(1997, 9, 2, 18, 18, 6),
            ],
        );
    }

    #[test]
    fn daily_by_setpos() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            count: Some(3),
            bymonth: vec![],
            dtstart: ymd_hms(1997, 9, 2, 9, 0, 0),
            byweekday: vec![],
            byhour: vec![6, 18],
            bysetpos: vec![3, -3],
            byweekno: vec![],
            byminute: vec![15, 45],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
        };
        test_recurring(
            &mut options,
            &vec![
                ymd_hms(1997, 9, 2, 18, 15, 0),
                ymd_hms(1997, 9, 3, 6, 45, 0),
                ymd_hms(1997, 9, 3, 18, 15, 0),
            ],
        );
    }
}
