extern crate chrono;
extern crate rust_ical;

use chrono::prelude::*;
use chrono::*;
use rust_ical::iter::*;
use rust_ical::yearinfo::*;

#[cfg(test)]
mod test {
    use super::*;

    fn test_recurring(msg: &str, options: &ParsedOptions, expected_dates: &DateTime<Utc>) {
        assert_eq!(2, 2, "{}", msg);
    }

    #[test]
    fn int_works() {
        let iter_args = IterArgs {
            inc: true,
            before: Utc::now(),
            after: Utc::now(),
            dt: Utc::now(),
            _value: Some(vec![]),
        };
        let mut iter_res = IterResult::new(QueryMethodTypes::ALL, iter_args);
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
            count: Some(5),
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
        let res = iter(&mut iter_res, &mut options);
        assert!(false);
        println!("Res: {:?}", res);
    }
}
