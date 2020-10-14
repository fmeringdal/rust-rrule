use crate::monthinfo::*;
use crate::yearinfo::*;

struct IterInfo<'a> {
    pub yearinfo: Option<YearInfo>,
    pub monthinfo: Option<MonthInfo>,
    options: &'a ParsedOptions,
}

impl<'a> IterInfo<'a> {
    pub fn new(options: &'a ParsedOptions) -> Self {
        Self {
            options,
            yearinfo: None,
            monthinfo: None,
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
    pub fn mdaymask(&self) -> Option<&Vec<usize>> {
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
}
