//! A performant rust implementation of recurrence rules as defined in the [iCalendar RFC](https://datatracker.ietf.org/doc/html/rfc5545).
//!
//! This crate provides [`RRuleSet`] for working with recurrence rules. It has a collection of `DTSTART`, `RRULE`s, `EXRULE`s, `RDATE`s and `EXDATE`s. Both the `RRULE` and `EXRULE`
//! properties are represented by the [`RRule`] type and the `DTSTART`, `RDATE` and `EXDATE` properties are represented by the [`chrono::DateTime<Tz>`].
//!
//! # Building `RRule` and `RRuleSet`
//! [`RRuleSet`] implements the [`std::str::FromStr`] trait so that it can be parsed and built from a string representation.
//! [`RRuleSet`] can also be built by composing multiple [`RRule`]s for its `rrule` and `exrule` properties and [`chrono::DateTime<Tz>`] for its
//! `dt_start`, `exdate` and `rdate` properties. See the examples below.
//!
//! ```rust
//! use chrono::{DateTime, TimeZone};
//! use chrono_tz::UTC;
//! use rrule::{RRuleSet, RRule};
//!
//! // Parse a RRuleSet string
//! let rrule_set: RRuleSet = "DTSTART:20120201T023000Z\n\
//!     RRULE:FREQ=MONTHLY;COUNT=5\n\
//!     RDATE:20120701T023000Z,20120702T023000Z\n\
//!     EXDATE:20120601T023000Z".parse().unwrap();
//!
//! assert_eq!(*rrule_set.get_dt_start(), UTC.ymd(2012, 2, 1).and_hms(2, 30, 0));
//! assert_eq!(rrule_set.get_rrule().len(), 1);
//! assert_eq!(rrule_set.get_rdate().len(), 2);
//! assert_eq!(rrule_set.get_exdate().len(), 1);
//! ```
//!
//! # Generating occurrences
//! You can loop over the occurrences of a [`RRuleSet`] by calling any of the following methods:
//! - [`RRuleSet::all`]: Generate all recurrences that match the rules (with a limit to prevent infinite loops).
//! - [`RRuleSet::all_unchecked`]: Generate all recurrences that match the rules (without a limit).
//! - ...
//!
//! If you have some additional filters or want to work with infinite recurrence rules
//! [`RRuleSet`] implements the `IntoIterator` trait which allows for very flexible queries.
//! All the methods above uses the iterator trait in its implementation as shown below.
//! ```rust
//! use chrono::{DateTime, TimeZone};
//! use chrono_tz::UTC;
//! use rrule::RRuleSet;
//!
//! let rrule: RRuleSet = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3".parse().unwrap();
//! let (events, _) = rrule.all(100);
//!
//! // All dates
//! assert_eq!(
//!     vec![
//!         DateTime::parse_from_rfc3339("2012-02-01T09:30:00+00:00").unwrap(),
//!         DateTime::parse_from_rfc3339("2012-02-02T09:30:00+00:00").unwrap(),
//!         DateTime::parse_from_rfc3339("2012-02-03T09:30:00+00:00").unwrap(),
//!     ],
//!     events
//! );
//! ```
//! Find all events that are within a given range.
//! ```rust
//! # use chrono::{DateTime, TimeZone};
//! # use chrono_tz::UTC;
//! # use rrule::RRuleSet;
//! #
//! let rrule: RRuleSet = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3".parse().unwrap();
//!
//! // Between two dates
//! let after = UTC.ymd(2012, 2, 1).and_hms(10, 0, 0);
//! let before = UTC.ymd(2012, 4, 1).and_hms(9, 0, 0);
//!
//! let rrule = rrule.after(after).before(before);
//! let (events, _) = rrule.all(100);
//!
//! assert_eq!(
//!     vec![
//!         DateTime::parse_from_rfc3339("2012-02-02T09:30:00+00:00").unwrap(),
//!         DateTime::parse_from_rfc3339("2012-02-03T09:30:00+00:00").unwrap(),
//!     ],
//!     events
//! );
//! ```
//!
//! Note: All the generated recurrence will be in the same time zone as the `dt_start` property.
//!

#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

mod core;
mod error;
mod iter;
mod parser;
mod tests;
mod validator;

pub use crate::core::{Frequency, NWeekday, RRule, RRuleSet};
pub use crate::core::{Unvalidated, Validated};
pub use chrono::Weekday;
pub use error::RRuleError;
pub(crate) use iter::RRuleIter;
pub use iter::RRuleSetIter;
