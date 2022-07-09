use chrono::{Datelike, Duration, Timelike, Weekday};

use crate::{
    core::{get_hour, get_minute, get_month, get_second, DateTime},
    validator::FREQ_HOURLY_INTERVAL_MAX,
    Frequency, RRule, RRuleError,
};

use super::{
    checks,
    operation_errors::{checked_add_datetime_duration, checked_add_u32, checked_mul_u32},
};

/// Increments the counter date based on the [`RRule`] values.
///
/// If `increment_day` is set to `true`, then the method will also make sure that
/// the `counter_date` day is incremented. This is useful is the case when `rrule.freq`
/// is higher than daily (e.g. hourly) where this function might return a date with the
/// same day, but the iterator already knows that the day of `counter_date` cannot
/// be part of the result.
pub(super) fn increment_counter_date(
    counter_date: DateTime,
    rrule: &RRule,
    increment_day: bool,
) -> Result<DateTime, RRuleError> {
    let RRule {
        interval,
        week_start,
        by_hour,
        by_minute,
        by_second,
        ..
    } = rrule;
    match rrule.freq {
        Frequency::Yearly => increment_yearly(counter_date, *interval),
        Frequency::Monthly => increment_monthly(counter_date, *interval),
        Frequency::Weekly => increment_weekly(counter_date, *interval, *week_start),
        Frequency::Daily => increment_daily(counter_date, *interval),
        Frequency::Hourly => increment_hourly(counter_date, *interval, by_hour, increment_day),
        Frequency::Minutely => {
            increment_minutely(counter_date, *interval, by_hour, by_minute, increment_day)
        }
        Frequency::Secondly => increment_secondly(
            counter_date,
            *interval,
            by_hour,
            by_minute,
            by_second,
            increment_day,
        ),
    }
}

fn increment_yearly(counter_date: DateTime, interval: u16) -> Result<DateTime, RRuleError> {
    let new_year = counter_date.year() + i32::from(interval);
    checks::check_year_range(new_year)?;

    let month = get_month(&counter_date);
    decrement_date_until_valid(counter_date, month, Some(new_year))
}

fn increment_monthly(counter_date: DateTime, interval: u16) -> Result<DateTime, RRuleError> {
    let new_month = counter_date.month() + u32::from(interval);
    if new_month > 12 {
        let mut year_div = i32::try_from(new_month).map_err(|_| {
            RRuleError::new_iter_err(
                "Encountered a too high new month. Please decrease the rrule interval.",
            )
        })? / 12;
        let mut new_month = new_month % 12;
        if new_month == 0 {
            new_month = 12;
            year_div -= 1;
        }
        let new_year = counter_date.year() + year_div;
        let new_month = u8::try_from(new_month).expect(
            "we know that new_month is less than < 12 at this stage which is within range of u8",
        );

        decrement_date_until_valid(counter_date, new_month, Some(new_year))
    } else {
        let new_month = u8::try_from(new_month).expect(
            "we know that new_month is less than < 12 at this stage which is within range of u8",
        );
        decrement_date_until_valid(counter_date, new_month, None)
    }
}

fn increment_weekly(
    counter_date: DateTime,
    interval: u16,
    week_start: Weekday,
) -> Result<DateTime, RRuleError> {
    let weekday = counter_date.weekday().num_days_from_monday();
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
    checked_add_datetime_duration(
        counter_date,
        Duration::days(i64::from(day_delta)),
        Some("please decrease `INTERVAL`"),
    )
}

fn increment_daily(counter_date: DateTime, interval: u16) -> Result<DateTime, RRuleError> {
    checked_add_datetime_duration(
        counter_date,
        Duration::days(i64::from(interval)),
        Some("please decrease `INTERVAL`"),
    )
}

