//! # Manual RRuleProperties
//!
//! Create an `RRuleProperties` object to create a `RRule`.

use chrono::{Datelike, TimeZone, Timelike};
use chrono_tz::UTC;
use rrule::{Frequency, RRule};

fn main() {
    // Build properties that starts first day in 2020 at 9:00AM and occurs daily 5 times
    let properties = RRule::default().count(5).freq(Frequency::Daily);

    // Construct `RRuleSet` from properties
    let rrule_set = properties
        .build(UTC.ymd(2020, 1, 1).and_hms(9, 0, 0))
        .expect("RRule invalid");

    let recurrences = rrule_set.all(10).expect("Error found during iterations.");
    for (i, rec) in recurrences.iter().enumerate() {
        assert_eq!(rec.year(), 2020);
        assert_eq!(rec.month(), 1);
        assert_eq!(rec.day(), 1 + i as u32);
        assert_eq!(rec.hour(), 9);
    }
    assert_eq!(recurrences.len(), 5);
    println!("Done, everything worked.");
}
