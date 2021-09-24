mod common;

use chrono::TimeZone;
use chrono_tz::UTC;
use rrule::RRule;
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
            UTC.ymd(2012, 2, 01).and_hms(2, 30, 0),
            UTC.ymd(2012, 2, 02).and_hms(2, 30, 0)
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
            UTC.ymd(2012, 2, 01).and_hms(2, 30, 0),
            UTC.ymd(2012, 2, 02).and_hms(2, 30, 0)
        ]
    )
}

/// Check if datetime errors are correctly handled
#[test]
fn parse_datetime_errors_invalid_hour() {
    match RRule::from_str("DTSTART:20120201T323000Z\nFREQ=DAILY;INTERVAL=1;COUNT=2") {
        Ok(_rrule) => panic!("This time should not be accepted"),
        Err(err) => assert_eq!(
            err.to_string(),
            "Encountered parsing error: Invalid time in: `20120201T323000Z`".to_owned()
        ),
    };
}

/// Check if datetime errors are correctly handled
#[test]
fn parse_datetime_errors_invalid_day() {
    match RRule::from_str("DTSTART:20120251T023000Z\nFREQ=DAILY;INTERVAL=1;COUNT=2") {
        Ok(_rrule) => panic!("This date should not be accepted"),
        Err(err) => assert_eq!(
            err.to_string(),
            "Encountered parsing error: Invalid date in: `20120251T023000Z`".to_owned()
        ),
    };
}

/// Check if datetime errors are correctly handled
#[test]
fn parse_datetime_errors_invalid_timezone() {
    match RRule::from_str("DTSTART:20120251T023000T\nFREQ=DAILY;INTERVAL=1;COUNT=2") {
        Ok(_rrule) => panic!("This timezone should not be accepted"),
        Err(err) => assert_eq!(
            err.to_string(),
            "Encountered parsing error: Invalid datetime: `20120251T023000T`".to_owned()
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
            err.to_string(),
            "Encountered parsing error: Invalid timezone: `America/Everywhere`".to_owned()
        ),
    };
}
