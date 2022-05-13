//! This file contains examples from:
//! <https://icalendar.org/iCalendar-RFC-5545/3-8-5-3-recurrence-rule.html>

mod common;

use rrule::{DateFilter, RRuleSet};

// ------------------------------------------ Daily ---------------------------------------------

/// Daily for 10 occurrences
#[test]
fn daily_10_occurrences() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=DAILY;COUNT=10"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(50)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-02T09:00:00-04:00",
            "1997-09-03T09:00:00-04:00",
            "1997-09-04T09:00:00-04:00",
            "1997-09-05T09:00:00-04:00",
            "1997-09-06T09:00:00-04:00",
            "1997-09-07T09:00:00-04:00",
            "1997-09-08T09:00:00-04:00",
            "1997-09-09T09:00:00-04:00",
            "1997-09-10T09:00:00-04:00",
            "1997-09-11T09:00:00-04:00",
        ],
    )
}

/// Daily until November 03, 1997
#[test]
fn daily_until_november() {
    // This has been changes to make it shorter, so it is not too long.
    // From september-november
    let dates = "DTSTART;TZID=America/New_York:19970920T090000\n\
        RRULE:FREQ=DAILY;UNTIL=19971103T000000Z"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(60)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            // September
            "1997-09-20T09:00:00-04:00",
            "1997-09-21T09:00:00-04:00",
            "1997-09-22T09:00:00-04:00",
            "1997-09-23T09:00:00-04:00",
            "1997-09-24T09:00:00-04:00",
            "1997-09-25T09:00:00-04:00",
            "1997-09-26T09:00:00-04:00",
            "1997-09-27T09:00:00-04:00",
            "1997-09-28T09:00:00-04:00",
            "1997-09-29T09:00:00-04:00",
            "1997-09-30T09:00:00-04:00",
            // October
            "1997-10-01T09:00:00-04:00",
            "1997-10-02T09:00:00-04:00",
            "1997-10-03T09:00:00-04:00",
            "1997-10-04T09:00:00-04:00",
            "1997-10-05T09:00:00-04:00",
            "1997-10-06T09:00:00-04:00",
            "1997-10-07T09:00:00-04:00",
            "1997-10-08T09:00:00-04:00",
            "1997-10-09T09:00:00-04:00",
            "1997-10-10T09:00:00-04:00",
            "1997-10-11T09:00:00-04:00",
            "1997-10-12T09:00:00-04:00",
            "1997-10-13T09:00:00-04:00",
            "1997-10-14T09:00:00-04:00",
            "1997-10-15T09:00:00-04:00",
            "1997-10-16T09:00:00-04:00",
            "1997-10-17T09:00:00-04:00",
            "1997-10-18T09:00:00-04:00",
            "1997-10-19T09:00:00-04:00",
            "1997-10-20T09:00:00-04:00",
            "1997-10-21T09:00:00-04:00",
            "1997-10-22T09:00:00-04:00",
            "1997-10-23T09:00:00-04:00",
            "1997-10-24T09:00:00-04:00",
            "1997-10-25T09:00:00-04:00",
            // Daylight saving time change
            "1997-10-26T08:00:00-05:00", // TODO Hour should be `09:00` NOT `08:00`
            "1997-10-27T09:00:00-05:00",
            "1997-10-28T09:00:00-05:00",
            "1997-10-29T09:00:00-05:00",
            "1997-10-30T09:00:00-05:00",
            "1997-10-31T09:00:00-05:00",
            // November
            "1997-11-01T09:00:00-05:00",
            "1997-11-02T09:00:00-05:00",
        ],
    )
}

