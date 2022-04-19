//! This module includes everything needed to validate an [RRuleProperties].
//! And in turn create a RRule.

pub(crate) mod check_limits;
mod error;
pub(crate) mod validate_properties;

pub(crate) use error::ValidationError;

#[allow(unused_imports)]
pub(crate) use check_limits::{
    FREQ_DAILY_INTERVAL_MAX, FREQ_HOURLY_INTERVAL_MAX, FREQ_MINUTELY_INTERVAL_MAX,
    FREQ_MONTHLY_INTERVAL_MAX, FREQ_SECONDLY_INTERVAL_MAX, FREQ_WEEKLY_INTERVAL_MAX,
    FREQ_YEARLY_INTERVAL_MAX,
};
pub(crate) use validate_properties::{DAY_RANGE, MONTH_RANGE, YEAR_RANGE};
