use crate::iter::iter;
use crate::options::*;
use crate::rrule_iter::*;
use chrono::prelude::*;
use chrono_tz::Tz;

#[derive(Clone, Debug)]
pub struct RRule {
    pub options: ParsedOptions,
}

impl RRule {
    pub fn new(options: ParsedOptions) -> Self {
        Self { options }
    }

    pub fn all(&mut self) -> Vec<DateTime<Tz>> {
        let iter_args = IterArgs {
            inc: true,
            before: None,
            after: None,
            dt: None,
        };
        let mut iter_res = RRuleIterRes::new(QueryMethodTypes::All, iter_args);

        let res = iter(&mut iter_res, &mut self.options);
        res
    }

    pub fn before(&mut self, dt: DateTime<Tz>, inc: bool) -> Vec<DateTime<Tz>> {
        let iter_args = IterArgs {
            inc,
            before: None,
            after: None,
            dt: Some(dt),
        };
        let mut iter_res = RRuleIterRes::new(QueryMethodTypes::Before, iter_args);

        iter(&mut iter_res, &mut self.options)
    }

    pub fn after(&mut self, dt: DateTime<Tz>, inc: bool) -> Vec<DateTime<Tz>> {
        let iter_args = IterArgs {
            inc,
            before: None,
            after: None,
            dt: Some(dt),
        };
        let mut iter_res = RRuleIterRes::new(QueryMethodTypes::After, iter_args);

        iter(&mut iter_res, &mut self.options)
    }

    pub fn between(
        &mut self,
        after: DateTime<Tz>,
        before: DateTime<Tz>,
        inc: bool,
    ) -> Vec<DateTime<Tz>> {
        let iter_args = IterArgs {
            inc,
            before: Some(before),
            after: Some(after),
            dt: None,
        };
        let mut iter_res = RRuleIterRes::new(QueryMethodTypes::Between, iter_args);

        iter(&mut iter_res, &mut self.options)
    }
}