/// Every other day - forever
#[test]
fn every_other_day() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=DAILY;INTERVAL=2"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(32)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-02T09:00:00-04:00",
            "1997-09-04T09:00:00-04:00",
            "1997-09-06T09:00:00-04:00",
            "1997-09-08T09:00:00-04:00",
            "1997-09-10T09:00:00-04:00",
            "1997-09-12T09:00:00-04:00",
            "1997-09-14T09:00:00-04:00",
            "1997-09-16T09:00:00-04:00",
            "1997-09-18T09:00:00-04:00",
            "1997-09-20T09:00:00-04:00",
            "1997-09-22T09:00:00-04:00",
            "1997-09-24T09:00:00-04:00",
            "1997-09-26T09:00:00-04:00",
            "1997-09-28T09:00:00-04:00",
            "1997-09-30T09:00:00-04:00",
            "1997-10-02T09:00:00-04:00",
            "1997-10-04T09:00:00-04:00",
            "1997-10-06T09:00:00-04:00",
            "1997-10-08T09:00:00-04:00",
            "1997-10-10T09:00:00-04:00",
            "1997-10-12T09:00:00-04:00",
            "1997-10-14T09:00:00-04:00",
            "1997-10-16T09:00:00-04:00",
            "1997-10-18T09:00:00-04:00",
            "1997-10-20T09:00:00-04:00",
            "1997-10-22T09:00:00-04:00",
            "1997-10-24T09:00:00-04:00",
            "1997-10-26T08:00:00-05:00", // TODO Hour should be `09:00` NOT `08:00`
            "1997-10-28T09:00:00-05:00",
            "1997-10-30T09:00:00-05:00",
            "1997-11-01T09:00:00-05:00",
            "1997-11-03T09:00:00-05:00",
        ],
    )
}

/// Every 10 days, 5 occurrences
#[test]
fn every_10_days_5_occurrences() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=DAILY;INTERVAL=10;COUNT=5"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(50)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-02T09:00:00-04:00",
            "1997-09-12T09:00:00-04:00",
            "1997-09-22T09:00:00-04:00",
            "1997-10-02T09:00:00-04:00",
            "1997-10-12T09:00:00-04:00",
        ],
    )
}

/// Every day in January, for 3 years
#[test]
fn every_days_in_jan_for_3_years() {
    // To patterns that have same result
    let dates = "DTSTART;TZID=America/New_York:19980101T090000\n\
        RRULE:FREQ=YEARLY;UNTIL=20000131T140000Z;BYMONTH=1;BYDAY=SU,MO,TU,WE,TH,FR,SA"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(100)
        .unwrap();
    let dates_alt = "DTSTART;TZID=America/New_York:19980101T090000\n\
        RRULE:FREQ=DAILY;UNTIL=20000131T140000Z;BYMONTH=1"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(100)
        .unwrap();
    let mut expected = vec![];
    for year in 1998..=2000 {
        for day in 1..=31 {
            expected.push(format!("{}-01-{:02}T09:00:00-05:00", year, day));
        }
    }
    common::check_occurrences(&dates, &expected);
    common::check_occurrences(&dates_alt, &expected);
}

// ------------------------------------------ Weekly ---------------------------------------------

/// Weekly for 10 occurrences
#[test]
fn weekly_10_occurrences() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=WEEKLY;COUNT=10"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(50)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-02T09:00:00-04:00",
            "1997-09-09T09:00:00-04:00",
            "1997-09-16T09:00:00-04:00",
            "1997-09-23T09:00:00-04:00",
            "1997-09-30T09:00:00-04:00",
            "1997-10-07T09:00:00-04:00",
            "1997-10-14T09:00:00-04:00",
            "1997-10-21T09:00:00-04:00",
            "1997-10-28T09:00:00-05:00",
            "1997-11-04T09:00:00-05:00",
        ],
    )
}

/// Weekly until November 03, 1997
#[test]
fn weekly_until_november() {
    // This has been changes to make it shorter, so it is not too long.
    // From september-november
    let dates = "DTSTART;TZID=America/New_York:19970923T090000\n\
        RRULE:FREQ=WEEKLY;UNTIL=19971105T000000Z"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(60)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            // September
            "1997-09-23T09:00:00-04:00",
            "1997-09-30T09:00:00-04:00",
            // October
            "1997-10-07T09:00:00-04:00",
            "1997-10-14T09:00:00-04:00",
            "1997-10-21T09:00:00-04:00",
            // Daylight saving time change
            "1997-10-28T09:00:00-05:00",
            // November
            "1997-11-04T09:00:00-05:00",
        ],
    )
}

/// Every other week - forever
#[test]
fn every_other_week() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=WEEKLY;INTERVAL=2;WKST=SU"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(13)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-02T09:00:00-04:00",
            "1997-09-16T09:00:00-04:00",
            "1997-09-30T09:00:00-04:00",
            "1997-10-14T09:00:00-04:00",
            "1997-10-28T09:00:00-05:00",
            "1997-11-11T09:00:00-05:00",
            "1997-11-25T09:00:00-05:00",
            "1997-12-09T09:00:00-05:00",
            "1997-12-23T09:00:00-05:00",
            "1998-01-06T09:00:00-05:00",
            "1998-01-20T09:00:00-05:00",
            "1998-02-03T09:00:00-05:00",
            "1998-02-17T09:00:00-05:00",
        ],
    )
}

