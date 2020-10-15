extern crate chrono;
extern crate rust_ical;

use chrono::prelude::*;
use rust_ical::iter::*;
use rust_ical::options::*;
use rust_ical::yearinfo::*;

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
}
