use crate::core::datetime::datetime_to_ical_format;
use crate::core::utils::{collect_or_error, collect_with_error};
use crate::core::DateTime;
use crate::parser::{ContentLine, Grammar};
use crate::{RRule, RRuleError};
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
    pub(crate) rrule: Vec<RRule>,
    /// List of rdates.
    pub(crate) rdate: Vec<DateTime>,
    /// List of exules.
    pub(crate) exrule: Vec<RRule>,
    /// List of exdates.
    pub(crate) exdate: Vec<DateTime>,
    /// The start datetime of the recurring event.
    pub(crate) dt_start: DateTime,
    /// TODO: document
    pub(crate) limit: Option<u16>,
}

impl RRuleSet {
    /// Creates an empty [`RRuleSet`], starting from `ds_start`.
    #[must_use]
    pub fn new(dt_start: DateTime) -> Self {
        Self {
            dt_start,
            rrule: vec![],
            rdate: vec![],
            exrule: vec![],
            exdate: vec![],
            limit: Some(u16::MAX),
        }
    }

    /// Disables validation limits
    #[must_use]
    pub fn disable_validation_limits(mut self) -> Self {
        self.limit = None;
        self
    }

    /// Set the validation limit
    #[must_use]
    pub fn set_limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Adds a new rrule to the set.
    #[must_use]
    pub fn rrule(mut self, rrule: RRule) -> Self {
        self.rrule.push(rrule);
        self
    }

    /// Adds a new exrule to the set.
    #[must_use]
    #[cfg(feature = "exrule")]
    pub fn exrule(mut self, rrule: RRule) -> Self {
        self.exrule.push(rrule);
        self
    }

    /// Adds a new rdate to the set.
    #[must_use]
    pub fn rdate(mut self, rdate: DateTime) -> Self {
        self.rdate.push(rdate);
        self
    }

    /// Adds a new exdate to the set.
    #[must_use]
    pub fn exdate(mut self, exdate: DateTime) -> Self {
        self.exdate.push(exdate);
        self
    }

    /// Sets the rrules of the set.
    #[must_use]
    pub fn set_rrules(mut self, rrules: Vec<RRule>) -> Self {
        self.rrule = rrules;
        self
    }

    /// Sets the exrules of the set.
    #[must_use]
    #[cfg(feature = "exrule")]
    pub fn set_exrules(mut self, exrules: Vec<RRule>) -> Self {
        self.exrule = exrules;
        self
    }

    /// Sets the rdates of the set.
    #[must_use]
    pub fn set_rdates(mut self, rdates: Vec<DateTime>) -> Self {
        self.rdate = rdates;
        self
    }

    /// Set the exdates of the set.
    #[must_use]
    pub fn set_exdates(mut self, exdates: Vec<DateTime>) -> Self {
        self.exdate = exdates;
        self
    }

    /// Returns the rrules of the set.
    #[must_use]
    pub fn get_rrule(&self) -> &Vec<RRule> {
        &self.rrule
    }

    /// Returns the exrules of the set.
    #[must_use]
    pub fn get_exrule(&self) -> &Vec<RRule> {
        &self.exrule
    }

    /// Returns the rdates of the set.
    #[must_use]
    pub fn get_rdate(&self) -> &Vec<DateTime> {
        &self.rdate
    }

    /// Returns the exdates of the set.
    #[must_use]
    pub fn get_exdate(&self) -> &Vec<DateTime> {
        &self.exdate
    }

    /// Returns the start datetime of the recurring event.
    #[must_use]
    pub fn get_dt_start(&self) -> &DateTime {
        &self.dt_start
    }

    /// Returns all the recurrences of the rrule.
    ///
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    pub fn all(self) -> Result<Vec<DateTime>, RRuleError> {
        collect_or_error(self.into_iter(), &None, &None, true, self.limit)
    }

    /// Returns all the recurrences of the rrule.
    ///
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    ///
    /// In case the iterator ended with an error, the error will be included,
    /// otherwise the second value of the return tuple will be `None`.
    #[must_use]
    pub fn all_with_error(self) -> (Vec<DateTime>, Option<RRuleError>) {
        collect_with_error(self.into_iter(), &None, &None, true, self.limit)
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
        Ok(collect_or_error(
            self.into_iter(),
            &None,
            &Some(before),
            inclusive,
            self.limit,
        )?
        .last()
        .copied())
    }

    /// Returns all the recurrences of the rrule before the given date.
    ///
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    ///
    /// In case the iterator ended with an error, the error will be included,
    /// otherwise the second value of the return tuple will be `None`.
    #[must_use]
    pub fn all_before_with_error(
        self,
        before: DateTime,
        inclusive: bool,
    ) -> (Vec<DateTime>, Option<RRuleError>) {
        collect_with_error(
            self.into_iter(),
            &None,
            &Some(before),
            inclusive,
            self.limit,
        )
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
            collect_or_error(self.into_iter(), &Some(after), &None, inclusive, Some(1))?
                .first()
                .copied(),
        )
    }

    /// Returns all the recurrences of the rrule after the given date.
    ///
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    ///
    /// In case the iterator ended with an error, the error will be included,
    /// otherwise the second value of the return tuple will be `None`.
    #[must_use]
    pub fn all_after_with_error(
        self,
        after: DateTime,
        inclusive: bool,
    ) -> (Vec<DateTime>, Option<RRuleError>) {
        collect_with_error(self.into_iter(), &Some(after), &None, inclusive, self.limit)
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
            self.limit,
        )
    }

    /// Returns all the recurrences of the rrule after the given date and before the other date.
    ///
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    ///
    /// In case the iterator ended with an error, the error will be included,
    /// otherwise the second value of the return tuple will be `None`.
    #[must_use]
    pub fn all_between_with_error(
        self,
        start: DateTime,
        end: DateTime,
        inclusive: bool,
    ) -> (Vec<DateTime>, Option<RRuleError>) {
        collect_with_error(
            self.into_iter(),
            &Some(start),
            &Some(end),
            inclusive,
            self.limit,
        )
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
        let Grammar {
            start,
            content_lines,
        } = Grammar::from_str(s)?;

        content_lines.into_iter().try_fold(
            RRuleSet::new(start.datetime),
            |mut rrule_set, content_line| match content_line {
                ContentLine::RRule(rrule) => {
                    let rrule = rrule.validate(start.datetime)?;
                    Ok::<RRuleSet, RRuleError>(rrule_set.rrule(rrule))
                }
                #[allow(unused_variables)]
                ContentLine::ExRule(rrule) => {
                    #[cfg(feature = "exrule")]
                    {
                        let rrule = rrule.validate(start.datetime)?;
                        Ok(rrule_set.exrule(rrule))
                    }
                    #[cfg(not(feature = "exrule"))]
                    {
                        log::warn!("Found EXRULE in input, but it will be ignored since the `exrule` feature is not enabled.");
                        Ok(rrule_set)
                    }
                }
                ContentLine::ExDate(exdates) => {
                    for exdate in exdates {
                        rrule_set = rrule_set.exdate(exdate);
                    }
                    Ok(rrule_set)
                }
                ContentLine::RDate(rdates) => {
                    for rdate in rdates {
                        rrule_set = rrule_set.rdate(rdate);
                    }
                    Ok(rrule_set)
                }
            },
        )
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