/// Weekly on Tuesday and Thursday for five weeks
#[test]
fn weekly_on_tue_and_thu_for_5_weeks() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=WEEKLY;UNTIL=19971007T000000Z;WKST=SU;BYDAY=TU,TH"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(50)
        .unwrap();
    let dates_alt = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=WEEKLY;COUNT=10;WKST=SU;BYDAY=TU,TH"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(50)
        .unwrap();
    let expected = vec![
        "1997-09-02T09:00:00-04:00",
        "1997-09-04T09:00:00-04:00",
        "1997-09-09T09:00:00-04:00",
        "1997-09-11T09:00:00-04:00",
        "1997-09-16T09:00:00-04:00",
        "1997-09-18T09:00:00-04:00",
        "1997-09-23T09:00:00-04:00",
        "1997-09-25T09:00:00-04:00",
        "1997-09-30T09:00:00-04:00",
        "1997-10-02T09:00:00-04:00",
    ];
    common::check_occurrences(&dates, &expected);
    common::check_occurrences(&dates_alt, &expected);
}

/// Every other week on Monday, Wednesday, and Friday until December 24, 1997,
/// starting on Monday, September 1, 1997
#[test]
fn every_other_week_some_days_until_dec() {
    let dates = "DTSTART;TZID=America/New_York:19970901T090000\n\
        RRULE:FREQ=WEEKLY;INTERVAL=2;UNTIL=19971224T000000Z;WKST=SU;BYDAY=MO,WE,FR"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(50)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-01T09:00:00-04:00",
            "1997-09-03T09:00:00-04:00",
            "1997-09-05T09:00:00-04:00",
            "1997-09-15T09:00:00-04:00",
            "1997-09-17T09:00:00-04:00",
            "1997-09-19T09:00:00-04:00",
            "1997-09-29T09:00:00-04:00",
            "1997-10-01T09:00:00-04:00",
            "1997-10-03T09:00:00-04:00",
            "1997-10-13T09:00:00-04:00",
            "1997-10-15T09:00:00-04:00",
            "1997-10-17T09:00:00-04:00",
            "1997-10-27T09:00:00-05:00",
            "1997-10-29T09:00:00-05:00",
            "1997-10-31T09:00:00-05:00",
            "1997-11-10T09:00:00-05:00",
            "1997-11-12T09:00:00-05:00",
            "1997-11-14T09:00:00-05:00",
            "1997-11-24T09:00:00-05:00",
            "1997-11-26T09:00:00-05:00",
            "1997-11-28T09:00:00-05:00",
            "1997-12-08T09:00:00-05:00",
            "1997-12-10T09:00:00-05:00",
            "1997-12-12T09:00:00-05:00",
            "1997-12-22T09:00:00-05:00",
        ],
    )
}

/// Every other week on Tuesday and Thursday, for 8 occurrences
#[test]
fn every_other_week_some_days_8_occurrences() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=WEEKLY;INTERVAL=2;COUNT=8;WKST=SU;BYDAY=TU,TH"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(50)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-02T09:00:00-04:00",
            "1997-09-04T09:00:00-04:00",
            "1997-09-16T09:00:00-04:00",
            "1997-09-18T09:00:00-04:00",
            "1997-09-30T09:00:00-04:00",
            "1997-10-02T09:00:00-04:00",
            "1997-10-14T09:00:00-04:00",
            "1997-10-16T09:00:00-04:00",
        ],
    )
}

// ------------------------------------------ Monthly ---------------------------------------------

/// Monthly on the first Friday for 10 occurrences
#[test]
fn monthly_on_first_friday_10_occurrences() {
    let dates = "DTSTART;TZID=America/New_York:19970905T090000\n\
        RRULE:FREQ=MONTHLY;COUNT=10;BYDAY=1FR"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(50)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-05T09:00:00-04:00",
            "1997-10-03T09:00:00-04:00",
            "1997-11-07T09:00:00-05:00",
            "1997-12-05T09:00:00-05:00",
            "1998-01-02T09:00:00-05:00",
            "1998-02-06T09:00:00-05:00",
            "1998-03-06T09:00:00-05:00",
            "1998-04-03T09:00:00-05:00",
            "1998-05-01T09:00:00-04:00",
            "1998-06-05T09:00:00-04:00",
        ],
    )
}

