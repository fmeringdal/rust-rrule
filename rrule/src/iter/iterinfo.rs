#[cfg(feature = "by-easter")]
use super::easter::easter;
use super::{
    monthinfo::{rebuild_month, MonthInfo},
    utils::to_ordinal,
    yearinfo::{rebuild_year, YearInfo},
};
use crate::{core::Time, Frequency, NWeekday, RRule, RRuleError, RRuleProperties};
use chrono::{Datelike, TimeZone};

#[derive(Debug, Clone)]
pub(crate) struct IterInfo<'a> {
    year_info: Option<YearInfo>,
    month_info: Option<MonthInfo>,
    easter_mask: Option<Vec<isize>>,
    rrule: &'a RRule,
}

impl<'a> IterInfo<'a> {
    /// Only used to create a dummy instance of this because
    /// `into_iter` does not return an error.
    pub(crate) fn new_no_rebuild(rrule: &'a RRule) -> Self {
        Self {
            rrule,
            year_info: None,
            month_info: None,
            easter_mask: None,
        }
    }

    pub fn new(rrule: &'a RRule) -> Result<Self, RRuleError> {
        let mut ii = Self {
            rrule,
            year_info: None,
            month_info: None,
            easter_mask: None,
        };
        let counter_date = &ii.rrule.dt_start;
        ii.rebuild(counter_date.year(), counter_date.month() as u8)?;

        Ok(ii)
    }

    pub fn rebuild(&mut self, year: i32, month: u8) -> Result<(), RRuleError> {
        if self.month_info.is_none() || year != self.month_info.as_ref().unwrap().last_year {
            self.year_info = Some(rebuild_year(year, self.rrule.get_properties())?);
        }

        let by_weekday_nth_only = self
            .rrule
            .get_properties()
            .by_weekday
            .iter()
            .filter(|by_weekday| match by_weekday {
                NWeekday::Every(_) => false,
                NWeekday::Nth(_, _) => true,
            })
            .count();

        if by_weekday_nth_only != 0
            && ((self.month_info.is_none()
                || month != self.month_info.as_ref().unwrap().last_month)
                || (self.month_info.is_none()
                    || year != self.month_info.as_ref().unwrap().last_year))
        {
            if let Some(year_info) = &self.year_info {
                self.month_info = Some(rebuild_month(
                    year,
                    month,
                    year_info.year_len,
                    year_info.month_range,
                    year_info.weekday_mask,
                    self.rrule.get_properties(),
                )?);
            }
        }

        #[cfg(feature = "by-easter")]
        if let Some(by_easter) = self.rrule.get_properties().by_easter {
            self.easter_mask = Some(easter(year, by_easter)?);
        }
        Ok(())
    }

    // pub fn last_year(&self) -> Option<i32> {
    //     self.month_info.as_ref().map(|info| info.last_year)
    // }
    // pub fn last_month(&self) -> Option<u8> {
    //     self.month_info.as_ref().map(|info| info.last_month)
    // }

    pub fn year_len(&self) -> Option<u32> {
        self.year_info.as_ref().map(|info| info.year_len)
    }

    pub fn year_ordinal(&self) -> Option<i64> {
        self.year_info.as_ref().map(|info| info.year_ordinal as i64)
    }

    pub fn month_range(&self) -> &[u16] {
        self.year_info
            .as_ref()
            .map(|info| &info.month_range)
            .unwrap()
    }

    pub fn easter_mask(&self) -> Option<&Vec<isize>> {
        self.easter_mask.as_ref()
    }

    pub fn weekday_mask(&self) -> &[u8] {
        self.year_info
            .as_ref()
            .map(|info| &info.weekday_mask)
            .unwrap()
    }

    pub fn month_mask(&self) -> &[u8] {
        self.year_info
            .as_ref()
            .map(|info| &info.month_mask)
            .unwrap()
    }

    pub fn week_no_mask(&self) -> Option<&Vec<u8>> {
        match &self.year_info {
            Some(info) => info.week_no_mask.as_ref(),
            None => None,
        }
    }

    pub fn neg_weekday_mask(&self) -> Option<&Vec<i8>> {
        self.month_info.as_ref().map(|info| &info.neg_weekday_mask)
    }

    pub fn next_year_len(&self) -> Option<u32> {
        self.year_info.as_ref().map(|info| info.next_year_len)
    }

