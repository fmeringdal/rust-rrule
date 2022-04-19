use thiserror::Error;

use crate::{parser::ParseError, validator::ValidationError};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum RRuleError {
    #[error("RRule parsing error: {0}")]
    ParserError(#[from] ParseError),
    #[error("RRule validation error: {0}")]
    ValidationError(#[from] ValidationError),
    #[error("RRule iterator error: {0}")]
    IterError(String),
}

impl RRuleError {
    /// Create a new iterator error with the given message.
    pub fn new_iter_err<S: AsRef<str>>(msg: S) -> Self {
        Self::IterError(msg.as_ref().to_owned())
    }
}

pub trait WithError {
    /// Return `true` if an error has occurred.
    fn has_err(&self) -> bool;
    /// Return the last error while iterating.
    fn get_err(&self) -> Option<&RRuleError>;
}
