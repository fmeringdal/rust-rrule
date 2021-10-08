//! A performant rust implementation of recurrence rules as defined in the iCalendar RFC.
//!
//! RRule provides two types for working with recurrence rules:
//! - `RRule`: For working with a single recurrence rule without any exception dates (exdates / exrules) and no additional dates (rdate).
//! - `RRuleSet`: For working with a collection of rrules, exrules, rdates and exdates. Both the rrule and exrule
//! properties are represented by the `RRule` type and the rdate and exdate properties are represented by the DateTime<Tz> type
//! provided by the [chrono](https://crates.io/crates/chrono) and [chrono-tz](https://crates.io/crates/chrono-tz) crates.
//!
//! # Building RRule and RRuleSet
//! Both types implements the `std::str::FromStr` trait so that they can be parsed and built from a string representation. `RRule`
//! can additionally be constructed from the `Option` type which help build the recurrence rule. `RRuleSet`
//! can also be built by composing multiple `RRule`s for its rrule and exrule properties and DateTime<Tz> for its
//! exdate and rdate properties. See the examples below.
//!
//! # Generating occurrences
//! `RRule` and `RRuleSet` have four quick start methods for "querying" for recurrences:
//! - `all`: Generate all recurrences that matches the rules (with a limit to prevent infinite loops).
//! - `between`: Generate all recurrences that matches the rules and are between two given dates.
//! - `before`: Generate the last recurrence that matches the rules and is before a given date.
//! - `after`: Generate the first recurrence that matches the rules and is after a given date.
//!
//! If you have some additional filters or want to work with infinite recurrence rules
//! both `RRule` and `RRuleSet` implements the `Iterator` traits which makes them very flexible.
//! All the methods above uses the iterator trait in its implementation as shown below.
//! ```rust
//! use chrono::TimeZone;
//! use chrono_tz::UTC;
//! use rrule::RRule;
//!
//! let rrule: RRule = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3".parse().unwrap();
//!
//! // All dates
//! let all_occurrences: Vec<_> = rrule.clone().into_iter().collect();
//! assert_eq!(all_occurrences, rrule.all(100));
//!
//! // Between two dates
//! let after = UTC.ymd(2012, 2, 1).and_hms(10, 0, 0);
//! let before = UTC.ymd(2012, 4, 1).and_hms(9, 0, 0);
//! let inc = true; // Whether dates equal to after or before should be added;
//!
//! let occurrences_between_dates: Vec<_> = rrule.clone()
//!     .into_iter()
//!     .skip_while(|d| if inc { *d <= after } else { *d < after })
//!     .take_while(|d| if inc { *d <= before } else { *d < before })
//!     .collect();
//! assert_eq!(occurrences_between_dates, rrule.between(after, before, inc));
//!
//! // After a date
//! let after = UTC.ymd(2012, 2, 1).and_hms(10, 0, 0);
//! let inc = true; // Whether dates equals to after should be added;
//!
//! let occurrence_after_date = rrule.clone()
//!     .into_iter()
//!     .skip_while(|d| if inc { *d <= after } else { *d < after })
//!     .next();
//! assert_eq!(occurrence_after_date, rrule.after(after, inc));
//!
//! // Before a date
//! let before = UTC.ymd(2012, 4, 1).and_hms(10, 0, 0);
//! let inc = true; // Whether dates equals to before should be added;
//!
//! let occurrence_before_date = rrule.clone()
//!     .into_iter()
//!     .take_while(|d| if inc { *d <= before } else { *d < before })
//!     .last();
//! assert_eq!(occurrence_before_date, rrule.before(before, inc));
//!
//! ```
//!
//! Note: All the generated recurrence will be in the same time zone as the `dt_start` property.
//!
//! # Example
//!
//! Quick start by parsing strings
//!
//! ```rust
//! use rrule::RRule;
//!
//! // Parse a RRule string
//! let rrule: RRule = "DTSTART:20120201T093000Z\n\
//!    RRULE:FREQ=WEEKLY;INTERVAL=5;UNTIL=20130130T230000Z;BYDAY=MO,FR".parse().unwrap();
//! assert_eq!(rrule.all(100).len(), 21);
//!
//! use rrule::RRuleSet;
//!
//! // Parse a RRuleSet string
//! let rrule_set: RRuleSet = "DTSTART:20120201T023000Z\n\
//!     RRULE:FREQ=MONTHLY;COUNT=5\n\
//!     RDATE:20120701T023000Z,20120702T023000Z\n\
//!     EXRULE:FREQ=MONTHLY;COUNT=2\n\
//!     EXDATE:20120601T023000Z".parse().unwrap();
//! assert_eq!(rrule_set.all(100).len(), 4);
//! ```
//!

#![forbid(unsafe_code)]
#![deny(clippy::all)]
// #![warn(missing_docs)]

mod core;
mod error;
mod iter;
mod parser;
mod validator;

pub use crate::core::{Frequency, NWeekday, Options, ParsedOptions, RRule, RRuleSet};
pub use chrono::Weekday;
pub use error::RRuleError;
pub use iter::{RRuleIter, RRuleSetIter};
