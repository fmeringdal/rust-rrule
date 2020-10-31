use crate::iter::{iter, IterResult};
use crate::rrule_iter::*;
use crate::rruleset::RRuleSet;
use chrono::prelude::*;
use chrono_tz::Tz;
use std::collections::HashMap;

/// Result iterator for the RRuleSet type. It mostly just wraps
/// `RRuleIterRes` and also before accepting any date makes sure that
/// it does not collide with any exdates provided by either the exdates property
/// or produced by the exrule rules.
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

    /// Computes all exdates between after and before `DateTime`s for all the exrules and inserts them into
    /// the exdate_hash.
    pub fn eval_exdate(&mut self, after: &DateTime<Tz>, before: &DateTime<Tz>) {
        for rrule in self.rrule_set.exrule.iter_mut() {
            for date in rrule.between(after.clone(), before.clone(), true) {
                self.exdate_hash.insert(date.timestamp(), ());
            }
        }
    }

    /// Here it is required to recompute exrules to see if some of the occurences will collide with the provided date.
    fn accept_when_unknown_bounds(&mut self, date: DateTime<Tz>) -> bool {
        let dt = date.timestamp();
        if !self.exdate_hash.contains_key(&dt) {
            self.eval_exdate(
                &date.timezone().timestamp(dt - 1, 0),
                &date.timezone().timestamp(dt + 1, 0),
            );
            if !self.exdate_hash.contains_key(&dt) {
                self.exdate_hash.insert(dt, ());
                return self.iter_res.accept(date);
            }
        }

        true
    }

    /// No need to recompute exrules as it has already beeen computed in the start of the iter method
    /// because the bounds where known.
    fn accept_when_known_bounds(&mut self, date: DateTime<Tz>) -> bool {
        let dt = date.timestamp();
        if !self.exdate_hash.contains_key(&dt) {
            self.exdate_hash.insert(dt, ());
            return self.iter_res.accept(date);
        }

        true
    }

    pub fn iter(&mut self) -> Vec<DateTime<Tz>> {
        // Add all exdates to exdate_hash
        for date in &self.rrule_set.exdate {
            println!("Exdate timestamp: {}", date.timestamp());
            self.exdate_hash.insert(date.timestamp(), ());
        }

        // Small performance improvement by computing all exdates between
        // before and after dates when. For all the other `QueryMethodTypes`
        // the iter has to eval_exdate for every DateTime value produced by rdate or rrule
        // to check if it conflicts with any exrule or exdate rules, this is because
        // All, Before, After QueryMethodTypes has no time bounds and all the exdates can not be known beforehand.
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
            if !self.accept(date.clone()) {
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
