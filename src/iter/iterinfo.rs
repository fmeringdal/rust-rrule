use crate::datetime::{to_ordinal, Time};
use crate::iter::easter::easter;
use crate::iter::monthinfo::{rebuild_month, MonthInfo};
use crate::iter::yearinfo::{rebuild_year, YearInfo};
use crate::options::{Frequency, ParsedOptions};
use chrono::prelude::*;

pub struct IterInfo<'a> {
    pub yearinfo: Option<YearInfo>,
    pub monthinfo: Option<MonthInfo>,
    pub eastermask: Option<Vec<isize>>,
    pub options: &'a ParsedOptions,
}

impl<'a> IterInfo<'a> {
    pub fn new(options: &'a ParsedOptions) -> Self {
        let mut ii = Self {
            options,
            yearinfo: None,
            monthinfo: None,
            eastermask: None,
        };
        let counter_date = ii.options.dtstart;
        ii.rebuild(counter_date.year() as isize, counter_date.month() as usize);

        ii
    }

    pub fn rebuild(&mut self, year: isize, month: usize) {
        if self.monthinfo.is_none() || year != self.monthinfo.as_ref().unwrap().lastyear {
            self.yearinfo = Some(rebuild_year(year as i32, &self.options));
        }

        if !self.options.bynweekday.is_empty()
            && ((self.monthinfo.is_none() || month != self.monthinfo.as_ref().unwrap().lastmonth)
                || (self.monthinfo.is_none() || year != self.monthinfo.as_ref().unwrap().lastyear))
        {
            if let Some(yearinfo) = &self.yearinfo {
                self.monthinfo = Some(rebuild_month(
                    year,
                    month,
                    yearinfo.yearlen,
                    &yearinfo.mrange,
                    &yearinfo.wdaymask,
                    &self.options,
                ));
            }
        }

        if let Some(byeaster) = self.options.byeaster {
            self.eastermask = Some(easter(year, byeaster));
        }
    }

    pub fn lastyear(&self) -> Option<isize> {
        self.monthinfo.as_ref().map(|info| info.lastyear)
    }
    pub fn lastmonth(&self) -> Option<usize> {
        self.monthinfo.as_ref().map(|info| info.lastmonth)
    }

    pub fn yearlen(&self) -> Option<usize> {
        self.yearinfo.as_ref().map(|info| info.yearlen)
    }

    pub fn yearordinal(&self) -> Option<isize> {
        self.yearinfo.as_ref().map(|info| info.yearordinal)
    }

    pub fn mrange(&self) -> &[usize] {
        self.yearinfo.as_ref().map(|info| &info.mrange).unwrap()
    }

    pub fn eastermask(&self) -> Option<&Vec<isize>> {
        self.eastermask.as_ref()
    }

    pub fn wdaymask(&self) -> &[usize] {
        self.yearinfo.as_ref().map(|info| &info.wdaymask).unwrap()
    }

    pub fn mmask(&self) -> &[usize] {
        self.yearinfo.as_ref().map(|info| &info.mmask).unwrap()
    }

    pub fn wnomask(&self) -> Option<&Vec<usize>> {
        match &self.yearinfo {
            Some(info) => info.wnomask.as_ref(),
            None => None,
        }
    }

    pub fn nwdaymask(&self) -> Option<&Vec<isize>> {
        self.monthinfo.as_ref().map(|info| &info.nwdaymask)
    }

    pub fn nextyearlen(&self) -> Option<usize> {
        self.yearinfo.as_ref().map(|info| info.nextyearlen)
    }

    pub fn mdaymask(&self) -> &[isize] {
        self.yearinfo.as_ref().unwrap().mdaymask
    }

    pub fn nmdaymask(&self) -> &[isize] {
        self.yearinfo.as_ref().unwrap().nmdaymask
    }

    pub fn ydayset(&self) -> (Vec<usize>, usize, usize) {
        let yearlen = self.yearlen().unwrap();
        let mut v = Vec::with_capacity(yearlen);
        for i in 0..yearlen {
            v.push(i);
        }
        (v, 0, yearlen)
    }

    pub fn mdayset(&self, month: usize) -> (Vec<usize>, usize, usize) {
        let mrange = self.mrange();
        let start = mrange[month - 1];
        let end = mrange[month];
        let mut set = vec![0; self.yearlen().unwrap()];
        for i in start..end {
            set[i] = i;
        }
        (set, start, end)
    }

    pub fn wdayset(&self, year: isize, month: usize, day: usize) -> (Vec<usize>, usize, usize) {
        let set_len = self.yearlen().unwrap() + 7;
        let mut set = vec![0; set_len];

        let mut i = (to_ordinal(
            &Utc.ymd(year as i32, month as u32, day as u32)
                .and_hms(0, 0, 0),
        ) - self.yearordinal().unwrap()) as usize;

        let start = i;
        for _ in 0..7 {
            if i >= set_len {
                break;
            }
            set[i] = i;
            i += 1;
            if self.wdaymask()[i] == self.options.wkst {
                break;
            }
        }
        (set, start, i)
    }

    pub fn ddayset(&self, year: isize, month: usize, day: usize) -> (Vec<usize>, usize, usize) {
        let mut set = vec![0; self.yearlen().unwrap()];

        let i = (to_ordinal(
            &Utc.ymd(year as i32, month as u32, day as u32)
                .and_hms(0, 0, 0),
        ) - self.yearordinal().unwrap()) as usize;

        set[i] = i;
        (set, i, i + 1)
    }

    pub fn htimeset(&self, hour: usize, _: usize, second: usize, millisecond: usize) -> Vec<Time> {
        let mut set = self
            .options
            .byminute
            .iter()
            .map(|minute| self.mtimeset(hour, *minute, second, millisecond))
            .flatten()
            .collect::<Vec<Time>>();
        set.sort_by_key(|a| a.time());
        set
    }

    pub fn mtimeset(&self, hour: usize, minute: usize, _: usize, millisecond: usize) -> Vec<Time> {
        let mut set = self
            .options
            .bysecond
            .iter()
            .map(|second| Time::new(hour, minute, *second, millisecond))
            .collect::<Vec<Time>>();
        set.sort_by_key(|a| a.time());
        set
    }

    pub fn stimeset(
        &self,
        hour: usize,
        minute: usize,
        second: usize,
        millisecond: usize,
    ) -> Vec<Time> {
        vec![Time::new(hour, minute, second, millisecond)]
    }

    pub fn getdayset(
        &self,
        freq: &Frequency,
        year: isize,
        month: usize,
        day: usize,
    ) -> (Vec<usize>, usize, usize) {
        match freq {
            Frequency::Yearly => self.ydayset(),
            Frequency::Monthly => self.mdayset(month),
            Frequency::Weekly => self.wdayset(year, month, day),
            Frequency::Daily => self.ddayset(year, month, day),
            _ => self.ddayset(year, month, day),
        }
    }

    pub fn gettimeset(
        &self,
        freq: &Frequency,
        hour: usize,
        minute: usize,
        second: usize,
        millisecond: usize,
    ) -> Vec<Time> {
        match freq {
            Frequency::Hourly => self.htimeset(hour, minute, second, millisecond),
            Frequency::Minutely => self.mtimeset(hour, minute, second, millisecond),
            Frequency::Secondly => self.stimeset(hour, minute, second, millisecond),
            _ => panic!("Invalid freq"),
        }
    }
}
