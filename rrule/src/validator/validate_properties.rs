use std::ops::RangeInclusive;

use crate::core::DateTime;
use crate::{Frequency, NWeekday, RRuleProperties};

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
/// It checks all values in the [`RRuleProperties`] and makes sure that they are in
/// the accepted ranges. If the function returns `Ok`, no errors where found.
///
/// This check should always be done and just enforces limits set by the standard.
/// Validation will always be enforced and can not be disabled using feature flags.
///
pub(crate) fn validate_properties_forced(
    properties: &RRuleProperties,
    dt_start: &DateTime,
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
    if let Some(until) = &properties.until {
        // Check if before `dt_start`
        if until < dt_start {
            return Err(ValidationError::UntilBeforeStart {
                until: until.to_rfc3339(),
                dt_start: dt_start.to_rfc3339(),
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
    validate_not_equal_for_vec(&0, &properties.by_set_pos, "BYSETPOS")?;
    let range = match properties.freq {
        Frequency::Yearly => -366..=366,
        Frequency::Monthly => -31..=31,
        Frequency::Weekly => -53..=53,
        Frequency::Daily => -366..=366, // TODO is this range correct?
        Frequency::Hourly => -24..=24,
        Frequency::Minutely => -60..=60,
        Frequency::Secondly => -60..=60,
    };
    if let Err(value) = validate_range_for_vec_error(&range, &properties.by_set_pos) {
        return Err(ValidationError::InvalidFieldValueRangeWithFreq {
            field: "BYSETPOS".into(),
            value: value.to_string(),
            freq: properties.freq,
            start_idx: range.start().to_string(),
            end_idx: range.end().to_string(),
        });
    }
    // - It MUST only be used in conjunction with another BYxxx rule part.
    if !properties.by_set_pos.is_empty()
        && properties.by_easter.is_none()
        && properties.by_hour.is_empty()
        && properties.by_minute.is_empty()
        && properties.by_second.is_empty()
        && properties.by_month_day.is_empty()
        && properties.by_month.is_empty()
        && properties.by_year_day.is_empty()
        && properties.by_week_no.is_empty()
        && properties.by_weekday.is_empty()
    {
        return Err(ValidationError::BySetPosWithoutByRule);
    }

    // By_month:
    // - Can be a value from 1 to 12.
    //   Validated below
    validate_range_for_vec(&MONTH_RANGE, &properties.by_month, "BYMONTH")?;

    // By_month_day:
    // - Can be a value from -31 to -1 and 1 to 31.
    //   Validated below
    validate_not_equal_for_vec(&0, &properties.by_month_day, "BYMONTHDAY")?;
    validate_range_for_vec(&(-31..=31), &properties.by_month_day, "BYMONTHDAY")?;
    // - MUST NOT be specified when the FREQ rule part is set to WEEKLY.
    //   Validated below
    if !properties.by_month_day.is_empty() {
        let valid = properties.freq != Frequency::Weekly;
        if !valid {
            return Err(ValidationError::InvalidByRuleAndFrequency {
                by_rule: "BYMONTHDAY".into(),
                freq: properties.freq,
            });
        }
    }

    // By_n_month_day:
    // TODO? Not part of spec itself. Used in iter? Should this by empty at start?

    // By_year_day:
    // - Can be a value from -366 to -1 and 1 to 366.
    //   Validated below
    validate_not_equal_for_vec(&0, &properties.by_year_day, "BYYEARDAY")?;
    validate_range_for_vec(&(-366..=366), &properties.by_year_day, "BYYEARDAY")?;
    // - MUST NOT be specified when the FREQ rule part is set to DAILY, WEEKLY, or MONTHLY.
    //   Validated below
    if !properties.by_year_day.is_empty() {
        let valid = !matches!(
            properties.freq,
            Frequency::Monthly | Frequency::Weekly | Frequency::Daily
        );
        if !valid {
            return Err(ValidationError::InvalidByRuleAndFrequency {
                by_rule: "BYYEARDAY".into(),
                freq: properties.freq,
            });
        }
    }

    // By_week_no:
    // - Can be a value from -53 to -1 and 1 to 53.
    //   Validated below
    validate_not_equal_for_vec(&0, &properties.by_week_no, "BYWEEKNO")?;
    validate_range_for_vec(&(-53..=53), &properties.by_week_no, "BYWEEKNO")?;
    // - MUST NOT be used when the FREQ rule part is set to anything other than YEARLY.
    //   Validated below
    if !properties.by_week_no.is_empty() {
        let valid = properties.freq == Frequency::Yearly;
        if !valid {
            return Err(ValidationError::InvalidByRuleAndFrequency {
                by_rule: "BYWEEKNO".into(),
                freq: properties.freq,
            });
        }
    }

    // By_weekday:
    // - Check if value for `Nth` is within range.
    //   Range depends on frequency and can only happen weekly, so `/7` from normal count.
    let range = match properties.freq {
        Frequency::Yearly => (-366 / 7)..=(366 / 7 + 1),
        Frequency::Monthly => (-31 / 7)..=(31 / 7 + 1),
        Frequency::Weekly => (-53 / 7)..=(53 / 7 + 1),
        Frequency::Daily => (-366 / 7)..=(366 / 7 + 1), // TODO is this range correct?
        Frequency::Hourly => (-24 / 7)..=(24 / 7 + 1),
        Frequency::Minutely => (-60 / 7)..=(60 / 7 + 1),
        Frequency::Secondly => (-60 / 7)..=(60 / 7 + 1),
    };
    for item in &properties.by_weekday {
        if let NWeekday::Nth(number, _weekday) = item {
            // If value not in range = error
            if !range.contains(number) {
                return Err(ValidationError::InvalidFieldValueRangeWithFreq {
                    field: "BYDAY".into(),
                    value: number.to_string(),
                    freq: properties.freq,
                    start_idx: range.start().to_string(),
                    end_idx: range.end().to_string(),
                });
            }
        }
    }

    // By_hour:
    // - Can be a value from 0 to 23.
    //   Validated below
    validate_range_for_vec(&(0..=23), &properties.by_hour, "BYHOUR")?;

    // By_minute:
    // - Can be a value from 0 to 59.
    //   Validated below
    validate_range_for_vec(&(0..=59), &properties.by_minute, "BYMINUTE")?;

    // By_second:
    // - Can be a value from 0 to 59.
    //   Validated below
    validate_range_for_vec(&(0..=59), &properties.by_second, "BYSECOND")?;

    #[cfg(feature = "by-easter")]
    {
        // By_easter:
        // - Can be a value from -366 to 366.
        //   Validated below
        if let Some(by_easter) = &properties.by_easter {
            validate_range_for_vec(&(-366..=366), &[*by_easter], "BYEASTER")?;
        }
        // - Can only be used on frequency: Yearly, Monthly, Daily
        //   Validated below
        if properties.by_easter.is_some() {
            let valid = matches!(
                properties.freq,
                Frequency::Yearly | Frequency::Monthly | Frequency::Daily
            );
            if !valid {
                return Err(ValidationError::InvalidByRuleAndFrequency {
                    by_rule: "BYEASTER".into(),
                    freq: properties.freq,
                });
            }
        }
        // - Can only be used when `by_hour`, `by_minute` and `by_second` are used.
        //   TODO don't know why this is true, but seems to loop forever otherwise.
        if properties.by_easter.is_some()
            && (properties.by_hour.is_empty()
                || properties.by_minute.is_empty()
                || properties.by_second.is_empty())
        {
            return Err(ValidationError::InvalidByRuleWithByEaster);
        }
    }
    #[cfg(not(feature = "by-easter"))]
    {
        if properties.by_easter.is_some() {
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
        let properties = RRuleProperties {
            by_set_pos: vec![-1],
            ..Default::default()
        };
        let dt_start = UTC.ymd(1970, 1, 1).and_hms(0, 0, 0);
        let res = validate_properties_forced(&properties, &dt_start);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, ValidationError::BySetPosWithoutByRule);

        // When creating RRule directly,
        // if `properties.by_set_hour` is empty then it is going to default to
        // `dt_start.hour()`, therefore there is always a BYXXX
        // rule and the properties are accepted.
        let res = properties.build(dt_start);
        assert!(res.is_ok());
    }

    #[test]
    fn rejects_by_rule_field_with_invalid_value() {
        let tests = [
            (
                "BYSETPOS",
                RRuleProperties {
                    by_set_pos: vec![0],
                    ..Default::default()
                },
            ),
            (
                "BYSETPOS",
                RRuleProperties {
                    by_set_pos: vec![1, -2, 0],
                    ..Default::default()
                },
            ),
            (
                "BYMONTHDAY",
                RRuleProperties {
                    by_month_day: vec![0],
                    ..Default::default()
                },
            ),
            (
                "BYYEARDAY",
                RRuleProperties {
                    by_year_day: vec![0],
                    ..Default::default()
                },
            ),
        ];
        for (field, properties) in tests {
            let res =
                validate_properties_forced(&properties, &UTC.ymd(1970, 1, 1).and_hms(0, 0, 0));
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
                RRuleProperties {
                    by_month_day: vec![34],
                    ..Default::default()
                },
                "34",
                "-31",
                "31",
            ),
            (
                "BYYEARDAY",
                RRuleProperties {
                    by_year_day: vec![17, 400],
                    ..Default::default()
                },
                "400",
                "-366",
                "366",
            ),
        ];
        for (field, properties, value, start_idx, end_idx) in tests {
            let res =
                validate_properties_forced(&properties, &UTC.ymd(1970, 1, 1).and_hms(0, 0, 0));
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
                RRuleProperties {
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
                RRuleProperties {
                    freq: Frequency::Yearly,
                    by_set_pos: vec![400],
                    ..Default::default()
                },
                "400",
                "-366",
                "366",
            ),
        ];
        for (field, properties, value, start_idx, end_idx) in tests {
            let res =
                validate_properties_forced(&properties, &UTC.ymd(1970, 1, 1).and_hms(0, 0, 0));
            assert!(res.is_err());
            let err = res.unwrap_err();
            assert_eq!(
                err,
                ValidationError::InvalidFieldValueRangeWithFreq {
                    field: field.into(),
                    freq: properties.freq,
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
                RRuleProperties {
                    freq: Frequency::Weekly,
                    by_month_day: vec![-1],
                    ..Default::default()
                },
            ),
            (
                "BYYEARDAY",
                RRuleProperties {
                    freq: Frequency::Monthly,
                    by_year_day: vec![120],
                    ..Default::default()
                },
            ),
            (
                "BYYEARDAY",
                RRuleProperties {
                    freq: Frequency::Weekly,
                    by_year_day: vec![120],
                    ..Default::default()
                },
            ),
            (
                "BYYEARDAY",
                RRuleProperties {
                    freq: Frequency::Daily,
                    by_year_day: vec![120],
                    ..Default::default()
                },
            ),
        ];
        for (field, properties) in tests {
            let res =
                validate_properties_forced(&properties, &UTC.ymd(1970, 1, 1).and_hms(0, 0, 0));
            assert!(res.is_err());
            let err = res.unwrap_err();
            assert_eq!(
                err,
                ValidationError::InvalidByRuleAndFrequency {
                    by_rule: field.into(),
                    freq: properties.freq
                }
            );
        }
    }

    #[test]
    fn rejects_start_date_after_until() {
        let properties = RRuleProperties {
            until: Some(UTC.ymd_opt(2020, 1, 1).and_hms_opt(0, 0, 0).unwrap()),
            ..Default::default()
        };
        let dt_start = UTC.ymd_opt(2020, 1, 2).and_hms_opt(0, 0, 0).unwrap();
        let res = validate_properties_forced(&properties, &dt_start);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err,
            ValidationError::UntilBeforeStart {
                until: properties.until.unwrap().to_rfc3339(),
                dt_start: dt_start.to_rfc3339()
            }
        );
    }
}
