extern crate chrono;
extern crate rust_ical;

use chrono::prelude::*;
use rust_ical::iter::*;
use rust_ical::yearinfo::*;

#[cfg(test)]
mod test {
    use super::*;

    fn test_recurring(msg: &str, options: &mut ParsedOptions, expected_dates: &Vec<DateTime<Utc>>) {
        let iter_args = IterArgs {
            inc: true,
            before: Utc::now(),
            after: Utc::now(),
            dt: Utc::now(),
            _value: Some(vec![]),
        };
        let mut iter_res = IterResult::new(QueryMethodTypes::ALL, iter_args);
        let res = iter(&mut iter_res, options);
        for (actual, exptected) in res.iter().zip(expected_dates) {
            assert_eq!(actual, exptected, "{}", msg);
        }
    }

    #[test]
    fn int_works() {
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            dtstart: Utc.ymd(2012, 1, 1).and_hms(10, 30, 0),
            until: Some(Utc.ymd(2012, 12, 31).and_hms(10, 30, 0)),
            tzid: None,
            interval: 1,
            wkst: 0,
            count: None,
            bysecond: vec![0],
            byminute: vec![30],
            byhour: vec![10],
            bymonth: vec![],
            bymonthday: vec![],
            bysetpos: vec![],
            byweekno: vec![],
            byyearday: vec![],
            byweekday: vec![0, 1, 2, 3, 4, 5, 6],
            bynweekday: vec![],
            bynmonthday: vec![],
        };
        let mut options_2 = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            dtstart: Utc.ymd(2012, 1, 1).and_hms(10, 30, 0),
            until: None,
            //until: Some(Utc.ymd(2012, 12, 31).and_hms(10, 30, 0)),
            tzid: None,
            interval: 5,
            wkst: 0,
            count: Some(3),
            bysecond: vec![0],
            byminute: vec![30],
            byhour: vec![10],
            bymonth: vec![6],
            bymonthday: vec![],
            bysetpos: vec![],
            byweekno: vec![],
            byyearday: vec![],
            byweekday: vec![0, 4],
            bynweekday: vec![],
            bynmonthday: vec![],
        };
        test_recurring(
            "should work",
            &mut options_2,
            &vec![Utc.ymd(2020, 0, 0).and_hms(0, 0, 0)],
        );
    }
}
