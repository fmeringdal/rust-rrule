//! # Timezone Support
//!
//! This examples uses `RRuleSet` with one `RRule` that yields recurrences
//! in the Europe/Berlin timezone, and one EXDATE that is specified
//! in UTC and collides with one of those recurrences.

use chrono::{DateTime, TimeZone};
use rrule::{Frequency, RRule, Tz};

fn main() {
    let tz = Tz::Tz(chrono_tz::Tz::Europe__Berlin);
    let start_date = tz.with_ymd_and_hms(2020, 1, 1, 9, 0, 0).unwrap();
    let exdate = Tz::Tz(chrono_tz::Tz::UTC)
        .with_ymd_and_hms(2020, 1, 2, 8, 0, 0)
        .unwrap();

    // Build an rrule set that occurs daily at 9:00 for 4 times
    let rrule_set = RRule::default()
        .count(4)
        .freq(Frequency::Daily)
        .build(start_date)
        .expect("RRule invalid")
        // Exdate in the UTC at 8:00 which is 9:00 in Berlin and therefore
        // collides with the second rrule occurrence.
        .exdate(exdate);

    let recurrences = rrule_set.all_unchecked();
    // RRule contained 4 recurrences but 1 was filtered away by the exdate
    assert_eq!(recurrences.len(), 3);

    // If you want to get back the DateTimes in another timezone, you can just iterate over the result
    // and convert them to another timezone by using the with_timezone method provided by the DateTime type.
    // Refer to the chrono and chrono-tz crates for more documentation on working with the DateTime type.

    // Convert to `chrono_tz::Tz`
    let _recurrences_in_moscow_tz: Vec<DateTime<chrono_tz::Tz>> = recurrences
        .iter()
        .map(|d| d.with_timezone(&chrono_tz::Europe::Moscow))
        .collect();

    println!("Done, everything worked.");
}
