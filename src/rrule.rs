use crate::iter::*;
use crate::iter_set::iter_v2;
use crate::options::*;
use chrono::prelude::*;
use chrono_tz::{Tz, UTC};

#[derive(Clone, Debug)]
pub struct RRule {
    cache: bool,
    pub options: ParsedOptions,
}

impl RRule {
    pub fn new(options: ParsedOptions) -> Self {
        Self {
            options,
            cache: true,
        }
    }

    pub fn disable_cache(&mut self) {
        self.cache = false;
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
