//! # Timezone Support
//!
//! This examples uses `RRuleSet` with one `RRule` that yields recurrences
//! in the Europe/Berlin timezone, and one EXDATE that is specified
//! in UTC and collides with one of those recurrences.

use chrono::{DateTime, Local, TimeZone};
use chrono_tz::{Europe::Berlin, Tz, UTC};
use rrule::{Frequency, RRule};

fn main() {
    // Build properties for rrule that occurs daily at 9:00 for 4 times
    let rrule = RRule::default().count(4).freq(Frequency::Daily);

    let mut rrule_set = rrule
        .build(Berlin.ymd(2020, 1, 1).and_hms(9, 0, 0))
        .expect("RRule invalid");

    // Exdate in the UTC at 8:00 which is 9:00 in Berlin and therefore
    // collides with the second rrule occurrence.
    let exdate = UTC.ymd(2020, 1, 2).and_hms(8, 0, 0);
    rrule_set.exdate(exdate);

    let recurrences = rrule_set.all(100).unwrap();
    // RRule contained 4 recurrences but 1 was filtered away by the exdate
    assert_eq!(recurrences.len(), 3);

    // If you want to get back the DateTimes in another timezone you can just iterate over the result
    // and convert them to another timezone by using the with_timezone method provided by the DateTime type.
    // Refer to the chrono and chrono-tz crates for more documentation on working with the DateTime type.

    // Example of converting to Moscow timezone
    use chrono_tz::Europe::Moscow;

    let _recurrences_in_moscow_tz: Vec<DateTime<Tz>> = recurrences
        .iter()
        .map(|d| d.with_timezone(&Moscow))
        .collect();

    // Example of converting to local timezone.
    let _recurrences_in_local_tz: Vec<DateTime<Local>> = recurrences
        .iter()
        .map(|d| d.with_timezone(&Local))
        .collect();

    println!("Done, everything worked.");
}
