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

    /// Returns all the recurrences of the rrule
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
    
    /// Returns the last recurrence before the given datetime instance.
    /// The inc keyword defines what happens if dt is an recurrence.
    /// With inc == true, if dt itself is an recurrence, it will be returned.
    pub fn before(&mut self, dt: DateTime<Tz>, inc: bool) -> Option<DateTime<Tz>> {
        let iter_args = IterArgs {
            inc,
            before: None,
            after: None,
            dt: Some(dt),
        };
        let mut iter_res = RRuleIterRes::new(QueryMethodTypes::Before, iter_args);

        let recurrences =  iter(&mut iter_res, &mut self.options);
        if recurrences.is_empty() {
            None
        } else {
            Some(recurrences[0])
        }
    }

    /// Returns the last recurrence after the given datetime instance.
    /// The inc keyword defines what happens if dt is an recurrence.
    /// With inc == true, if dt itself is an recurrence, it will be returned.
    pub fn after(&mut self, dt: DateTime<Tz>, inc: bool) -> Option<DateTime<Tz>> {
        let iter_args = IterArgs {
            inc,
            before: None,
            after: None,
            dt: Some(dt),
        };
        let mut iter_res = RRuleIterRes::new(QueryMethodTypes::After, iter_args);
        let recurrences =  iter(&mut iter_res, &mut self.options);
        if recurrences.is_empty() {
            None
        } else {
            Some(recurrences[0])
        }
    }

    /// Returns all the recurrences of the rrule between after and before.
    /// The inc keyword defines what happens if after and/or before are
    /// themselves recurrences. With inc == true, they will be included in the
    /// list, if they are found in the recurrence set.
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