/// Monthly on the first Friday until December 24, 1997
#[test]
fn monthly_on_first_friday_until_dec() {
    let dates = "DTSTART;TZID=America/New_York:19970905T090000\n\
        RRULE:FREQ=MONTHLY;UNTIL=19971224T000000Z;BYDAY=1FR"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(50)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-05T09:00:00-04:00",
            "1997-10-03T09:00:00-04:00",
            "1997-11-07T09:00:00-05:00",
            "1997-12-05T09:00:00-05:00",
        ],
    )
}

/// Every other month on the first and last Sunday of the month for 10 occurrences
#[test]
fn every_other_month_on_first_and_last_sunday_10_occurrences() {
    let dates = "DTSTART;TZID=America/New_York:19970907T090000\n\
        RRULE:FREQ=MONTHLY;INTERVAL=2;COUNT=10;BYDAY=1SU,-1SU"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(50)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-07T09:00:00-04:00",
            "1997-09-28T09:00:00-04:00",
            "1997-11-02T09:00:00-05:00",
            "1997-11-30T09:00:00-05:00",
            "1998-01-04T09:00:00-05:00",
            "1998-01-25T09:00:00-05:00",
            "1998-03-01T09:00:00-05:00",
            "1998-03-29T09:00:00-05:00",
            "1998-05-03T09:00:00-04:00",
            "1998-05-31T09:00:00-04:00",
        ],
    )
}

/// Monthly on the second-to-last Monday of the month for 6 months
#[test]
fn monthly_on_second_to_last_monday_for_6_months() {
    let dates = "DTSTART;TZID=America/New_York:19970922T090000\n\
        RRULE:FREQ=MONTHLY;COUNT=6;BYDAY=-2MO"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(50)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-22T09:00:00-04:00",
            "1997-10-20T09:00:00-04:00",
            "1997-11-17T09:00:00-05:00",
            "1997-12-22T09:00:00-05:00",
            "1998-01-19T09:00:00-05:00",
            "1998-02-16T09:00:00-05:00",
        ],
    )
}

/// Monthly on the third-to-the-last day of the month, forever
#[test]
fn monthly_on_third_to_last_day_forever() {
    let dates = "DTSTART;TZID=America/New_York:19970928T090000\n\
        RRULE:FREQ=MONTHLY;BYMONTHDAY=-3"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(6)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-28T09:00:00-04:00",
            "1997-10-29T09:00:00-05:00",
            "1997-11-28T09:00:00-05:00",
            "1997-12-29T09:00:00-05:00",
            "1998-01-29T09:00:00-05:00",
            "1998-02-26T09:00:00-05:00",
        ],
    )
}

/// Monthly on the 2nd and 15th of the month for 10 occurrences
#[test]
fn monthly_on_2nd_and_15th_10_occurrences() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=MONTHLY;COUNT=10;BYMONTHDAY=2,15"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(20)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-02T09:00:00-04:00",
            "1997-09-15T09:00:00-04:00",
            "1997-10-02T09:00:00-04:00",
            "1997-10-15T09:00:00-04:00",
            "1997-11-02T09:00:00-05:00",
            "1997-11-15T09:00:00-05:00",
            "1997-12-02T09:00:00-05:00",
            "1997-12-15T09:00:00-05:00",
            "1998-01-02T09:00:00-05:00",
            "1998-01-15T09:00:00-05:00",
        ],
    )
}

/// Monthly on the first and last day of the month for 10 occurrences
#[test]
fn monthly_on_first_and_last_day_10_occurrences() {
    let dates = "DTSTART;TZID=America/New_York:19970930T090000\n\
        RRULE:FREQ=MONTHLY;COUNT=10;BYMONTHDAY=1,-1"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(20)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-30T09:00:00-04:00",
            "1997-10-01T09:00:00-04:00",
            "1997-10-31T09:00:00-05:00",
            "1997-11-01T09:00:00-05:00",
            "1997-11-30T09:00:00-05:00",
            "1997-12-01T09:00:00-05:00",
            "1997-12-31T09:00:00-05:00",
            "1998-01-01T09:00:00-05:00",
            "1998-01-31T09:00:00-05:00",
            "1998-02-01T09:00:00-05:00",
        ],
    )
}

