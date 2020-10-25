//! A partial implementation of recurrence rules as defined in the iCalendar RFC.
//!
//!
//! # Examples
//!
//! RRule
//!
//! ```
//! extern crate rrule;
//! extern crate chrono; 
//!
//! use chrono::prelude::*;
//! use rrule::build_rrule;
//!
//! // Parse a RRule string, return a RRule type
//! let mut rrule = build_rrule("DTSTART:20120201T093000Z\nRRULE:FREQ=WEEKLY;INTERVAL=5;UNTIL=20130130T230000Z;BYDAY=MO,FR");
//! assert_eq!(rrule.all().len(), 21);
//! ```
//!
//!
//! RRuleSet
//!
//! ```
//! extern crate rrule;
//! extern crate chrono; 
//!
//! use chrono::prelude::*;
//! use rrule::build_rruleset;
//! 
//! // Parse a RRuleSet string, return a RRuleSet type
//! let mut rrule = build_rruleset("DTSTART:20120201T023000Z\nRRULE:FREQ=MONTHLY;COUNT=5\nRDATE:20120701T023000Z,20120702T023000Z\nEXRULE:FREQ=MONTHLY;COUNT=2\nEXDATE:20120601T023000Z");
//! assert_eq!(rrule.all().len(), 6);
//! ```

extern crate chrono;
extern crate chrono_tz;
extern crate once_cell;
extern crate regex;

mod datetime;
mod iter;
mod parse_options;
mod options;
mod rrulestr;
mod rrule;
mod rrule_iter;
mod rruleset;
mod rruleset_iter;

pub use crate::rrule::RRule;
pub use crate::rruleset::RRuleSet;
pub use crate::rrulestr::{build_rrule, build_rruleset};
pub use crate::options::{Frequenzy, ParsedOptions, PartialOptions};