use crate::iter::*;
use crate::iter_set::{iter_v2, IterResult};
use crate::options::*;
use crate::rrule::*;
use chrono::prelude::*;
use chrono_tz::{Tz, UTC};
use std::collections::HashMap;

struct RRuleSet {
    rrule: Vec<RRule>,
    rdate: Vec<DateTime<Utc>>,
    exrule: Vec<RRule>,
    exdate: Vec<DateTime<Utc>>,
    dtstart: Option<DateTime<Utc>>,
    exdate_hash: HashMap<i64, ()>,
}

struct RRuleSetIter<'a> {
    exdate_hash: HashMap<i64, ()>,
    iter_res: RRuleIterRes,
    rrule_set: &'a mut RRuleSet,
}

impl<'a> RRuleSetIter<'a> {
    pub fn new(rrule_set: &'a mut RRuleSet) -> Self {
        let iter_args = IterArgs {
            inc: true,
            before: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            after: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
            dt: UTC.ymd(2020, 1, 1).and_hms(0, 0, 0),
        };
        let iter_res = RRuleIterRes::new(QueryMethodTypes::ALL, iter_args);

        Self {
            exdate_hash: HashMap::new(),
            iter_res,
            rrule_set,
        }
    }

    pub fn eval_exdate(&mut self, after: &DateTime<Tz>, before: &DateTime<Tz>) {
        for rrule in self.rrule_set.exrule.iter_mut() {
            for date in rrule.between(after, before, true) {
                self.exdate_hash.insert(date.timestamp(), ());
            }
        }
    }

    fn accept_2(&mut self, date: DateTime<Tz>) -> bool {
        let dt = date.timestamp();
        if !self.exdate_hash.contains_key(&dt) {
            self.eval_exdate(&UTC.timestamp(dt - 1, 0), &UTC.timestamp(dt + 1, 0));
            if !self.exdate_hash.contains_key(&dt) {
                self.exdate_hash.insert(dt, ());
                return self.iter_res.accept(date.clone());
            }
        }

        true
    }

