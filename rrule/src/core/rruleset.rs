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
    /// If set, all returned recurrences must be before this date.
    pub(crate) before: Option<DateTime>,
    /// If set, all returned recurrences must be after this date.
    pub(crate) after: Option<DateTime>,
}

impl RRuleSet {
    /// Creates an empty [`RRuleSet`], starting from `ds_start`.
    #[must_use]
    pub fn new<T>(dt_start: T) -> Self
    where
        DateTime: From<T>,
    {
        Self {
            dt_start: dt_start.into(),
            rrule: vec![],
            rdate: vec![],
            exrule: vec![],
            exdate: vec![],
            before: None,
            after: None,
        }
    }

    /// Only return recurrences that comes before this `DateTime`.
    ///
    /// This value will not be used if you use the `Iterator` API directly.
    #[must_use]
    pub fn before<T>(mut self, dt: T) -> Self
    where
        DateTime: From<T>,
    {
        self.before = Some(dt.into());
        self
    }

    /// Only return recurrences that comes after this `DateTime`.
    ///
    /// This value will not be used if you use the `Iterator` API directly.
    #[must_use]
    pub fn after<T>(mut self, dt: T) -> Self
    where
        DateTime: From<T>,
    {
        self.after = Some(dt.into());
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
    pub fn rdate<T>(mut self, rdate: T) -> Self
    where
        DateTime: From<T>,
    {
        self.rdate.push(rdate.into());
        self
    }

    /// Adds a new exdate to the set.
    #[must_use]
    pub fn exdate<T>(mut self, exdate: T) -> Self
    where
        DateTime: From<T>,
    {
        self.exdate.push(exdate.into());
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
    pub fn set_rdates<T>(mut self, rdates: Vec<T>) -> Self
    where
        DateTime: From<T>,
    {
        self.rdate = rdates.into_iter().map(From::from).collect();
        self
    }

    /// Set the exdates of the set.
    #[must_use]
    pub fn set_exdates<T>(mut self, exdates: Vec<T>) -> Self
    where
        DateTime: From<T>,
    {
        self.exdate = exdates.into_iter().map(From::from).collect();
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
    ///
    /// # Usage
    ///
    /// ```
    /// use rrule::RRuleSet;
    ///
    /// let rrule_set: RRuleSet = "DTSTART:20210101T090000Z\nRRULE:FREQ=DAILY;".parse().unwrap();
    ///
    /// // Limit the results to 2 recurrences
    /// let result = rrule_set.all(2).unwrap();
    /// assert_eq!(result.len(), 2);
    /// ```
    ///
    /// # Error
    ///
    /// An error will be returned if the iterator encountered an error which
    /// does not allow the iteration to continue. One example would be hitting the validation-limit
    /// or the `INTERVAL` value was too large. Please use [`RRuleSet::all_with_error`] if you
    /// want the datetimes generated before encountering the error to be returned as well.
    pub fn all(self, limit: u16) -> Result<Vec<DateTime>, RRuleError> {
        collect_or_error(
            self.into_iter(),
            &self.after,
            &self.before,
            true,
            Some(limit),
        )
    }

    /// Returns all the recurrences of the rrule.
    ///
    /// # Note
    ///
    /// This method does not enforce any validation limits and might lead to
    /// very long iteration times. Please read the `SECURITY.md` for more information.
    ///
    /// # Error
    ///
    /// An error will be returned if the iterator encountered an error which
    /// does not allow the iteration to continue. One example would be hitting a datetime
    /// during the iteration which is outside the allowed range of `chrono::DateTime`.
    pub fn all_unchecked(self) -> Result<Vec<DateTime>, RRuleError> {
        collect_or_error(self.into_iter(), &self.after, &self.before, true, None)
    }

    /// Returns all the recurrences of the rrule.
    ///
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    ///
    /// In case the iterator ended with an error, the error will be included,
    /// otherwise the second value of the return tuple will be `None`.
    #[must_use]
    pub fn all_with_error(self, limit: u16) -> (Vec<DateTime>, Option<RRuleError>) {
        collect_with_error(
            self.into_iter(),
            &self.after,
            &self.before,
            true,
            Some(limit),
        )
    }

    /// Returns all the recurrences of the rrule.
    ///
    /// In case the iterator ended with an error, the error will be included,
    /// otherwise the second value of the return tuple will be `None`.
    ///
    /// # Note
    ///
    /// This method does not enforce any validation limits and might lead to
    /// very long iteration times. Please read the `SECURITY.md` for more information.
    #[must_use]
    pub fn all_with_error_unchecked(self) -> (Vec<DateTime>, Option<RRuleError>) {
        collect_with_error(self.into_iter(), &self.after, &self.before, true, None)
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
            |rrule_set, content_line| match content_line {
                ContentLine::RRule(rrule) => rrule
                    .validate(start.datetime)
                    .map(|rrule| rrule_set.rrule(rrule)),
                #[allow(unused_variables)]
                ContentLine::ExRule(exrule) => {
                    #[cfg(feature = "exrule")]
                    {
                        exrule
                            .validate(start.datetime)
                            .map(|exrule| rrule_set.rrule(exrule))
                    }
                    #[cfg(not(feature = "exrule"))]
                    {
                        log::warn!("Found EXRULE in input, but it will be ignored since the `exrule` feature is not enabled.");
                        Ok(rrule_set)
                    }
                }
                ContentLine::ExDate(exdates) => {
                    Ok(exdates.into_iter().fold(rrule_set, RRuleSet::exdate))
                }
                ContentLine::RDate(rdates) => {
                    Ok(rdates.into_iter().fold(rrule_set, RRuleSet::rdate))
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
