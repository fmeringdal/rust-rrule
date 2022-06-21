use super::{
    masks::MASKS,
    utils::{get_year_len, pymod, to_ordinal},
};
use crate::RRule;
use chrono::{Datelike, TimeZone, Utc};

#[derive(Debug, Clone)]
pub(crate) struct YearInfo {
    /// Amount of days in the current year (365 or 366)
    pub year_len: u32,
    /// Amount of days in the next year (365 or 366)
    pub next_year_len: u32,
    /// Number of days since Unix epoch
    pub year_ordinal: i64,
    /// Get day of the week from first day of the year (1 jan)
    /// So if `YYYY/01/01` is a wednesday this value will be `2`.
    /// Can be any value from 0..=6 (monday = 0)
    // TODO: Why is this field never used?
    #[allow(dead_code)]
    pub year_weekday: u8,
    pub month_mask: &'static [u8],
    pub month_day_mask: &'static [i8],
    pub neg_month_day_mask: &'static [i8],
    pub month_range: &'static [u16],
    pub weekday_mask: &'static [u8],
    /// Week number mask
    // TODO: Seems to be only set to 0 or 1 ever. And hardly used, only in filter.
    pub week_no_mask: Option<Vec<u8>>,
}

#[derive(Debug)]
pub(crate) struct BaseMasks {
    month_mask: &'static [u8],
    month_day_mask: &'static [i8],
    neg_month_day_mask: &'static [i8],
    month_range: &'static [u16],
    weekday_mask: &'static [u8],
}

fn base_year_masks(year: i32) -> BaseMasks {
    let first_year_day = Utc.ymd(year, 1, 1).and_hms(0, 0, 0);
    let year_len = get_year_len(year);
    #[allow(clippy::cast_possible_truncation)]
    let weekday = first_year_day.weekday().num_days_from_monday() as u8;

    if year_len == 365 {
        BaseMasks {
            month_mask: &MASKS.month_365,
            month_day_mask: &MASKS.month_day_365,
            neg_month_day_mask: &MASKS.neg_month_day_365,
            month_range: &MASKS.month_365_range,
            weekday_mask: &MASKS.weekday[weekday as usize..],
        }
    } else {
        BaseMasks {
            month_mask: &MASKS.month_366,
            month_day_mask: &MASKS.month_day_366,
            neg_month_day_mask: &MASKS.neg_month_day_366,
            month_range: &MASKS.month_366_range,
            weekday_mask: &MASKS.weekday[weekday as usize..],
        }
    }
}

// TODO: too many lines of code.
#[warn(clippy::too_many_lines)]
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
pub(crate) fn rebuild_year(year: i32, rrule: &RRule) -> YearInfo {
    let first_year_day = Utc.ymd(year, 1, 1).and_hms(0, 0, 0);

    let year_len = u32::from(get_year_len(year));
    let next_year_len = u32::from(get_year_len(year + 1));
    let year_ordinal = to_ordinal(&first_year_day) as i64;
    let year_weekday = first_year_day.weekday().num_days_from_monday() as u8;

    let base_masks = base_year_masks(year);

    let mut result = YearInfo {
        year_len,
        next_year_len,
        year_ordinal,
        year_weekday,
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

    let mut week_no_mask = vec![0; year_len as usize + 7];

    let option_week_start = rrule.week_start.num_days_from_monday() as u8;
    let mut no1_week_start = pymod((7 - year_weekday + option_week_start) as isize, 7);
    let first_week_start = no1_week_start;
    let year_len_ext = if no1_week_start >= 4 {
        no1_week_start = 0;
        // Number of days in the year, plus the days we got
        // from last year.
        result.year_len as isize + pymod(year_weekday as isize - rrule.week_start as isize, 7)
    } else {
        // Number of days in the year, minus the days we
        // left in last year.
        year_len as isize - no1_week_start
    };

    let div = (year_len_ext as f32 / 7.).floor() as isize;
    let year_mod = pymod(year_len_ext, 7);
    //const num_weeks = Math.floor(div + mod / 4)
    let num_weeks = div + (year_mod / 4);

    for &(mut n) in &rrule.by_week_no {
        if n < 0 {
            n += (num_weeks + 1) as i8;
        }
        if !(n > 0 && n <= num_weeks as i8) {
            continue;
        }
        let mut i;
        if n > 1 {
            i = no1_week_start + ((n as isize - 1) * 7);
            if no1_week_start != first_week_start {
                i -= 7 - first_week_start;
            }
        } else {
            i = no1_week_start;
        }

        for _ in 0..7 {
            week_no_mask[i as usize] = 1;
            i += 1;
            if result.weekday_mask[i as usize] == rrule.week_start.num_days_from_monday() as u8 {
                break;
            }
        }
    }

    if rrule.by_week_no.iter().any(|&week_no| week_no == 1) {
        // Check week number 1 of next year as well
        // orig-TODO : Check -num_weeks for next year.
        let mut i = no1_week_start + num_weeks * 7;
        if no1_week_start != first_week_start {
            i -= 7 - first_week_start;
        }
        if i < year_len as isize {
            // If week starts in next year, we
            // don't care about it.
            for _ in 0..7 {
                week_no_mask[i as usize] = 1;
                i += 1;
                if result.weekday_mask[i as usize] == rrule.week_start.num_days_from_monday() as u8
                {
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
        let l_num_weeks = if rrule.by_week_no.iter().any(|&week_no| week_no == -1) {
            -1
        } else {
            let l_year_weekday = Utc.ymd(year - 1, 1, 1).weekday().num_days_from_monday() as isize;

            let ln_no1_week_start = pymod(7 - l_year_weekday + rrule.week_start as isize, 7);

            let l_year_len = get_year_len(year - 1) as isize;
            let week_start = if ln_no1_week_start >= 4 {
                //ln_no1_week_start = 0;
                l_year_len + pymod(l_year_weekday - rrule.week_start as isize, 7)
            } else {
                year_len as isize - no1_week_start
            };

            52 + (pymod(week_start, 7) / 4) as i8
        };

        if rrule
            .by_week_no
            .iter()
            .any(|&week_no| week_no == l_num_weeks)
        {
            for i in 0..no1_week_start {
                week_no_mask[i as usize] = 1;
            }
        }
    }

    result.week_no_mask = Some(week_no_mask);

    result
}
