#![allow(clippy::module_name_repetitions)]

mod checks;
mod counter_date;
mod easter;
pub(crate) mod filters;
pub(crate) mod iterinfo;
mod masks;
mod monthinfo;
mod operation_errors;
mod pos_list;
pub(crate) mod rrule_iter;
mod rruleset_iter;
mod utils;
mod yearinfo;

use iterinfo::IterInfo;
use pos_list::build_pos_list;
pub(crate) use rrule_iter::RRuleIter;
pub use rruleset_iter::RRuleSetIter;

/// Prevent loops when searching for the next event in the iterator.
/// If after X number of iterations it still has not found an event,
/// we can assume it will not find an event.
static MAX_ITER_LOOP: u32 = 100_000;