/// Every 18 months on the 10th through 15th of the month for 10 occurrences
#[test]
fn every_18_months_10th_to_15th_10_occurrences() {
    let dates = "DTSTART;TZID=America/New_York:19970910T090000\n\
        RRULE:FREQ=MONTHLY;INTERVAL=18;COUNT=10;BYMONTHDAY=10,11,12,13,14,15"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(20)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-10T09:00:00-04:00",
            "1997-09-11T09:00:00-04:00",
            "1997-09-12T09:00:00-04:00",
            "1997-09-13T09:00:00-04:00",
            "1997-09-14T09:00:00-04:00",
            "1997-09-15T09:00:00-04:00",
            "1999-03-10T09:00:00-05:00",
            "1999-03-11T09:00:00-05:00",
            "1999-03-12T09:00:00-05:00",
            "1999-03-13T09:00:00-05:00",
        ],
    )
}

/// Every Tuesday, every other month
#[test]
fn every_tuesday_every_other_month() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=MONTHLY;INTERVAL=2;BYDAY=TU"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(18)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-02T09:00:00-04:00",
            "1997-09-09T09:00:00-04:00",
            "1997-09-16T09:00:00-04:00",
            "1997-09-23T09:00:00-04:00",
            "1997-09-30T09:00:00-04:00",
            "1997-11-04T09:00:00-05:00",
            "1997-11-11T09:00:00-05:00",
            "1997-11-18T09:00:00-05:00",
            "1997-11-25T09:00:00-05:00",
            "1998-01-06T09:00:00-05:00",
            "1998-01-13T09:00:00-05:00",
            "1998-01-20T09:00:00-05:00",
            "1998-01-27T09:00:00-05:00",
            "1998-03-03T09:00:00-05:00",
            "1998-03-10T09:00:00-05:00",
            "1998-03-17T09:00:00-05:00",
            "1998-03-24T09:00:00-05:00",
            "1998-03-31T09:00:00-05:00",
        ],
    )
}

// ------------------------------------------ Yearly ---------------------------------------------

/// Yearly in June and July for 10 occurrences
///
/// Note: Since none of the BYDAY, BYMONTHDAY, or BYYEARDAY components are specified,
/// the day is gotten from "DTSTART".
#[test]
fn yearly_in_june_and_july_for_10_occurrences() {
    let dates = "DTSTART;TZID=America/New_York:19970610T090000\n\
        RRULE:FREQ=YEARLY;COUNT=10;BYMONTH=6,7"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(20)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-06-10T09:00:00-04:00",
            "1997-07-10T09:00:00-04:00",
            "1998-06-10T09:00:00-04:00",
            "1998-07-10T09:00:00-04:00",
            "1999-06-10T09:00:00-04:00",
            "1999-07-10T09:00:00-04:00",
            "2000-06-10T09:00:00-04:00",
            "2000-07-10T09:00:00-04:00",
            "2001-06-10T09:00:00-04:00",
            "2001-07-10T09:00:00-04:00",
        ],
    )
}

/// Every other year on January, February, and March for 10 occurrences
#[test]
fn every_other_year_on_jan_feb_and_march_for_10_occurrences() {
    let dates = "DTSTART;TZID=America/New_York:19970310T090000\n\
        RRULE:FREQ=YEARLY;INTERVAL=2;COUNT=10;BYMONTH=1,2,3"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(20)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-03-10T09:00:00-05:00",
            "1999-01-10T09:00:00-05:00",
            "1999-02-10T09:00:00-05:00",
            "1999-03-10T09:00:00-05:00",
            "2001-01-10T09:00:00-05:00",
            "2001-02-10T09:00:00-05:00",
            "2001-03-10T09:00:00-05:00",
            "2003-01-10T09:00:00-05:00",
            "2003-02-10T09:00:00-05:00",
            "2003-03-10T09:00:00-05:00",
        ],
    )
}

/// Every third year on the 1st, 100th, and 200th day for 10 occurrences
#[test]
fn every_third_year_on_1st_100th_and_200th_day_for_10_occurrences() {
    let dates = "DTSTART;TZID=America/New_York:19970101T090000\n\
        RRULE:FREQ=YEARLY;INTERVAL=3;COUNT=10;BYYEARDAY=1,100,200"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(20)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-01-01T09:00:00-05:00",
            "1997-04-10T09:00:00-04:00",
            "1997-07-19T09:00:00-04:00",
            "2000-01-01T09:00:00-05:00",
            "2000-04-09T09:00:00-04:00",
            "2000-07-18T09:00:00-04:00",
            "2003-01-01T09:00:00-05:00",
            "2003-04-10T09:00:00-04:00",
            "2003-07-19T09:00:00-04:00",
            "2006-01-01T09:00:00-05:00",
        ],
    )
}

