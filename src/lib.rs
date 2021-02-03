//! A performant rust implementation of recurrence rules as defined in the iCalendar RFC.
//!
//! RRule provides two types for working with recurrence rules:
//! - `RRule`: For working with a single recurrence rule without any exception dates (exdates / exrules) and no additonal dates (rdate).
//! - `RRuleSet`: For working with a collection of rrules, exrules, rdates and exdates. Both the rrule and exrule
//! properties are represented by the `RRule` type and the rdate and exdate properties are represented by the DateTime<Tz> type
//! provided by the [chrono](https://crates.io/crates/chrono) and [chrono-tz](https://crates.io/crates/chrono-tz) crates.
//!
//! # Building RRule and RRuleSet
//! Both types implements the `std::str::FromStr` trait so that they can be parsed and built from a string representation. `RRule`
//! can additionally be constructured from the `Option` type which help build the recurrence rule. `RRuleSet`
//! can also be built by composing mutliple `RRule`s for its rrule and exrule properties and DateTime<Tz> for its
//! exdate and rdate properties. See the examples below.
//!
//! # Generating occurences
//! `RRule` and `RRuleSet` have four quick start methods for "querying" for recurrences:
//! - `all`: Generate all recurrences that matches the rules
//! - `between`: Generate all recurrences that matches the rules and are between two given dates
//! - `before`: Generate the last recurrence that matches the rules and is before a given date
//! - `after`: Generate the first recurrence that matches the rules and is after a given date
//!
//! If you have some additional filters or want to work with inifite recurrence rules
//! both `RRule` and `RRuleSet` implements the `Iterator` traits which makes them very flexible.
//! All the methods above uses the iterator trait in its implementation as shown below.
//! ````
//! extern crate rrule;
//! extern crate chrono;
//! extern crate chrono_tz;
//!
//! use chrono::prelude::*;
//! use chrono_tz::UTC;
//! use rrule::RRule;
//!
//! let mut rrule: RRule = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3".parse().unwrap();
//!
//!
//! // All dates
//! let all_occurences: Vec<_> = rrule.clone().into_iter().collect();
//! assert_eq!(all_occurences, rrule.all());
//!
//! // Between two dates
//! let after = UTC.ymd(2012, 2, 1).and_hms(10, 0, 0);
//! let before = UTC.ymd(2012, 4, 1).and_hms(9, 0, 0);
//! let inc = true; // Wheter dates equal to after or before should be added;
//!
//! let occurences_between_dates: Vec<_> = rrule.clone()
//!     .into_iter()
//!     .skip_while(|d| if inc { *d <= after } else { *d < after })
//!     .take_while(|d| if inc { *d <= before } else { *d < before })
//!     .collect();
//! assert_eq!(occurences_between_dates, rrule.between(after, before, inc));
//!
//!
//! // After a date
//! let after = UTC.ymd(2012, 2, 1).and_hms(10, 0, 0);
//! let inc = true; // Wheter dates equals to after should be added;
//!
//! let occurence_after_date = rrule.clone()
//!     .into_iter()
//!     .skip_while(|d| if inc { *d <= after } else { *d < after })
//!     .next();
//! assert_eq!(occurence_after_date, rrule.after(after, inc));
//!
//!
//! // Before a date
//! let before = UTC.ymd(2012, 4, 1).and_hms(10, 0, 0);
//! let inc = true; // Wheter dates equals to before should be added;
//!
//! let occurence_before_date = rrule.clone()
//!     .into_iter()
//!     .take_while(|d| if inc { *d <= before } else { *d < before })
//!     .last();
//! assert_eq!(occurence_before_date, rrule.before(before, inc));
//!
//! ````
//!
//! Note: All the generated recurrence will be in the same time zone as the dtstart property.
//!
//! # Examples
//!
//! Quick start by parsing strings
//!
//! ```
//! extern crate rrule;
//!
//! use rrule::RRule;
//!
//! // Parse a RRule string
//! let mut rrule: RRule = "DTSTART:20120201T093000Z\nRRULE:FREQ=WEEKLY;INTERVAL=5;UNTIL=20130130T230000Z;BYDAY=MO,FR".parse().unwrap();
//! assert_eq!(rrule.all().len(), 21);
//!
//!
//! use rrule::RRuleSet;
//!
//! // Parse a RRuleSet string
//! let mut rrule_set: RRuleSet = "DTSTART:20120201T023000Z\nRRULE:FREQ=MONTHLY;COUNT=5\nRDATE:20120701T023000Z,20120702T023000Z\nEXRULE:FREQ=MONTHLY;COUNT=2\nEXDATE:20120601T023000Z".parse().unwrap();
//! assert_eq!(rrule_set.all().len(), 4);
//! ```
//!
//!
//!
//! Using `Options` to build RRule
//!
//! ```
//! extern crate rrule;
//! extern crate chrono;
//! extern crate chrono_tz;
//!
//! use chrono::prelude::*;
//! use chrono_tz::UTC;
//! use rrule::{RRule, Options, Frequenzy, Weekday};
//!
//! // Build options that starts first day in 2020 at 9:00AM and occurs daily 5 times
//! let mut options = Options::new()
//!     .dtstart(UTC.ymd(2020, 1, 1).and_hms(9, 0, 0))
//!     .count(5)
//!     .freq(Frequenzy::Daily)
//!     .build()
//!     .unwrap();
//!
//! // Construct `RRule` from options
//! let mut rrule = RRule::new(options);
//! let recurrences = rrule.all();
//! for i in 0..5 {
//!     assert_eq!(recurrences[i].year(), 2020);
//!     assert_eq!(recurrences[i].month(), 1);
//!     assert_eq!(recurrences[i].day(), 1 + i as u32);
//!     assert_eq!(recurrences[i].hour(), 9);
//! }
//! assert_eq!(recurrences.len(), 5);
//! ```
//!
//!
//! Construct RRuleSet from one rrule and exrule.
//! The rrule will occur weekly on Tuesday and Wednesday and the exrule
//! will occur weekly on Wednesday, and therefore the end result will contain
//! weekly recurrences on Wednesday only.
//! ```
//! extern crate rrule;
//! extern crate chrono;
//! extern crate chrono_tz;
//!
//! use chrono::prelude::*;
//! use chrono_tz::UTC;
//! use rrule::{RRule, RRuleSet, Options, Frequenzy, Weekday};
//!
//!
//! // Build options for rrule that occurs weekly on Tuesday and Wednesday
//! let mut rrule_options = Options::new()
//!     .dtstart(UTC.ymd(2020, 1, 1).and_hms(9, 0, 0))
//!     .count(4)
//!     .freq(Frequenzy::Weekly)
//!     .byweekday(vec![Weekday::Tue, Weekday::Wed])
//!     .build()
//!     .unwrap();
//!
//! // Construct `RRule` from options
//! let mut rrule = RRule::new(rrule_options);
//!
//!
//! // Build options for exrule that occurs weekly on Wednesday
//! let mut exrule_options = Options::new()
//!     .dtstart(UTC.ymd(2020, 1, 1).and_hms(9, 0, 0))
//!     .count(4)
//!     .freq(Frequenzy::Weekly)
//!     .byweekday(vec![Weekday::Wed])
//!     .build()
//!     .unwrap();
//!
//! // Construct `RRule` from options
//! let mut exrule = RRule::new(exrule_options);
//!
//! // Now create the RRuleSet and add rrule and exrule
//! let mut rrule_set = RRuleSet::new();
//! rrule_set.rrule(rrule);
//! rrule_set.exrule(exrule);
//!
//! let recurrences = rrule_set.all();
//!
//! // Check that all the recurrences are on a Tuesday
//! for occurence in &recurrences {
//!     assert_eq!(occurence.weekday(), Weekday::Tue);
//! }
//!
//! assert_eq!(recurrences.len(), 2);
//! ```
//!
//!
//!
//!
//! Timezone support.
//! The following examples uses `RRuleSet` with one `RRule` that yields recurrences
//! in the Europe/Berlin timezone, and one EXDATE that is specified
//! in UTC and collides with one of those recurrences.  
//! ```
//! extern crate rrule;
//! extern crate chrono;
//! extern crate chrono_tz;
//!
//! use chrono::prelude::*;
//! use chrono_tz::{UTC, Tz};
//! use chrono_tz::Europe::Berlin;
//! use rrule::{RRule, RRuleSet, Options, Frequenzy, Weekday};
//!
//!
//!
//! // Build options for rrule that occurs daily at 9 oclock for 4 times
//! let mut rrule_options = Options::new()
//!     .dtstart(Berlin.ymd(2020, 1, 1).and_hms(9, 0, 0))
//!     .count(4)
//!     .freq(Frequenzy::Daily)
//!     .build()
//!     .unwrap();
//!
//! let mut rrule = RRule::new(rrule_options);
//!
//! // Exdate in the UTC at 8 oclock which is 9 oclock in Berlin and therefore
//! // collides with the second rrule occurence.
//! let exdate = UTC.ymd(2020, 1, 2).and_hms(8, 0, 0);
//!
//! // Now create the RRuleSet and add rrule and exdate
//! let mut rrule_set = RRuleSet::new();
//! rrule_set.rrule(rrule);
//! rrule_set.exdate(exdate);
//!
//! let recurrences = rrule_set.all();
//! // RRule contained 4 recurrences but 1 was filtered away by the exdate
//! assert_eq!(recurrences.len(), 3);
//!
//! // If you want to get back the DateTimes in another timezone you can just iterate over the result
//! // and convert them to another timezone by using the with_timzone method provided by the DateTime type.
//! // Refer to the chrono and chrono-tz crates for more documenation on working with the DateTime type.
//!
//! // Example of converting to mocow timezone
//! use chrono_tz::Europe::Moscow;
//!
//! let recurrences_in_moscow_tz: Vec<DateTime<Tz>> = recurrences.iter()
//!     .map(|d| d.with_timezone(&Moscow)).collect();
//!
//!
//! // Example of converting to local timezone (Local comes from chrono::prelude::*)
//! let recurrences_in_local_tz: Vec<DateTime<Local>> = recurrences.iter()
//!     .map(|d| d.with_timezone(&Local)).collect();
//!
//!
//! ```

extern crate chrono;
extern crate chrono_tz;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate lazy_static;

mod datetime;
mod iter;
mod options;
mod parse_options;
mod rrule;
mod rrule_iter;
mod rruleset;
mod rruleset_iter;
mod rrulestr;
mod utils;

pub use crate::options::{Frequenzy, NWeekday, Options, ParsedOptions};
pub use crate::rrule::RRule;
pub use crate::rruleset::RRuleSet;
pub use chrono::Weekday;
