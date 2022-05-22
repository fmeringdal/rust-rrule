#![allow(clippy::module_name_repetitions, clippy::cast_possible_truncation)]

mod checks;
mod easter;
pub(crate) mod iterinfo;
mod masks;
mod monthinfo;
mod operation_errors;
mod pos_list;
pub(crate) mod rrule_iter;
mod rruleset_iter;
mod utils;
mod yearinfo;

use crate::{
    core::{DateTime, Time},
    validator::FREQ_HOURLY_INTERVAL_MAX,
    Frequency, NWeekday, RRule, RRuleError,
};
use chrono::{Datelike, Duration, Timelike};
use iterinfo::IterInfo;
use operation_errors::{checked_add_datetime_duration, checked_add_u32, checked_mul_u32};
use pos_list::build_pos_list;
pub(crate) use rrule_iter::RRuleIter;
pub use rruleset_iter::RRuleSetIter;
use utils::includes;

/// Used to prevent infinite loops when searching for a time.
/// This loop might be able to be prevented by using a math in the future.
#[cfg(not(feature = "no-validation-limits"))]
static FORMULA_LOOP_LIMIT: u16 = 10_000;
#[cfg(feature = "no-validation-limits")]
static FORMULA_LOOP_LIMIT: u16 = u16::MAX;

/// Prevent loops when searching for the next event in the iterator.
/// If after X amount of iterations it still has not found an event
/// we can assume it will not find an event.
#[cfg(not(feature = "no-validation-limits"))]
static MAX_ITER_LOOP: u32 = 100_000;
#[cfg(feature = "no-validation-limits")]
static MAX_ITER_LOOP: u32 = u32::MAX;

fn decrement_date_until_valid(
    date: DateTime,
    new_month: u8,
    new_year: Option<i32>,
) -> Result<DateTime, RRuleError> {
    // Check if month and year are valid
    checks::check_month_range(new_month)?;
    if let Some(year) = new_year {
        checks::check_year_range(year)?;
    }

    // Set year when day is valid
    let new_date = if let Some(new_year) = new_year {
        // Set the year
        let mut new_date = date.with_year(new_year);
        // If day does not exist in this year, decrease until valid
        // `i` can not be bigger or equal to `date.day()`
        // Because day would be `0` and should be in range of `0..=31`
        for i in 1..date.day() {
            // Date was invalid
            let day_number = date.day() - i;
            if new_date.is_none() && day_number > 0 {
                // Change day
                let temp_date = date.with_day(day_number).ok_or_else(|| {
                    RRuleError::new_iter_err(format!("Day number `{}` is invalid.", day_number))
                })?;
                // Change year (again)
                new_date = temp_date.with_year(new_year);
            }
        }
        new_date.ok_or_else(|| {
            RRuleError::new_iter_err("No valid date by changing day and year could be found.")
        })?
    } else {
        date
    };
    // Set month when day is valid
    let mut new_date = new_date.with_month(u32::from(new_month));
    // If day does not exist in this year, decrease until valid.
    for day_number in 0..=date.day0() {
        // If Date was invalid
        if new_date.is_none() && day_number > 0 {
            // Change day
            let temp_date = date.with_day(day_number).ok_or_else(|| {
                RRuleError::new_iter_err(format!("Day number `{}` is invalid.", day_number))
            })?;
            // Change month (again)
            new_date = temp_date.with_month(u32::from(new_month));
        }
    }
    // Return changed date
    new_date.ok_or_else(|| {
        RRuleError::new_iter_err("No valid date by changing day and month could be found.")
    })
}

