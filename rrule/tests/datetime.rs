mod common;

use chrono::TimeZone;
use chrono_tz::UTC;
use rrule::{RRule, RRuleError};
use std::str::FromStr;

/// Check if datetime can be parsed correctly
#[test]
fn parse_datetime() {
    let rrule: RRule = "DTSTART:20120201T023000Z\nFREQ=DAILY;INTERVAL=1;COUNT=2"
        .parse()
        .expect("RRule could not be parsed");

    assert_eq!(
        rrule.all(50),
        vec![
            UTC.ymd(2012, 2, 1).and_hms(2, 30, 0),
            UTC.ymd(2012, 2, 2).and_hms(2, 30, 0)
        ]
    )
}

/// Check if datetime with timezone can be parsed correctly
#[test]
fn parse_datetime_with_timezone() {
    let rrule: RRule =
        "DTSTART;TZID=America/New_York:20120201T023000Z\nFREQ=DAILY;INTERVAL=1;COUNT=2"
            .parse()
            .expect("RRule could not be parsed");

    assert_eq!(
        rrule.all(50),
        vec![
            UTC.ymd(2012, 2, 1).and_hms(2, 30, 0),
            UTC.ymd(2012, 2, 2).and_hms(2, 30, 0)
        ]
    )
}

/// Check if datetime errors are correctly handled
#[test]
fn parse_datetime_errors_invalid_hour() {
    match RRule::from_str("DTSTART:20120201T323000Z\nFREQ=DAILY;INTERVAL=1;COUNT=2") {
        Ok(_rrule) => panic!("This time should not be accepted"),
        Err(err) => assert_eq!(
            err,
            RRuleError::new_parse_err("Invalid time in: `20120201T323000Z`")
        ),
    };
}

/// Check if datetime errors are correctly handled
#[test]
fn parse_datetime_errors_invalid_day() {
    match RRule::from_str("DTSTART:20120251T023000Z\nFREQ=DAILY;INTERVAL=1;COUNT=2") {
        Ok(_rrule) => panic!("This date should not be accepted"),
        Err(err) => assert_eq!(
            err,
            RRuleError::new_parse_err("Invalid date in: `20120251T023000Z`")
        ),
    };
}

/// Check if datetime errors are correctly handled
#[test]
fn parse_datetime_errors_invalid_timezone() {
    match RRule::from_str("DTSTART:20120251T023000T\nFREQ=DAILY;INTERVAL=1;COUNT=2") {
        Ok(_rrule) => panic!("This timezone should not be accepted"),
        Err(err) => assert_eq!(
            err,
            RRuleError::new_parse_err("Invalid datetime: `20120251T023000T`")
        ),
    };
}

/// Check if datetime errors are correctly handled
#[test]
fn parse_datetime_errors_invalid_tzid_timezone() {
    match RRule::from_str(
        "DTSTART;TZID=America/Everywhere:20120251T023000Z\nFREQ=DAILY;INTERVAL=1;COUNT=2",
    ) {
        Ok(_rrule) => panic!("This timezone should not be accepted"),
        Err(err) => assert_eq!(
            err,
            RRuleError::new_parse_err("Invalid timezone: `America/Everywhere`")
        ),
    };
}

/// Monthly on the 31st of the month
#[test]
fn monthly_on_31th() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=MONTHLY;COUNT=10;BYMONTHDAY=31"
        .parse::<RRule>()
        .unwrap()
        .all(20);
    // TODO: Is this the correct behavior?
    common::check_occurrences(
        &dates,
        &[
            "1997-10-31T09:00:00-05:00",
            "1997-12-31T09:00:00-05:00",
            "1998-01-31T09:00:00-05:00",
            "1998-03-31T09:00:00-05:00",
            "1998-05-31T09:00:00-04:00",
            "1998-07-31T09:00:00-04:00",
            "1998-08-31T09:00:00-04:00",
            "1998-10-31T09:00:00-05:00",
            "1998-12-31T09:00:00-05:00",
            "1999-01-31T09:00:00-05:00",
        ],
    )
}

/// Monthly on the 31th-to-last of the month
#[test]
fn monthly_on_31th_to_last() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=MONTHLY;COUNT=10;BYMONTHDAY=-31"
        .parse::<RRule>()
        .unwrap()
        .all(20);
    // TODO: Is this the correct behavior?
    common::check_occurrences(
        &dates,
        &[
            "1997-10-01T09:00:00-04:00",
            "1997-12-01T09:00:00-05:00",
            "1998-01-01T09:00:00-05:00",
            "1998-03-01T09:00:00-05:00",
            "1998-05-01T09:00:00-04:00",
            "1998-07-01T09:00:00-04:00",
            "1998-08-01T09:00:00-04:00",
            "1998-10-01T09:00:00-04:00",
            "1998-12-01T09:00:00-05:00",
            "1999-01-01T09:00:00-05:00",
        ],
    )
}
