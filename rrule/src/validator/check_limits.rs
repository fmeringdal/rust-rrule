use crate::core::DateTime;
use crate::{validator::ValidationError, RRuleProperties};

/// Maximum value of `option.interval` when frequency is yearly.
/// Limit: 10000 years
/// Does not apply when `no-validation-limits` feature flag is set.
#[allow(dead_code)]
pub(crate) static FREQ_YEARLY_INTERVAL_MAX: u16 = 10_000;
/// Maximum value of `option.interval` when frequency is monthly.
/// Limit: about 83 years
/// Does not apply when `no-validation-limits` feature flag is set.
#[allow(dead_code)]
pub(crate) static FREQ_MONTHLY_INTERVAL_MAX: u16 = 1_000;
/// Maximum value of `option.interval` when frequency is weekly.
/// Limit: about 19 years
/// Does not apply when `no-validation-limits` feature flag is set.
#[allow(dead_code)]
pub(crate) static FREQ_WEEKLY_INTERVAL_MAX: u16 = 1_000;
/// Maximum value of `option.interval` when frequency is daily.
/// Limit: about 27 years
/// Does not apply when `no-validation-limits` feature flag is set.
#[allow(dead_code)]
pub(crate) static FREQ_DAILY_INTERVAL_MAX: u16 = 10_000;
/// Maximum value of `option.interval` when frequency is hourly.
/// Limit: about 416 days
/// Does not apply when `no-validation-limits` feature flag is set.
#[allow(dead_code)]
pub(crate) static FREQ_HOURLY_INTERVAL_MAX: u16 = 10_000;
/// Maximum value of `option.interval` when frequency is minutely.
/// Limit: about 7 days
/// Does not apply when `no-validation-limits` feature flag is set.
#[allow(dead_code)]
pub(crate) static FREQ_MINUTELY_INTERVAL_MAX: u16 = 10_000;
/// Maximum value of `option.interval` when frequency is secondly.
/// Limit: about 13 hours
/// Does not apply when `no-validation-limits` feature flag is set.
#[allow(dead_code)]
pub(crate) static FREQ_SECONDLY_INTERVAL_MAX: u16 = 50_000;

/// Check (arbitrary) validator limits.
/// It checks all values in the [`RRuleProperties`] and makes sure that they are in
/// a reasonable range. If the function returns `Ok`, no errors where found.
///
/// This will prevent the creation of iterators that will always fail.
/// This validation will limit, if not eliminate, large groups of potential panics when using the
/// iterator.
///
/// When `no-validation-limits` feature is set this function will always return `Ok`.
/// See README.md for more info.
#[cfg(not(feature = "no-validation-limits"))]
pub(crate) fn check_limits(
    properties: &RRuleProperties,
    dt_start: &DateTime,
) -> Result<(), ValidationError> {
    use crate::{validator::YEAR_RANGE, Frequency};
    use chrono::Datelike;

    // Interval:
    // - Value should not be to big
    //   `Chrono` is limited to +/- 262,000 years from the common epoch.
    let limit = match &properties.freq {
        Frequency::Yearly => FREQ_YEARLY_INTERVAL_MAX, // 10000 years
        Frequency::Monthly => FREQ_MONTHLY_INTERVAL_MAX, // About 83 years
        Frequency::Weekly => FREQ_WEEKLY_INTERVAL_MAX, // About 19 years
        Frequency::Daily => FREQ_DAILY_INTERVAL_MAX,   // About 27 years
        Frequency::Hourly => FREQ_HOURLY_INTERVAL_MAX, // About 416 days
        Frequency::Minutely => FREQ_MINUTELY_INTERVAL_MAX, // About 7 days
        Frequency::Secondly => FREQ_SECONDLY_INTERVAL_MAX, // About 13 hours
    };
    if properties.interval > limit {
        return Err(ValidationError::TooBigInterval(properties.interval));
    }

    // Count:
    // Does not interfere with the iterator so does not have to be limited.

    // Dt_start:
    // - Year should be reasonable.
    //   `Chrono` is limited to +/- 262,000 years from the common epoch.
    let year = dt_start.year();
    if !YEAR_RANGE.contains(&year) {
        return Err(ValidationError::StartYearOutOfRange(year));
    }

    // All checked, no errors found
    Ok(())
}

/// Check validator limits.
/// It checks all values in the [`RRuleProperties`] and makes sure that they are in
/// a reasonable range. If the function returns `Ok`, no errors where found.
///
/// This will prevent the creation of iterators that will always fail.
/// This validation will limit, if not eliminate, large groups of potential panics when using the
/// iterator.
///
/// When `no-validation-limits` feature is set this function will always return `Ok`.
/// See README.md for more info.
#[cfg(feature = "no-validation-limits")]
pub(crate) fn check_limits(_: &RRuleProperties, _: &DateTime) -> Result<(), ValidationError> {
    Ok(())
}
