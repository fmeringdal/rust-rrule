use chrono::prelude::*;

#[derive(Debug)]
pub struct YearInfo {
    pub yearlen: usize,
    pub nextyearlen: usize,
    pub yearordinal: isize,
    pub yearweekday: usize,
    pub mmask: Vec<usize>,
    pub mrange: Vec<usize>,
    pub mdaymask: Vec<isize>,
    pub nmdaymask: Vec<isize>,
    pub wdaymask: Vec<usize>,
    pub wnomask: Option<Vec<usize>>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Frequenzy {
    YEARLY = 0,
    MONTHLY = 1,
    WEEKLY = 2,
    DAILY = 3,
    HOURLY = 4,
    MINUTELY = 5,
    SECONDLY = 6,
}

#[derive(Debug)]
pub struct ParsedOptions {
    pub freq: Frequenzy,
    pub interval: usize,
    pub count: Option<u32>,
    pub until: Option<DateTime<Utc>>,
    pub tzid: Option<String>,
    pub dtstart: DateTime<Utc>,
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

impl ParsedOptions {
    pub fn new(freq: Frequenzy, dtstart: &DateTime<Utc>) -> Self {
        Self {
            freq,
            interval: 1,
            count: None,
            until: None,
            tzid: None,
            dtstart: *dtstart,
            wkst: 0,
            bysetpos: vec![],
            bymonth: vec![dtstart.month() as usize],
            bymonthday: vec![dtstart.day() as isize],
            bynmonthday: vec![],
            byyearday: vec![],
            byweekno: vec![],
            byweekday: vec![],
            bynweekday: vec![],
            byhour: vec![dtstart.hour() as usize],
            byminute: vec![dtstart.minute() as usize],
            bysecond: vec![dtstart.second() as usize],
            byeaster: None,
        }
    }

    pub fn freq(mut self, freq: Frequenzy) -> Self {
        self.freq = freq;
        self
    }
    pub fn interval(mut self, interval: usize) -> Self {
        self.interval = interval;
        self
    }
    pub fn until(mut self, until: &DateTime<Utc>) -> Self {
        self.until = Some(*until);
        self
    }
    pub fn tzid(mut self, tzid: &String) -> Self {
        self.tzid = Some(tzid.clone());
        self
    }
    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }
    pub fn dtstart(mut self, dtstart: DateTime<Utc>) -> Self {
        self.dtstart = dtstart;
        self
    }
    pub fn wkst(mut self, wkst: usize) -> Self {
        self.wkst = wkst;
        self
    }
    pub fn bysetpos(mut self, bysetpos: Vec<isize>) -> Self {
        self.bysetpos = bysetpos;
        self
    }
    pub fn bymonth(mut self, bymonth: Vec<usize>) -> Self {
        self.bymonth = bymonth;
        self
    }
    pub fn bymonthday(mut self, bymonthday: Vec<isize>) -> Self {
        self.bymonthday = bymonthday;
        self
    }
    pub fn bynmonthday(mut self, bynmonthday: Vec<isize>) -> Self {
        self.bynmonthday = bynmonthday;
        self
    }
    pub fn byyearday(mut self, byyearday: Vec<isize>) -> Self {
        self.byyearday = byyearday;
        self
    }
    pub fn byweekno(mut self, byweekno: Vec<isize>) -> Self {
        self.byweekno = byweekno;
        self
    }
    pub fn byweekday(mut self, byweekday: Vec<usize>) -> Self {
        self.byweekday = byweekday;
        self
    }
    pub fn bynweekday(mut self, bynweekday: Vec<Vec<isize>>) -> Self {
        self.bynweekday = bynweekday;
        self
    }
    pub fn byhour(mut self, byhour: Vec<usize>) -> Self {
        self.byhour = byhour;
        self
    }
    pub fn byminute(mut self, byminute: Vec<usize>) -> Self {
        self.byminute = byminute;
        self
    }
    pub fn bysecond(mut self, bysecond: Vec<usize>) -> Self {
        self.bysecond = bysecond;
        self
    }
}