// TODO too many lines
#[warn(clippy::too_many_lines)]
#[allow(
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
fn increment_counter_date(
    counter_date: DateTime,
    rrule: &RRule,
    filtered: bool,
) -> Result<DateTime, RRuleError> {
    match rrule.freq {
        Frequency::Yearly => {
            let new_year = counter_date.year() + i32::from(rrule.interval);
            checks::check_year_range(new_year)?;
            counter_date
                .with_year(new_year)
                .ok_or_else(|| RRuleError::new_iter_err(format!("Year `{}` is invalid", new_year)))
        }
        Frequency::Monthly => {
            let new_month = counter_date.month() + u32::from(rrule.interval);
            if new_month > 12 {
                let mut year_div = new_month / 12;
                let mut new_month = new_month % 12;
                if new_month == 0 {
                    new_month = 12;
                    year_div -= 1;
                }
                let new_year = counter_date.year() + year_div as i32;

                decrement_date_until_valid(counter_date, new_month as u8, Some(new_year))
            } else {
                decrement_date_until_valid(counter_date, new_month as u8, None)
            }
        }
        Frequency::Weekly => {
            let weekday = counter_date.weekday().num_days_from_monday();
            let option_week_start = rrule.week_start.num_days_from_monday();
            // Calculate amount of day to move forward.
            let day_delta = if option_week_start > weekday {
                // `weekday` and `option_week_start` can only be in range `0..=6`
                // `option_week_start` > `weekday` so:
                // `(weekday + 1 + 6 - option_week_start)` > 0 so can never be negative.
                (u32::from(rrule.interval)) * 7 - (weekday + 7 - option_week_start)
            } else {
                // This can also never be negative
                (u32::from(rrule.interval)) * 7 - (weekday - option_week_start)
            };
            checked_add_datetime_duration(
                counter_date,
                Duration::days(i64::from(day_delta)),
                Some("please decrease `INTERVAL`"),
            )
        }
        Frequency::Daily => checked_add_datetime_duration(
            counter_date,
            Duration::days(i64::from(rrule.interval)),
            Some("please decrease `INTERVAL`"),
        ),
        Frequency::Hourly => {
            let mut new_hours = counter_date.hour();
            if filtered {
                let temp_value =
                    ((23 - new_hours) as f32 / f32::from(rrule.interval)).floor() as u32;
                new_hours += checked_mul_u32(
                    temp_value,
                    u32::from(rrule.interval),
                    Some("please decrease `INTERVAL`"),
                )?;
            }

            // TODO: loop below should be replaced with just a math formal,
            // it looks like a loop should not be needed.
            let mut loop_count: u16 = 0;
            loop {
                new_hours = checked_add_u32(
                    new_hours,
                    u32::from(rrule.interval),
                    Some("please decrease `INTERVAL`"),
                )?;
                if rrule.by_hour.is_empty()
                    || rrule.by_hour.iter().any(|bh| *bh == (new_hours % 24) as u8)
                {
                    break;
                }
                loop_count += 1;
                if loop_count >= FORMULA_LOOP_LIMIT {
                    return Err(RRuleError::new_iter_err(
                        "Loop limit reached to prevent infinite loops.",
                    ));
                }
            }
            // If higher than expected this will return an error
            if !cfg!(feature = "no-validation-limits")
                && new_hours > u32::from(FREQ_HOURLY_INTERVAL_MAX)
            {
                return Err(RRuleError::new_iter_err(format!(
                    "Hour interval (`{}`) is higher than expected, make sure this is correct. \
                        See 'validator limits' in docs for more info",
                    new_hours
                )));
            }
            let duration = Duration::hours(i64::from(new_hours));
            let counter_date_hour_0 = counter_date.with_hour(0).ok_or_else(|| {
                RRuleError::new_iter_err(format!(
                    "Could not set hour to `0` for date {}",
                    counter_date
                ))
            })?;
            checked_add_datetime_duration(
                counter_date_hour_0,
                duration,
                Some("please decrease `INTERVAL`"),
            )
        }
        Frequency::Minutely => {
            let mut minutes_inc = 0;
            let hours = counter_date.hour();
            let minutes = counter_date.minute();
            if filtered {
                // Jump to one iteration before next day
                let temp_value = (1439.
                    - ((hours * 60 + minutes) as f32 / f32::from(rrule.interval)))
                .floor() as u32;
                minutes_inc = checked_mul_u32(
                    temp_value,
                    u32::from(rrule.interval),
                    Some("please decrease `INTERVAL`"),
                )?;
            }

            let mut counter_date = checked_add_datetime_duration(
                counter_date,
                Duration::minutes(i64::from(minutes_inc)),
                Some("please decrease `INTERVAL`"),
            )?;
            // TODO: loop below might be replaced with just a math formal,
            // it looks like a loop might not be needed.
            let mut loop_count: u16 = 0;
            loop {
                counter_date = checked_add_datetime_duration(
                    counter_date,
                    Duration::minutes(i64::from(rrule.interval)),
                    Some("please decrease `INTERVAL`"),
                )?;
                let hours = counter_date.hour() as u8;
                let minutes = counter_date.minute() as u8;

                if (rrule.by_hour.is_empty() || includes(&rrule.by_hour, &hours))
                    && (rrule.by_minute.is_empty() || includes(&rrule.by_minute, &minutes))
                {
                    break;
                }
                loop_count += 1;
                if loop_count >= FORMULA_LOOP_LIMIT {
                    return Err(RRuleError::new_iter_err(
                        "Loop limit reached to prevent infinite loops.",
                    ));
                }
            }

            Ok(counter_date)
        }
        Frequency::Secondly => {
            let mut seconds_inc = 0;
            let hours = counter_date.hour();
            let minutes = counter_date.minute();
            let seconds = counter_date.second();
            if filtered {
                // Jump to one iteration before next day
                let temp_value = (86399.
                    - ((hours * 3600 + minutes * 60 + seconds) as f32 / f32::from(rrule.interval)))
                .floor() as u32;
                seconds_inc = checked_mul_u32(
                    temp_value,
                    u32::from(rrule.interval),
                    Some("please decrease `INTERVAL`"),
                )?;
            }

            let mut counter_date = checked_add_datetime_duration(
                counter_date,
                Duration::seconds(i64::from(seconds_inc)),
                Some("please decrease `INTERVAL`"),
            )?;
            // TODO: loop below might be replaced with just a math formal,
            // it looks like a loop might not be needed.
            let mut loop_count: u16 = 0;
            loop {
                counter_date = checked_add_datetime_duration(
                    counter_date,
                    Duration::seconds(i64::from(rrule.interval)),
                    Some("please decrease `INTERVAL`"),
                )?;
                let hours = counter_date.hour() as u8;
                let minutes = counter_date.minute() as u8;
                let seconds = counter_date.second() as u8;

                if (rrule.by_hour.is_empty() || includes(&rrule.by_hour, &hours))
                    && (rrule.by_minute.is_empty() || includes(&rrule.by_minute, &minutes))
                    && (rrule.by_second.is_empty() || includes(&rrule.by_second, &seconds))
                {
                    break;
                }
                loop_count += 1;
                if loop_count >= FORMULA_LOOP_LIMIT {
                    return Err(RRuleError::new_iter_err(
                        "Loop limit reached to prevent infinite loops.",
                    ));
                }
            }

            Ok(counter_date)
        }
    }
}