/// Every 20th Monday of the year, forever
#[test]
fn every_20th_monday_of_year_forever() {
    let dates = "DTSTART;TZID=America/New_York:19970519T090000\n\
        RRULE:FREQ=YEARLY;BYDAY=20MO"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(3)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-05-19T09:00:00-04:00",
            "1998-05-18T09:00:00-04:00",
            "1999-05-17T09:00:00-04:00",
        ],
    )
}

/// Monday of week number 20 (where the default start of the week is Monday), forever
#[test]
fn monday_of_week_20_forever() {
    let dates = "DTSTART;TZID=America/New_York:19970512T090000\n\
        RRULE:FREQ=YEARLY;BYWEEKNO=20;BYDAY=MO"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(3)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-05-12T09:00:00-04:00",
            "1998-05-11T09:00:00-04:00",
            "1999-05-17T09:00:00-04:00",
        ],
    )
}

/// Every Thursday in March, forever
#[test]
fn every_thursday_in_march_forever() {
    let dates = "DTSTART;TZID=America/New_York:19970313T090000\n\
        RRULE:FREQ=YEARLY;BYMONTH=3;BYDAY=TH"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(11)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-03-13T09:00:00-05:00",
            "1997-03-20T09:00:00-05:00",
            "1997-03-27T09:00:00-05:00",
            "1998-03-05T09:00:00-05:00",
            "1998-03-12T09:00:00-05:00",
            "1998-03-19T09:00:00-05:00",
            "1998-03-26T09:00:00-05:00",
            "1999-03-04T09:00:00-05:00",
            "1999-03-11T09:00:00-05:00",
            "1999-03-18T09:00:00-05:00",
            "1999-03-25T09:00:00-05:00",
        ],
    )
}

/// Every Thursday, but only during June, July, and August, forever
#[test]
fn every_thursday_only_during_june_july_and_august_forever() {
    let dates = "DTSTART;TZID=America/New_York:19970605T090000\n\
        RRULE:FREQ=YEARLY;BYDAY=TH;BYMONTH=6,7,8"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(39)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-06-05T09:00:00-04:00",
            "1997-06-12T09:00:00-04:00",
            "1997-06-19T09:00:00-04:00",
            "1997-06-26T09:00:00-04:00",
            "1997-07-03T09:00:00-04:00",
            "1997-07-10T09:00:00-04:00",
            "1997-07-17T09:00:00-04:00",
            "1997-07-24T09:00:00-04:00",
            "1997-07-31T09:00:00-04:00",
            "1997-08-07T09:00:00-04:00",
            "1997-08-14T09:00:00-04:00",
            "1997-08-21T09:00:00-04:00",
            "1997-08-28T09:00:00-04:00",
            "1998-06-04T09:00:00-04:00",
            "1998-06-11T09:00:00-04:00",
            "1998-06-18T09:00:00-04:00",
            "1998-06-25T09:00:00-04:00",
            "1998-07-02T09:00:00-04:00",
            "1998-07-09T09:00:00-04:00",
            "1998-07-16T09:00:00-04:00",
            "1998-07-23T09:00:00-04:00",
            "1998-07-30T09:00:00-04:00",
            "1998-08-06T09:00:00-04:00",
            "1998-08-13T09:00:00-04:00",
            "1998-08-20T09:00:00-04:00",
            "1998-08-27T09:00:00-04:00",
            "1999-06-03T09:00:00-04:00",
            "1999-06-10T09:00:00-04:00",
            "1999-06-17T09:00:00-04:00",
            "1999-06-24T09:00:00-04:00",
            "1999-07-01T09:00:00-04:00",
            "1999-07-08T09:00:00-04:00",
            "1999-07-15T09:00:00-04:00",
            "1999-07-22T09:00:00-04:00",
            "1999-07-29T09:00:00-04:00",
            "1999-08-05T09:00:00-04:00",
            "1999-08-12T09:00:00-04:00",
            "1999-08-19T09:00:00-04:00",
            "1999-08-26T09:00:00-04:00",
        ],
    )
}