    fn accept_1(&mut self, date: DateTime<Tz>) -> bool {
        let dt = date.timestamp();
        if !self.exdate_hash.contains_key(&dt) {
            if !self.exdate_hash.contains_key(&dt) {
                self.exdate_hash.insert(dt, ());
                return self.iter_res.accept(date.clone());
            }
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
            QueryMethodTypes::BETWEEN => {
                self.eval_exdate(
                    &self.iter_res.args.after.clone(),
                    &self.iter_res.args.before.clone(),
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
            iter_v2(self, &mut rule.options);
        }

        let mut res = self.iter_res.get_value();
        res.sort();
        res
    }
}

impl<'a> IterResult for RRuleSetIter<'a> {
    fn accept(&mut self, date: DateTime<Tz>) -> bool {
        match &self.iter_res.method {
            QueryMethodTypes::BETWEEN => self.accept_1(date),
            _ => self.accept_2(date),
        }
    }

    fn get_value(&self) -> Vec<DateTime<Tz>> {
        self.iter_res.get_value()
    }
}

impl RRuleSet {
    pub fn new() -> Self {
        Self {
            rrule: vec![],
            rdate: vec![],
            exrule: vec![],
            exdate: vec![],
            dtstart: None,
            exdate_hash: HashMap::new(),
        }
    }

    pub fn rrule(&mut self, rrule: RRule) {
        self.rrule.push(rrule);
    }

    pub fn exrule(&mut self, rrule: RRule) {
        self.exrule.push(rrule);
    }

    pub fn rdate(&mut self, rdate: DateTime<Utc>) {
        self.rdate.push(rdate);
    }

    pub fn exdate(&mut self, exdate: DateTime<Utc>) {
        self.exdate.push(exdate);
    }

    pub fn all(&mut self) -> Vec<DateTime<Tz>> {
        let mut iter = RRuleSetIter::new(self);
        // self.iter(&mut iter_res, None)
        iter.iter(None)
    }

    pub fn value_of(&mut self) -> Vec<String> {
        let mut result = vec![];

        if !self.rrule.is_empty() && self.dtstart.is_some() {
            //result = result.concat(optionsToString({ dtstart: this._dtstart }))
            result.push(String::from("yeah"));
        }

        for rrule in &self.rrule {
            // result = result.concat(rrule.toString().split('\n'))
            result.push(String::from("test"));
        }

        for exrule in &self.exrule {
            //result = result.concat(
            //exrule.toString().split('\n')
            //.map(line => line.replace(/^RRULE:/, 'EXRULE:'))
            //.filter(line => !/^DTSTART/.test(line))
            //)
            result.push(String::from("hi"));
        }

        if !self.rdate.is_empty() {
            //result.push(
            //rdatesToString('RDATE', this._rdate, this.tzid())
            //)
        }

        if !self.exdate.is_empty() {
            //result.push(
            //rdatesToString('EXDATE', this._exdate, this.tzid())
            //)
        }

        result
    }
}

#[cfg(test)]
mod test_iter_set {
    use super::*;

    fn ymd_hms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> DateTime<Utc> {
        Utc.ymd(year, month, day).and_hms(hour, minute, second)
    }

    fn ymd_hms_2(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,    
        second: u32,
    ) -> DateTime<Tz> {
        UTC.ymd(year, month, day).and_hms(hour, minute, second)
    }

    fn test_recurring(actual_dates: Vec<DateTime<Tz>>, expected_dates: Vec<DateTime<Tz>>) {
        assert_eq!(
            actual_dates.len(),
            expected_dates.len(),
            "Expected number of returned dates to be equal to the expected"
        );

        println!("Acutal: {:?}", actual_dates);
        for (actual, exptected) in actual_dates.into_iter().zip(expected_dates) {
            assert_eq!(actual, exptected);
        }
    }

    #[test]
    fn rrule_and_exrule() {
        let mut set = RRuleSet::new();

        let mut options1 = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(6),
            bymonth: vec![],
            dtstart: Utc.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
            byeaster: None,
        };
        let rrule = RRule::new(options1);
        set.rrule(rrule);
        let mut options2 = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: Utc.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
            byeaster: None,
        };
        let exrule = RRule::new(options2);
        set.exrule(exrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1997, 9, 2, 9, 0, 0),
                ymd_hms_2(1997, 9, 9, 9, 0, 0),
                ymd_hms_2(1997, 9, 16, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn setdate_and_exdate() {
        let mut set = RRuleSet::new();

        set.rdate(ymd_hms(1997, 9, 2, 9, 0, 0));
        set.rdate(ymd_hms(1997, 9, 4, 9, 0, 0));
        set.rdate(ymd_hms(1997, 9, 9, 9, 0, 0));
        set.rdate(ymd_hms(1997, 9, 11, 9, 0, 0));
        set.rdate(ymd_hms(1997, 9, 16, 9, 0, 0));
        set.rdate(ymd_hms(1997, 9, 18, 9, 0, 0));

        set.exdate(ymd_hms(1997, 9, 4, 9, 0, 0));
        set.exdate(ymd_hms(1997, 9, 11, 9, 0, 0));
        set.exdate(ymd_hms(1997, 9, 18, 9, 0, 0));

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1997, 9, 2, 9, 0, 0),
                ymd_hms_2(1997, 9, 9, 9, 0, 0),
                ymd_hms_2(1997, 9, 16, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn setdate_and_exrule() {
        let mut set = RRuleSet::new();

        set.rdate(ymd_hms(1997, 9, 2, 9, 0, 0));
        set.rdate(ymd_hms(1997, 9, 4, 9, 0, 0));
        set.rdate(ymd_hms(1997, 9, 9, 9, 0, 0));
        set.rdate(ymd_hms(1997, 9, 11, 9, 0, 0));
        set.rdate(ymd_hms(1997, 9, 16, 9, 0, 0));
        set.rdate(ymd_hms(1997, 9, 18, 9, 0, 0));

        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(3),
            bymonth: vec![],
            dtstart: Utc.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
            byeaster: None,
        };
        let exrrule = RRule::new(options);
        set.exrule(exrrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1997, 9, 2, 9, 0, 0),
                ymd_hms_2(1997, 9, 9, 9, 0, 0),
                ymd_hms_2(1997, 9, 16, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn rrule_and_exdate() {
        let mut set = RRuleSet::new();

        let mut options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            count: Some(6),
            bymonth: vec![],
            dtstart: Utc.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
            byeaster: None,
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        set.exdate(ymd_hms(1997, 9, 2, 9, 0, 0));
        set.exdate(ymd_hms(1997, 9, 4, 9, 0, 0));
        set.exdate(ymd_hms(1997, 9, 9, 9, 0, 0));

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1997, 9, 11, 9, 0, 0),
                ymd_hms_2(1997, 9, 16, 9, 0, 0),
                ymd_hms_2(1997, 9, 18, 9, 0, 0),
            ],
        );
    }
}