// TODO break this up into parts because this is unmaintainable.
#[allow(clippy::cast_possible_wrap)]
fn is_filtered(ii: &IterInfo, current_day: u64, rrule: &RRule) -> bool {
    let by_month: bool = !rrule.by_month.is_empty()
        && !rrule
            .by_month
            .contains(&ii.month_mask()[current_day as usize]);

    let by_week_no: bool =
        !rrule.by_week_no.is_empty() && (ii.week_no_mask().unwrap()[current_day as usize]) == 0;

    let by_weekday_every_week_only = rrule
        .by_weekday
        .iter()
        .filter_map(|by_weekday| match by_weekday {
            // Filter out only `Every` occurrences.
            NWeekday::Every(weekday) => Some(weekday.num_days_from_monday() as i16),
            NWeekday::Nth(_number, _weekday) => None,
        })
        .collect::<Vec<_>>();
    let by_weekday: bool = !by_weekday_every_week_only.is_empty()
        && !includes(
            &by_weekday_every_week_only,
            &i16::from(ii.weekday_mask()[current_day as usize]),
        );

    let neg_weekday_mask: bool = ii.neg_weekday_mask().is_some()
        && !ii.neg_weekday_mask().unwrap().is_empty()
        && (ii.neg_weekday_mask().unwrap()[current_day as usize]) == 0;

    // Can only be set to true if feature flag is set.
    let by_easter: bool = if cfg!(feature = "by-easter") {
        rrule.by_easter.is_some() && !(includes(ii.easter_mask().unwrap(), &(current_day as isize)))
    } else {
        false
    };

    let by_month_day: bool = (!rrule.by_month_day.is_empty() || !rrule.by_n_month_day.is_empty())
        && !includes(
            &rrule.by_month_day,
            &(ii.month_day_mask()[current_day as usize]),
        )
        && !includes(
            &rrule.by_n_month_day,
            &(ii.neg_month_day_mask()[current_day as usize]),
        );

    let by_year_day: bool = !rrule.by_year_day.is_empty()
        && ((current_day < u64::from(ii.year_len().unwrap())
            && !includes(&rrule.by_year_day, &(current_day as i16 + 1))
            && !includes(
                &rrule.by_year_day,
                &(current_day as i16 - ii.year_len().unwrap() as i16),
            ))
            || (current_day >= u64::from(ii.year_len().unwrap())
                && !includes(
                    &rrule.by_year_day,
                    &(current_day as i16 + 1 - ii.year_len().unwrap() as i16),
                )
                && !includes(
                    &rrule.by_year_day,
                    &(current_day as i16
                        - ii.next_year_len().unwrap() as i16
                        - ii.year_len().unwrap() as i16),
                )));
    by_month
        || by_week_no
        || by_weekday
        || neg_weekday_mask
        || by_easter
        || by_month_day
        || by_year_day
}

