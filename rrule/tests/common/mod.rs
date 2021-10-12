#![cfg(test)]
#![allow(dead_code)]

use chrono::{DateTime, TimeZone};
use chrono_tz::{Tz, UTC};
use rrule::{RRule, RRuleProperties, RRuleSet};
use std::fmt::Debug;

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

pub fn test_recurring_rrule(options: RRuleProperties, expected_dates: &Vec<DateTime<Tz>>) {
    let rrule = RRule::new(options).unwrap();
    let res = rrule.all(100);

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

pub fn test_recurring_rrule_set(rrule_set: RRuleSet, expected_dates: &Vec<DateTime<Tz>>) {
    let res = rrule_set.all(100);

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

/// Print and compare 2 lists of dates and panic it they are not the same.
pub fn check_occurrences<S: AsRef<str> + Debug>(occurrences: &[DateTime<Tz>], expected: &[S]) {
    let formatter = |dt: &DateTime<Tz>| -> String { format!("    \"{}\",\n", dt.to_rfc3339()) };
    println!(
        "Given: [\n{}]\nExpected: {:#?}",
        occurrences
            .iter()
            .map(formatter)
            .collect::<Vec<_>>()
            .join(""),
        expected
    );
    assert_eq!(occurrences.len(), expected.len(), "List sizes don't match");
    for (given, expected) in occurrences.iter().zip(expected.iter()) {
        let exp_datetime = chrono::DateTime::parse_from_rfc3339(expected.as_ref()).unwrap();
        // Compair items and check if in the same offset/timezone
        assert_eq!(
            given.to_rfc3339(),
            exp_datetime.to_rfc3339(),
            "Dates not in same timezone"
        );
    }
}