fn increment_hourly(
    counter_date: DateTime,
    interval: u16,
    by_hour: &[u8],
    increment_day: bool,
) -> Result<DateTime, RRuleError> {
    let mut new_hours = counter_date.hour();
    if increment_day {
        // Jump to one iteration before next day
        let temp_value = (23 - new_hours) / u32::from(interval);
        new_hours += checked_mul_u32(
            temp_value,
            u32::from(interval),
            Some("please decrease `INTERVAL`"),
        )?;
    }

    let first_new_hours_mod_24 = new_hours % 24;
    loop {
        new_hours = checked_add_u32(
            new_hours,
            u32::from(interval),
            Some("please decrease `INTERVAL`"),
        )?;
        let new_hours_mod_24 = new_hours % 24;
        if by_hour.is_empty()
            || by_hour.iter().any(|bh| {
                *bh == u8::try_from(new_hours_mod_24)
                    .expect("we know that it is less than < 24 which is range of u8")
            })
        {
            break;
        }
        if new_hours_mod_24 == first_new_hours_mod_24 {
            return Err(RRuleError::new_iter_err(
                "Infinite loop detected. It can be resolved by changing `BYHOUR` or `INTERVAL`",
            ));
        }
    }

    // If higher than expected this will return an error
    if !cfg!(feature = "no-validation-limits") && new_hours > u32::from(FREQ_HOURLY_INTERVAL_MAX) {
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

const MINUTES_IN_A_DAY: u32 = 60 * 24;

fn increment_minutely(
    counter_date: DateTime,
    interval: u16,
    by_hour: &[u8],
    by_minute: &[u8],
    increment_day: bool,
) -> Result<DateTime, RRuleError> {
    let mut minutes_inc = 0;
    if increment_day {
        // Jump to one iteration before next day
        let minutes_total = counter_date.hour() * 60 + counter_date.minute();
        let temp_value = (MINUTES_IN_A_DAY - 1 - minutes_total) / u32::from(interval);
        minutes_inc = checked_mul_u32(
            temp_value,
            u32::from(interval),
            Some("please decrease `INTERVAL`"),
        )?;
    }

    let mut counter_date = checked_add_datetime_duration(
        counter_date,
        Duration::minutes(i64::from(minutes_inc)),
        Some("please decrease `INTERVAL`"),
    )?;

    let first_hours = get_hour(&counter_date);
    let first_minutes = get_minute(&counter_date);
    loop {
        counter_date = checked_add_datetime_duration(
            counter_date,
            Duration::minutes(i64::from(interval)),
            Some("please decrease `INTERVAL`"),
        )?;
        let hours = get_hour(&counter_date);
        let minutes = get_minute(&counter_date);

        if (by_hour.is_empty() || by_hour.contains(&hours))
            && (by_minute.is_empty() || by_minute.contains(&minutes))
        {
            break;
        }

        if hours == first_hours && minutes == first_minutes {
            return Err(RRuleError::new_iter_err(
                "Infinite loop detected. It can be resolved by changing `BYMINUTE`, `BYHOUR` or `INTERVAL`",
            ));
        }
    }

    Ok(counter_date)
}

const SECONDS_IN_A_DAY: u32 = 60 * 60 * 24;

fn increment_secondly(
    counter_date: DateTime,
    interval: u16,
    by_hour: &[u8],
    by_minute: &[u8],
    by_second: &[u8],
    increment_day: bool,
) -> Result<DateTime, RRuleError> {
    let mut seconds_inc = 0;
    if increment_day {
        // Jump to one iteration before next day
        let total_seconds =
            counter_date.hour() * 3600 + counter_date.minute() * 60 + counter_date.second();
        let temp_value = (SECONDS_IN_A_DAY - 1 - total_seconds) / u32::from(interval);
        seconds_inc = checked_mul_u32(
            temp_value,
            u32::from(interval),
            Some("please decrease `INTERVAL`"),
        )?;
    }

    let mut counter_date = checked_add_datetime_duration(
        counter_date,
        Duration::seconds(i64::from(seconds_inc)),
        Some("please decrease `INTERVAL`"),
    )?;

    let first_hours = get_hour(&counter_date);
    let first_minutes = get_minute(&counter_date);
    let first_seconds = get_second(&counter_date);
    loop {
        counter_date = checked_add_datetime_duration(
            counter_date,
            Duration::seconds(i64::from(interval)),
            Some("please decrease `INTERVAL`"),
        )?;
        let hours = get_hour(&counter_date);
        let minutes = get_minute(&counter_date);
        let seconds = get_second(&counter_date);

        if (by_hour.is_empty() || by_hour.contains(&hours))
            && (by_minute.is_empty() || by_minute.contains(&minutes))
            && (by_second.is_empty() || by_second.contains(&seconds))
        {
            break;
        }

        if hours == first_hours && minutes == first_minutes && seconds == first_seconds {
            return Err(RRuleError::new_iter_err(
                "Infinite loop detected. It can be resolved by changing `BYSECOND`, `BYMINUTE`, `BYHOUR` or `INTERVAL`",
            ));
        }
    }

    Ok(counter_date)
}

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

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use chrono_tz::UTC;

    use super::*;

    fn ymd_hms(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime {
        UTC.ymd(year, month, day).and_hms(hour, min, sec)
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
                ymd_hms(2021, 2, 28, 0, 0, 0),
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
        for (counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Yearly,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let output = increment_counter_date(counter_date, &rrule, false);
            assert_eq!(output, Ok(expected_output));
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
                ymd_hms(2020, 1, 31, 0, 0, 0),
                1,
                ymd_hms(2020, 2, 1, 0, 0, 0),
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
        for (counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Monthly,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let output = increment_counter_date(counter_date, &rrule, false);
            assert_eq!(output, Ok(expected_output));
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
        for (counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Weekly,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let output = increment_counter_date(counter_date, &rrule, false);
            assert_eq!(output, Ok(expected_output));
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
        for (counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Daily,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let output = increment_counter_date(counter_date, &rrule, false);
            assert_eq!(output, Ok(expected_output));
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
        for (counter_date, interval, by_hour, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Hourly,
                by_hour,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let output = increment_counter_date(counter_date, &rrule, false);
            assert_eq!(output, Ok(expected_output));
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
        for (counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Hourly,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let output = increment_counter_date(counter_date, &rrule, true);
            assert_eq!(output, Ok(expected_output));
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
        for (counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Minutely,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let output = increment_counter_date(counter_date, &rrule, false);
            assert_eq!(output, Ok(expected_output));
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
        for (counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Minutely,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let output = increment_counter_date(counter_date, &rrule, true);
            assert_eq!(output, Ok(expected_output));
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
        for (counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Secondly,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let output = increment_counter_date(counter_date, &rrule, false);
            assert_eq!(output, Ok(expected_output));
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
        for (counter_date, interval, expected_output) in tests {
            let rrule = RRule {
                interval,
                freq: Frequency::Secondly,
                ..Default::default()
            }
            .validate(UTC.ymd(1997, 1, 1).and_hms(1, 1, 1))
            .unwrap();

            let output = increment_counter_date(counter_date, &rrule, true);
            assert_eq!(output, Ok(expected_output));
        }
    }
}
