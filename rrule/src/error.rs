use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum RRuleError {
    ParseError(String),
    ValidationError(String),
    IterError(String),
}

impl RRuleError {
    /// Create a new parsing error with the given message.
    pub fn new_parse_err<S: AsRef<str>>(msg: S) -> Self {
        Self::ParseError(msg.as_ref().to_owned())
    }
    /// Create a new validator error with the given message.
    pub fn new_validation_err<S: AsRef<str>>(msg: S) -> Self {
        Self::ValidationError(msg.as_ref().to_owned())
    }
    /// Create a new iterator error with the given message.
    pub fn new_iter_err<S: AsRef<str>>(msg: S) -> Self {
        Self::IterError(msg.as_ref().to_owned())
    }
}

impl Display for RRuleError {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "RRule parsing error: {}", msg),
            Self::ValidationError(msg) => write!(f, "RRule validation error: {}", msg),
            Self::IterError(msg) => write!(f, "RRule iterator error: {}", msg),
        }
    }
}

impl Error for RRuleError {}
