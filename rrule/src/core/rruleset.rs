use super::{datetime::DateTime, rrule::*};
use crate::core::datetime::datetime_to_ical_format;
use crate::parser::build_rruleset;
use crate::RRuleError;
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
    /// The properties specified by this rule.
    pub rrule: Vec<RRule>,
    pub rdate: Vec<DateTime>,
    pub exrule: Vec<RRule>,
    pub exdate: Vec<DateTime>,
    /// The start datetime of the recurring event.
    pub dt_start: DateTime,
}

impl Default for RRuleSet {
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
    pub fn new(dt_start: DateTime) -> Self {
        Self {
            dt_start,
            ..Default::default()
        }
    }

    pub fn rrule(&mut self, rrule: RRule) {
        self.rrule.push(rrule);
    }

    pub fn exrule(&mut self, rrule: RRule) {
        self.exrule.push(rrule);
    }

    pub fn rdate(&mut self, rdate: DateTime) {
        self.rdate.push(rdate);
    }

    pub fn exdate(&mut self, exdate: DateTime) {
        self.exdate.push(exdate);
    }
}

impl FromStr for RRuleSet {
    type Err = RRuleError;

    /// Creates an [`RRuleSet`] from a [`String`] if input is valid.
    ///
    /// If RRule contains invalid parts then [`RRuleError`] will be returned.
    /// This should never panic, but it might be in odd cases.
    /// Please report if it does panic.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        build_rruleset(s)
    }
}

impl Display for RRuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let properties = self
            .rrule
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        let datetime = datetime_to_ical_format(&self.dt_start);

        write!(f, "DTSTART{}\n{}", datetime, properties)
    }
}
