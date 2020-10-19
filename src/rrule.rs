use crate::iter::*;
use crate::options::*;
use chrono::prelude::*;
use chrono_tz::{Tz, UTC};

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
            before: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            after: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            dt: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            _value: Some(vec![]),
        };
        let mut iter_res = IterResult::new(QueryMethodTypes::ALL, iter_args);

        let res = iter(&mut iter_res, &mut self.options);
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
            before: before.clone(),
            after: after.clone(),
            dt: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            _value: Some(vec![]),
        };
        let mut iter_res = IterResult::new(QueryMethodTypes::ALL, iter_args);

        let res = iter(&mut iter_res, &mut self.options);
        res
    }
}
