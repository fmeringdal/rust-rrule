use super::{
    masks::MASKS,
    utils::{days_since_unix_epoch, get_year_len, pymod},
};
use crate::RRule;
use chrono::{Datelike, TimeZone, Utc};

#[derive(Debug)]
pub(crate) struct BaseMasks {
    month_mask: &'static [u8],
    month_day_mask: &'static [i8],
    neg_month_day_mask: &'static [i8],
    month_range: &'static [u16],
    weekday_mask: &'static [u32],
}

fn base_year_masks(year_weekday: u16, year_len: u16) -> BaseMasks {
    if year_len == 365 {
        BaseMasks {
            month_mask: &MASKS.month_365,
            month_day_mask: &MASKS.month_day_365,
            neg_month_day_mask: &MASKS.neg_month_day_365,
            month_range: &MASKS.month_365_range,
            weekday_mask: &MASKS.weekday[usize::from(year_weekday)..],
        }
    } else {
        BaseMasks {
            month_mask: &MASKS.month_366,
            month_day_mask: &MASKS.month_day_366,
            neg_month_day_mask: &MASKS.neg_month_day_366,
            month_range: &MASKS.month_366_range,
            weekday_mask: &MASKS.weekday[usize::from(year_weekday)..],
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct YearInfo {
    /// The year
    pub year: i32,
    /// Amount of days in the current year (365 or 366)
    pub year_len: u16,
    /// Amount of days in the next year (365 or 366)
    pub next_year_len: u16,
    /// Number of days since Unix epoch
    pub year_ordinal: i64,
    pub month_mask: &'static [u8],
    pub month_day_mask: &'static [i8],
    pub neg_month_day_mask: &'static [i8],
    pub month_range: &'static [u16],
    pub weekday_mask: &'static [u32],
    /// Week number mask
    pub week_no_mask: Option<Vec<u8>>,
}

impl YearInfo {
    pub fn new<TZ: chrono::TimeZone>(year: i32, rrule: &RRule<TZ>) -> Self {
        let first_year_day = Utc.ymd(year, 1, 1).and_hms(0, 0, 0);

        let year_len = get_year_len(year);
        let next_year_len = get_year_len(year + 1);
        let year_ordinal = days_since_unix_epoch(&first_year_day);
        let year_start_weekday = u16::try_from(first_year_day.weekday().num_days_from_monday())
            .expect("num_days_from_monday is between 0 and 6 which is covered by u16");

        let base_masks = base_year_masks(year_start_weekday, year_len);

        let mut result = YearInfo {
            year,
            year_len,
            next_year_len,
            year_ordinal,
            week_no_mask: None,
            month_mask: base_masks.month_mask,
            month_day_mask: base_masks.month_day_mask,
            neg_month_day_mask: base_masks.neg_month_day_mask,
            month_range: base_masks.month_range,
            weekday_mask: base_masks.weekday_mask,
        };

        if rrule.by_week_no.is_empty() {
            return result;
        }

        let mut week_no_mask = vec![0; usize::from(year_len) + 7];

        let rrule_week_start = u16::try_from(rrule.week_start.num_days_from_monday())
            .expect("num_days_from_monday is between 0 and 6 which is covered by u16");
        let mut no1_week_start = pymod(7 - year_start_weekday + rrule_week_start, 7);
        let first_week_start = no1_week_start;
        let year_len_ext = if no1_week_start >= 4 {
            no1_week_start = 0;
            // Number of days in the year, plus the days we got
            // from last year.
            let diff = i32::from(year_start_weekday) - i32::from(rrule_week_start);
            result.year_len
                + u16::try_from(pymod(diff, 7)).expect("to be positive because 7 is the modulus")
        } else {
            // Number of days in the year, minus the days we
            // left in last year.
            year_len - no1_week_start
        };

        let div = year_len_ext / 7;
        let year_mod = pymod(year_len_ext, 7);
        let num_weeks = div + (year_mod / 4);

        for &(mut n) in &rrule.by_week_no {
            let num_weeks =
                i8::try_from(num_weeks).expect("num_weeks is 52-54 which is covered by i8::MAX");
            if n < 0 {
                n += num_weeks + 1;
            }
            if !(n > 0 && n <= num_weeks) {
                continue;
            }

            let i = if n > 1 {
                let n = u16::try_from(n)
                    .expect("We know that 1 < n < i8::MAX which is in covered by u16");
                let mut i = no1_week_start + ((n - 1) * 7);
                if no1_week_start != first_week_start {
                    i -= 7 - first_week_start;
                }
                i
            } else {
                no1_week_start
            };

            for j in i..i + 7 {
                let j = usize::from(j);
                week_no_mask[j] = 1;
                if result.weekday_mask[j + 1] == rrule.week_start.num_days_from_monday() {
                    break;
                }
            }
        }

        if rrule.by_week_no.contains(&1) {
            // Check week number 1 of next year as well
            let mut i = no1_week_start + num_weeks * 7;
            if no1_week_start != first_week_start {
                i -= 7 - first_week_start;
            }
            if i < year_len {
                // If week starts in next year, we
                // don't care about it.
                for j in i..i + 7 {
                    let j = usize::from(j);
                    week_no_mask[j] = 1;
                    if result.weekday_mask[j + 1] == rrule.week_start.num_days_from_monday() {
                        break;
                    }
                }
            }
        }

        if no1_week_start > 0 {
            // Check last week number of last year as
            // well. If no1_week_start is 0, either the year
            // started on week start, or week number 1
            // got days from last year, so there are no
            // days from last year's last week number in
            // this year.
            let l_num_weeks = if rrule.by_week_no.contains(&-1) {
                -1
            } else {
                let l_year_weekday =
                    u16::try_from(Utc.ymd(year - 1, 1, 1).weekday().num_days_from_monday())
                        .expect("num_days_from_monday is between 0 and 6 which is covered by u16");

                let rrule_week_start = u16::try_from(rrule.week_start.num_days_from_monday())
                    .expect("num_days_from_monday is between 0 and 6 which is covered by u16");
                let ln_no1_week_start = pymod(7 - l_year_weekday + rrule_week_start, 7);

                let l_year_len = get_year_len(year - 1);
                let week_start = if ln_no1_week_start >= 4 {
                    l_year_len
                        + u16::try_from(pymod(
                            i32::from(l_year_weekday) - i32::from(rrule_week_start),
                            7,
                        ))
                        .expect("7 is the modulo so range is 0-6 and u16 covers that range")
                } else {
                    year_len - no1_week_start
                };

                52 + i8::try_from(pymod(week_start, 7) / 4)
                    .expect("7 is the modulo so range is 0-6 and i8 covers that range")
            };

            if rrule.by_week_no.contains(&l_num_weeks) {
                for i in 0..no1_week_start {
                    let i = usize::from(i);
                    week_no_mask[i] = 1;
                }
            }
        }

        result.week_no_mask = Some(week_no_mask);

        result
    }
}
