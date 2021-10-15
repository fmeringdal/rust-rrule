use std::{fmt::Display, str::FromStr};

use rrule::{RRule, RRuleSet, WithError};
use structopt::StructOpt;

/// RRule parser and iterator
///
/// This program expects an Recurrence Rule (RRule) as defined by the
/// iCalendar (RFC-5545) specification (https://icalendar.org/iCalendar-RFC-5545/)..
///
/// An example of a valid RRule is:
///
/// - `DTSTART:20120201T093000Z\nRRULE:FREQ=YEARLY`
///
/// - `DTSTART:20120201T093000Z\nRRULE:FREQ=WEEKLY;INTERVAL=5;BYDAY=MO,FR`
#[derive(StructOpt, Debug)]
#[structopt(
    name = "rrule",
    about = "A parser and iterator for recurrence rules as defined in the iCalendar RFC."
)]
struct Opt {
    /// Limit the amount of iteration
    /// If no limit is set, it will default to `100`.
    /// The maximum limit is `65535`.
    #[structopt(short, long)]
    limit: Option<u16>,

    /// The RRule string you want to iterator over.
    input: String,
}

fn main() -> Result<(), String> {
    let opts = Opt::from_args();

    let limit = opts.limit.unwrap_or(100);
    let rrule_str = opts.input.replace("\\n", "\n");

    if rrule_str.contains("EXRULE") || rrule_str.contains("RDATE") || rrule_str.contains("EXDATE") {
        let rrule: RRuleSet = parse_rule(&rrule_str)?;
        let iter = rrule.into_iter();
        iterator_dates(iter, limit);
    } else {
        let rrule: RRule = parse_rule(&rrule_str)?;
        let iter = rrule.into_iter();
        iterator_dates(iter, limit);
    }
    Ok(())
}

fn parse_rule<R: FromStr>(rrule_str: &str) -> Result<R, String>
where
    <R as FromStr>::Err: std::fmt::Display,
{
    match rrule_str.parse() {
        Ok(rrule) => Ok(rrule),
        Err(err) => {
            eprintln!("Error: {}", err);
            Err("Input string needs to be a valid rrule string.".to_owned())
        }
    }
}

fn iterator_dates<T>(mut rule_iter: T, limit: u16)
where
    T: Iterator + WithError,
    <T as Iterator>::Item: Display,
{
    for _i in 0..limit {
        let next = rule_iter.next();
        match next {
            Some(value) => {
                println!("{}", value);
            }
            None => {
                if let Some(error) = rule_iter.get_err() {
                    eprintln!("Error: {}", error);
                }
            }
        }
    }
}