fn remove_filtered_days(day_set: &mut [Option<u64>], start: u64, end: u64, ii: &IterInfo) -> bool {
    let mut filtered = false;

    // Loop over `start..end`
    for day_set_counter in day_set.iter_mut().take(end as usize).skip(start as usize) {
        match day_set_counter {
            Some(current_day) => {
                filtered = is_filtered(ii, *current_day, ii.get_rrule());
                if filtered {
                    *day_set_counter = None;
                }
            }
            None => continue,
        }
    }
    filtered
}

#[allow(clippy::cast_sign_loss)]
fn build_timeset(rrule: &RRule, dt_start: &DateTime) -> Vec<Time> {
    let millisecond_mod = (dt_start.timestamp_millis() % 1000) as u16;

    if rrule.freq > Frequency::Daily {
        return vec![];
    }

    let mut timeset =
        Vec::with_capacity(rrule.by_hour.len() * rrule.by_minute.len() * rrule.by_second.len());
    for hour in &rrule.by_hour {
        for minute in &rrule.by_minute {
            for second in &rrule.by_second {
                timeset.push(Time::new(*hour, *minute, *second, millisecond_mod));
            }
        }
    }

    timeset
}

fn make_timeset(
    ii: &IterInfo,
    counter_date: &DateTime,
    rrule: &RRule,
) -> Result<Vec<Time>, RRuleError> {
    if rrule.freq < Frequency::Hourly {
        return Ok(build_timeset(rrule, counter_date));
    }

    if (rrule.freq >= Frequency::Hourly
        && !rrule.by_hour.is_empty()
        && !rrule
            .by_hour
            .iter()
            .any(|&h| h == counter_date.hour() as u8))
        || (rrule.freq >= Frequency::Minutely
            && !rrule.by_minute.is_empty()
            && !rrule
                .by_minute
                .iter()
                .any(|&m| m == counter_date.minute() as u8))
        || (rrule.freq >= Frequency::Secondly
            && !rrule.by_second.is_empty()
            && !rrule
                .by_second
                .iter()
                .any(|&s| s == counter_date.second() as u8))
    {
        return Ok(vec![]);
    }

    ii.get_timeset(
        rrule.freq,
        counter_date.hour() as u8,
        counter_date.minute() as u8,
        counter_date.second() as u8,
        counter_date.timestamp_subsec_millis() as u16,
    )
}
