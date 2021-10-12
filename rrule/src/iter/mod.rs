mod checks;
mod easter;
mod iterinfo;
mod masks;
mod monthinfo;
mod operation_errors;
mod pos_list;
mod rrule_iter;
mod rruleset_iter;
mod utils;
mod yearinfo;

use crate::{
    core::{DateTime, Time},
    validator::FREQ_HOURLY_INTERVAL_MAX,
    Frequency, NWeekday, ParsedOptions, RRuleError,
};
use chrono::{Datelike, Duration, Timelike};
use iterinfo::IterInfo;
use operation_errors::*;
use pos_list::build_pos_list;
pub use rrule_iter::RRuleIter;
pub use rruleset_iter::RRuleSetIter;
use utils::includes;

/// Used to prevent infinite loops when searching for a time.
/// This loop might be able to be prevent by using a math in the future.
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

    // Set year where day is valid
    let new_date = if let Some(new_year) = new_year {
        // Set the year
        let mut new_date = date.with_year(new_year);
        // If day does not exists in this year, decrease until valid
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
    // Set month where day is valid
    let mut new_date = new_date.with_month(new_month as u32);
    // If day does not exists in this year, decrease until valid.
    for day_number in 0..=date.day0() {
        // If Date was invalid
        if new_date.is_none() && day_number > 0 {
            // Change day
            let temp_date = date.with_day(day_number).ok_or_else(|| {
                RRuleError::new_iter_err(format!("Day number `{}` is invalid.", day_number))
            })?;
            // Change month (again)
            new_date = temp_date.with_month(new_month as u32);
        }
    }
    // Return changed date
    new_date.ok_or_else(|| {
        RRuleError::new_iter_err("No valid date by changing day and month could be found.")
    })
}

