use crate::core::datetime::datetime_to_ical_format;
use crate::core::utils::collect_with_error;
use crate::parser::{ContentLine, Grammar};
use crate::{ParseError, RRule, RRuleError, Tz};
use chrono::DateTime;
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
    pub(crate) rdate: Vec<DateTime<Tz>>,
    /// List of exules.
    pub(crate) exrule: Vec<RRule>,
    /// List of exdates.
    pub(crate) exdate: Vec<DateTime<Tz>>,
    /// The start datetime of the recurring event.
    pub(crate) dt_start: DateTime<Tz>,
    /// If set, all returned recurrences must be before this date.
    pub(crate) before: Option<DateTime<Tz>>,
    /// If set, all returned recurrences must be after this date.
    pub(crate) after: Option<DateTime<Tz>>,
    /// If validation limits are enabled
    pub(crate) limited: bool,
}

/// The return result of `RRuleSet::all`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RRuleResult {
    /// List of recurrences.
    pub dates: Vec<DateTime<Tz>>,
    /// It is being true if the list of dates is limited.
    /// To indicate that it can potentially contain more dates.
    pub limited: bool,
}

impl RRuleSet {
    /// Creates an empty [`RRuleSet`], starting from `ds_start`.
    #[must_use]
    pub fn new(dt_start: DateTime<Tz>) -> Self {
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
    pub fn before(mut self, dt: DateTime<Tz>) -> Self {
        self.before = Some(dt);
        self
    }

    /// Only return recurrences that comes after this `DateTime`.
    ///
    /// This value will not be used if you use the `Iterator` API directly.
    #[must_use]
    pub fn after(mut self, dt: DateTime<Tz>) -> Self {
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
    pub fn rdate(mut self, rdate: DateTime<Tz>) -> Self {
        self.rdate.push(rdate);
        self
    }

    /// Adds a new exdate to the set.
    #[must_use]
    pub fn exdate(mut self, exdate: DateTime<Tz>) -> Self {
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
    pub fn set_rdates(mut self, rdates: Vec<DateTime<Tz>>) -> Self {
        self.rdate = rdates;
        self
    }

    /// Set the exdates of the set.
    #[must_use]
    pub fn set_exdates(mut self, exdates: Vec<DateTime<Tz>>) -> Self {
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
    pub fn get_rdate(&self) -> &Vec<DateTime<Tz>> {
        &self.rdate
    }

    /// Returns the exdates of the set.
    #[must_use]
    pub fn get_exdate(&self) -> &Vec<DateTime<Tz>> {
        &self.exdate
    }

    /// Returns the start datetime of the recurring event.
    #[must_use]
    pub fn get_dt_start(&self) -> &DateTime<Tz> {
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
    pub fn all_unchecked(self) -> Vec<DateTime<Tz>> {
        collect_with_error(self.into_iter(), &self.after, &self.before, true, None).dates
    }

    fn set_from_content_lines(self, content_lines: Vec<ContentLine>) -> Result<Self, RRuleError> {
        let dt_start = self.dt_start;

        content_lines.into_iter().try_fold(
            self,
            |rrule_set, content_line| match content_line {
                ContentLine::RRule(rrule) => rrule
                    .validate(dt_start)
                    .map(|rrule| rrule_set.rrule(rrule)),
                #[allow(unused_variables)]
                ContentLine::ExRule(exrule) => {
                    #[cfg(feature = "exrule")]
                    {
                        exrule
                            .validate(dt_start)
                            .map(|exrule| rrule_set.exrule(exrule))
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

    /// Set the [`RRuleSet`] properties from a string. If a DTSTART is found, it will be used as the start datetime.
    pub fn set_from_string(mut self, s: &str) -> Result<Self, RRuleError> {
        let Grammar {
            start,
            content_lines,
        } = Grammar::from_str(s)?;

        if let Some(dtstart) = start {
            self.dt_start = dtstart.datetime;
        }

        self.set_from_content_lines(content_lines)
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

        let start = start.ok_or(ParseError::MissingStartDate)?;

        Self::new(start.datetime).set_from_content_lines(content_lines)
    }
}

impl Display for RRuleSet {
    /// Prints a valid set of iCalendar properties which can be used to create a new [`RRuleSet`] later.
    /// You may use the generated string to create a new iCalendar component, like VEVENT.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start_datetime = format!("DTSTART{}", datetime_to_ical_format(&self.dt_start));

        let mut rrules = self
            .rrule
            .iter()
            .map(|rrule| format!("RRULE:{rrule}"))
            .collect::<Vec<_>>()
            .join("\n");
        if !rrules.is_empty() {
            rrules = format!("\n{rrules}");
        }

        let mut rdates = self
            .rdate
            .iter()
            .map(|dt| {
                let maybe_zulu = if dt.timezone().is_local() { "" } else { "Z" };

                format!("{}{}", dt.format("%Y%m%dT%H%M%S"), maybe_zulu)
            })
            .collect::<Vec<_>>()
            .join(",");
        if !rdates.is_empty() {
            // TODO: check if original VALUE prop was DATE or PERIOD
            rdates = format!("\nRDATE;VALUE=DATE-TIME:{rdates}");
        }

        let mut exrules = self
            .exrule
            .iter()
            .map(|exrule| format!("EXRULE:{exrule}"))
            .collect::<Vec<_>>()
            .join("\n");
        if !exrules.is_empty() {
            exrules = format!("\n{exrules}");
        }

        let mut exdates = self
            .exdate
            .iter()
            .map(|dt| {
                let maybe_zulu = if dt.timezone().is_local() { "" } else { "Z" };

                format!("{}{}", dt.format("%Y%m%dT%H%M%S"), maybe_zulu)
            })
            .collect::<Vec<_>>()
            .join(",");
        if !exdates.is_empty() {
            // TODO: check if original VALUE prop was DATE or PERIOD
            exdates = format!("\nEXDATE;VALUE=DATE-TIME:{exdates}");
        }

        write!(f, "{start_datetime}{rrules}{rdates}{exrules}{exdates}")
    }
}

#[cfg(feature = "exrule")]
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::{Month, TimeZone};

    use crate::{Frequency, RRule, RRuleSet, Tz};

    #[test]
    fn rruleset_string_roundtrip() {
        let rruleset_str = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3;BYHOUR=9;BYMINUTE=30;BYSECOND=0\nRDATE;VALUE=DATE-TIME:19970101T000000Z,19970120T000000Z\nEXRULE:FREQ=YEARLY;COUNT=8;BYMONTH=6,7;BYMONTHDAY=1;BYHOUR=9;BYMINUTE=30;BYSECOND=0\nEXDATE;VALUE=DATE-TIME:19970121T000000Z";
        let rruleset = RRuleSet::from_str(rruleset_str).unwrap();

        // Check start date
        let dt_start = Tz::UTC.with_ymd_and_hms(2012, 2, 1, 9, 30, 0).unwrap();
        assert_eq!(rruleset.dt_start, dt_start);

        // Check rrule
        assert_eq!(
            rruleset.rrule,
            vec![RRule::new(Frequency::Daily)
                .count(3)
                .validate(dt_start)
                .unwrap()]
        );

        // Check rdate
        assert_eq!(
            rruleset.rdate,
            vec![
                Tz::UTC.with_ymd_and_hms(1997, 1, 1, 0, 0, 0).unwrap(),
                Tz::UTC.with_ymd_and_hms(1997, 1, 20, 0, 0, 0).unwrap()
            ]
        );

        // Check exrule
        assert_eq!(
            rruleset.exrule,
            vec![RRule::new(Frequency::Yearly)
                .count(8)
                .by_month(&[Month::June, Month::July])
                .validate(dt_start)
                .unwrap()]
        );

        // Check exdate
        assert_eq!(
            rruleset.exdate,
            vec![Tz::UTC.with_ymd_and_hms(1997, 1, 21, 0, 0, 0).unwrap()]
        );

        // Serialize to string again
        assert_eq!(rruleset.to_string(), rruleset_str);
    }

    #[test]
    fn respect_local_timezone_in_exdates_rdates() {
        let rruleset_str = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3;BYHOUR=9;BYMINUTE=30;BYSECOND=0\nRDATE;VALUE=DATE-TIME:19970101T000000,19970120T000000\nEXRULE:FREQ=YEARLY;COUNT=8;BYMONTH=6,7;BYMONTHDAY=1;BYHOUR=9;BYMINUTE=30;BYSECOND=0\nEXDATE;VALUE=DATE-TIME:19970121T000000";
        let rruleset = RRuleSet::from_str(rruleset_str).unwrap();

        // Serialize to string again
        assert_eq!(rruleset.to_string(), rruleset_str);
    }

    #[test]
    fn respect_utc_timezone_in_exdates_rdates() {
        let rruleset_str = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3;BYHOUR=9;BYMINUTE=30;BYSECOND=0\nRDATE;VALUE=DATE-TIME:19970101T000000Z,19970120T000000Z\nEXRULE:FREQ=YEARLY;COUNT=8;BYMONTH=6,7;BYMONTHDAY=1;BYHOUR=9;BYMINUTE=30;BYSECOND=0\nEXDATE;VALUE=DATE-TIME:19970121T000000Z";
        let rruleset = RRuleSet::from_str(rruleset_str).unwrap();

        // Serialize to string again
        assert_eq!(rruleset.to_string(), rruleset_str);
    }
}
