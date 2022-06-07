#![allow(clippy::module_name_repetitions)]

use thiserror::Error;

use crate::Frequency;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    #[error("BYSETPOS should only be used in conjunction with another BYxxx rule part.")]
    BySetPosWithoutByRule,
    #[error("`{field}` can not be `{value}`, must be larger or smaller then `{value}`.")]
    InvalidFieldValue { field: String, value: String },
    #[error("`{field}` is `{value}`, but is not allowed outside of the range: `{start_idx}..={end_idx}`.")]
    InvalidFieldValueRange {
        field: String,
        value: String,
        start_idx: String,
        end_idx: String,
    },
    #[error(
        "`{field}` is `{value}`, but with the current frequency ({freq}) is not allowed \
            outside the range: `{start_idx}..={end_idx}`."
    )]
    InvalidFieldValueRangeWithFreq {
        field: String,
        value: String,
        freq: Frequency,
        start_idx: String,
        end_idx: String,
    },
    #[error("`{by_rule}` can not be used with the current frequency ({freq}).")]
    InvalidByRuleAndFrequency { by_rule: String, freq: Frequency },
    #[error("`UNTIL` is `{until}`, but `DTSTART` (`{dt_start}`) is later. That should not be happening.")]
    UntilBeforeStart { until: String, dt_start: String },
    #[error(
        "`INTERVAL` is `{0}`, is higher than expected, make sure this is correct. \
            See 'validator limits' in docs for more info."
    )]
    TooBigInterval(u16),
    #[error(
        "`DTSTART` year is `{0}`, is higher/lower than expected, make sure this is correct. \
            See 'validator limits' in docs for more info."
    )]
    StartYearOutOfRange(i32),
    #[cfg(feature = "by-easter")]
    #[error("`BYEASTER` can only be used when `BYHOUR`, `BYMINUTE` and `BYSECOND` are set.")]
    InvalidByRuleWithByEaster,
}
