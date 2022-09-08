#![allow(dead_code, unused_imports)]

use chrono::{DateTime, TimeZone, Weekday};
use rrule::{Frequency, RRule, RRuleSet, Tz};

/// This function can be used to test anything and can be changes as you wish.
pub fn run_debug_function() {
    test_from_string();
    // test_parsed_rrule();
}

fn test_from_string() {
    let rrule: RRuleSet<_> = "DTSTART;TZID=America/New_York:19970519T090000\n\
    RRULE:FREQ=YEARLY;BYDAY=20MO"
        .parse()
        .unwrap();
    println!("RRule: {:#?}", rrule);
    let result = rrule.all(20);
    println!("Limited: {}", result.limited);
    crate::print_all_datetimes(&result.dates);
}

fn test_parsed_rrule() {
    let properties = RRule::new(Frequency::Daily)
        .count(20)
        .week_start(Weekday::Sun)
        .by_hour(vec![9])
        .by_minute(vec![0])
        .by_second(vec![0]);

    let rrule = properties.build(ymd_hms(1997, 9, 2, 9, 0, 0)).unwrap();
    let result = rrule.all(50);

    println!("Limited: {}", result.limited);
    crate::print_all_datetimes(&result.dates);
}

fn ymd_hms(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> DateTime<Tz> {
    Tz::UTC.ymd(year, month, day).and_hms(hour, minute, second)
}
