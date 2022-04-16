use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParseError {
    #[error("")]
    MissingStartDate,
    #[error("`{0}` is not a valid frequency.")]
    InvalidFrequency(String),
    #[error("The field `{0}` was found twice.")]
    DuplicatedField(String),
}