fn increment_counter_date(
    counter_date: DateTime,
    options: &ParsedOptions,
    filtered: bool,
) -> Result<DateTime, RRuleError> {
    match options.freq {
        Frequency::Yearly => {
            let new_year = counter_date.year() + options.interval as i32;
            checks::check_year_range(new_year)?;
            counter_date
                .with_year(new_year)
                .ok_or_else(|| RRuleError::new_iter_err(format!("Year `{}` is invalid", new_year)))
        }
        Frequency::Monthly => {
            let new_month = counter_date.month() + options.interval as u32;
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
            let option_week_start = options.week_start.num_days_from_monday();
            // Calculate amount of day to move forward.
            let day_delta = if option_week_start > weekday {
                // `weekday` and `option_week_start` can only be in range `0..=6`
                // `option_week_start` > `weekday` so:
                // `(weekday + 1 + 6 - option_week_start)` > 0 so can never be negative.
                (options.interval as u32) * 7 - (weekday + 7 - option_week_start)
            } else {
                // This can also never be negative
                (options.interval as u32) * 7 - (weekday - option_week_start)
            };
            checked_add_datetime_duration(
                counter_date,
                Duration::days(day_delta as i64),
                Some("please decrease `INTERVAL`"),
            )
        }
        Frequency::Daily => checked_add_datetime_duration(
            counter_date,
            Duration::days(options.interval as i64),
            Some("please decrease `INTERVAL`"),
        ),
        Frequency::Hourly => {
            let mut new_hours = counter_date.hour();
            if filtered {
                let temp_value = ((23 - new_hours) as f32 / options.interval as f32).floor() as u32;
                new_hours += checked_mul_u32(
                    temp_value,
                    options.interval as u32,
                    Some("please decrease `INTERVAL`"),
                )?;
            }

            // TODO: loop below should be replaced with just a math formal,
            // it looks like a loop should not be needed.
            let mut loop_count: u16 = 0;
            loop {
                new_hours = checked_add_u32(
                    new_hours,
                    options.interval as u32,
                    Some("please decrease `INTERVAL`"),
                )?;
                if options.by_hour.is_empty()
                    || options
                        .by_hour
                        .iter()
                        .any(|bh| *bh == (new_hours % 24) as u8)
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
            // If higher then expected this will return an error
            if !cfg!(feature = "no-validation-limits")
                && new_hours > FREQ_HOURLY_INTERVAL_MAX as u32
            {
                return Err(RRuleError::new_iter_err(format!(
                    "Hour interval (`{}`) is higher then expected, make sure this is correct. \
                        See 'validator limits' in docs for more info",
                    new_hours
                )));
            }
            let duration = Duration::hours(new_hours as i64);
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
                let temp_value = (1439. - ((hours * 60 + minutes) as f32 / options.interval as f32))
                    .floor() as u32;
                minutes_inc = checked_mul_u32(
                    temp_value,
                    options.interval as u32,
                    Some("please decrease `INTERVAL`"),
                )?;
            }

            let mut counter_date = checked_add_datetime_duration(
                counter_date,
                Duration::minutes(minutes_inc as i64),
                Some("please decrease `INTERVAL`"),
            )?;
            // TODO: loop below might be replaced with just a math formal,
            // it looks like a loop might not be needed.
            let mut loop_count: u16 = 0;
            loop {
                counter_date = checked_add_datetime_duration(
                    counter_date,
                    Duration::minutes(options.interval as i64),
                    Some("please decrease `INTERVAL`"),
                )?;
                let hours = counter_date.hour() as u8;
                let minutes = counter_date.minute() as u8;

                if (options.by_hour.is_empty() || includes(&options.by_hour, &hours))
                    && (options.by_minute.is_empty() || includes(&options.by_minute, &minutes))
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
                    - ((hours * 3600 + minutes * 60 + seconds) as f32 / options.interval as f32))
                    .floor() as u32;
                seconds_inc = checked_mul_u32(
                    temp_value,
                    options.interval as u32,
                    Some("please decrease `INTERVAL`"),
                )?;
            }

            let mut counter_date = checked_add_datetime_duration(
                counter_date,
                Duration::seconds(seconds_inc as i64),
                Some("please decrease `INTERVAL`"),
            )?;
            // TODO: loop below might be replaced with just a math formal,
            // it looks like a loop might not be needed.
            let mut loop_count: u16 = 0;
            loop {
                counter_date = checked_add_datetime_duration(
                    counter_date,
                    Duration::seconds(options.interval as i64),
                    Some("please decrease `INTERVAL`"),
                )?;
                let hours = counter_date.hour() as u8;
                let minutes = counter_date.minute() as u8;
                let seconds = counter_date.second() as u8;

                if (options.by_hour.is_empty() || includes(&options.by_hour, &hours))
                    && (options.by_minute.is_empty() || includes(&options.by_minute, &minutes))
                    && (options.by_second.is_empty() || includes(&options.by_second, &seconds))
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

fn is_filtered(
    ii: &IterInfo,
    current_day: u64,
    options: &ParsedOptions,
) -> Result<bool, RRuleError> {
    // TODO break this up into parts because this is unmaintainable.

    let by_month: bool = !options.by_month.is_empty()
        && !options
            .by_month
            .contains(&ii.month_mask()[current_day as usize]);

    let by_week_no: bool =
        !options.by_week_no.is_empty() && (ii.week_no_mask().unwrap()[current_day as usize]) == 0;

    let by_weekday_every_week_only = options
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
            &(ii.weekday_mask()[current_day as usize] as i16),
        );

    let neg_weekday_mask: bool = ii.neg_weekday_mask().is_some()
        && !ii.neg_weekday_mask().unwrap().is_empty()
        && (ii.neg_weekday_mask().unwrap()[current_day as usize]) == 0;

    // Can only be set to true if feature flag is set.
    let by_easter: bool = if cfg!(feature = "by-easter") {
        options.by_easter.is_some()
            && !(includes(ii.easter_mask().unwrap(), &(current_day as isize)))
    } else {
        false
    };

    let by_month_day: bool = (!options.by_month_day.is_empty()
        || !options.by_n_month_day.is_empty())
        && !includes(
            &options.by_month_day,
            &(ii.month_day_mask()[current_day as usize]),
        )
        && !includes(
            &options.by_n_month_day,
            &(ii.neg_month_day_mask()[current_day as usize]),
        );

    let by_year_day: bool = !options.by_year_day.is_empty()
        && ((current_day < ii.year_len().unwrap() as u64
            && !includes(&options.by_year_day, &(current_day as i16 + 1))
            && !includes(
                &options.by_year_day,
                &(current_day as i16 - ii.year_len().unwrap() as i16),
            ))
            || (current_day >= ii.year_len().unwrap() as u64
                && !includes(
                    &options.by_year_day,
                    &(current_day as i16 + 1 - ii.year_len().unwrap() as i16),
                )
                && !includes(
                    &options.by_year_day,
                    &(current_day as i16
                        - ii.next_year_len().unwrap() as i16
                        - ii.year_len().unwrap() as i16),
                )));
    Ok(by_month
        || by_week_no
        || by_weekday
        || neg_weekday_mask
        || by_easter
        || by_month_day
        || by_year_day)
}

fn remove_filtered_days(
    day_set: &mut Vec<Option<u64>>,
    start: u64,
    end: u64,
    ii: &IterInfo,
) -> Result<bool, RRuleError> {
    let mut filtered = false;

    // Loop over `start..end`
    for day_set_counter in day_set.iter_mut().take(end as usize).skip(start as usize) {
        match day_set_counter {
            Some(current_day) => {
                filtered = is_filtered(ii, *current_day, ii.get_options())?;
                if filtered {
                    *day_set_counter = None;
                }
            }
            None => continue,
        }
    }
    Ok(filtered)
}

fn build_timeset(options: &ParsedOptions) -> Vec<Time> {
    let millisecond_mod = (options.dt_start.timestamp_millis() % 1000) as u16;

    if options.freq > Frequency::Daily {
        return vec![];
    }

    let mut timeset = Vec::with_capacity(
        options.by_hour.len() * options.by_minute.len() * options.by_second.len(),
    );
    for hour in &options.by_hour {
        for minute in &options.by_minute {
            for second in &options.by_second {
                timeset.push(Time::new(*hour, *minute, *second, millisecond_mod));
            }
        }
    }

    timeset
}

fn make_timeset(
    ii: &IterInfo,
    counter_date: &DateTime,
    options: &ParsedOptions,
) -> Result<Vec<Time>, RRuleError> {
    if options.freq < Frequency::Hourly {
        return Ok(build_timeset(options));
    }

    if (options.freq >= Frequency::Hourly
        && !options.by_hour.is_empty()
        && !options
            .by_hour
            .iter()
            .any(|&h| h == counter_date.hour() as u8))
        || (options.freq >= Frequency::Minutely
            && !options.by_minute.is_empty()
            && !options
                .by_minute
                .iter()
                .any(|&m| m == counter_date.minute() as u8))
        || (options.freq >= Frequency::Secondly
            && !options.by_second.is_empty()
            && !options
                .by_second
                .iter()
                .any(|&s| s == counter_date.second() as u8))
    {
        return Ok(vec![]);
    }

    ii.get_timeset(
        &options.freq,
        counter_date.hour() as u8,
        counter_date.minute() as u8,
        counter_date.second() as u8,
        counter_date.timestamp_subsec_millis() as u16,
    )
}
