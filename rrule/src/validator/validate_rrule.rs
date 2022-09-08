use std::ops::RangeInclusive;

use crate::{Frequency, NWeekday, RRule, Unvalidated};

use super::ValidationError;

/// Range of values that a weekday can be.
/// Range: `0..=6`
#[allow(dead_code)]
pub(crate) static WEEKDAY_RANGE: RangeInclusive<u8> = 0..=6;

/// Range of values that a day can be.
/// Range: `1..=31`
pub(crate) static DAY_RANGE: RangeInclusive<u8> = 1..=31;

/// Range of values that a month can be.
/// Range: `1..=12`
pub(crate) static MONTH_RANGE: RangeInclusive<u8> = 1..=12;

/// Range of values that a year can be.
/// Range:
///  - `-10_000..=10_000`
///  - `-262_000..=262_000` when `no-validation-limits` flag is set
#[cfg(not(feature = "no-validation-limits"))]
pub(crate) static YEAR_RANGE: RangeInclusive<i32> = -10_000..=10_000;
#[cfg(feature = "no-validation-limits")]
pub(crate) static YEAR_RANGE: RangeInclusive<i32> = -262_000..=262_000;

/// Check if rules are valid as defined by the RFC and crate limitations.
/// It checks all values in the [`RRule`] and makes sure that they are in
/// the accepted ranges. If the function returns `Ok`, no errors where found.
///
/// This check should always be done and just enforces limits set by the standard.
/// Validation will always be enforced and can not be disabled using feature flags.
///
// TODO too many lines
#[warn(clippy::too_many_lines)]
pub(crate) fn validate_rrule_forced<TZ: chrono::TimeZone>(
    rrule: &RRule<TZ, Unvalidated>,
    dt_start: &chrono::DateTime<TZ>,
) -> Result<(), ValidationError> {
    // Freq:
    // - Enum, so always valid on its own.

    // Interval:
    // - Any positive number allowed.
    //   Value is u32, so does not allow for negative numbers anyway.

    // Count:
    // - Any positive number allowed
    //   Value is u32, so does not allow for negative numbers anyway.

    // Until:
    // - Must be same type as `dt_start`, so Date/DateTime.
    //   Can only be checked during parsing.
    // - Timezone should meet certain criteria depending on `dt_start`
    //   TODO: NOT validated for now. For more info see:
    //   https://icalendar.org/iCalendar-RFC-5545/3-3-10-recurrence-rule.html
    // - Value should be later then `dt_start`.
    //   TODO: Does this need to be checked? Will always return no events anyway.
    //   Validated below
    if let Some(until) = &rrule.until {
        // Check if before `dt_start`
        if until < dt_start {
            return Err(ValidationError::UntilBeforeStart {
                // Using `Debug` here since not `TimeZone` does not imply `Display`.
                // Notably, `chrono::Local` is not `Display`.
                until: format!("{:?}", until),
                dt_start: format!("{:?}", dt_start),
            });
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
    validate_not_equal_for_vec(&0, &rrule.by_set_pos, "BYSETPOS")?;
    let range = match rrule.freq {
        Frequency::Yearly | Frequency::Daily => -366..=366, // TODO is the daily range correct?
        Frequency::Monthly => -31..=31,
        Frequency::Weekly => -53..=53,
        Frequency::Hourly => -24..=24,
        Frequency::Minutely | Frequency::Secondly => -60..=60,
    };
    if let Err(value) = validate_range_for_vec_error(&range, &rrule.by_set_pos) {
        return Err(ValidationError::InvalidFieldValueRangeWithFreq {
            field: "BYSETPOS".into(),
            value: value.to_string(),
            freq: rrule.freq,
            start_idx: range.start().to_string(),
            end_idx: range.end().to_string(),
        });
    }
    // - It MUST only be used in conjunction with another BYxxx rule part.
    if !rrule.by_set_pos.is_empty()
        && rrule.by_easter.is_none()
        && rrule.by_hour.is_empty()
        && rrule.by_minute.is_empty()
        && rrule.by_second.is_empty()
        && rrule.by_month_day.is_empty()
        && rrule.by_month.is_empty()
        && rrule.by_year_day.is_empty()
        && rrule.by_week_no.is_empty()
        && rrule.by_weekday.is_empty()
    {
        return Err(ValidationError::BySetPosWithoutByRule);
    }

    // By_month:
    // - Can be a value from 1 to 12.
    //   Validated below
    validate_range_for_vec(&MONTH_RANGE, &rrule.by_month, "BYMONTH")?;

    // By_month_day:
    // - Can be a value from -31 to -1 and 1 to 31.
    //   Validated below
    validate_not_equal_for_vec(&0, &rrule.by_month_day, "BYMONTHDAY")?;
    validate_range_for_vec(&(-31..=31), &rrule.by_month_day, "BYMONTHDAY")?;
    // - MUST NOT be specified when the FREQ rule part is set to WEEKLY.
    //   Validated below
    if !rrule.by_month_day.is_empty() {
        let valid = rrule.freq != Frequency::Weekly;
        if !valid {
            return Err(ValidationError::InvalidByRuleAndFrequency {
                by_rule: "BYMONTHDAY".into(),
                freq: rrule.freq,
            });
        }
    }

    // By_n_month_day:
    // TODO? Not part of spec itself. Used in iter? Should this by empty at start?

    // By_year_day:
    // - Can be a value from -366 to -1 and 1 to 366.
    //   Validated below
    validate_not_equal_for_vec(&0, &rrule.by_year_day, "BYYEARDAY")?;
    validate_range_for_vec(&(-366..=366), &rrule.by_year_day, "BYYEARDAY")?;
    // - MUST NOT be specified when the FREQ rule part is set to DAILY, WEEKLY, or MONTHLY.
    //   Validated below
    if !rrule.by_year_day.is_empty() {
        let valid = !matches!(
            rrule.freq,
            Frequency::Monthly | Frequency::Weekly | Frequency::Daily
        );
        if !valid {
            return Err(ValidationError::InvalidByRuleAndFrequency {
                by_rule: "BYYEARDAY".into(),
                freq: rrule.freq,
            });
        }
    }

    // By_week_no:
    // - Can be a value from -53 to -1 and 1 to 53.
    //   Validated below
    validate_not_equal_for_vec(&0, &rrule.by_week_no, "BYWEEKNO")?;
    validate_range_for_vec(&(-53..=53), &rrule.by_week_no, "BYWEEKNO")?;
    // - MUST NOT be used when the FREQ rule part is set to anything other than YEARLY.
    //   Validated below
    if !rrule.by_week_no.is_empty() {
        let valid = rrule.freq == Frequency::Yearly;
        if !valid {
            return Err(ValidationError::InvalidByRuleAndFrequency {
                by_rule: "BYWEEKNO".into(),
                freq: rrule.freq,
            });
        }
    }

    // By_weekday:
    // - Check if value for `Nth` is within range.
    //   Range depends on frequency and can only happen weekly, so `/7` from normal count.
    let range = match rrule.freq {
        Frequency::Yearly | Frequency::Daily => (-366 / 7)..=(366 / 7 + 1), // TODO is the daily range correct?
        Frequency::Monthly => (-31 / 7)..=(31 / 7 + 1),
        Frequency::Weekly => (-53 / 7)..=(53 / 7 + 1),
        Frequency::Hourly => (-24 / 7)..=(24 / 7 + 1),
        Frequency::Minutely | Frequency::Secondly => (-60 / 7)..=(60 / 7 + 1),
    };
    for item in &rrule.by_weekday {
        if let NWeekday::Nth(number, _weekday) = item {
            // If value not in range = error
            if !range.contains(number) {
                return Err(ValidationError::InvalidFieldValueRangeWithFreq {
                    field: "BYDAY".into(),
                    value: number.to_string(),
                    freq: rrule.freq,
                    start_idx: range.start().to_string(),
                    end_idx: range.end().to_string(),
                });
            }
        }
    }

    // By_hour:
    // - Can be a value from 0 to 23.
    //   Validated below
    validate_range_for_vec(&(0..=23), &rrule.by_hour, "BYHOUR")?;

    // By_minute:
    // - Can be a value from 0 to 59.
    //   Validated below
    validate_range_for_vec(&(0..=59), &rrule.by_minute, "BYMINUTE")?;

    // By_second:
    // - Can be a value from 0 to 59.
    //   Validated below
    validate_range_for_vec(&(0..=59), &rrule.by_second, "BYSECOND")?;

    #[cfg(feature = "by-easter")]
    {
        // By_easter:
        // - Can be a value from -366 to 366.
        //   Validated below
        if let Some(by_easter) = &rrule.by_easter {
            validate_range_for_vec(&(-366..=366), &[*by_easter], "BYEASTER")?;
        }
        // - Can only be used on frequency: Yearly, Monthly, Daily
        //   Validated below
        if rrule.by_easter.is_some() {
            let valid = matches!(
                rrule.freq,
                Frequency::Yearly | Frequency::Monthly | Frequency::Daily
            );
            if !valid {
                return Err(ValidationError::InvalidByRuleAndFrequency {
                    by_rule: "BYEASTER".into(),
                    freq: rrule.freq,
                });
            }
        }
        // - Can only be used when `by_hour`, `by_minute` and `by_second` are used.
        //   TODO don't know why this is true, but seems to loop forever otherwise.
        if rrule.by_easter.is_some()
            && (rrule.by_hour.is_empty()
                || rrule.by_minute.is_empty()
                || rrule.by_second.is_empty())
        {
            return Err(ValidationError::InvalidByRuleWithByEaster);
        }
    }
    #[cfg(not(feature = "by-easter"))]
    {
        if rrule.by_easter.is_some() {
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

fn validate_range_for_vec<T: PartialOrd + ToString>(
    range: &RangeInclusive<T>,
    list: &[T],
    field: &str,
) -> Result<(), ValidationError> {
    for item in list {
        // If value not in range = error
        if !range.contains(item) {
            return Err(ValidationError::InvalidFieldValueRange {
                field: field.to_string(),
                value: item.to_string(),
                start_idx: range.start().to_string(),
                end_idx: range.end().to_string(),
            });
        }
    }
    Ok(())
}

fn validate_not_equal_for_vec<T: PartialEq<T> + ToString>(
    value: &T,
    list: &[T],
    field: &str,
) -> Result<(), ValidationError> {
    if list.iter().any(|item| item == value) {
        return Err(ValidationError::InvalidFieldValue {
            field: field.to_string(),
            value: value.to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use chrono_tz::UTC;

    use super::*;

    #[test]
    fn rejects_by_set_pos_without_byxxx_rule() {
        let rrule = RRule {
            by_set_pos: vec![-1],
            ..Default::default()
        };
        let dt_start = UTC.ymd(1970, 1, 1).and_hms(0, 0, 0);
        let res = validate_rrule_forced(&rrule, &dt_start);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, ValidationError::BySetPosWithoutByRule);

        // When creating RRule directly,
        // if `rrule.by_set_hour` is empty then it is going to default to
        // `dt_start.hour()`, therefore there is always a BYXXX
        // rule and the rrule is accepted.
        let res = rrule.build(dt_start);
        assert!(res.is_ok());
    }

    #[test]
    fn rejects_by_rule_field_with_invalid_value() {
        let tests = [
            (
                "BYSETPOS",
                RRule {
                    by_set_pos: vec![0],
                    ..Default::default()
                },
            ),
            (
                "BYSETPOS",
                RRule {
                    by_set_pos: vec![1, -2, 0],
                    ..Default::default()
                },
            ),
            (
                "BYMONTHDAY",
                RRule {
                    by_month_day: vec![0],
                    ..Default::default()
                },
            ),
            (
                "BYYEARDAY",
                RRule {
                    by_year_day: vec![0],
                    ..Default::default()
                },
            ),
        ];
        for (field, rrule) in tests {
            let res = validate_rrule_forced(&rrule, &UTC.ymd(1970, 1, 1).and_hms(0, 0, 0));
            assert!(res.is_err());
            let err = res.unwrap_err();
            assert_eq!(
                err,
                ValidationError::InvalidFieldValue {
                    field: field.into(),
                    value: "0".into()
                }
            );
        }
    }

    #[test]
    fn rejects_by_rule_field_with_value_outside_allowed_range() {
        let tests = [
            (
                "BYMONTHDAY",
                RRule {
                    by_month_day: vec![34],
                    ..Default::default()
                },
                "34",
                "-31",
                "31",
            ),
            (
                "BYYEARDAY",
                RRule {
                    by_year_day: vec![17, 400],
                    ..Default::default()
                },
                "400",
                "-366",
                "366",
            ),
        ];
        for (field, rrule, value, start_idx, end_idx) in tests {
            let res = validate_rrule_forced(&rrule, &UTC.ymd(1970, 1, 1).and_hms(0, 0, 0));
            assert!(res.is_err());
            let err = res.unwrap_err();
            assert_eq!(
                err,
                ValidationError::InvalidFieldValueRange {
                    field: field.into(),
                    value: value.into(),
                    start_idx: start_idx.into(),
                    end_idx: end_idx.into()
                }
            );
        }
    }

    #[test]
    fn rejects_by_rule_value_outside_allowed_freq_range() {
        let tests = [
            (
                "BYSETPOS",
                RRule {
                    freq: Frequency::Hourly,
                    by_set_pos: vec![30],
                    ..Default::default()
                },
                "30",
                "-24",
                "24",
            ),
            (
                "BYSETPOS",
                RRule {
                    freq: Frequency::Yearly,
                    by_set_pos: vec![400],
                    ..Default::default()
                },
                "400",
                "-366",
                "366",
            ),
        ];
        for (field, rrule, value, start_idx, end_idx) in tests {
            let res = validate_rrule_forced(&rrule, &UTC.ymd(1970, 1, 1).and_hms(0, 0, 0));
            assert!(res.is_err());
            let err = res.unwrap_err();
            assert_eq!(
                err,
                ValidationError::InvalidFieldValueRangeWithFreq {
                    field: field.into(),
                    freq: rrule.freq,
                    value: value.into(),
                    start_idx: start_idx.into(),
                    end_idx: end_idx.into(),
                }
            );
        }
    }

    #[test]
    fn rejects_invalid_by_rule_and_freq_combos() {
        let tests = [
            (
                "BYMONTHDAY",
                RRule {
                    freq: Frequency::Weekly,
                    by_month_day: vec![-1],
                    ..Default::default()
                },
            ),
            (
                "BYYEARDAY",
                RRule {
                    freq: Frequency::Monthly,
                    by_year_day: vec![120],
                    ..Default::default()
                },
            ),
            (
                "BYYEARDAY",
                RRule {
                    freq: Frequency::Weekly,
                    by_year_day: vec![120],
                    ..Default::default()
                },
            ),
            (
                "BYYEARDAY",
                RRule {
                    freq: Frequency::Daily,
                    by_year_day: vec![120],
                    ..Default::default()
                },
            ),
        ];
        for (field, rrule) in tests {
            let res = validate_rrule_forced(&rrule, &UTC.ymd(1970, 1, 1).and_hms(0, 0, 0));
            assert!(res.is_err());
            let err = res.unwrap_err();
            assert_eq!(
                err,
                ValidationError::InvalidByRuleAndFrequency {
                    by_rule: field.into(),
                    freq: rrule.freq
                }
            );
        }
    }

    #[test]
    fn rejects_start_date_after_until() {
        let rrule = RRule {
            until: Some(UTC.ymd_opt(2020, 1, 1).and_hms_opt(0, 0, 0).unwrap()),
            ..Default::default()
        };
        let dt_start = UTC.ymd_opt(2020, 1, 2).and_hms_opt(0, 0, 0).unwrap();
        let res = validate_rrule_forced(&rrule, &dt_start);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err,
            ValidationError::UntilBeforeStart {
                until: format!("{:?}", rrule.until.unwrap()),
                dt_start: format!("{:?}", dt_start),
            }
        );
    }
}
