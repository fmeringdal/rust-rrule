//! This module includes everything needed to validate an [`crate::core::RRule<Unvalidated>`].
//! And in turn create a [`crate::core::RRule<Validated>`].

mod error;
pub(crate) mod validate_rrule;
pub use error::ValidationError;

pub(crate) use validate_rrule::YEAR_RANGE;
