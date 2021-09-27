use crate::datetime::DateTime;
use crate::iter::RRuleIterError;
use crate::options::*;
use crate::rrulestr::build_rrule;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct RRule {
    pub options: ParsedOptions,
}

impl RRule {
    pub fn new(options: ParsedOptions) -> Self {
        Self { options }
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
    pub fn all_with_error(&self, limit: u16) -> (Vec<DateTime>, Option<RRuleIterError>) {
        let mut iterator = self.into_iter();
        let mut list = vec![];
        let mut err = None;
        for _i in 0..limit {
            let next = iterator.next();
            match next {
                Some(value) => list.push(value),
                None => {
                    if iterator.has_err() {
                        err = iterator.get_err().clone();
                    }
                    break;
                }
            }
        }
        (list, err)
    }

    /// Returns the last recurrence before the given datetime instance.
    /// The inc keyword defines what happens if dt is an recurrence.
    /// With inc == true, if dt itself is an recurrence, it will be returned.
    pub fn before(&self, dt: DateTime, inc: bool) -> Option<DateTime> {
        self.into_iter()
            .take_while(|d| if inc { *d <= dt } else { *d < dt })
            .last()
    }

    /// Returns the last recurrence after the given datetime instance.
    /// The inc keyword defines what happens if dt is an recurrence.
    /// With inc == true, if dt itself is an recurrence, it will be returned.
    pub fn after(&self, dt: DateTime, inc: bool) -> Option<DateTime> {
        self.into_iter()
            .skip_while(|d| if inc { *d < dt } else { *d <= dt })
            .next()
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
    type Err = RRuleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        build_rrule(s)
    }
}
