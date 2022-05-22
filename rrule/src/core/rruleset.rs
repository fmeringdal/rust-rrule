use crate::core::datetime::datetime_to_ical_format;
use crate::core::utils::{check_str_validity, collect_or_error, collect_with_error};
use crate::core::DateTime;
use crate::parser::build_rruleset;
use crate::{RRule, RRuleError};
use chrono::TimeZone;
use chrono_tz::UTC;
#[cfg(feature = "serde")]
use serde_with::{serde_as, DeserializeFromStr, SerializeDisplay};
use std::fmt::Display;
use std::str::FromStr;

/// A validated Recurrence Rule that can be used to create an iterator.
#[cfg_attr(feature = "serde", serde_as)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(DeserializeFromStr, SerializeDisplay))]
pub struct RRuleSet {
    /// List of rrules.
    pub rrule: Vec<RRule>,
    /// List of rdates.
    pub rdate: Vec<DateTime>,
    /// List of exules.
    pub exrule: Vec<RRule>,
    /// List of exdates.
    pub exdate: Vec<DateTime>,
    /// The start datetime of the recurring event.
    pub dt_start: DateTime,
}

impl Default for RRuleSet {
    /// Creates an empty [`RRuleSet`], starting from 1970-01-01 00:00:00 UTC.
    fn default() -> Self {
        Self {
            rrule: vec![],
            rdate: vec![],
            exrule: vec![],
            exdate: vec![],
            dt_start: UTC.ymd(1970, 1, 1).and_hms(0, 0, 0), // Unix Epoch
        }
    }
}

impl RRuleSet {
    /// Creates an empty [`RRuleSet`], starting from `ds_start`.
    #[must_use]
    pub fn new(dt_start: DateTime) -> Self {
        Self {
            dt_start,
            ..Default::default()
        }
    }

    /// Adds a new rrule to the set.
    pub fn rrule(&mut self, rrule: RRule) {
        self.rrule.push(rrule);
    }

    /// Adds a new exrule to the set.
    #[deprecated(
        note = "Based on [iCalendar RFC](https://datatracker.ietf.org/doc/html/rfc5545#appendix-A.3), exrules are deprecated."
    )]
    pub fn exrule(&mut self, rrule: RRule) {
        self.exrule.push(rrule);
    }

    /// Adds a new rdate to the set.
    pub fn rdate(&mut self, rdate: DateTime) {
        self.rdate.push(rdate);
    }

    /// Adds a new exdate to the set.
    pub fn exdate(&mut self, exdate: DateTime) {
        self.exdate.push(exdate);
    }

    /// Returns all the recurrences of the rrule.
    ///
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    pub fn all(self, limit: u16) -> Result<Vec<DateTime>, RRuleError> {
        collect_or_error(self.into_iter(), &None, &None, true, limit)
    }

    /// Returns all the recurrences of the rrule.
    ///
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    ///
    /// In case the iterator ended with an error, the error will be included,
    /// otherwise the second value of the return tuple will be `None`.
    pub fn all_with_error(self, limit: u16) -> (Vec<DateTime>, Option<RRuleError>) {
        collect_with_error(self.into_iter(), &None, &None, true, limit)
    }

    /// Returns the last recurrence before the given datetime instance.
    ///
    /// The `inclusive` keyword defines what happens if `before` is a recurrence.
    /// With `inclusive == true`, if `before` itself is a recurrence, it will be returned.
    pub fn just_before(
        self,
        before: DateTime,
        inclusive: bool,
    ) -> Result<Option<DateTime>, RRuleError> {
        Ok(
            collect_or_error(self.into_iter(), &None, &Some(before), inclusive, u16::MAX)?
                .last()
                .cloned(),
        )
    }

    /// Returns all the recurrences of the rrule before the given date.
    ///
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    ///
    /// In case the iterator ended with an error, the error will be included,
    /// otherwise the second value of the return tuple will be `None`.
    pub fn all_before_with_error(
        self,
        before: DateTime,
        inclusive: bool,
        limit: u16,
    ) -> (Vec<DateTime>, Option<RRuleError>) {
        collect_with_error(self.into_iter(), &None, &Some(before), inclusive, limit)
    }

    /// Returns the last recurrence after the given datetime instance.
    ///
    /// The `inclusive` keyword defines what happens if `after` is a recurrence.
    /// With `inclusive == true`, if `after` itself is a recurrence, it will be returned.
    pub fn just_after(
        self,
        after: DateTime,
        inclusive: bool,
    ) -> Result<Option<DateTime>, RRuleError> {
        Ok(
            collect_or_error(self.into_iter(), &Some(after), &None, inclusive, 1)?
                .first()
                .cloned(),
        )
    }

    /// Returns all the recurrences of the rrule after the given date.
    ///
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    ///
    /// In case the iterator ended with an error, the error will be included,
    /// otherwise the second value of the return tuple will be `None`.
    pub fn all_after_with_error(
        self,
        after: DateTime,
        inclusive: bool,
        limit: u16,
    ) -> (Vec<DateTime>, Option<RRuleError>) {
        collect_with_error(self.into_iter(), &Some(after), &None, inclusive, limit)
    }

    /// Returns all the recurrences of the rrule between after and before.
    ///
    /// The `inclusive` keyword defines what happens if after and/or before are
    /// themselves recurrences. With `inclusive == true`, they will be included in the
    /// list, if they are found in the recurrence set.
    pub fn all_between(
        self,
        start: DateTime,
        end: DateTime,
        inclusive: bool,
    ) -> Result<Vec<DateTime>, RRuleError> {
        collect_or_error(
            self.into_iter(),
            &Some(start),
            &Some(end),
            inclusive,
            u16::MAX,
        )
    }

    /// Returns all the recurrences of the rrule after the given date and before the other date.
    ///
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    ///
    /// In case the iterator ended with an error, the error will be included,
    /// otherwise the second value of the return tuple will be `None`.
    pub fn all_between_with_error(
        self,
        start: DateTime,
        end: DateTime,
        inclusive: bool,
        limit: u16,
    ) -> (Vec<DateTime>, Option<RRuleError>) {
        collect_with_error(self.into_iter(), &Some(start), &Some(end), inclusive, limit)
    }
}

impl FromStr for RRuleSet {
    type Err = RRuleError;

    /// Creates an [`RRuleSet`] from a string if input is valid.
    ///
    /// # Errors
    ///
    /// Returns [`RRuleError`], if iCalendar string contains invalid parts
    /// This should never panic, but it might be in odd cases.
    /// Please report if it does panic.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        check_str_validity(s)?;

        build_rruleset(s)
    }
}

impl Display for RRuleSet {
    /// Prints a valid set of iCalendar properties which can be used to create a new [`RRuleSet`] later.
    /// You may use the generated string to create a new iCalendar component, like VEVENT.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let properties = self
            .rrule
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        let datetime = datetime_to_ical_format(&self.dt_start);

        write!(f, "DTSTART{}\n{}", datetime, properties)
    }
}
