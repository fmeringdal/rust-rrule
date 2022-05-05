use super::{datetime::DateTime, properties::*};
use crate::core::datetime::datetime_to_ical_format;
use crate::{DateFilter, RRuleError, RRuleIter};
use chrono_tz::Tz;
#[cfg(feature = "serde")]
use serde_with::{serde_as, DeserializeFromStr, SerializeDisplay};
use std::fmt::Display;
use std::str::FromStr;

/// A validated Recurrence Rule that can be used to create an iterator.
#[cfg_attr(feature = "serde", serde_as)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(DeserializeFromStr, SerializeDisplay))]
pub struct RRule {
    /// The properties specified by this rule.
    pub(crate) properties: RRuleProperties,
    /// The timezone used during the creation of the events.
    pub(crate) tz: Tz,
    /// The start datetime of the recurring event.
    // TODO: add info about how timezone is used.
    pub(crate) dt_start: DateTime,
}

impl RRule {
    /// Get the parameters set by the RRule.
    pub fn get_properties(&self) -> &RRuleProperties {
        &self.properties
    }
}

impl FromStr for RRule {
    type Err = RRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rrule = crate::parser::parse_rrule_string_to_properties(s)?;
        let dt_start = crate::parser::parse_dtstart(s)?;
        rrule.build(dt_start)
    }
}

impl Display for RRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let properties = self.properties.to_string();
        let datetime = datetime_to_ical_format(&self.dt_start, self.tz);

        write!(f, "DTSTART{}\n{}", datetime, properties)
    }
}

impl<'a> DateFilter<'a, RRuleIter<'a>> for RRule {}
