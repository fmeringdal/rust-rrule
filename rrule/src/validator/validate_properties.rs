use std::{fmt::Display, ops::RangeInclusive};

use crate::{Frequency, NWeekday, RRuleError, RRuleProperties};

/// Range of values that a weekday can be.
/// Range: `0..=6`
#[allow(dead_code)]
pub static WEEKDAY_RANGE: RangeInclusive<u8> = 0..=6;

/// Range of values that a day can be.
/// Range: `1..=31`
pub static DAY_RANGE: RangeInclusive<u8> = 1..=31;

/// Range of values that a month can be.
/// Range: `1..=12`
pub static MONTH_RANGE: RangeInclusive<u8> = 1..=12;

/// Range of values that a year can be.
/// Range:
///  - `-10_000..=10_000`
///  - `-262_000..=262_000` when `no-validation-limits` flag is set
#[cfg(not(feature = "no-validation-limits"))]
pub static YEAR_RANGE: RangeInclusive<i32> = -10_000..=10_000;
#[cfg(feature = "no-validation-limits")]
pub static YEAR_RANGE: RangeInclusive<i32> = -262_000..=262_000;

/// Check if rules are valid as defined by the RFC and crate limitations.
/// It checks all values in the [`RRuleProperties`] and makes sure that they are in
/// the accepted ranges. If the function returns `Ok`, no errors where found.
///
/// This check should always be done and just enforces limits set by the standard.
/// Validation will always be enforced and can not be disabled using feature flags.
///
pub fn validate_properties_forced(option: &RRuleProperties) -> Result<(), RRuleError> {
    // Freq:
    // - Enum, so always valid on its own.

    // Interval:
    // - Any positive number allowed.
    //   Value is u32, no does not allows for negative numbers anyway.

    // Count:
    // - Any positive number allowed
    //   Value is u32, no does not allows for negative numbers anyway.

    // Until:
    // - Must be same type as `dt_start`, so Date/DateTime.
    //   Can only be checked during parsing.
    // - Timezone should meet certain criteria depending on `dt_start`
    //   TODO: NOT validated for now. For more info see:
    //   https://icalendar.org/iCalendar-RFC-5545/3-3-10-recurrence-rule.html
    // - Value should be later then `dt_start`.
    //   TODO: Does this need to be checked? Will always return no events anyway.
    //   TODO: Add test for this.
    //   Validated below
    if let Some(until) = &option.until {
        // Check if before `dt_start`
        if until < &option.dt_start {
            return Err(RRuleError::new_validation_err(format!(
                "`UNTIL` is `{}`, but `DTSTART` (`{}`) is later. \
                That should not be happening.",
                until.to_rfc3339(),
                option.dt_start.to_rfc3339()
            )));
        }
    }

    // Tz:
    // - Any timezone is allowed
    // - TODO: Might need to match `until` and `dt_start`.

    // Dt_start:
    // - All values are allowed.
    // - TODO: Timezone still need to be checked.

    // Week_start:
    // - Enum, so always valid on its own.

    // By_set_pos:
    // - Can be a value from -366 to -1 and 1 to 366 depending on `freq`
    //   Validated below
    validate_not_equal_for_vec(&0, &option.by_set_pos, "BYSETPOS")?;
    let range = match option.freq {
        Frequency::Yearly => -366..=366,
        Frequency::Monthly => -31..=31,
        Frequency::Weekly => -53..=53,
        Frequency::Daily => -366..=366, // TODO is this range correct?
        Frequency::Hourly => -24..=24,
        Frequency::Minutely => -60..=60,
        Frequency::Secondly => -60..=60,
    };
    if let Err(value) = validate_range_for_vec_error(&range, &option.by_set_pos) {
        return Err(RRuleError::new_validation_err(format!(
            "`BYSETPOS` is `{}`, but with the current frequency ({}) is not allowed \
            outside of the range: `{}..={}`.",
            value,
            option.freq,
            range.start(),
            range.end()
        )));
    }
    // - It MUST only be used in conjunction with another BYxxx rule part.
    //   TODO

    // By_month:
    // - Can be a value from 1 to 12.
    //   Validated below
    validate_range_for_vec(&MONTH_RANGE, &option.by_month, "BYMONTH")?;

    // By_month_day:
    // - Can be a value from -31 to -1 and 1 to 31.
    //   Validated below
    validate_not_equal_for_vec(&0, &option.by_month_day, "BYMONTHDAY")?;
    validate_range_for_vec(&(-31..=31), &option.by_month_day, "BYMONTHDAY")?;
    // - MUST NOT be specified when the FREQ rule part is set to WEEKLY.
    //   Validated below
    if !option.by_month_day.is_empty() {
        let valid = option.freq != Frequency::Weekly;
        if !valid {
            return Err(RRuleError::new_validation_err(format!(
                "`BYMONTHDAY` can not be used with the current frequency ({}).",
                option.freq,
            )));
        }
    }

    // By_n_month_day:
    // TODO? Not part of spec itself. Used in iter? Should this by empty at start?

    // By_year_day:
    // - Can be a value from -366 to -1 and 1 to 366.
    //   Validated below
    validate_not_equal_for_vec(&0, &option.by_year_day, "BYYEARDAY")?;
    validate_range_for_vec(&(-366..=366), &option.by_year_day, "BYYEARDAY")?;
    // - MUST NOT be specified when the FREQ rule part is set to DAILY, WEEKLY, or MONTHLY.
    //   Validated below
    if !option.by_year_day.is_empty() {
        let valid = !matches!(
            option.freq,
            Frequency::Monthly | Frequency::Weekly | Frequency::Daily
        );
        if !valid {
            return Err(RRuleError::new_validation_err(format!(
                "`BYYEARDAY` can not be used with the current frequency ({}).",
                option.freq,
            )));
        }
    }

    // By_week_no:
    // - Can be a value from -53 to -1 and 1 to 53.
    //   Validated below
    validate_not_equal_for_vec(&0, &option.by_week_no, "BYWEEKNO")?;
    validate_range_for_vec(&(-53..=53), &option.by_week_no, "BYWEEKNO")?;
    // - MUST NOT be used when the FREQ rule part is set to anything other than YEARLY.
    //   Validated below
    if !option.by_week_no.is_empty() {
        let valid = option.freq == Frequency::Yearly;
        if !valid {
            return Err(RRuleError::new_validation_err(format!(
                "`BYWEEKNO` can not be used with the current frequency ({}).",
                option.freq,
            )));
        }
    }

    // By_weekday:
    // - Check if value for `Nth` is within range.
    //   Range depends on frequency and can only happen weekly, so `/7` from normal count.
    let range = match option.freq {
        Frequency::Yearly => (-366 / 7)..=(366 / 7 + 1),
        Frequency::Monthly => (-31 / 7)..=(31 / 7 + 1),
        Frequency::Weekly => (-53 / 7)..=(53 / 7 + 1),
        Frequency::Daily => (-366 / 7)..=(366 / 7 + 1), // TODO is this range correct?
        Frequency::Hourly => (-24 / 7)..=(24 / 7 + 1),
        Frequency::Minutely => (-60 / 7)..=(60 / 7 + 1),
        Frequency::Secondly => (-60 / 7)..=(60 / 7 + 1),
    };
    for item in &option.by_weekday {
        if let NWeekday::Nth(number, _weekday) = item {
            // If value not in range = error
            if !range.contains(number) {
                return Err(RRuleError::new_validation_err(format!(
                    "`BYDAY` nth occurrence is `{}`, but is not allowed outside of the range: `{}..={}`.",
                    number,
                    range.start(),
                    range.end()
                )));
            }
        }
    }

    // By_hour:
    // - Can be a value from 0 to 23.
    //   Validated below
    validate_range_for_vec(&(0..=23), &option.by_hour, "BYHOUR")?;

    // By_minute:
    // - Can be a value from 0 to 59.
    //   Validated below
    validate_range_for_vec(&(0..=59), &option.by_minute, "BYMINUTE")?;

    // By_second:
    // - Can be a value from 0 to 59.
    //   Validated below
    validate_range_for_vec(&(0..=59), &option.by_second, "BYSECOND")?;

    #[cfg(feature = "by-easter")]
    {
        // By_easter:
        // - Can be a value from -366 to 366.
        //   Validated below
        if let Some(by_easter) = &option.by_easter {
            validate_range_for_vec(&(-366..=366), &vec![*by_easter], "BYEASTER")?;
        }
        // - Can only be used on frequency: Yearly, Monthly, Daily
        //   Validated below
        if option.by_easter.is_some() {
            let valid = match option.freq {
                Frequency::Yearly => true,
                Frequency::Monthly => true,
                Frequency::Daily => true,
                _ => false,
            };
            if !valid {
                return Err(RRuleError::new_validation_err(format!(
                    "`BYEASTER` can not be used with the current frequency ({}).",
                    option.freq,
                )));
            }
        }
        // - Can only be used when `by_hour`, `by_minute` and `by_second` are used.
        //   TODO don't know why this is true, but seems to loop forever otherwise.
        if option.by_easter.is_some()
            && (option.by_hour.is_empty()
                || option.by_minute.is_empty()
                || option.by_second.is_empty())
        {
            return Err(RRuleError::new_validation_err(
                "`BYEASTER` can only be used when `BYHOUR`, `BYMINUTE` and `BYSECOND` are set.",
            ));
        }
    }
    #[cfg(not(feature = "by-easter"))]
    {
        if option.by_easter.is_some() {
            log::warn!(
                "The `by-easter` feature flag is not set, but `by_easter` is used.\
                The `by_easter` will be ignored, as if it was set to `None`."
            );
        }
    }

    // All checked, no errors found
    Ok(())
}

fn validate_range_for_vec_error<'a, T: PartialOrd>(
    range: &RangeInclusive<T>,
    list: &'a [T],
) -> Result<(), &'a T> {
    for item in list {
        // If value not in range = error
        if !range.contains(item) {
            return Err(item);
        }
    }
    Ok(())
}

fn validate_range_for_vec<T: PartialOrd + Display>(
    range: &RangeInclusive<T>,
    list: &[T],
    field: &str,
) -> Result<(), RRuleError> {
    for item in list {
        // If value not in range = error
        if !range.contains(item) {
            return Err(RRuleError::new_validation_err(format!(
                "`{}` is `{}`, but is not allowed outside of the range: `{}..={}`.",
                field,
                item,
                range.start(),
                range.end()
            )));
        }
    }
    Ok(())
}

fn validate_not_equal_for_vec<T: PartialEq<T> + Display>(
    value: &T,
    list: &[T],
    field: &str,
) -> Result<(), RRuleError> {
    for item in list {
        if item == value {
            return Err(RRuleError::new_validation_err(format!(
                "`{}` can not be `{}`, must be larger or smaller then `{}`.",
                field, value, value,
            )));
        }
    }
    Ok(())
}
