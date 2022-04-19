use super::{datetime::DateTime, properties::*};
use crate::{DateFilter, RRuleError, RRuleIter};
use chrono_tz::Tz;
use std::str::FromStr;

/// A validated Recurrence Rule that can be used to create an iterator.
#[derive(Clone, Debug)]
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

impl<'a> DateFilter<'a, RRuleIter<'a>> for RRule {}
