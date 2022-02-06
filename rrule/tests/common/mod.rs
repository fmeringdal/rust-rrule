#![cfg(test)]
#![allow(dead_code)]

use chrono::{DateTime, TimeZone};
use chrono_tz::{Tz, UTC};
use rrule::{DateFilter, RRuleProperties, RRuleSet};
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

pub fn test_recurring_rrule(
    properties: RRuleProperties,
    dt_start: DateTime<Tz>,
    expected_dates: &[DateTime<Tz>],
) {
    let rrule = properties.build(dt_start).unwrap();
    let res = rrule.all(100).unwrap();

    println!("Actual: {:?}", res);
    println!("Expected: {:?}", expected_dates);
    assert_eq!(
        res.len(),
        expected_dates.len(),
        "Expected number of returned dates to be equal to the expected"
    );

    for (actual, expected) in res.iter().zip(expected_dates) {
        assert_eq!(actual, expected);
    }
}

pub fn test_recurring_rrule_set(rrule_set: RRuleSet, expected_dates: &[DateTime<Tz>]) {
    let res = rrule_set.all(100).unwrap();

    println!("Actual: {:?}", res);
    println!("Expected: {:?}", expected_dates);
    assert_eq!(
        res.len(),
        expected_dates.len(),
        "Expected number of returned dates to be equal to the expected"
    );

    for (actual, expected) in res.iter().zip(expected_dates) {
        assert_eq!(actual, expected);
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
        // Compare items and check if in the same offset/timezone
        assert_eq!(
            given.to_rfc3339(),
            exp_datetime.to_rfc3339(),
            "Dates not in same timezone"
        );
    }
}
