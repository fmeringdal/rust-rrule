use crate::{
    iter::{make_timeset, IterInfo, IterResult, RRuleIter},
    RRule,
};
use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::Tz;

pub enum QueryMethodTypes {
    All,
    Between,
    Before,
    After,
}

pub struct IterArgs {
    pub inc: bool,
    pub before: Option<DateTime<Tz>>,
    pub after: Option<DateTime<Tz>>,
    pub dt: Option<DateTime<Tz>>,
}

pub struct RRuleIterRes {
    pub method: QueryMethodTypes,
    pub args: IterArgs,
    pub min_date: Option<DateTime<Tz>>,
    pub max_date: Option<DateTime<Tz>>,
    pub result: Vec<DateTime<Tz>>,
    pub total: usize,
}

impl RRuleIterRes {
    pub fn new(method: QueryMethodTypes, args: IterArgs) -> Self {
        let (max_date, min_date) = match method {
            QueryMethodTypes::Between if args.inc => {
                (Some(args.before.unwrap()), Some(args.after.unwrap()))
            }
            QueryMethodTypes::Between => (
                Some(args.before.unwrap() - Duration::milliseconds(1)),
                Some(args.after.unwrap() + Duration::milliseconds(1)),
            ),
            QueryMethodTypes::Before if args.inc => (Some(args.dt.unwrap()), None),
            QueryMethodTypes::Before => (Some(args.dt.unwrap() - Duration::milliseconds(1)), None),
            QueryMethodTypes::After if args.inc => (None, Some(args.dt.unwrap())),
            QueryMethodTypes::After => (None, Some(args.dt.unwrap() + Duration::milliseconds(1))),
            _ => (None, None),
        };

        Self {
            method,
            args,
            min_date,
            max_date,
            total: 0,
            result: vec![],
        }
    }

    pub fn add(&mut self, date: DateTime<Tz>) -> bool {
        self.result.push(date);
        true
    }
}

impl IterResult for RRuleIterRes {
    // Returns tuple of flags indicating whether to add and continue
    // iteration (add_date, continue_iteration)
    fn accept(&self, date: &DateTime<Tz>) -> (bool, bool) {
        let too_early = match self.min_date {
            Some(d) => d > *date,
            None => false,
        };
        let too_late = match self.max_date {
            Some(d) => d < *date,
            None => false,
        };
        match self.method {
            QueryMethodTypes::Between if too_early => (false, true),
            QueryMethodTypes::Between if too_late => (false, false),
            QueryMethodTypes::Before if too_late => (false, false),
            QueryMethodTypes::After => (!too_early, too_early),
            _ => (true, true),
        }
    }
}

impl IntoIterator for RRule {
    type Item = DateTime<Tz>;

    type IntoIter = RRuleIter;

    fn into_iter(self) -> Self::IntoIter {
        let mut ii = IterInfo::new(self.options);
        let mut counter_date = ii.options.dtstart;
        ii.rebuild(counter_date.year() as isize, counter_date.month() as usize);

        let timeset = make_timeset(&ii, &counter_date, &ii.options);

        RRuleIter {
            counter_date,
            ii,
            timeset,
            remain: vec![],
            finished: false,
        }
    }
}
