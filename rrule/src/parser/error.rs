use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParseError {
    #[error(
        "Did not find any start date for the recurrence rule. Please specify a `DTSTART` field."
    )]
    MissingStartDate,
    #[error("`{0}` is not a valid frequency.")]
    InvalidFrequency(String),
    #[error("`{0}` is not a valid weekday.")]
    InvalidWeekday(String),
    #[error("The field `{0}` was found twice.")]
    DuplicatedField(String),
}
