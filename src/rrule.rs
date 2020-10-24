use crate::iter::*;
use crate::iter_set::iter_v2;
use crate::options::*;
use chrono::prelude::*;
use chrono_tz::{Tz, UTC};


/// A type that produces instances of a given a RFC1241 string representation.
///
/// The first element is traditionally the path of the executable, but it can be
/// set to arbitrary text, and may not even exist. This means this property should
/// not be relied upon for security purposes.
///
/// On Unix systems shell usually expands unquoted arguments with glob patterns
/// (such as `*` and `?`). On Windows this is not done, and such arguments are
/// passed as-is.
///
/// # Panics
///
/// The returned iterator will panic during iteration if any argument to the
/// process is not valid unicode. If this is not desired,
/// use the [`args_os`] function instead.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// // Prints each argument on a separate line
/// for argument in env::args() {
///     println!("{}", argument);
/// }
/// ```
#[derive(Clone, Debug)]
pub struct RRule {
    pub options: ParsedOptions,
}

impl RRule {
    pub fn new(options: ParsedOptions) -> Self {
        Self {
            options
        }
    }

    pub fn all(&mut self) -> Vec<DateTime<Tz>> {
        let iter_args = IterArgs {
            inc: true,
            before: None,
            after: None,
            dt: None,
        };
        let mut iter_res = RRuleIterRes::new(QueryMethodTypes::ALL, iter_args);

        let res = iter_v2(&mut iter_res, &mut self.options);
        res
    }

    pub fn between(
        &mut self,
        after: &DateTime<Tz>,
        before: &DateTime<Tz>,
        inc: bool,
    ) -> Vec<DateTime<Tz>> {
        let iter_args = IterArgs {
            inc,
            before: Some(before.clone()),
            after: Some(after.clone()),
            dt: None,
        };
        let mut iter_res = RRuleIterRes::new(QueryMethodTypes::ALL, iter_args);

        let res = iter_v2(&mut iter_res, &mut self.options);
        res
    }
}
