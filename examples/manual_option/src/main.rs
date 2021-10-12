use chrono::{Datelike, TimeZone, Timelike};
use chrono_tz::UTC;
use rrule::{Frequency, ParsedOptions, RRule};

fn main() {
    // Build options that starts first day in 2020 at 9:00AM and occurs daily 5 times
    let options = ParsedOptions::default()
        .dt_start(UTC.ymd(2020, 1, 1).and_hms(9, 0, 0))
        .count(5)
        .freq(Frequency::Daily);

    // Construct `RRule` from options
    let rrule = RRule::new(options).expect("RRule invalid");
    let recurrences = rrule.all(100);
    for i in 0..5 {
        assert_eq!(recurrences[i].year(), 2020);
        assert_eq!(recurrences[i].month(), 1);
        assert_eq!(recurrences[i].day(), 1 + i as u32);
        assert_eq!(recurrences[i].hour(), 9);
    }
    assert_eq!(recurrences.len(), 5);
}
