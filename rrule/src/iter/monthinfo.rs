use super::{utils::pymod, yearinfo::YearInfo};
use crate::{Frequency, NWeekday, RRule};

#[derive(Debug, Clone)]
pub(crate) struct MonthInfo {
    pub last_year: i32,
    pub last_month: u8,
    pub neg_weekday_mask: Vec<u8>,
}

impl MonthInfo {
    pub fn new<TZ: chrono::TimeZone>(year_info: &YearInfo, month: u8, rrule: &RRule<TZ>) -> Self {
        let neg_weekday_mask = Self::get_neg_weekday_mask(year_info, month, rrule);
        Self {
            last_year: year_info.year,
            last_month: month,
            neg_weekday_mask,
        }
    }

    fn get_neg_weekday_mask<TZ: chrono::TimeZone>(
        year_info: &YearInfo,
        month: u8,
        rrule: &RRule<TZ>,
    ) -> Vec<u8> {
        let YearInfo {
            year_len,
            month_range,
            weekday_mask,
            ..
        } = year_info;

        // Build up `ranges`
        let mut ranges = vec![];
        if rrule.freq == Frequency::Yearly {
            if rrule.by_month.is_empty() {
                ranges.push((0, u32::from(*year_len) - 1));
            } else {
                for month in &rrule.by_month {
                    let month = usize::from(*month);
                    let first = u32::from(month_range[month - 1]);
                    let last = u32::from(month_range[month]);
                    ranges.push((first, last - 1));
                }
            }
        } else if rrule.freq == Frequency::Monthly {
            let month = usize::from(month);
            let first = u32::from(month_range[month - 1]);
            let last = u32::from(month_range[month]);
            ranges.push((first, last - 1));
        }

        if ranges.is_empty() {
            return vec![];
        }

        // Weekly frequency won't get here, so we may not
        // care about cross-year weekly periods.
        let mut neg_weekday_mask = vec![0; usize::from(*year_len)];

        // Loop over `ranges`
        for (first, last) in ranges {
            for by_weekday in &rrule.by_weekday {
                // Only check Nth occurrences here
                if let NWeekday::Nth(number, weekday) = by_weekday {
                    let weekday = i16::try_from(weekday.num_days_from_monday())
                        .expect("num_days_from_monday is between 0 and 6 which is covered by i16");
                    let nth_weekday = if *number < 0 {
                        let number = match u32::try_from(-number) {
                            Ok(num) => num,
                            _ => continue,
                        };
                        let nth_last_week = match last.checked_sub((number - 1) * 7) {
                            Some(val) => val,
                            None => continue,
                        };
                        let index = match usize::try_from(nth_last_week) {
                            Ok(i) => i,
                            _ => continue,
                        };
                        let nth_last_weekday = match weekday_mask.get(index) {
                            Some(val) => i16::try_from(*val).expect("values in weekday mask are all between 0 - 6 which is covered by i32"),
                            None => continue,
                        };

                        // Adjust to get the correct weekday
                        let modulo = u32::try_from(pymod(nth_last_weekday - weekday, 7))
                            .expect("pymod is positive because 7 is the modulus");
                        match nth_last_week.checked_sub(modulo) {
                            Some(val) => val,
                            None => continue,
                        }
                    } else {
                        let number = match u32::try_from(number - 1) {
                            Ok(num) => num,
                            _ => continue,
                        };
                        let nth_first_day = first + number * 7;
                        let index = match usize::try_from(nth_first_day) {
                            Ok(i) => i,
                            _ => continue,
                        };
                        let nth_first_day_weekday = match weekday_mask.get(index) {
                            Some(val) => i16::try_from(*val).expect("values in weekday mask are all between 0 - 6 which is covered by i32"),
                            None => continue,
                        };

                        // Adjust to get the correct weekday
                        let a = u32::try_from(7 - nth_first_day_weekday + weekday)
                            .expect("to be positive because nth_first_day_weekday is at most 6");
                        nth_first_day + pymod(a, 7)
                    };
                    if first <= nth_weekday && nth_weekday <= last {
                        if let Ok(nth_weekday) = usize::try_from(nth_weekday) {
                            neg_weekday_mask[nth_weekday] = 1;
                        }
                    }
                }
            }
        }

        neg_weekday_mask
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Weekday};

    use crate::core::Tz;

    use super::*;

    const UTC: Tz = Tz::UTC;

    #[test]
    fn get_neg_weekday_mask_with_daily_freq() {
        let rrule = RRule {
            freq: Frequency::Daily,
            ..Default::default()
        }
        .validate(UTC.ymd(1997, 1, 1).and_hms(0, 0, 0))
        .unwrap();

        let year_info = YearInfo::new(1997, &rrule);

        let neg_weekday_mask = MonthInfo::get_neg_weekday_mask(&year_info, 1, &rrule);
        assert!(neg_weekday_mask.is_empty());
    }

    #[test]
    fn get_neg_weekday_mask_with_yearly_freq() {
        let rrule = RRule {
            freq: Frequency::Yearly,
            ..Default::default()
        }
        .validate(UTC.ymd(1997, 1, 1).and_hms(0, 0, 0))
        .unwrap();

        let year_info = YearInfo::new(1997, &rrule);

        let neg_weekday_mask = MonthInfo::get_neg_weekday_mask(&year_info, 1, &rrule);
        assert_eq!(neg_weekday_mask.len(), year_info.year_len as usize);
        assert!(neg_weekday_mask.into_iter().all(|val| val == 0));
    }

    #[test]
    fn get_neg_weekday_mask_with_yearly_freq_and_byweekday() {
        let rrule = RRule {
            freq: Frequency::Yearly,
            by_weekday: vec![
                NWeekday::new(None, Weekday::Mon),
                NWeekday::new(Some(-2), Weekday::Thu),
                NWeekday::new(Some(1), Weekday::Thu),
            ],
            ..Default::default()
        }
        .validate(UTC.ymd(1997, 1, 1).and_hms(0, 0, 0))
        .unwrap();

        let year_info = YearInfo::new(1997, &rrule);

        let neg_weekday_mask = MonthInfo::get_neg_weekday_mask(&year_info, 1, &rrule);
        assert_eq!(neg_weekday_mask.len(), year_info.year_len as usize);
        assert!(neg_weekday_mask
            .into_iter()
            .enumerate()
            .all(|(idx, val)| match idx {
                1 | 351 => val == 1,
                _ => val == 0,
            }));
    }

    #[test]
    fn get_neg_weekday_mask_with_monthly_freq_and_byweekday() {
        let rrule = RRule {
            freq: Frequency::Monthly,
            by_weekday: vec![
                NWeekday::new(None, Weekday::Mon),
                NWeekday::new(Some(-2), Weekday::Thu),
                NWeekday::new(Some(1), Weekday::Thu),
            ],
            ..Default::default()
        }
        .validate(UTC.ymd(1997, 1, 1).and_hms(0, 0, 0))
        .unwrap();

        let year_info = YearInfo::new(1997, &rrule);

        let neg_weekday_mask = MonthInfo::get_neg_weekday_mask(&year_info, 1, &rrule);
        assert_eq!(neg_weekday_mask.len(), year_info.year_len as usize);
        assert!(neg_weekday_mask
            .into_iter()
            .enumerate()
            .all(|(idx, val)| match idx {
                1 | 22 => val == 1,
                _ => val == 0,
            }));
    }
}
