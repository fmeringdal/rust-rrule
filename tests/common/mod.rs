#![cfg(test)]
#![allow(dead_code)]

use chrono::{DateTime, TimeZone};
use chrono_tz::{Tz, UTC};
use rrule::{ParsedOptions, RRule};

pub fn ymd_hms(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> DateTime<Tz> {
    UTC.ymd(year, month, day).and_hms(hour, minute, second)
}

pub fn test_recurring(options: ParsedOptions, expected_dates: &Vec<DateTime<Tz>>) {
    let rrule = RRule::new(options);
    let res = rrule.all();

    println!("Actual: {:?}", res);
    println!("Expected: {:?}", expected_dates);
    assert_eq!(
        res.len(),
        expected_dates.len(),
        "Expected number of returned dates to be equal to the expected"
    );

    for (actual, exptected) in res.iter().zip(expected_dates) {
        assert_eq!(actual, exptected);
    }
}
