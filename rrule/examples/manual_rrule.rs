//! # Manual [`RRule`]
//!
//! Create an [`RRule`] object.

use chrono::{Datelike, TimeZone, Timelike};
use rrule::{Frequency, RRule, Tz};

fn main() {
    // Build an RRuleSet that starts first day in 2020 at 9:00AM and occurs daily 5 times
    let start_date = Tz::utc().ymd(2020, 1, 1).and_hms(9, 0, 0);
    let rrule_set = RRule::default()
        .count(5)
        .freq(Frequency::Daily)
        .build(start_date)
        .expect("RRule invalid");

    let recurrences = rrule_set.all_unchecked();
    for (i, rec) in recurrences.iter().enumerate() {
        assert_eq!(rec.year(), 2020);
        assert_eq!(rec.month(), 1);
        assert_eq!(rec.day(), 1 + i as u32);
        assert_eq!(rec.hour(), 9);
    }
    assert_eq!(recurrences.len(), 5);
    println!("Done, everything worked.");
}
