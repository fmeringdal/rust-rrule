//! # Manual [`rrule::RRuleSet`]
//!
//! Create an [`rrule::RRuleSet`] object manually.

use chrono::{Datelike, TimeZone};
use chrono_tz::UTC;
use rrule::{Frequency, NWeekday, RRule, Weekday};

/// ## Construct [`rrule::RRuleSet`] from one `rrule` and `exrule`
/// The rrule will occur weekly on Tuesday and Wednesday and the exrule
/// will occur weekly on Wednesday, and therefore the end result will contain
/// weekly recurrences on Wednesday only.
fn main() {
    // Build rrule set that occurs weekly on Tuesday and Wednesday
    let rrule_set = RRule::default()
        .count(4)
        .freq(Frequency::Weekly)
        .by_weekday(vec![
            NWeekday::Every(Weekday::Tue),
            NWeekday::Every(Weekday::Wed),
        ])
        .build(UTC.ymd(2020, 1, 1).and_hms(9, 0, 0))
        .expect("RRule invalid");

    // Build exrule that occurs weekly on Wednesday
    let exrule = RRule::default()
        .count(4)
        .freq(Frequency::Weekly)
        .by_weekday(vec![NWeekday::Every(Weekday::Wed)])
        .validate(UTC.ymd(2020, 1, 1).and_hms(9, 0, 0))
        .expect("RRule invalid");

    let recurrences = rrule_set.exrule(exrule).all(100).unwrap();

    // Check that all the recurrences are on a Tuesday
    for occurrence in &recurrences {
        assert_eq!(occurrence.weekday(), Weekday::Tue);
    }

    assert_eq!(recurrences.len(), 2);
    println!("Done, everything worked.");
}
