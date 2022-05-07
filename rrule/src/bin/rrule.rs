use std::{fmt::Display, str::FromStr};

use clap::Parser;
use rrule::{RRule, RRuleSet, WithError};

/// RRule parser and iterator
///
/// This program expects a Recurrence Rule (RRule) as defined by the
/// iCalendar (RFC-5545) specification (https://icalendar.org/iCalendar-RFC-5545/)..
///
/// An example of a valid RRule is:
///
/// - `DTSTART:20120201T093000Z\nRRULE:FREQ=YEARLY`
///
/// - `DTSTART:20120201T093000Z\nRRULE:FREQ=WEEKLY;INTERVAL=5;BYDAY=MO,FR`
#[derive(Parser, Debug)]
#[clap(
    name = "rrule",
    about = "A parser and iterator for recurrence rules as defined in the iCalendar RFC."
)]
struct Opts {
    /// Limit the amount of iteration
    /// If no limit is set, it will default to `100`.
    /// The maximum limit is `65535`.
    #[clap(short, long)]
    limit: Option<u16>,

    /// The RRule string you want to iterator over.
    input: String,
}

fn main() -> Result<(), String> {
    let opts: Opts = Parser::parse();

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
    <R as FromStr>::Err: Display,
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