/// Every Friday the 13th, forever
#[test]
fn every_friday_the_13th_forever() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        EXDATE;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=MONTHLY;BYDAY=FR;BYMONTHDAY=13"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(5)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1998-02-13T09:00:00-05:00",
            "1998-03-13T09:00:00-05:00",
            "1998-11-13T09:00:00-05:00",
            "1999-08-13T09:00:00-04:00",
            "2000-10-13T09:00:00-04:00",
        ],
    )
}

/// The first Saturday that follows the first Sunday of the month, forever
#[test]
fn first_sat_follows_first_sunday_of_month_forever() {
    let dates = "DTSTART;TZID=America/New_York:19970913T090000\n\
        RRULE:FREQ=MONTHLY;BYDAY=SA;BYMONTHDAY=7,8,9,10,11,12,13"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(10)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-13T09:00:00-04:00",
            "1997-10-11T09:00:00-04:00",
            "1997-11-08T09:00:00-05:00",
            "1997-12-13T09:00:00-05:00",
            "1998-01-10T09:00:00-05:00",
            "1998-02-07T09:00:00-05:00",
            "1998-03-07T09:00:00-05:00",
            "1998-04-11T09:00:00-04:00",
            "1998-05-09T09:00:00-04:00",
            "1998-06-13T09:00:00-04:00",
        ],
    )
}

/// Every 4 years, the first Tuesday after a Monday in November, forever
/// (U.S. Presidential Election day)
#[test]
fn every_4_years_us_presidential_election_day_forever() {
    let dates = "DTSTART;TZID=America/New_York:19961105T090000\n\
        RRULE:FREQ=YEARLY;INTERVAL=4;BYMONTH=11;BYDAY=TU;BYMONTHDAY=2,3,4,5,6,7,8"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(3)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1996-11-05T09:00:00-05:00",
            "2000-11-07T09:00:00-05:00",
            "2004-11-02T09:00:00-05:00",
        ],
    )
}

/// The third instance into the month of one of Tuesday, Wednesday, or Thursday,
/// for the next 3 months
#[test]
fn every_third_instance_of_weekday_in_month_for_3_months() {
    let dates = "DTSTART;TZID=America/New_York:19970904T090000\n\
        RRULE:FREQ=MONTHLY;COUNT=3;BYDAY=TU,WE,TH;BYSETPOS=3"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(10)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-04T09:00:00-04:00",
            "1997-10-07T09:00:00-04:00",
            "1997-11-06T09:00:00-05:00",
        ],
    )
}

/// The second-to-last weekday of the month
#[test]
fn second_to_last_weekday_of_month() {
    let dates = "DTSTART;TZID=America/New_York:19970929T090000\n\
        RRULE:FREQ=MONTHLY;BYDAY=MO,TU,WE,TH,FR;BYSETPOS=-2"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(7)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-29T09:00:00-04:00",
            "1997-10-30T09:00:00-05:00",
            "1997-11-27T09:00:00-05:00",
            "1997-12-30T09:00:00-05:00",
            "1998-01-29T09:00:00-05:00",
            "1998-02-26T09:00:00-05:00",
            "1998-03-30T09:00:00-05:00",
        ],
    )
}

/// Every 3 hours from 9:00 AM to 5:00 PM on a specific day
#[test]
fn every_3_hours_on_specific_day() {
    // https://www.rfc-editor.org/errata/eid3883
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=HOURLY;INTERVAL=3;UNTIL=19970902T210000Z"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(10)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-02T09:00:00-04:00",
            "1997-09-02T12:00:00-04:00",
            "1997-09-02T15:00:00-04:00", // Missing from result
        ],
    )
}

/// Every 15 minutes for 6 occurrences
#[test]
fn every_15_min_for_6_occurrences() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=MINUTELY;INTERVAL=15;COUNT=6"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(10)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-02T09:00:00-04:00",
            "1997-09-02T09:15:00-04:00",
            "1997-09-02T09:30:00-04:00",
            "1997-09-02T09:45:00-04:00",
            "1997-09-02T10:00:00-04:00",
            "1997-09-02T10:15:00-04:00",
        ],
    )
}

