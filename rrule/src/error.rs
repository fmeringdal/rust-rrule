#![allow(clippy::module_name_repetitions)]

use thiserror::Error;

use crate::{parser::ParseError, validator::ValidationError};

#[derive(Error, Debug, Clone, PartialEq)]
/// The error type for the rrule crate.
pub enum RRuleError {
    /// Parsing error
    #[error("RRule parsing error: {0}")]
    ParserError(#[from] ParseError),
    /// Validation error
    #[error("RRule validation error: {0}")]
    ValidationError(#[from] ValidationError),
    /// Iterator error
    #[error("RRule iterator error: {0}")]
    IterError(String),
}

impl RRuleError {
    /// Create a new iterator error with the given message.
    pub fn new_iter_err<S: AsRef<str>>(msg: S) -> Self {
        Self::IterError(msg.as_ref().to_owned())
    }
}
