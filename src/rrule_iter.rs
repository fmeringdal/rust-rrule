use crate::iter::IterResult;
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
    fn accept(&mut self, date: DateTime<Tz>) -> bool {
        self.total += 1;
        let too_early = self.min_date.is_some() && date < self.min_date.unwrap();
        let too_late = self.max_date.is_some() && date > self.max_date.unwrap();

        match self.method {
            QueryMethodTypes::Between if too_early => true,
            QueryMethodTypes::Between if too_late => false,
            QueryMethodTypes::Before if too_late => false,
            QueryMethodTypes::After if too_early => true,
            QueryMethodTypes::After => {
                self.add(date);
                false
            }
            _ => self.add(date),
        }
    }

    // before and after returns only one date whereas all and between an array
    fn get_value(&self) -> Vec<DateTime<Tz>> {
        match self.method {
            QueryMethodTypes::Between | QueryMethodTypes::All => self.result.clone(),
            _ => {
                if self.result.is_empty() {
                    return vec![];
                }
                vec![self.result[self.result.len() - 1].clone()]
            }
        }
    }
}
