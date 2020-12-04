use crate::datetime::{get_weekday_val, DTime};
use crate::parse_options::parse_options;
use chrono::prelude::*;
use chrono_tz::{Tz, UTC};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum Frequenzy {
    Yearly = 0,
    Monthly = 1,
    Weekly = 2,
    Daily = 3,
    Hourly = 4,
    Minutely = 5,
    Secondly = 6,
}

#[derive(Copy, Clone, Debug)]
pub struct NWeekday {
    pub weekday: usize,
    pub n: isize,
}

impl NWeekday {
    pub fn new(weekday: usize, n: isize) -> Self {
        // if (n === 0) throw new Error("Can't create weekday with n == 0")
        Self { weekday, n }
    }

    pub fn from(weekday: &Weekday, n: isize) -> Self {
        // if (n === 0) throw new Error("Can't create weekday with n == 0")
        Self {
            weekday: get_weekday_val(weekday),
            n,
        }
    }

    pub fn nth(&self, n: isize) -> Self {
        if self.n == n {
            return self.clone();
        }
        Self::new(self.weekday, n)
    }
}

impl PartialEq for NWeekday {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n && self.weekday == other.weekday
    }
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
    pub bynweekday: Vec<Vec<isize>>,
    pub byhour: Vec<usize>,
    pub byminute: Vec<usize>,
    pub bysecond: Vec<usize>,
    pub byeaster: Option<isize>,
}

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
    pub byyearday: Option<Vec<isize>>,
    pub byweekno: Option<Vec<isize>>,
    pub byweekday: Option<Vec<NWeekday>>,
    pub byhour: Option<Vec<usize>>,
    pub byminute: Option<Vec<usize>>,
    pub bysecond: Option<Vec<usize>>,
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
            byyearday: None,
            byweekno: None,
            byweekday: None,
            byhour: None,
            byminute: None,
            bysecond: None,
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
            byyearday: Self::is_some_or_none(&opt1.byyearday, &opt2.byyearday).clone(),
            byweekno: Self::is_some_or_none(&opt1.byweekno, &opt2.byweekno).clone(),
            byweekday: Self::is_some_or_none(&opt1.byweekday, &opt2.byweekday).clone(),
            byhour: Self::is_some_or_none(&opt1.byhour, &opt2.byhour).clone(),
            byminute: Self::is_some_or_none(&opt1.byminute, &opt2.byminute).clone(),
            bysecond: Self::is_some_or_none(&opt1.bysecond, &opt2.bysecond).clone(),
            byeaster: Self::is_some_or_none(&opt1.byeaster, &opt2.byeaster).clone(),
        }
    }

    /// The FREQ rule part identifies the type of recurrence rule.
    pub fn freq(mut self, freq: Frequenzy) -> Self {
        self.freq = Some(freq);
        self
    }

    /// The interval between each freq iteration.
    pub fn interval(mut self, interval: usize) -> Self {
        self.interval = Some(interval);
        self
    }

    /// If given, this determines how many occurrences will be generated.
    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    /// If given, this must be a datetime instance specifying the
    /// upper-bound limit of the recurrence.
    pub fn until(mut self, until: DateTime<Utc>) -> Self {
        self.until = Some(until.with_timezone(&UTC));
        self
    }

    /// The recurrence start. Recurrences generated by the rrule will
    /// be in the same time zone as the start date.
    pub fn dtstart(mut self, dtstart: DTime) -> Self {
        self.dtstart = Some(dtstart);
        self.tzid = Some(dtstart.timezone());
        self
    }

    /// The week start day. This will affect recurrences based on weekly periods. The default week start is Weekday::Mon.
    pub fn wkst(mut self, wkst: Weekday) -> Self {
        self.wkst = Some(get_weekday_val(&wkst));
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, positive or negative.
    /// Each given integer will specify an occurrence number, corresponding to the nth occurrence
    /// of the rule inside the frequency period. For example, a bysetpos of -1 if combined with
    /// a MONTHLY frequency, and a byweekday of (MO, TU, WE, TH, FR), will result in the last
    /// work day of every month.
    pub fn bysetpos(mut self, bysetpos: Vec<isize>) -> Self {
        self.bysetpos = Some(bysetpos);
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, meaning
    /// the months to apply the recurrence to.
    pub fn bymonth(mut self, bymonth: Vec<usize>) -> Self {
        self.bymonth = Some(bymonth);
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, meaning
    /// the month days to apply the recurrence to.
    pub fn bymonthday(mut self, bymonthday: Vec<isize>) -> Self {
        self.bymonthday = Some(bymonthday);
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, meaning
    /// the year days to apply the recurrence to.
    pub fn byyearday(mut self, byyearday: Vec<isize>) -> Self {
        self.byyearday = Some(byyearday);
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, meaning
    /// the week numbers to apply the recurrence to. Week numbers have the meaning
    /// described in ISO8601, that is, the first week of the year is that containing
    /// at least four days of the new year.
    pub fn byweekno(mut self, byweekno: Vec<isize>) -> Self {
        self.byweekno = Some(byweekno);
        self
    }

    /// If given, it must be either an integer (0 == MO), a sequence of integers, one
    /// of the weekday constants (MO, TU, etc), or a sequence of these constants.
    /// When given, these variables will define the weekdays where the recurrence
    /// will be applied.
    pub fn byweekday(mut self, byweekday: Vec<Weekday>) -> Self {
        let byweekday = byweekday
            .iter()
            .map(|w| get_weekday_val(w))
            .map(|w| NWeekday::new(w, 1))
            .collect();
        self.byweekday = Some(byweekday);
        self
    }

    /// If given, it must be either an integer, or a sequence of integers,
    /// meaning the hours to apply the recurrence to.
    pub fn byhour(mut self, byhour: Vec<usize>) -> Self {
        self.byhour = Some(byhour);
        self
    }

    /// If given, it must be either an integer, or a sequence of integers,
    /// meaning the minutes to apply the recurrence to.
    pub fn byminute(mut self, byminute: Vec<usize>) -> Self {
        self.byminute = Some(byminute);
        self
    }

    /// If given, it must be either an integer, or a sequence of integers,
    /// meaning the seconds to apply the recurrence to.
    pub fn bysecond(mut self, bysecond: Vec<usize>) -> Self {
        self.bysecond = Some(bysecond);
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, positive or negative.
    /// Each integer will define an offset from the Easter Sunday. Passing the offset 0 to
    /// byeaster will yield the Easter Sunday itself. This is an extension to the RFC specification.
    pub fn byeaster(mut self, byeaster: isize) -> Self {
        self.byeaster = Some(byeaster);
        self
    }

    /// Parses the opptions and build `ParsedOptions` if they are valid.
    /// Otherwise an `RRuleParseError` will be returned.
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

impl Error for RRuleParseError {}

pub fn weekday_from_str(val: &str) -> Result<Weekday, String> {
    match val {
        "MO" => Ok(Weekday::Mon),
        "TU" => Ok(Weekday::Tue),
        "WE" => Ok(Weekday::Wed),
        "TH" => Ok(Weekday::Thu),
        "FR" => Ok(Weekday::Fri),
        "SA" => Ok(Weekday::Sat),
        "SU" => Ok(Weekday::Sun),
        _ => Err(format!("Invalid weekday: {}", val)),
    }
}
