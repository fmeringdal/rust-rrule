mod parse_options;
mod rrulestr;

pub use parse_options::parse_options;
pub(crate) use rrulestr::{build_rruleset, parse_rrule_string_to_options};

fn is_some_and_not_empty<T>(v: &Option<Vec<T>>) -> bool {
    match v {
        Some(v) => !v.is_empty(),
        None => false,
    }
}
