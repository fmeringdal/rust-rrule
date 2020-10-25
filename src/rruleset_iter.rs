use crate::iter::{iter, IterResult};
use chrono::prelude::*;
use crate::rruleset::RRuleSet;
use crate::rrule_iter::*;
use std::collections::HashMap;
use chrono_tz::{Tz, UTC};

pub struct RRuleSetIter<'a> {
    exdate_hash: HashMap<i64, ()>,
    iter_res: RRuleIterRes,
    rrule_set: &'a mut RRuleSet,
}

impl<'a> RRuleSetIter<'a> {
    pub fn all(rrule_set: &'a mut RRuleSet) -> Self {
        let iter_args = IterArgs {
            inc: true,
            before: None,
            after: None,
            dt: None,
        };
        let iter_res = RRuleIterRes::new(QueryMethodTypes::All, iter_args);

        Self {
            exdate_hash: HashMap::new(),
            iter_res,
            rrule_set,
        }
    }

    pub fn before(rrule_set: &'a mut RRuleSet, dt: DateTime<Tz>, inc: bool) -> Self {
        let iter_args = IterArgs {
            inc,
            before: None,
            after: None,
            dt: Some(dt),
        };
        let iter_res = RRuleIterRes::new(QueryMethodTypes::Before, iter_args);

        Self {
            exdate_hash: HashMap::new(),
            iter_res,
            rrule_set,
        }
    }

    pub fn after(rrule_set: &'a mut RRuleSet, dt: DateTime<Tz>, inc: bool) -> Self {
        let iter_args = IterArgs {
            inc,
            before: None,
            after: None,
            dt: Some(dt),
        };
        let iter_res = RRuleIterRes::new(QueryMethodTypes::After, iter_args);

        Self {
            exdate_hash: HashMap::new(),
            iter_res,
            rrule_set,
        }
    }

    pub fn between(
        rrule_set: &'a mut RRuleSet,
        after: DateTime<Tz>,
        before: DateTime<Tz>,
        inc: bool,
    ) -> Self {
        let iter_args = IterArgs {
            inc,
            before: Some(before),
            after: Some(after),
            dt: None,
        };
        let iter_res = RRuleIterRes::new(QueryMethodTypes::Between, iter_args);

        Self {
            exdate_hash: HashMap::new(),
            iter_res,
            rrule_set,
        }
    }

    pub fn eval_exdate(&mut self, after: &DateTime<Tz>, before: &DateTime<Tz>) {
        for rrule in self.rrule_set.exrule.iter_mut() {
            for date in rrule.between(after.clone(), before.clone(), true) {
                self.exdate_hash.insert(date.timestamp(), ());
            }
        }
    }

    fn accept_when_unknown_bounds(&mut self, date: DateTime<Tz>) -> bool {
        let dt = date.timestamp();
        if !self.exdate_hash.contains_key(&dt) {
            self.eval_exdate(&date.timezone().timestamp(dt - 1, 0), &date.timezone().timestamp(dt + 1, 0));
            if !self.exdate_hash.contains_key(&dt) {
                self.exdate_hash.insert(dt, ());
                return self.iter_res.accept(date.clone());
            }
        }

        true
    }

    fn accept_when_known_bounds(&mut self, date: DateTime<Tz>) -> bool {
        let dt = date.timestamp();
        if !self.exdate_hash.contains_key(&dt) {
            self.exdate_hash.insert(dt, ());
            return self.iter_res.accept(date.clone());
        }

        true
    }

    pub fn iter(&mut self, tzid: Option<String>) -> Vec<DateTime<Tz>> {
        let tzid: Tz = tzid.unwrap_or(String::from("UTC")).parse().unwrap_or(UTC);

        for date in &self.rrule_set.exdate {
            let zoned_date = date.with_timezone(&tzid);
            self.exdate_hash.insert(zoned_date.timestamp(), ());
        }

        match &self.iter_res.method {
            QueryMethodTypes::Between => {
                self.eval_exdate(
                    &self.iter_res.args.after.unwrap().clone(),
                    &self.iter_res.args.before.unwrap().clone(),
                );
            }
            _ => (),
        };

        for date in &self.rrule_set.rdate.clone() {
            let zoned_date = date.with_timezone(&tzid);
            if !self.accept(zoned_date) {
                break;
            }
        }

        for rule in self.rrule_set.rrule.clone().iter_mut() {
            iter(self, &mut rule.options);
        }

        let mut res = self.iter_res.get_value();
        res.sort();
        res
    }
}

impl<'a> IterResult for RRuleSetIter<'a> {
    fn accept(&mut self, date: DateTime<Tz>) -> bool {
        match &self.iter_res.method {
            QueryMethodTypes::Between => self.accept_when_known_bounds(date),
            _ => self.accept_when_unknown_bounds(date),
        }
    }

    fn get_value(&self) -> Vec<DateTime<Tz>> {
        self.iter_res.get_value()
    }
}
