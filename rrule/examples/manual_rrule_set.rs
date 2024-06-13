//! # Manual [`rrule::RRuleSet`]
//!
//! Create an [`rrule::RRuleSet`] object manually.

/// ## Construct [`rrule::RRuleSet`] from one `rrule` and `exrule`
/// The rrule will occur weekly on Tuesday and Wednesday, and the exrule
/// will occur weekly on Wednesday, and therefore the end result will contain
/// weekly recurrences on Wednesday only.
fn main() {
    #[cfg(feature = "exrule")]
    {
        use chrono::{Datelike, TimeZone};
        use rrule::{Frequency, NWeekday, RRule, Tz, Weekday};
        // Build an rrule set that occurs weekly on Tuesday and Wednesday
        let rrule_set = RRule::default()
            .count(4)
            .freq(Frequency::Weekly)
            .by_weekday(vec![
                NWeekday::Every(Weekday::Tue),
                NWeekday::Every(Weekday::Wed),
            ])
            .build(
                Tz::Tz(chrono_tz::UTC)
                    .with_ymd_and_hms(2020, 1, 1, 9, 0, 0)
                    .unwrap(),
            )
            .expect("RRule invalid");

        // Build exrule that occurs weekly on Wednesday
        let exrule = RRule::default()
            .count(4)
            .freq(Frequency::Weekly)
            .by_weekday(vec![NWeekday::Every(Weekday::Wed)])
            .validate(
                Tz::Tz(chrono_tz::UTC)
                    .with_ymd_and_hms(2020, 1, 1, 9, 0, 0)
                    .unwrap(),
            )
            .expect("RRule invalid");

        let recurrences = rrule_set.exrule(exrule).all(10).dates;

        // Check that all the recurrences are on a Tuesday
        for occurrence in &recurrences {
            assert_eq!(occurrence.weekday(), Weekday::Tue);
        }

        assert_eq!(recurrences.len(), 2);
        println!("Done, everything worked.");
    }
}