    pub fn month_day_mask(&self) -> &[i8] {
        self.year_info.as_ref().unwrap().month_day_mask
    }

    pub fn neg_month_day_mask(&self) -> &[i8] {
        self.year_info.as_ref().unwrap().neg_month_day_mask
    }

    pub fn year_dayset(&self) -> Result<(Vec<u64>, u64, u64), RRuleError> {
        let year_len = self
            .year_len()
            .ok_or_else(|| RRuleError::new_iter_err("`year_len()` returned `None`"))?;
        let v = (0..year_len as u64).collect();
        Ok((v, 0, year_len as u64))
    }

    pub fn month_dayset(&self, month: u8) -> Result<(Vec<u64>, u64, u64), RRuleError> {
        let month_range = self.month_range();
        let start = month_range[month as usize - 1];
        let end = month_range[month as usize] as u64;
        let set = (0..self.year_len().unwrap_or_default() as u64)
            .map(|i| if i < end { i } else { 0 })
            .collect();
        Ok((set, start as u64, end as u64))
    }

    pub fn weekday_set(
        &self,
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<(Vec<u64>, u64, u64), RRuleError> {
        let set_len = self.year_len().unwrap() + 7;
        let mut set = vec![0; set_len as usize];

        let mut i: u64 = (to_ordinal(
            &chrono::Utc
                .ymd(year as i32, month as u32, day as u32)
                .and_hms(0, 0, 0),
        ) - self.year_ordinal().unwrap()) as u64; // TODO can panic when number was negative

        let start = i;
        for _ in 0..7 {
            if i >= set_len as u64 {
                break;
            }
            set[i as usize] = i;
            i += 1;
            if self.weekday_mask()[i as usize]
                == self
                    .rrule
                    .get_properties()
                    .week_start
                    .num_days_from_monday() as u8
            {
                break;
            }
        }
        Ok((set, start, i))
    }

    pub fn day_dayset(
        &self,
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<(Vec<u64>, u64, u64), RRuleError> {
        let mut set = vec![0; self.year_len().unwrap() as usize];

        let i = (to_ordinal(
            &chrono::Utc
                .ymd(year as i32, month as u32, day as u32)
                .and_hms(0, 0, 0),
        ) - self.year_ordinal().unwrap()) as u64;

        set[i as usize] = i;
        Ok((set, i, i + 1))
    }

    pub fn hour_timeset(&self, hour: u8, _minute: u8, second: u8, millisecond: u16) -> Vec<Time> {
        let mut set = self
            .rrule
            .get_properties()
            .by_minute
            .iter()
            .flat_map(|minute| self.min_timeset(hour, *minute, second, millisecond))
            .collect::<Vec<Time>>();
        set.sort_by_key(|a| a.time());
        set
    }

    pub fn min_timeset(&self, hour: u8, minute: u8, _second: u8, millisecond: u16) -> Vec<Time> {
        let mut set = self
            .rrule
            .get_properties()
            .by_second
            .iter()
            .map(|second| Time::new(hour, minute, *second, millisecond))
            .collect::<Vec<Time>>();
        set.sort_by_key(|a| a.time());
        set
    }

    pub fn sec_timeset(&self, hour: u8, minute: u8, second: u8, millisecond: u16) -> Vec<Time> {
        vec![Time::new(hour, minute, second, millisecond)]
    }

    pub fn get_dayset(
        &self,
        freq: &Frequency,
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<(Vec<u64>, u64, u64), RRuleError> {
        match freq {
            Frequency::Yearly => self.year_dayset(),
            Frequency::Monthly => self.month_dayset(month),
            Frequency::Weekly => self.weekday_set(year, month, day),
            Frequency::Daily => self.day_dayset(year, month, day),
            _ => self.day_dayset(year, month, day),
        }
    }

    pub fn get_timeset(
        &self,
        freq: &Frequency,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
    ) -> Result<Vec<Time>, RRuleError> {
        match freq {
            Frequency::Hourly => Ok(self.hour_timeset(hour, minute, second, millisecond)),
            Frequency::Minutely => Ok(self.min_timeset(hour, minute, second, millisecond)),
            Frequency::Secondly => Ok(self.sec_timeset(hour, minute, second, millisecond)),
            _ => Err(RRuleError::new_iter_err("Invalid freq")),
        }
    }

    pub fn get_properties(&'a self) -> &'a RRuleProperties {
        self.rrule.get_properties()
    }
}
