#![allow(dead_code, unused_imports)]
use chrono::{DateTime, TimeZone, Weekday};
use chrono_tz::{Tz, UTC};
use rrule::{Frequency, RRule, RRuleSet};

/// This function can be used to test anything and can be changes as you wish.
pub fn run_debug_function() {
    test_from_string();
    // test_parsed_rrule();
}

fn test_from_string() {
    let rrule: RRuleSet = "DTSTART;TZID=America/New_York:19970519T090000\n\
    RRULE:FREQ=YEARLY;BYDAY=20MO"
        .parse()
        .unwrap();
    println!("RRule: {:#?}", rrule);
    let (list, err) = rrule.all_with_error(20);
    println!("Error: {:#?}", err);
    crate::print_all_datetimes(&list);
}

fn test_parsed_rrule() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(20),
        week_start: Weekday::Sun,
        by_month: vec![],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![],
        by_easter: Some(0),
        ..Default::default()
    };
    let rrule = rrule.build(ymd_hms(1997, 9, 2, 9, 0, 0)).unwrap();
    let (list, err) = rrule.all_with_error(50);

    println!("Error: {:#?}", err);
    crate::print_all_datetimes(&list);
}

fn ymd_hms(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> DateTime<Tz> {
    UTC.ymd(year, month, day).and_hms(hour, minute, second)
}
