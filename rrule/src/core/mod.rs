mod datetime;
mod rrule;
mod rruleset;
pub(crate) mod utils;

pub use self::rrule::{Frequency, NWeekday, RRule};
pub use self::rruleset::RRuleSet;
pub(crate) use datetime::Time;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
/// An empty struct to keep the validated stage
pub struct Validated;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
/// An empty struct to keep the unvalidated (or not-yet-validated) stage
pub struct Unvalidated;
