//! # Manual Iterations
//!
//! Manually iterate over an `RRule`.

use chrono::Datelike;
use rrule::{RRule, WithError};

fn main() {
    let rrule: RRule = "DTSTART;TZID=America/New_York:20200902T130000\n\
        RRULE:FREQ=Weekly"
        .parse()
        .expect("The RRule is not valid");

    let mut iter = rrule.into_iter();

    // Note that the code below is similar to `all_with_error(limit)` in its implementation.
    let limit = 200;
    for _i in 0..limit {
        let next = iter.next();
        match next {
            Some(date) => {
                if date.year() == 2021 {
                    println!("These are all the weeks before 2021.");
                    break;
                }
                println!("Date: {}", date.to_rfc3339());
            }
            None => {
                if let Some(error) = iter.get_err() {
                    println!("Oh no, something went wrong.");
                    println!("Error: {}", error);
                }
                break;
            }
        }
    }
}
