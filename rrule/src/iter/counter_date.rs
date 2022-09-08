use std::collections::HashSet;

use chrono::{Datelike, TimeZone, Timelike, Utc, Weekday};

use crate::{Frequency, RRule, RRuleError};

use super::{
    checks,
    masks::MASKS,
    operation_errors::{checked_add_u32, checked_mul_u32},
    utils::is_leap_year,
};

const MINUTES_IN_A_DAY: u32 = 60 * 24;
const SECONDS_IN_A_DAY: u32 = 60 * 60 * 24;

/// A simple date time type used during iteration.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DateTimeIter {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl DateTimeIter {
    /// Increments the datetime based on the [`RRule`] values.
    ///
    /// If `increment_day` is set to `true`, then the method will also make sure that
    /// the day is incremented. This is useful is the case when `rrule.freq`
    /// is higher than daily (e.g. hourly) where this function might return a date with the
    /// same day, but the iterator already knows that the current day cannot
    /// be part of the result.
    pub fn increment<TZ: chrono::TimeZone>(
        &mut self,
        rrule: &RRule<TZ>,
        increment_day: bool,
    ) -> Result<(), RRuleError> {
        let RRule {
            interval,
            week_start,
            by_hour,
            by_minute,
            by_second,
            ..
        } = rrule;
        match rrule.freq {
            Frequency::Yearly => self.increment_yearly(*interval),
            Frequency::Monthly => self.increment_monthly(*interval),
            Frequency::Weekly => self.increment_weekly(*interval, *week_start),
            Frequency::Daily => self.increment_daily(*interval),
            Frequency::Hourly => self.increment_hourly(*interval, by_hour, increment_day),
            Frequency::Minutely => {
                self.increment_minutely(*interval, by_hour, by_minute, increment_day)
            }
            Frequency::Secondly => {
                self.increment_secondly(*interval, by_hour, by_minute, by_second, increment_day)
            }
        }
    }

    fn increment_yearly(&mut self, interval: u16) -> Result<(), RRuleError> {
        self.year += i32::from(interval);
        checks::check_year_range(self.year)?;
        self.fix_day()
    }

    fn increment_monthly(&mut self, interval: u16) -> Result<(), RRuleError> {
        self.month += u32::from(interval);
        if self.month > 12 {
            let mut year_div = u16::try_from(self.month).map_err(|_| {
                RRuleError::new_iter_err(
                    "Encountered a too high new month. Please decrease the rrule interval.",
                )
            })? / 12;
            self.month %= 12;
            if self.month == 0 {
                self.month = 12;
                year_div -= 1;
            }
            self.increment_yearly(year_div)?;
        }
        Ok(())
    }

    fn get_weekday(&self) -> u32 {
        let month_range_mask = if is_leap_year(self.year) {
            &MASKS.month_366_range
        } else {
            &MASKS.month_365_range
        };
        let year_day = u32::from(month_range_mask[self.month as usize - 1]) + self.day - 1;
        let year_day_mod = year_day % 7;
        let year_start_weekday = Utc.ymd(self.year, 1, 1).weekday().num_days_from_monday();

        (year_day_mod + year_start_weekday) % 7
    }

    fn increment_weekly(&mut self, interval: u16, week_start: Weekday) -> Result<(), RRuleError> {
        let weekday = self.get_weekday();
        let option_week_start = week_start.num_days_from_monday();
        let interval = u32::from(interval);
        // Calculate amount of day to move forward.
        let day_delta = if option_week_start > weekday {
            // `weekday` and `option_week_start` can only be in range `0..=6`
            // `option_week_start` > `weekday` so:
            // `(weekday + 1 + 6 - option_week_start)` > 0 so can never be negative.
            interval * 7 - (weekday + 7 - option_week_start)
        } else {
            // This can also never be negative
            interval * 7 - (weekday - option_week_start)
        };
        self.day += day_delta;
        self.fix_day()
    }

    fn days_in_month(&self) -> u32 {
        if self.month == 0 {
            31
        } else {
            let month_range_mask = if is_leap_year(self.year) {
                &MASKS.month_366_range
            } else {
                &MASKS.month_365_range
            };
            let days_in_month =
                month_range_mask[self.month as usize] - month_range_mask[self.month as usize - 1];
            u32::from(days_in_month)
        }
    }

    fn fix_day(&mut self) -> Result<(), RRuleError> {
        if self.day <= 28 {
            return Ok(());
        }

        let mut days_in_month = self.days_in_month();
        if self.day <= days_in_month {
            return Ok(());
        }

        while self.day > days_in_month {
            self.day -= days_in_month;
            self.increment_monthly(1)?;

            days_in_month = self.days_in_month();
        }
        Ok(())
    }

    fn increment_daily(&mut self, interval: u16) -> Result<(), RRuleError> {
        self.day += u32::from(interval);
        self.fix_day()
    }

    fn increment_hourly(
        &mut self,
        interval: u16,
        by_hour: &[u8],
        increment_day: bool,
    ) -> Result<(), RRuleError> {
        if increment_day {
            // Jump to one iteration before next day
            let temp_value = (23 - self.hour) / u32::from(interval);
            self.hour += checked_mul_u32(
                temp_value,
                u32::from(interval),
                Some("please decrease `INTERVAL`"),
            )?;
        }

        let mut prev_hours = HashSet::new();
        loop {
            self.hour = checked_add_u32(
                self.hour,
                u32::from(interval),
                Some("please decrease `INTERVAL`"),
            )?;
            let new_hours = u8::try_from(self.hour % 24).expect("range 0 - 23 is covered by u8");
            if by_hour.is_empty() || by_hour.iter().any(|bh| *bh == new_hours) {
                break;
            }
            if prev_hours.contains(&new_hours) {
                return Err(RRuleError::new_iter_err(
                    "Infinite loop detected. It can be resolved by changing `BYHOUR` or `INTERVAL`",
                ));
            }
            prev_hours.insert(new_hours);
        }

        let new_days = u16::try_from(self.hour / 24).map_err(|_| {
            RRuleError::new_iter_err("unexpected high hour, please decrease the interval")
        })?;
        self.hour %= 24;
        if new_days > 0 {
            self.increment_daily(new_days)
        } else {
            Ok(())
        }
    }

    fn increment_minutely(
        &mut self,
        interval: u16,
        by_hour: &[u8],
        by_minute: &[u8],
        increment_day: bool,
    ) -> Result<(), RRuleError> {
        if increment_day {
            // Jump to one iteration before next day
            let minutes_total = self.hour * 60 + self.minute;
            let temp_value = (MINUTES_IN_A_DAY - 1 - minutes_total) / u32::from(interval);
            self.minute += checked_mul_u32(
                temp_value,
                u32::from(interval),
                Some("please decrease `INTERVAL`"),
            )?;
        }

        let mut prev_values = HashSet::new();
        loop {
            self.minute += u32::from(interval);
            let hours_div = u16::try_from(self.minute / 60).map_err(|_| {
                RRuleError::new_iter_err("unexpected high minute, please decrease interval")
            })?;
            if hours_div > 0 {
                self.minute %= 60;
                self.increment_hourly(hours_div, by_hour, increment_day)?;
            }

            let hours = u8::try_from(self.hour % 24).expect("range 0 - 23 is covered by u8");
            let minutes = u8::try_from(self.minute % 60).expect("range 0 - 59 is covered by u8");

            if (by_hour.is_empty() || by_hour.contains(&hours))
                && (by_minute.is_empty() || by_minute.contains(&minutes))
            {
                break;
            }

            if prev_values.contains(&(hours, minutes)) {
                return Err(RRuleError::new_iter_err(
                    "Infinite loop detected. It can be resolved by changing `BYMINUTE`, `BYHOUR` or `INTERVAL`",
                ));
            }
            prev_values.insert((hours, minutes));
        }

        Ok(())
    }

    fn increment_secondly(
        &mut self,
        interval: u16,
        by_hour: &[u8],
        by_minute: &[u8],
        by_second: &[u8],
        increment_day: bool,
    ) -> Result<(), RRuleError> {
        if increment_day {
            // Jump to one iteration before next day
            let total_seconds = self.hour * 3600 + self.minute * 60 + self.second;
            let temp_value = (SECONDS_IN_A_DAY - 1 - total_seconds) / u32::from(interval);
            self.second += checked_mul_u32(
                temp_value,
                u32::from(interval),
                Some("please decrease `INTERVAL`"),
            )?;
        }

        let mut prev_values = HashSet::new();
        loop {
            self.second += u32::from(interval);
            let minutes_div = u16::try_from(self.second / 60).map_err(|_| {
                RRuleError::new_iter_err("unexpected high second, please decrease interval")
            })?;
            if minutes_div > 0 {
                self.second %= 60;
                self.increment_minutely(minutes_div, by_hour, by_minute, increment_day)?;
            }

            let hours = u8::try_from(self.hour % 24).expect("range 0 - 23 is covered by u8");
            let minutes = u8::try_from(self.minute % 60).expect("range 0 - 59 is covered by u8");
            let seconds = u8::try_from(self.second % 60).expect("range 0 - 59 is covered by u8");

            if (by_hour.is_empty() || by_hour.contains(&hours))
                && (by_minute.is_empty() || by_minute.contains(&minutes))
                && (by_second.is_empty() || by_second.contains(&seconds))
            {
                break;
            }

            if prev_values.contains(&(hours, minutes, seconds)) {
                return Err(RRuleError::new_iter_err(
                    "Infinite loop detected. It can be resolved by changing `BYSECOND`, `BYMINUTE`, `BYHOUR` or `INTERVAL`",
                ));
            }
            prev_values.insert((hours, minutes, seconds));
        }

        Ok(())
    }
}

