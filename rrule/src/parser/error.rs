#![allow(clippy::module_name_repetitions)]

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    #[error(
        "Did not find any start date for the recurrence rule. Please specify a `DTSTART` field."
    )]
    MissingStartDate,
    #[error("`{0}` is not a valid timezone.")]
    InvalidTimezone(String),
    #[error("`{value}` is not a valid datetime format for `{field}`.")]
    InvalidDateTime { value: String, field: String },
    #[error("{field}:{value} is not a valid datetime in local timezone.")]
    InvalidDateTimeInLocalTimezone { value: String, field: String },
    #[error("{field}:{value} is not a valid datetime in local timezone. This value is ambiguous and can be `{date1}` or `{date2}`")]
    DateTimeInLocalTimezoneIsAmbiguous {
        value: String,
        field: String,
        date1: String,
        date2: String,
    },
    #[error("`{0}` is not a valid frequency.")]
    InvalidFrequency(String),
    #[error("`{0}` is not a valid weekday.")]
    InvalidWeekday(String),
    #[error("The field `{0}` was found twice.")]
    DuplicatedField(String),
    // TODO: remove this variant and use specific errors
    #[error("{0}")]
    Generic(String),
    #[error("Input string contains some invalid characters.")]
    InvalidInputString,
}
