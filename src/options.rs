use chrono::prelude::*;
use chrono_tz::Tz;
use crate::datetime::{DTime, get_weekday_val};
use crate::parse_options::parse_options;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Frequenzy {
    Yearly = 0,
    Monthly = 1,
    Weekly = 2,
    Daily = 3,
    Hourly = 4,
    Minutely = 5,
    Secondly = 6,
}

#[derive(Debug, Clone)]
pub struct ParsedOptions {
    pub freq: Frequenzy,
    pub interval: usize,
    pub count: Option<u32>,
    pub until: Option<DTime>,
    pub tzid: Tz,
    pub dtstart: DTime,
    pub wkst: usize,
    pub bysetpos: Vec<isize>,
    pub bymonth: Vec<usize>,
    pub bymonthday: Vec<isize>,
    pub bynmonthday: Vec<isize>,
    pub byyearday: Vec<isize>,
    pub byweekno: Vec<isize>,
    pub byweekday: Vec<usize>,
    pub byhour: Vec<usize>,
    pub byminute: Vec<usize>,
    pub bysecond: Vec<usize>,
    pub bynweekday: Vec<Vec<isize>>,
    pub byeaster: Option<isize>,
}

// TODO: PartialOptions shouldnt have all of these fields
#[derive(Debug, Clone)]
pub struct Options {
    pub freq: Option<Frequenzy>,
    pub interval: Option<usize>,
    pub count: Option<u32>,
    pub until: Option<DTime>,
    pub tzid: Option<Tz>,
    pub dtstart: Option<DTime>,
    pub wkst: Option<usize>,
    pub bysetpos: Option<Vec<isize>>,
    pub bymonth: Option<Vec<usize>>,
    pub bymonthday: Option<Vec<isize>>,
    pub bynmonthday: Option<Vec<isize>>,
    pub byyearday: Option<Vec<isize>>,
    pub byweekno: Option<Vec<isize>>,
    pub byweekday: Option<Vec<usize>>,
    pub byhour: Option<Vec<usize>>,
    pub byminute: Option<Vec<usize>>,
    pub bysecond: Option<Vec<usize>>,
    pub bynweekday: Option<Vec<Vec<isize>>>,
    pub byeaster: Option<isize>,
}

impl Options {
    pub fn new() -> Self {
        Self {
            freq: None,
            interval: None,
            count: None,
            until: None,
            tzid: None,
            dtstart: None,
            wkst: None,
            bysetpos: None,
            bymonth: None,
            bymonthday: None,
            bynmonthday: None,
            byyearday: None,
            byweekno: None,
            byweekday: None,
            byhour: None,
            byminute: None,
            bysecond: None,
            bynweekday: None,
            byeaster: None,
        }
    }

    // TODO: better name
    fn is_some_or_none<'a, T>(prop1: &'a Option<T>, prop2: &'a Option<T>) -> &'a Option<T> {
        if prop2.is_some() {
            return prop2;
        } 
        prop1
    }

    pub fn concat(opt1: &Self, opt2: &Self) -> Self {
        Self {
            freq: Self::is_some_or_none(&opt1.freq, &opt2.freq).clone(),
            interval: Self::is_some_or_none(&opt1.interval, &opt2.interval).clone(),
            count: Self::is_some_or_none(&opt1.count, &opt2.count).clone(),
            until: Self::is_some_or_none(&opt1.until, &opt2.until).clone(),
            tzid: Self::is_some_or_none(&opt1.tzid, &opt2.tzid).clone(),
            dtstart: Self::is_some_or_none(&opt1.dtstart, &opt2.dtstart).clone(),
            wkst: Self::is_some_or_none(&opt1.wkst, &opt2.wkst).clone(),
            bysetpos: Self::is_some_or_none(&opt1.bysetpos, &opt2.bysetpos).clone(),
            bymonth: Self::is_some_or_none(&opt1.bymonth, &opt2.bymonth).clone(),
            bymonthday: Self::is_some_or_none(&opt1.bymonthday, &opt2.bymonthday).clone(),
            bynmonthday: Self::is_some_or_none(&opt1.bynmonthday, &opt2.bynmonthday).clone(),
            byyearday: Self::is_some_or_none(&opt1.byyearday, &opt2.byyearday).clone(),
            byweekno: Self::is_some_or_none(&opt1.byweekno, &opt2.byweekno).clone(),
            byweekday: Self::is_some_or_none(&opt1.byweekday, &opt2.byweekday).clone(),
            byhour: Self::is_some_or_none(&opt1.byhour, &opt2.byhour).clone(),
            byminute: Self::is_some_or_none(&opt1.byminute, &opt2.byminute).clone(),
            bysecond: Self::is_some_or_none(&opt1.bysecond, &opt2.bysecond).clone(),
            bynweekday: Self::is_some_or_none(&opt1.bynweekday, &opt2.bynweekday).clone(),
            byeaster: Self::is_some_or_none(&opt1.byeaster, &opt2.byeaster).clone(),
        }
    }

    pub fn freq(mut self, freq: Frequenzy) -> Self {
        self.freq = Some(freq);
        self
    }

    pub fn interval(mut self, interval: usize) -> Self {
        self.interval = Some(interval);
        self
    }

    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    pub fn until(mut self, until: DTime) -> Self {
        self.until = Some(until);
        self
    }

    pub fn dtstart(mut self, dtstart: DTime) -> Self {
        self.dtstart = Some(dtstart);
        self.tzid = Some(dtstart.timezone());
        self
    }

    pub fn wkst(mut self, wkst: Weekday) -> Self {
        self.wkst = Some(get_weekday_val(&wkst));
        self
    }

    pub fn bysetpos(mut self, bysetpos: Vec<isize>) -> Self {
        self.bysetpos = Some(bysetpos);
        self
    }

    pub fn bymonth(mut self, bymonth: Vec<usize>) -> Self {
        self.bymonth = Some(bymonth);
        self
    }

    pub fn bymonthday(mut self, bymonthday: Vec<isize>) -> Self {
        self.bymonthday = Some(bymonthday);
        self
    }

    pub fn byyearday(mut self, byyearday: Vec<isize>) -> Self {
        self.byyearday = Some(byyearday);
        self
    }

    pub fn byweekno(mut self, byweekno: Vec<isize>) -> Self {
        self.byweekno = Some(byweekno);
        self
    }

    pub fn byweekday(mut self, byweekday: Vec<Weekday>) -> Self {
        let byweekday = byweekday.iter().map(|w| get_weekday_val(w)).collect(); 
        self.byweekday = Some(byweekday);
        self
    }

    pub fn byhour(mut self, byhour: Vec<usize>) -> Self {
        self.byhour = Some(byhour);
        self
    }

    pub fn byminute(mut self, byminute: Vec<usize>) -> Self {
        self.byminute = Some(byminute);
        self
    }

    pub fn bysecond(mut self, bysecond: Vec<usize>) -> Self {
        self.bysecond = Some(bysecond);
        self
    }

    pub fn byeaster(mut self, byeaster: isize) -> Self {
        self.byeaster = Some(byeaster);
        self
    }

    pub fn build(self) -> Result<ParsedOptions, RRuleParseError> {
        parse_options(&self)
    }
}

#[derive(Debug, Clone)]
pub struct RRuleParseError(pub String);

impl Display for RRuleParseError {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "Encountered parsing error: {}", self.0)
    }
}

impl Error for RRuleParseError{}