impl<TZ: chrono::TimeZone> From<&chrono::DateTime<TZ>> for DateTimeIter {
    fn from(dt: &chrono::DateTime<TZ>) -> Self {
        Self {
            year: dt.year(),
            month: dt.month(),
            day: dt.day(),
            hour: dt.hour(),
            minute: dt.minute(),
            second: dt.second(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Tz;

    use super::*;
    use chrono::TimeZone;

    const UTC: Tz = Tz::UTC;

    fn ymd_hms(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTimeIter {
        let dt = UTC.ymd(year, month, day).and_hms(hour, min, sec);
        DateTimeIter::from(&dt)
    }

    #[test]
    fn increments_counter_date_with_yearly_freq() {
        let tests = [
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                1,
                ymd_hms(1998, 1, 1, 0, 0, 0),
            ),
            (
                ymd_hms(2020, 2, 29, 0, 0, 0),
                1,
                ymd_hms(2021, 3, 1, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                3,
                ymd_hms(2000, 1, 1, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                20,
                ymd_hms(2017, 1, 1, 0, 0, 0),
            ),
        ];
        for (mut counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Yearly,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let res = counter_date.increment(&rrule, false);
            assert!(res.is_ok());
            assert_eq!(counter_date, expected_output);
        }
    }

    #[test]
    fn increments_counter_date_with_monthly_freq() {
        let tests = [
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                1,
                ymd_hms(1997, 2, 1, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                11,
                ymd_hms(1997, 12, 1, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                12,
                ymd_hms(1998, 1, 1, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 12, 1, 0, 0, 0),
                1,
                ymd_hms(1998, 1, 1, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 11, 1, 0, 0, 0),
                12 * 10,
                ymd_hms(2007, 11, 1, 0, 0, 0),
            ),
        ];
        for (mut counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Monthly,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let res = counter_date.increment(&rrule, false);
            assert!(res.is_ok());
            assert_eq!(counter_date, expected_output);
        }
    }

    #[test]
    fn increments_counter_date_with_weekly_freq() {
        let tests = [
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                1,
                ymd_hms(1997, 1, 6, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                2,
                ymd_hms(1997, 1, 13, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                5,
                ymd_hms(1997, 2, 3, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                52,
                ymd_hms(1997, 12, 29, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                53,
                ymd_hms(1998, 1, 5, 0, 0, 0),
            ),
        ];
        for (mut counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Weekly,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let res = counter_date.increment(&rrule, false);
            assert!(res.is_ok());
            assert_eq!(counter_date, expected_output);
        }
    }

    #[test]
    fn increments_counter_date_with_daily_freq() {
        let tests = [
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                1,
                ymd_hms(1997, 1, 2, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                2,
                ymd_hms(1997, 1, 3, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                31,
                ymd_hms(1997, 2, 1, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                365,
                ymd_hms(1998, 1, 1, 0, 0, 0),
            ),
        ];
        for (mut counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Daily,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let res = counter_date.increment(&rrule, false);
            assert!(res.is_ok());
            assert_eq!(counter_date, expected_output);
        }
    }

    #[test]
    fn increments_counter_date_with_hourly_freq() {
        let tests = [
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                1,
                vec![],
                ymd_hms(1997, 1, 1, 1, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                2,
                vec![],
                ymd_hms(1997, 1, 1, 2, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                1,
                vec![4],
                ymd_hms(1997, 1, 1, 4, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 10, 0, 0),
                1,
                vec![4, 14],
                ymd_hms(1997, 1, 1, 14, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                24,
                vec![],
                ymd_hms(1997, 1, 2, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                24 * 31,
                vec![],
                ymd_hms(1997, 2, 1, 0, 0, 0),
            ),
        ];
        for (mut counter_date, interval, by_hour, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Hourly,
                by_hour,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let res = counter_date.increment(&rrule, false);
            assert!(res.is_ok());
            assert_eq!(counter_date, expected_output);
        }
    }

    #[test]
    fn increments_counter_date_with_hourly_freq_and_day_increment() {
        let tests = [
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                1,
                ymd_hms(1997, 1, 2, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                2,
                ymd_hms(1997, 1, 2, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                5,
                ymd_hms(1997, 1, 2, 1, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                24,
                ymd_hms(1997, 1, 2, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                24 * 31,
                ymd_hms(1997, 2, 1, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                24 * 31 + 1,
                ymd_hms(1997, 2, 1, 1, 0, 0),
            ),
        ];
        for (mut counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Hourly,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let res = counter_date.increment(&rrule, true);
            assert!(res.is_ok());
            assert_eq!(counter_date, expected_output);
        }
    }

    #[test]
    fn increments_counter_date_with_minutely_freq() {
        let tests = [
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                1,
                ymd_hms(1997, 1, 1, 0, 1, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                2,
                ymd_hms(1997, 1, 1, 0, 2, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                60,
                ymd_hms(1997, 1, 1, 1, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                60 * 24,
                ymd_hms(1997, 1, 2, 0, 0, 0),
            ),
        ];
        for (mut counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Minutely,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let res = counter_date.increment(&rrule, false);
            assert!(res.is_ok());
            assert_eq!(counter_date, expected_output);
        }
    }

    #[test]
    fn increments_counter_date_with_minutely_freq_and_day_increment() {
        let tests = [
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                1,
                ymd_hms(1997, 1, 2, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                2,
                ymd_hms(1997, 1, 2, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                60 * 24 + 1,
                ymd_hms(1997, 1, 2, 0, 1, 0),
            ),
        ];
        for (mut counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Minutely,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let res = counter_date.increment(&rrule, true);
            assert!(res.is_ok());
            assert_eq!(counter_date, expected_output);
        }
    }

    #[test]
    fn increments_counter_date_with_secondly_freq() {
        let tests = [
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                1,
                ymd_hms(1997, 1, 1, 0, 0, 1),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                2,
                ymd_hms(1997, 1, 1, 0, 0, 2),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                60,
                ymd_hms(1997, 1, 1, 0, 1, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                60 * 60,
                ymd_hms(1997, 1, 1, 1, 0, 0),
            ),
        ];
        for (mut counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Secondly,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let res = counter_date.increment(&rrule, false);
            assert!(res.is_ok());
            assert_eq!(counter_date, expected_output);
        }
    }

    #[test]
    fn increments_counter_date_with_secondly_freq_and_day_increment() {
        let tests = [
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                1,
                ymd_hms(1997, 1, 2, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                2,
                ymd_hms(1997, 1, 2, 0, 0, 0),
            ),
            (
                ymd_hms(1997, 1, 1, 0, 0, 0),
                60 * 24 + 1,
                ymd_hms(1997, 1, 2, 0, 1, 0),
            ),
        ];
        for (mut counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Secondly,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let res = counter_date.increment(&rrule, true);
            assert!(res.is_ok());
            assert_eq!(counter_date, expected_output);
        }
    }
}
