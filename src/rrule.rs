use crate::options::*;
use crate::rrulestr::build_rrule;
use chrono::prelude::*;
use chrono_tz::Tz;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct RRule {
    pub options: ParsedOptions,
}

impl RRule {
    pub fn new(options: ParsedOptions) -> Self {
        Self { options }
    }

    /// Returns all the recurrences of the rrule
    pub fn all(&mut self) -> Vec<DateTime<Tz>> {
        self.clone().into_iter().collect()
    }

    /// Returns the last recurrence before the given datetime instance.
    /// The inc keyword defines what happens if dt is an recurrence.
    /// With inc == true, if dt itself is an recurrence, it will be returned.
    pub fn before(&mut self, dt: DateTime<Tz>, inc: bool) -> Option<DateTime<Tz>> {
        self.clone()
            .into_iter()
            .take_while(|d| if inc { *d <= dt } else { *d < dt })
            .last()
    }

    /// Returns the last recurrence after the given datetime instance.
    /// The inc keyword defines what happens if dt is an recurrence.
    /// With inc == true, if dt itself is an recurrence, it will be returned.
    pub fn after(&mut self, dt: DateTime<Tz>, inc: bool) -> Option<DateTime<Tz>> {
        self.clone()
            .into_iter()
            .skip_while(|d| if inc { *d <= dt } else { *d < dt })
            .next()
    }

    /// Returns all the recurrences of the rrule between after and before.
    /// The inc keyword defines what happens if after and/or before are
    /// themselves recurrences. With inc == true, they will be included in the
    /// list, if they are found in the recurrence set.
    pub fn between(
        &mut self,
        after: DateTime<Tz>,
        before: DateTime<Tz>,
        inc: bool,
    ) -> Vec<DateTime<Tz>> {
        self.clone()
            .into_iter()
            .skip_while(|d| if inc { *d <= after } else { *d < after })
            .take_while(|d| if inc { *d <= before } else { *d < before })
            .collect()
    }
}

impl FromStr for RRule {
    type Err = RRuleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        build_rrule(s)
    }
}
