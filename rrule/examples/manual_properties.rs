//! # Manual RRuleProperties
//!
//! Create an `RRuleProperties` object to create a `RRule`.

use chrono::{Datelike, TimeZone, Timelike};
use chrono_tz::UTC;
use rrule::{Frequency, RRule, RRuleProperties};

fn main() {
    // Build properties that starts first day in 2020 at 9:00AM and occurs daily 5 times
    let properties = RRuleProperties::default()
        .dt_start(UTC.ymd(2020, 1, 1).and_hms(9, 0, 0))
        .count(5)
        .freq(Frequency::Daily);

    // Construct `RRule` from properties
    let rrule = RRule::new(properties).expect("RRule invalid");
    let recurrences = rrule.all(100);
    for (i, rec) in rrule.all(100).iter().enumerate().take(5) {
        assert_eq!(rec.year(), 2020);
        assert_eq!(rec.month(), 1);
        assert_eq!(rec.day(), 1 + i as u32);
        assert_eq!(rec.hour(), 9);
    }
    assert_eq!(recurrences.len(), 5);
    println!("Done, everything worked.");
}
