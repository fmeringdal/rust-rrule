use crate::iter::*;
use crate::iter_set::{iter_v2, IterResult};
use crate::options::*;
use crate::rrule::*;
use chrono::prelude::*;
use chrono_tz::{Tz, UTC};
use std::collections::HashMap;

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
#[derive(Debug)]
pub struct RRuleSet {
    rrule: Vec<RRule>,
    rdate: Vec<DateTime<Utc>>,
    exrule: Vec<RRule>,
    exdate: Vec<DateTime<Utc>>,
    pub dtstart: Option<DateTime<Utc>>,
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
            before: None,
            after: None,
            dt: None,
        };
        let iter_res = RRuleIterRes::new(QueryMethodTypes::ALL, iter_args);

        Self {
            exdate_hash: HashMap::new(),
            iter_res,
            rrule_set,
        }
    }

    pub fn from_before(rrule_set: &'a mut RRuleSet, dt: DateTime<Tz>, inc: bool) -> Self {
        let iter_args = IterArgs {
            inc,
            before: None,
            after: None,
            dt: Some(dt),
        };
        let iter_res = RRuleIterRes::new(QueryMethodTypes::BEFORE, iter_args);

        Self {
            exdate_hash: HashMap::new(),
            iter_res,
            rrule_set,
        }
    }

    pub fn from_after(rrule_set: &'a mut RRuleSet, dt: DateTime<Tz>, inc: bool) -> Self {
        let iter_args = IterArgs {
            inc,
            before: None,
            after: None,
            dt: Some(dt),
        };
        let iter_res = RRuleIterRes::new(QueryMethodTypes::AFTER, iter_args);

        Self {
            exdate_hash: HashMap::new(),
            iter_res,
            rrule_set,
        }
    }

    pub fn from_between(
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
        let iter_res = RRuleIterRes::new(QueryMethodTypes::BETWEEN, iter_args);

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
        iter.iter(None)
    }

    /// Returns the last recurrence before the given datetime instance.
    /// The inc keyword defines what happens if dt is an occurrence.
    /// With inc == true, if dt itself is an occurrence, it will be returned.
    pub fn before(&mut self, date: DateTime<Tz>, inc: bool) -> Vec<DateTime<Tz>> {
        let mut iter = RRuleSetIter::from_before(self, date, inc);
        iter.iter(None)
    }

    /// Returns the last recurrence after the given datetime instance.
    /// The inc keyword defines what happens if dt is an occurrence.
    /// With inc == true, if dt itself is an occurrence, it will be returned.
    pub fn after(&mut self, date: DateTime<Tz>, inc: bool) -> Vec<DateTime<Tz>> {
        let mut iter = RRuleSetIter::from_after(self, date, inc);
        iter.iter(None)
    }

    /// Returns all the occurrences of the rrule between after and before.
    /// The inc keyword defines what happens if after and/or before are
    /// themselves occurrences. With inc == True, they will be included in the
    /// list, if they are found in the recurrence set.
    pub fn between(
        &mut self,
        after: DateTime<Tz>,
        before: DateTime<Tz>,
        inc: bool,
    ) -> Vec<DateTime<Tz>> {
        let mut iter = RRuleSetIter::from_between(self, after, before, inc);
        iter.iter(None)
    }

    pub fn value_of(&mut self) -> Vec<String> {
        let mut result = vec![];

        if !self.rrule.is_empty() && self.dtstart.is_some() {
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
        println!("Acutal: {:?}", actual_dates);
        assert_eq!(
            actual_dates.len(),
            expected_dates.len(),
            "Expected number of returned dates to be equal to the expected"
        );

        for (actual, exptected) in actual_dates.into_iter().zip(expected_dates) {
            assert_eq!(actual, exptected);
        }
    }

    #[test]
    fn rrule_and_exrule() {
        let mut set = RRuleSet::new();

        let options1 = ParsedOptions {
            freq: Frequenzy::Yearly,
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
        let options2 = ParsedOptions {
            freq: Frequenzy::Yearly,
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

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
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

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
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

    #[test]
    fn rrule_and_exyearly_yearly_big() {
        let mut set = RRuleSet::new();

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(13),
            bymonth: vec![9],
            dtstart: Utc.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
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

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(10),
            bymonth: vec![9],
            dtstart: Utc.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
            byeaster: None,
        };
        let rrule = RRule::new(options);
        set.exrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(2007, 9, 2, 9, 0, 0),
                ymd_hms_2(2008, 9, 2, 9, 0, 0),
                ymd_hms_2(2009, 9, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn before() {
        let mut set = RRuleSet::new();

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: None,
            bymonth: vec![9],
            dtstart: Utc.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
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

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(10),
            bymonth: vec![9],
            dtstart: Utc.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
            byeaster: None,
        };
        let rrule = RRule::new(options);
        set.exrule(rrule);

        test_recurring(
            set.before(ymd_hms_2(2015, 9, 2, 9, 0, 0), false),
            vec![ymd_hms_2(2014, 9, 2, 9, 0, 0)],
        );
    }

    #[test]
    fn after() {
        let mut set = RRuleSet::new();

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: None,
            bymonth: vec![9],
            dtstart: Utc.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
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

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(10),
            bymonth: vec![9],
            dtstart: Utc.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
            byeaster: None,
        };
        let rrule = RRule::new(options);
        set.exrule(rrule);

        test_recurring(
            set.after(ymd_hms_2(2000, 9, 2, 9, 0, 0), false),
            vec![ymd_hms_2(2007, 9, 2, 9, 0, 0)],
        );
    }

    #[test]
    fn between() {
        let mut set = RRuleSet::new();

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: None,
            bymonth: vec![9],
            dtstart: Utc.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
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

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(10),
            bymonth: vec![9],
            dtstart: Utc.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![2],
            bynweekday: vec![],
            bynmonthday: vec![],
            until: None,
            wkst: 0,
            tzid: None,
            interval: 1,
            byeaster: None,
        };
        let rrule = RRule::new(options);
        set.exrule(rrule);

        test_recurring(
            set.between(
                ymd_hms_2(2000, 9, 2, 9, 0, 0),
                ymd_hms_2(2010, 9, 2, 9, 0, 0),
                false,
            ),
            vec![
                ymd_hms_2(2007, 9, 2, 9, 0, 0),
                ymd_hms_2(2008, 9, 2, 9, 0, 0),
                ymd_hms_2(2009, 9, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn before_70s() {
        let mut set = RRuleSet::new();

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(2),
            bymonth: vec![1],
            dtstart: Utc.ymd(1960, 1, 1).and_hms(9, 0, 0),
            byweekday: vec![],
            byhour: vec![9],
            bysetpos: vec![],
            byweekno: vec![],
            byminute: vec![0],
            bysecond: vec![0],
            byyearday: vec![],
            bymonthday: vec![1],
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

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1960, 1, 1, 9, 0, 0),
                ymd_hms_2(1961, 1, 1, 9, 0, 0),
            ],
        );
    }
}