/// Every hour and a half for 4 occurrences
#[test]
fn every_hour_and_a_half_for_4_occurrences() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=MINUTELY;INTERVAL=90;COUNT=4"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(10)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-09-02T09:00:00-04:00",
            "1997-09-02T10:30:00-04:00",
            "1997-09-02T12:00:00-04:00",
            "1997-09-02T13:30:00-04:00",
        ],
    )
}

/// Every 20 minutes from 9:00 AM to 4:40 PM every day
#[test]
fn every_20_min_at_time_every_day() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=DAILY;BYHOUR=9,10,11,12,13,14,15,16;BYMINUTE=0,20,40"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(72)
        .unwrap();
    let dates_alt = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=MINUTELY;INTERVAL=20;BYHOUR=9,10,11,12,13,14,15,16"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(72)
        .unwrap();

    let mut expected = vec![];
    for day in 2..=4 {
        expected.append(&mut vec![
            format!("1997-09-{:02}T09:00:00-04:00", day),
            format!("1997-09-{:02}T09:20:00-04:00", day),
            format!("1997-09-{:02}T09:40:00-04:00", day),
            format!("1997-09-{:02}T10:00:00-04:00", day),
            format!("1997-09-{:02}T10:20:00-04:00", day),
            format!("1997-09-{:02}T10:40:00-04:00", day),
            format!("1997-09-{:02}T11:00:00-04:00", day),
            format!("1997-09-{:02}T11:20:00-04:00", day),
            format!("1997-09-{:02}T11:40:00-04:00", day),
            format!("1997-09-{:02}T12:00:00-04:00", day),
            format!("1997-09-{:02}T12:20:00-04:00", day),
            format!("1997-09-{:02}T12:40:00-04:00", day),
            format!("1997-09-{:02}T13:00:00-04:00", day),
            format!("1997-09-{:02}T13:20:00-04:00", day),
            format!("1997-09-{:02}T13:40:00-04:00", day),
            format!("1997-09-{:02}T14:00:00-04:00", day),
            format!("1997-09-{:02}T14:20:00-04:00", day),
            format!("1997-09-{:02}T14:40:00-04:00", day),
            format!("1997-09-{:02}T15:00:00-04:00", day),
            format!("1997-09-{:02}T15:20:00-04:00", day),
            format!("1997-09-{:02}T15:40:00-04:00", day),
            format!("1997-09-{:02}T16:00:00-04:00", day),
            format!("1997-09-{:02}T16:20:00-04:00", day),
            format!("1997-09-{:02}T16:40:00-04:00", day),
        ]);
    }
    common::check_occurrences(&dates, &expected);
    common::check_occurrences(&dates_alt, &expected);
}

/// An example where the days generated makes a difference because of `WKST` (week start)
#[test]
fn week_day_start_monday_generated_days() {
    let dates = "DTSTART;TZID=America/New_York:19970805T090000\n\
        RRULE:FREQ=WEEKLY;INTERVAL=2;COUNT=4;BYDAY=TU,SU;WKST=MO"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(10)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-08-05T09:00:00-04:00",
            "1997-08-10T09:00:00-04:00",
            "1997-08-19T09:00:00-04:00",
            "1997-08-24T09:00:00-04:00",
        ],
    )
}

/// Changing only WKST from MO to SU, yields different results...
#[test]
fn week_day_start_sunday_generated_days() {
    let dates = "DTSTART;TZID=America/New_York:19970805T090000\n\
        RRULE:FREQ=WEEKLY;INTERVAL=2;COUNT=4;BYDAY=TU,SU;WKST=SU"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(10)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "1997-08-05T09:00:00-04:00",
            "1997-08-17T09:00:00-04:00",
            "1997-08-19T09:00:00-04:00",
            "1997-08-31T09:00:00-04:00",
        ],
    )
}

/// An example where an invalid date (i.e., February 30) is ignored.
#[test]
fn invalid_date_is_ignored() {
    let dates = "DTSTART;TZID=America/New_York:20070115T090000\n\
        RRULE:FREQ=MONTHLY;BYMONTHDAY=15,30;COUNT=5"
        .parse::<RRuleSet>()
        .unwrap()
        .into_iter()
        .all(10)
        .unwrap();
    common::check_occurrences(
        &dates,
        &[
            "2007-01-15T09:00:00-05:00",
            "2007-01-30T09:00:00-05:00",
            "2007-02-15T09:00:00-05:00",
            "2007-03-15T09:00:00-04:00",
            "2007-03-30T09:00:00-04:00",
        ],
    )
}
