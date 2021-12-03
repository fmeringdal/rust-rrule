use super::{datetime::DateTime, properties::*};
use crate::{RRuleError, WithError};
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

    /// Returns all the recurrences of the rrule.
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    pub fn all(&self, limit: u16) -> Vec<DateTime> {
        self.into_iter().take(limit as usize).collect()
    }

    /// Returns all the recurrences of the rrule.
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    ///
    /// In case where the iterator ended with an errors the error will be included,
    /// otherwise the second value of the return tuple will be `None`.
    pub fn all_with_error(&self, limit: u16) -> (Vec<DateTime>, Option<RRuleError>) {
        let mut iterator = self.into_iter();
        let mut list = vec![];
        let mut err = None;
        for _i in 0..limit {
            let next = iterator.next();
            match next {
                Some(value) => list.push(value),
                None => {
                    err = iterator.get_err();
                    break;
                }
            }
        }
        (list, err.cloned())
    }

    /// Returns the last recurrence before the given datetime instance.
    /// The inc keyword defines what happens if dt is a recurrence.
    /// With inc == true, if dt itself is a recurrence, it will be returned.
    pub fn before(&self, dt: DateTime, inc: bool) -> Option<DateTime> {
        self.into_iter()
            .take_while(|d| if inc { *d <= dt } else { *d < dt })
            .last()
    }

    /// Returns the last recurrence after the given datetime instance.
    /// The inc keyword defines what happens if dt is a recurrence.
    /// With inc == true, if dt itself is a recurrence, it will be returned.
    pub fn after(&self, dt: DateTime, inc: bool) -> Option<DateTime> {
        self.into_iter()
            .find(|d| !(if inc { *d < dt } else { *d <= dt }))
    }

    /// Returns all the recurrences of the rrule between after and before.
    /// The inc keyword defines what happens if after and/or before are
    /// themselves recurrences. With inc == true, they will be included in the
    /// list, if they are found in the recurrence set.
    pub fn between(&self, after: DateTime, before: DateTime, inc: bool) -> Vec<DateTime> {
        self.into_iter()
            .skip_while(|d| if inc { *d < after } else { *d <= after })
            .take_while(|d| if inc { *d <= before } else { *d < before })
            .collect()
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
