use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone)]
pub struct RRuleIterError(pub String);

impl Error for RRuleIterError {}

impl Display for RRuleIterError {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "Encountered iteration error: {}", self.0)
    }
}
