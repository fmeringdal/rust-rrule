//! A partial implementation of recurrence rules as defined in the iCalendar RFC.
//!
//! This map implementation allows reads and writes to execute entirely in parallel, with no
//! implicit synchronization overhead. Reads never take locks on their critical path, and neither
//! do writes assuming there is a single writer (multi-writer is possible using a `Mutex`), which
//! significantly improves performance under contention.
//!
//! The trade-off exposed by this module is one of eventual consistency: writes are not visible to
//! readers except following explicit synchronization. Specifically, readers only see the
//! operations that preceeded the last call to `WriteHandle::refresh` by a writer. This lets
//! writers decide how stale they are willing to let reads get. They can refresh the map after
//! every write to emulate a regular concurrent `HashMap`, or they can refresh only occasionally to
//! reduce the synchronization overhead at the cost of stale reads.
//!
//! For read-heavy workloads, the scheme used by this module is particularly useful. Writers can
//! afford to refresh after every write, which provides up-to-date reads, and readers remain fast
//! as they do not need to ever take locks.
//!
//! The map is multi-value, meaning that every key maps to a *collection* of values. This
//! introduces some memory cost by adding a layer of indirection through a `Vec` for each value,
//! but enables more advanced use. This choice was made as it would not be possible to emulate such
//! functionality on top of the semantics of this map (think about it -- what would the operational
//! log contain?).
//!
//! To faciliate more advanced use-cases, each of the two maps also carry some customizeable
//! meta-information. The writers may update this at will, and when a refresh happens, the current
//! meta will also be made visible to readers. This could be useful, for example, to indicate what
//! time the refresh happened.
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
mod easter;
mod iter;
mod iter_set;
mod iterinfo;
mod masks;
mod monthinfo;
mod poslist;
mod parse_options;
mod yearinfo;
mod options;
mod rrulestr;
mod rrule;
mod rruleset;

pub use crate::rrule::RRule;
pub use crate::rruleset::RRuleSet;
pub use crate::rrulestr::{build_rrule, build_rruleset};
pub use crate::options::{Frequenzy, ParsedOptions, PartialOptions};