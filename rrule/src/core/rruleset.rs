use crate::core::datetime::datetime_to_ical_format;
use crate::core::utils::collect_with_error;
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
    /// If validation limits are enabled
    pub(crate) limited: bool,
}

/// The return result of `RRuleSet::all`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RRuleResult {
    /// List of recurrences.
    pub dates: Vec<DateTime>,
    /// It is being true if the list of dates is limited.
    /// To indicate that it can potentially contain more dates.
    pub limited: bool,
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
            before: None,
            after: None,
            limited: false,
        }
    }

    /// Enable validation limits.
    ///
    /// This is only needed if you are going to use the Iterator api directly.
    #[must_use]
    pub fn limit(mut self) -> Self {
        self.limited = true;
        self
    }

    /// Only return recurrences that comes before this `DateTime`.
    ///
    /// This value will not be used if you use the `Iterator` API directly.
    #[must_use]
    pub fn before(mut self, dt: DateTime) -> Self {
        self.before = Some(dt);
        self
    }

    /// Only return recurrences that comes after this `DateTime`.
    ///
    /// This value will not be used if you use the `Iterator` API directly.
    #[must_use]
    pub fn after(mut self, dt: DateTime) -> Self {
        self.after = Some(dt);
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
    ///
    /// # Usage
    ///
    /// ```
    /// use rrule::RRuleSet;
    ///
    /// let rrule_set: RRuleSet = "DTSTART:20210101T090000Z\nRRULE:FREQ=DAILY".parse().unwrap();
    ///
    /// // Limit the results to 2 recurrences
    /// let result = rrule_set.all(2);
    /// assert_eq!(result.dates.len(), 2);
    /// assert_eq!(result.limited, true);
    /// ```
    #[must_use]
    pub fn all(mut self, limit: u16) -> RRuleResult {
        self.limited = true;
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
    /// # Note
    ///
    /// This method does not enforce any validation limits and might lead to
    /// very long iteration times. Please read the `SECURITY.md` for more information.
    #[must_use]
    pub fn all_unchecked(self) -> Vec<DateTime> {
        collect_with_error(self.into_iter(), &self.after, &self.before, true, None).dates
    }
}

impl FromStr for RRuleSet {
    type Err = RRuleError;

    /// Creates an [`RRuleSet`] from a string if input is valid.
    ///
    /// # Errors
    ///
    /// Returns [`RRuleError`], if iCalendar string contains invalid parts.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Grammar {
            start,
            content_lines,
        } = Grammar::from_str(s)?;

        content_lines.into_iter().try_fold(
            Self::new(start.datetime),
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
                    Ok(exdates.into_iter().fold(rrule_set, Self::exdate))
                }
                ContentLine::RDate(rdates) => {
                    Ok(rdates.into_iter().fold(rrule_set, Self::rdate))
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
