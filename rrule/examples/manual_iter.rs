//! # Manual Iterations
//!
//! Manually iterate over an `RRule`.

use rrule::RRuleSet;

fn main() {
    let rrule: RRuleSet = "DTSTART;TZID=America/New_York:20200902T130000\n\
        RRULE:FREQ=Weekly"
        .parse()
        .expect("The RRule is not valid");

    let iter = rrule.into_iter();

    for next in iter.take(50) {
        if next.year() == 2021 {
            println!("These are all the weeks before 2021.");
            break;
        }
        println!("Date: {}", next.to_rfc3339());
    }
}
