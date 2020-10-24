use crate::datetime::*;
use crate::easter::*;
use crate::monthinfo::*;
use crate::options::*;
use crate::yearinfo::*;
use chrono::prelude::*;

pub struct IterInfo<'a> {
    pub yearinfo: Option<YearInfo>,
    pub monthinfo: Option<MonthInfo>,
    pub eastermask: Option<Vec<isize>>,
    options: &'a ParsedOptions,
}

impl<'a> IterInfo<'a> {
    pub fn new(options: &'a ParsedOptions) -> Self {
        Self {
            options,
            yearinfo: None,
            monthinfo: None,
            eastermask: None,
        }
    }

    pub fn rebuild(&mut self, year: isize, month: usize) {
        if self.monthinfo.is_none() || year != self.monthinfo.as_ref().unwrap().lastyear {
            self.yearinfo = Some(rebuild_year(year as i32, self.options));
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
                    self.options,
                ));
            }
        }

        if let Some(byeaster) = self.options.byeaster {
            self.eastermask = Some(easter(year, byeaster));
        }
    }

    pub fn lastyear(&self) -> Option<isize> {
        match &self.monthinfo {
            Some(info) => Some(info.lastyear),
            None => None,
        }
    }
    pub fn lastmonth(&self) -> Option<usize> {
        match &self.monthinfo {
            Some(info) => Some(info.lastmonth),
            None => None,
        }
    }
    pub fn yearlen(&self) -> Option<usize> {
        match &self.yearinfo {
            Some(info) => Some(info.yearlen),
            None => None,
        }
    }
    pub fn yearordinal(&self) -> Option<isize> {
        match &self.yearinfo {
            Some(info) => Some(info.yearordinal),
            None => None,
        }
    }
    pub fn mrange(&self) -> Option<&Vec<usize>> {
        match &self.yearinfo {
            Some(info) => Some(&info.mrange),
            None => None,
        }
    }

    pub fn eastermask(&self) -> Option<&Vec<isize>> {
        match &self.eastermask {
            Some(mask) => Some(&mask),
            None => None,
        }
    }
    pub fn wdaymask(&self) -> Option<&Vec<usize>> {
        match &self.yearinfo {
            Some(info) => Some(&info.wdaymask),
            None => None,
        }
    }

    pub fn mmask(&self) -> Option<&Vec<usize>> {
        match &self.yearinfo {
            Some(info) => Some(&info.mmask),
            None => None,
        }
    }

    pub fn wnomask(&self) -> Option<&Vec<usize>> {
        match &self.yearinfo {
            Some(info) => match &info.wnomask {
                Some(mask) => Some(mask),
                None => None,
            },
            None => None,
        }
    }
    pub fn nwdaymask(&self) -> Option<&Vec<isize>> {
        match &self.monthinfo {
            Some(info) => Some(&info.nwdaymask),
            None => None,
        }
    }
    pub fn nextyearlen(&self) -> Option<usize> {
        match &self.yearinfo {
            Some(info) => Some(info.nextyearlen),
            None => None,
        }
    }
    pub fn mdaymask(&self) -> Option<&Vec<isize>> {
        match &self.yearinfo {
            Some(info) => Some(&info.mdaymask),
            None => None,
        }
    }
    pub fn nmdaymask(&self) -> Option<&Vec<isize>> {
        match &self.yearinfo {
            Some(info) => Some(&info.nmdaymask),
            None => None,
        }
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
        let mrange = self.mrange().unwrap();
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
            if self.wdaymask().unwrap()[i] == self.options.wkst {
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
        freq: &Frequenzy,
        year: isize,
        month: usize,
        day: usize,
    ) -> (Vec<usize>, usize, usize) {
        match freq {
            Frequenzy::Yearly => self.ydayset(),
            Frequenzy::Monthly => self.mdayset(month),
            Frequenzy::Weekly => self.wdayset(year, month, day),
            Frequenzy::Daily => self.ddayset(year, month, day),
            _ => self.ddayset(year, month, day),
        }
    }

    pub fn gettimeset(
        &self,
        freq: &Frequenzy,
        hour: usize,
        minute: usize,
        second: usize,
        millisecond: usize,
    ) -> Vec<Time> {
        match freq {
            Frequenzy::Hourly => self.htimeset(hour, minute, second, millisecond),
            Frequenzy::Minutely => self.mtimeset(hour, minute, second, millisecond),
            Frequenzy::Secondly => self.stimeset(hour, minute, second, millisecond),
            _ => panic!("Invalid freq"),
        }
    }
}
