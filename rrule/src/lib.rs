//! A performant rust implementation of recurrence rules as defined in the [iCalendar RFC](https://datatracker.ietf.org/doc/html/rfc5545).
//!
//! This crate provides two types for working with recurrence rules:
//! - [`RRuleSet`]: For working with a collection of `DTSTART`, `RRULE`s, `EXRULE`s, `RDATE`s and `EXDATE`s. Both the `RRULE` and `EXRULE`
//! properties are represented by the [`RRule`] type and the `DTSTART`, `RDATE` and `EXDATE` properties are represented by the [`chrono::DateTime<Tz>`].
//! - [`RRule`]: Which represents a single `RRULE` definition.
//!
//! # Building `RRule` and `RRuleSet`
//! Both types implements the [`std::str::FromStr`] trait so that they can be parsed and built from a string representation.
//! [`RRuleSet`] can also be built by composing multiple `RRule`s for its `rrule` and `exrule` properties and [`chrono::DateTime<Tz>`] for its
//! `dt_start`, `exdate` and `rdate` properties. See the examples below.
//!
//! # Generating occurrences
//! [`RRule`] and [`RRuleSet`] both implement [`DateFilter`] which implements methods for easy filtering:
//! - `all`: Generate all recurrences that match the rules (with a limit to prevent infinite loops).
//! - `all_between`: Generate all recurrences that match the rules and are between two given dates.
//! - `just_before`: Generate the last recurrence that matches the rules and is before a given date.
//! - `just_after`: Generate the first recurrence that matches the rules and is after a given date.
//! - ...
//!
//! If you have some additional filters or want to work with infinite recurrence rules
//! both [`RRule`] and [`RRuleSet`] implements the `Iterator` traits which makes them very flexible.
//! All the methods above uses the iterator trait in its implementation as shown below.
//! ```rust
//! use chrono::{DateTime, TimeZone};
//! use chrono_tz::UTC;
//! use rrule::{RRuleSet};
//!
//! let rrule: RRuleSet = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3".parse().unwrap();
//!
//! // All dates
//! assert_eq!(
//!     vec![
//!         DateTime::parse_from_rfc3339("2012-02-01T09:30:00+00:00").unwrap(),
//!         DateTime::parse_from_rfc3339("2012-02-02T09:30:00+00:00").unwrap(),
//!         DateTime::parse_from_rfc3339("2012-02-03T09:30:00+00:00").unwrap(),
//!     ],
//!     rrule.all(100).unwrap()
//! );
//! ```
//! Find all events that are within a given range.
//! ```rust
//! # use chrono::{DateTime, TimeZone};
//! # use chrono_tz::UTC;
//! # use rrule::{RRuleSet};
//! #
//! let rrule: RRuleSet = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3".parse().unwrap();
//! // Between two dates
//! let after = UTC.ymd(2012, 2, 1).and_hms(10, 0, 0);
//! let before = UTC.ymd(2012, 4, 1).and_hms(9, 0, 0);
//! let inc = true; // Whether dates equal to after or before should be added;
//!
//! assert_eq!(
//!     vec![
//!         DateTime::parse_from_rfc3339("2012-02-02T09:30:00+00:00").unwrap(),
//!         DateTime::parse_from_rfc3339("2012-02-03T09:30:00+00:00").unwrap(),
//!     ],
//!     rrule.all_between(after, before, inc).unwrap()
//! );
//! ```
//!
//! Note: All the generated recurrence will be in the same time zone as the `dt_start` property.
//!
//! # Example
//!
//! Quick start by parsing strings
//!
//! ```rust
//! use chrono::DateTime;
//! use rrule::{RRuleSet};
//!
//! // Parse a RRule string
//! let rrule: RRuleSet = "DTSTART:20120201T093000Z\n\
//!    RRULE:FREQ=WEEKLY;INTERVAL=5;UNTIL=20130130T230000Z;BYDAY=MO,FR".parse().unwrap();
//! assert_eq!(rrule.all(100).unwrap().len(), 21);
//!
//! // Parse a RRuleSet string
//! let rrule_set: RRuleSet = "DTSTART:20120201T023000Z\n\
//!     RRULE:FREQ=MONTHLY;COUNT=5\n\
//!     RDATE:20120701T023000Z,20120702T023000Z\n\
//!     EXRULE:FREQ=MONTHLY;COUNT=2\n\
//!     EXDATE:20120601T023000Z".parse().unwrap();
//! let all_dates = rrule_set.all(100).unwrap();
//! assert_eq!(all_dates.len(), 4);
//!
//! assert_eq!(
//!     vec![
//!         DateTime::parse_from_rfc3339("2012-04-01T02:30:00+00:00").unwrap(),
//!         DateTime::parse_from_rfc3339("2012-05-01T02:30:00+00:00").unwrap(),
//!         DateTime::parse_from_rfc3339("2012-07-01T02:30:00+00:00").unwrap(),
//!         DateTime::parse_from_rfc3339("2012-07-02T02:30:00+00:00").unwrap(),
//!     ],
//!     all_dates
//! );
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

pub use crate::core::{Frequency, NWeekday, RRule, RRuleSet};
pub use crate::core::{Unvalidated, Validated};
pub use chrono::Weekday;
pub use error::{RRuleError, WithError};
pub use iter::{RRuleIter, RRuleSetIter};
