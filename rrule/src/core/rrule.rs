use super::datetime::DateTime;
use crate::core::get_day;
use crate::core::get_hour;
use crate::core::get_minute;
use crate::core::get_month;
use crate::core::get_second;
use crate::iter::RRuleIter;
use crate::parser::str_to_weekday;
use crate::parser::ContentLineCaptures;
use crate::parser::ParseError;
use crate::validator::validate_rrule;
use crate::validator::ValidationError;
use crate::{RRuleError, RRuleSet, Unvalidated, Validated};
use chrono::{Datelike, Month, Weekday};
#[cfg(feature = "serde")]
use serde_with::{serde_as, DeserializeFromStr, SerializeDisplay};
use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(DeserializeFromStr, SerializeDisplay))]
/// The frequency of a recurrence.
pub enum Frequency {
    /// The recurrence occurs on a yearly basis.
    Yearly = 0,
    /// The recurrence occurs on a monthly basis.
    Monthly = 1,
    /// The recurrence occurs on a weekly basis.
    Weekly = 2,
    /// The recurrence occurs on a daily basis.
    Daily = 3,
    /// The recurrence occurs on an hourly basis.
    Hourly = 4,
    /// The recurrence occurs on a minutely basis.
    Minutely = 5,
    /// The recurrence occurs on a second basis.
    Secondly = 6,
}

impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Frequency::Yearly => "yearly",
            Frequency::Monthly => "monthly",
            Frequency::Weekly => "weekly",
            Frequency::Daily => "daily",
            Frequency::Hourly => "hourly",
            Frequency::Minutely => "minutely",
            Frequency::Secondly => "secondly",
        };
        write!(f, "{}", name)
    }
}

impl FromStr for Frequency {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let freq = match &value.to_uppercase()[..] {
            "YEARLY" => Frequency::Yearly,
            "MONTHLY" => Frequency::Monthly,
            "WEEKLY" => Frequency::Weekly,
            "DAILY" => Frequency::Daily,
            "HOURLY" => Frequency::Hourly,
            "MINUTELY" => Frequency::Minutely,
            "SECONDLY" => Frequency::Secondly,
            val => return Err(ParseError::InvalidFrequency(val.to_string())),
        };
        Ok(freq)
    }
}

/// This indicates the nth occurrence of a specific day within a MONTHLY or YEARLY RRULE.
///
/// For example, `NWeekday::Nth(1, MO)` represents the first Monday within the month or year,
/// whereas `NWeekday::Nth(-1, MO)` represents the last Monday of the month or year.
/// And `NWeekday::Every(MO)`, means all Mondays of the month or year.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(DeserializeFromStr, SerializeDisplay))]
pub enum NWeekday {
    /// When it is every weekday of the month or year.
    Every(Weekday),
    /// When it is the nth weekday of the month or year.
    /// The first member's value is from -366 to -1 and 1 to 366 depending on frequency
    Nth(i16, Weekday),
}

// The ordering here doesn't really matter as it is only used to sort for display purposes
fn n_weekday_cmp(val1: NWeekday, val2: NWeekday) -> Ordering {
    match val1 {
        NWeekday::Every(wday) => match val2 {
            NWeekday::Every(other_wday) => wday
                .num_days_from_monday()
                .cmp(&other_wday.num_days_from_monday()),
            NWeekday::Nth(_n, _other_wday) => Ordering::Less,
        },
        NWeekday::Nth(n, wday) => match val2 {
            NWeekday::Every(_) => Ordering::Greater,
            NWeekday::Nth(other_n, other_wday) => match n.cmp(&other_n) {
                Ordering::Equal => wday
                    .num_days_from_monday()
                    .cmp(&other_wday.num_days_from_monday()),
                less_or_greater => less_or_greater,
            },
        },
    }
}

impl PartialOrd for NWeekday {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(n_weekday_cmp(*self, *other))
    }
}

impl Ord for NWeekday {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        n_weekday_cmp(*self, *other)
    }
}

impl NWeekday {
    /// Creates a new week occurrence
    ///
    /// # Arguments
    ///
    /// * `n` - The nth occurrence of the week day. Should be between -366 and 366, and not 0.
    /// * `weekday` - The week day
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::Weekday;
    /// use rrule::NWeekday;
    ///
    /// let nth_weekday = NWeekday::new(Some(1), Weekday::Mon);
    /// ```
    #[must_use]
    pub fn new(number: Option<i16>, weekday: Weekday) -> Self {
        match number {
            Some(number) => Self::Nth(number, weekday),
            None => Self::Every(weekday),
        }
    }
}

impl FromStr for NWeekday {
    type Err = ParseError;

    /// Generates an [`NWeekday`] from a string.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let length = value.len();

        if length < 2 {
            return Err(ParseError::InvalidWeekday(value.into()));
        }

        // it doesn't have any issue, because we checked the string is ASCII above
        let wd = str_to_weekday(&value[(length - 2)..])
            .map_err(|_| ParseError::InvalidWeekday(value.into()))?;
        let nth = value[..(length - 2)].parse::<i16>().unwrap_or_default();

        if nth == 0 {
            Ok(NWeekday::Every(wd))
        } else {
            Ok(NWeekday::new(Some(nth), wd))
        }
    }
}

impl Display for NWeekday {
    /// Returns a string representation of the [`NWeekday`]
    ///
    /// ```
    /// use chrono::Weekday;
    /// use rrule::NWeekday;
    ///
    /// assert_eq!(format!("{}", NWeekday::Every(Weekday::Mon)), "MO");
    /// assert_eq!(format!("{}", NWeekday::Nth(1, Weekday::Mon)), "MO");
    /// assert_eq!(format!("{}", NWeekday::Nth(2, Weekday::Mon)), "2MO");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let weekday = match self {
            NWeekday::Every(wd) => weekday_to_str(*wd),
            NWeekday::Nth(number, wd) => {
                let mut wd_str = weekday_to_str(*wd);
                if *number != 1 {
                    wd_str = format!("{}{}", number, wd_str);
                };
                wd_str
            }
        };

        write!(f, "{}", weekday)
    }
}

fn weekday_to_str(d: Weekday) -> String {
    match d {
        Weekday::Mon => "MO".to_string(),
        Weekday::Tue => "TU".to_string(),
        Weekday::Wed => "WE".to_string(),
        Weekday::Thu => "TH".to_string(),
        Weekday::Fri => "FR".to_string(),
        Weekday::Sat => "SA".to_string(),
        Weekday::Sun => "SU".to_string(),
    }
}

/// Represents a complete RRULE property based on the [iCalendar specification](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.3)
/// It has two stages, based on the attached type, `Validated` or `Unvalidated`.
/// - `Unvalidated`, which is the raw string representation of the RRULE
/// - `Validated`, which is when the `RRule` has been parsed and validated, based on the start date
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", serde_as)]
#[cfg_attr(feature = "serde", derive(DeserializeFromStr, SerializeDisplay))]
pub struct RRule<Stage = Validated> {
    /// The frequency of the rrule.
    /// For example, yearly, weekly, hourly
    pub(crate) freq: Frequency,
    /// The interval between each frequency iteration.
    /// For example,
    /// - A yearly frequency with an interval of `2` creates 1 event every two years.
    /// - An hourly frequency with an interval of `2` created 1 event every two hours.
    pub(crate) interval: u16,
    /// How many occurrences will be generated.
    pub(crate) count: Option<u32>,
    /// The end date after which new events will no longer be generated.
    /// If the `DateTime` is equal to an instance of the event, it will be the last event.
    #[cfg_attr(feature = "serde", serde_as(as = "DisplayFromStr"))]
    pub(crate) until: Option<DateTime>,
    /// The start day of the week.
    /// This will affect recurrences based on weekly periods.
    pub(crate) week_start: Weekday,
    /// Occurrence number corresponding to the frequency period.
    /// For example:
    /// - A monthly frequency with an `by_set_pos` of `-1` meaning the last day of the month.
    /// - An hourly frequency with an `by_set_pos` of `2` meaning the 2nd hour. (TODO Check)
    pub(crate) by_set_pos: Vec<i32>,
    /// The months to apply the recurrence to.
    /// Can be a value from 1 to 12.
    pub(crate) by_month: Vec<u8>,
    /// The month days to apply the recurrence to.
    /// Can be a value from -31 to -1 and 1 to 31.
    pub(crate) by_month_day: Vec<i8>,
    pub(crate) by_n_month_day: Vec<i8>,
    /// The year days to apply the recurrence to.
    /// Can be a value from -366 to -1 and 1 to 366.
    pub(crate) by_year_day: Vec<i16>,
    /// The week numbers to apply the recurrence to.
    /// Week numbers have the meaning described in ISO8601, that is,
    /// the first week of the year is that it contains at least four days of the new year.
    /// Week day starts counting on from `week_start` value.
    /// Can be a value from -53 to -1 and 1 to 53.
    pub(crate) by_week_no: Vec<i8>,
    /// The days of the week the rules should be recurring.
    /// Should be a value of `Weekday` and optionally with a prefix of -366 to 366 depending on frequency.
    /// Corresponds with `BYDAY` field.
    pub(crate) by_weekday: Vec<NWeekday>,
    /// The hours to apply the recurrence to.
    /// Can be a value from 0 to 23.
    pub(crate) by_hour: Vec<u8>,
    /// The minutes to apply the recurrence to.
    /// Can be a value from 0 to 59.
    pub(crate) by_minute: Vec<u8>,
    /// The seconds to apply the recurrence to.
    /// Can be a value from 0 to 59.
    pub(crate) by_second: Vec<u8>,
    /// Extension, not part of RFC spec.
    /// Amount of days/months from Easter Sunday itself.
    /// Can be a value from -366 to 366.
    /// Note: Only used when `by-easter` feature flag is set. Otherwise, it is ignored.
    pub(crate) by_easter: Option<i16>,
    /// A phantom data to have the stage (unvalidated or validated).
    #[cfg_attr(feature = "serde", serde_as(as = "ignore"))]
    pub(crate) stage: PhantomData<Stage>,
}

impl Default for RRule<Unvalidated> {
    /// Creates a new unvalidated `RRule` with default values and Yearly frequency.
    fn default() -> Self {
        Self {
            freq: Frequency::Yearly,
            interval: 1,
            count: None,
            until: None,
            week_start: Weekday::Mon,
            by_set_pos: Vec::new(),
            by_month: Vec::new(),
            by_month_day: Vec::new(),
            by_n_month_day: Vec::new(),
            by_year_day: Vec::new(),
            by_week_no: Vec::new(),
            by_weekday: Vec::new(),
            by_hour: Vec::new(),
            by_minute: Vec::new(),
            by_second: Vec::new(),
            by_easter: None,
            stage: PhantomData,
        }
    }
}

impl RRule<Unvalidated> {
    /// Creates a new unvalidated `RRule` with default values and the given frequency.
    #[must_use]
    pub fn new(freq: Frequency) -> Self {
        Self {
            freq,
            ..RRule::default()
        }
    }

    /// The FREQ rule part identifies the type of recurrence rule.
    #[must_use]
    pub fn freq(mut self, freq: Frequency) -> Self {
        self.freq = freq;
        self
    }

    /// The interval between each freq iteration.
    #[must_use]
    pub fn interval(mut self, interval: u16) -> Self {
        self.interval = interval;
        self
    }

    /// If given, this determines how many occurrences will be generated.
    #[must_use]
    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    /// If given, this must be a datetime instance specifying the
    /// upper-bound limit of the recurrence.
    #[must_use]
    pub fn until(mut self, until: DateTime) -> Self {
        self.until = Some(until);
        self
    }

    /// The week start day. This will affect recurrences based on weekly periods.
    /// The default week start is [`Weekday::Mon`].
    #[must_use]
    pub fn week_start(mut self, week_start: Weekday) -> Self {
        self.week_start = week_start;
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, positive or negative.
    /// Each given integer will specify an occurrence number, corresponding to the nth occurrence
    /// of the rule inside the frequency period. For example, a `by_set_pos` of -1 if combined with
    /// a MONTHLY frequency, and a `by_weekday` of (MO, TU, WE, TH, FR), will result in the last
    /// work day of every month.
    #[must_use]
    pub fn by_set_pos(mut self, by_set_pos: Vec<i32>) -> Self {
        self.by_set_pos = by_set_pos;
        self
    }

    /// When given, these variables will define the months to apply the recurrence to.
    #[must_use]
    pub fn by_month(mut self, by_month: &[Month]) -> Self {
        self.by_month = by_month
            .iter()
            .map(|month| {
                u8::try_from(month.number_from_month()).expect("1-12 is within range of u8")
            })
            .collect();
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, meaning
    /// the month days to apply the recurrence to.
    #[must_use]
    pub fn by_month_day(mut self, by_month_day: Vec<i8>) -> Self {
        self.by_month_day = by_month_day;
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, meaning
    /// the year days to apply the recurrence to.
    #[must_use]
    pub fn by_year_day(mut self, by_year_day: Vec<i16>) -> Self {
        self.by_year_day = by_year_day;
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, meaning
    /// the week numbers to apply the recurrence to. Week numbers have the meaning
    /// described in ISO8601, that is, the first week of the year is that it contains
    /// at least four days of the new year.
    #[must_use]
    pub fn by_week_no(mut self, by_week_no: Vec<i8>) -> Self {
        self.by_week_no = by_week_no;
        self
    }

    /// When given, these variables will define the weekdays where the recurrence
    /// will be applied.
    #[must_use]
    pub fn by_weekday(mut self, by_weekday: Vec<NWeekday>) -> Self {
        self.by_weekday = by_weekday;
        self
    }

    /// If given, it must be either an integer, or a sequence of integers,
    /// meaning the hours to apply the recurrence to.
    #[must_use]
    pub fn by_hour(mut self, by_hour: Vec<u8>) -> Self {
        self.by_hour = by_hour;
        self
    }

    /// If given, it must be either an integer, or a sequence of integers,
    /// meaning the minutes to apply the recurrence to.
    #[must_use]
    pub fn by_minute(mut self, by_minute: Vec<u8>) -> Self {
        self.by_minute = by_minute;
        self
    }

    /// If given, it must be either an integer, or a sequence of integers,
    /// meaning the seconds to apply the recurrence to.
    #[must_use]
    pub fn by_second(mut self, by_second: Vec<u8>) -> Self {
        self.by_second = by_second;
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, positive or negative.
    /// Each integer will define an offset from the Easter Sunday. Passing the offset 0 to
    /// `by_easter` will yield the Easter Sunday itself. This is an extension to the RFC specification.
    #[cfg(feature = "by-easter")]
    #[must_use]
    pub fn by_easter(mut self, by_easter: i16) -> Self {
        self.by_easter = Some(by_easter);
        self
    }

    /// Fills in some additional fields in order to make iter work correctly.
    pub(crate) fn finalize_parsed_rrule(mut self, dt_start: &DateTime) -> RRule<Unvalidated> {
        // TEMP: move negative months to another list
        let mut by_month_day = vec![];
        let mut by_n_month_day = self.by_n_month_day;
        for by_month_day_item in self.by_month_day {
            match by_month_day_item.cmp(&0) {
                Ordering::Greater => by_month_day.push(by_month_day_item),
                Ordering::Less => by_n_month_day.push(by_month_day_item),
                Ordering::Equal => {}
            }
        }
        self.by_month_day = by_month_day;
        self.by_n_month_day = by_n_month_day;

        // Can only be set to true if the feature flag is set.
        let by_easter_is_some = if cfg!(feature = "by-easter") {
            self.by_easter.is_some()
        } else {
            false
        };

        // Add some freq-specific additional properties
        if !(!self.by_week_no.is_empty()
            || !self.by_year_day.is_empty()
            || !self.by_month_day.is_empty()
            || !self.by_n_month_day.is_empty()
            || !self.by_weekday.is_empty()
            || by_easter_is_some)
        {
            match self.freq {
                Frequency::Yearly => {
                    if self.by_month.is_empty() {
                        let month = get_month(dt_start);
                        self.by_month = vec![month];
                    }
                    let day = get_day(dt_start);
                    self.by_month_day = vec![day];
                }
                Frequency::Monthly => {
                    let day = get_day(dt_start);
                    self.by_month_day = vec![day];
                }
                Frequency::Weekly => {
                    self.by_weekday = vec![NWeekday::Every(dt_start.weekday())];
                }
                _ => (),
            };
        }

        // by_hour
        if self.by_hour.is_empty() && self.freq < Frequency::Hourly {
            let hour = get_hour(dt_start);
            self.by_hour = vec![hour];
        }

        // by_minute
        if self.by_minute.is_empty() && self.freq < Frequency::Minutely {
            let minute = get_minute(dt_start);
            self.by_minute = vec![minute];
        }

        // by_second
        if self.by_second.is_empty() && self.freq < Frequency::Secondly {
            let second = get_second(dt_start);
            self.by_second = vec![second];
        }

        // make sure all BYXXX are unique and sorted
        self.by_hour.sort_unstable();
        self.by_hour.dedup();

        self.by_minute.sort_unstable();
        self.by_minute.dedup();

        self.by_second.sort_unstable();
        self.by_second.dedup();

        self.by_month.sort_unstable();
        self.by_month.dedup();

        self.by_month_day.sort_unstable();
        self.by_month_day.dedup();

        self.by_n_month_day.sort_unstable();
        self.by_n_month_day.dedup();

        self.by_year_day.sort_unstable();
        self.by_year_day.dedup();

        self.by_week_no.sort_unstable();
        self.by_week_no.dedup();

        self.by_set_pos.sort_unstable();
        self.by_set_pos.dedup();

        self.by_weekday.sort_unstable();
        self.by_weekday.dedup();

        self
    }

    /// Validates the [`RRule`] with the given `dt_start`.
    ///
    /// # Errors
    ///
    /// If the properties aren't valid, it will return [`RRuleError`].
    pub fn validate(self, dt_start: DateTime) -> Result<RRule<Validated>, RRuleError> {
        let rrule = self.finalize_parsed_rrule(&dt_start);

        // Validate required checks (defined by RFC 5545)
        validate_rrule::validate_rrule_forced(&rrule, &dt_start)?;

        // Check if it is possible to generate a timeset
        match rrule.freq {
            Frequency::Hourly => {
                if rrule.by_minute.is_empty() && rrule.by_second.is_empty() {
                    return Err(ValidationError::UnableToGenerateTimeset.into());
                }
            }
            Frequency::Minutely => {
                if rrule.by_second.is_empty() {
                    return Err(ValidationError::UnableToGenerateTimeset.into());
                }
            }
            Frequency::Secondly => {}
            _ => {
                if rrule.by_hour.is_empty()
                    && rrule.by_minute.is_empty()
                    && rrule.by_second.is_empty()
                {
                    return Err(ValidationError::UnableToGenerateTimeset.into());
                }
            }
        }

        Ok(RRule {
            freq: rrule.freq,
            interval: rrule.interval,
            count: rrule.count,
            until: rrule.until,
            week_start: rrule.week_start,
            by_set_pos: rrule.by_set_pos,
            by_month: rrule.by_month,
            by_month_day: rrule.by_month_day,
            by_n_month_day: rrule.by_n_month_day,
            by_year_day: rrule.by_year_day,
            by_week_no: rrule.by_week_no,
            by_weekday: rrule.by_weekday,
            by_hour: rrule.by_hour,
            by_minute: rrule.by_minute,
            by_second: rrule.by_second,
            by_easter: rrule.by_easter,
            stage: PhantomData,
        })
    }

    /// Validates the [`RRule`] with the given `dt_start` and creates an [`RRuleSet`] struct.
    ///
    /// # Errors
    ///
    /// Returns [`RRuleError::ValidationError`] in case the rrule is invalid.
    pub fn build(self, dt_start: DateTime) -> Result<RRuleSet, RRuleError> {
        let rrule = self.validate(dt_start)?;
        let rrule_set = RRuleSet::new(dt_start).rrule(rrule);
        Ok(rrule_set)
    }
}

impl RRule {
    pub(crate) fn iter_with_ctx(&self, dt_start: DateTime, limited: bool) -> RRuleIter {
        RRuleIter::new(self, &dt_start, limited)
    }
}

impl FromStr for RRule<Unvalidated> {
    type Err = RRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = ContentLineCaptures::new(s)?;
        RRule::try_from(parts).map_err(From::from)
    }
}

impl<S> Display for RRule<S> {
    /// Generates a string based on the [iCalendar RRULE spec](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.3).
    /// It doesn't prepend "RRULE:" to the string.
    /// When you call this function on [`RRule<Unvalidated>`], it can generate an invalid string, like 'FREQ=YEARLY;INTERVAL=-1'
    /// But it is supposed to always generate a valid string on [`RRule<Validated>`].
    /// So if you want a valid string, it's smarter to always use `rrule.validate(ds_start)?.to_string()`.
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = Vec::with_capacity(15);
        res.push(format!("FREQ={}", &self.freq));

        if let Some(until) = &self.until {
            let maybe_zulu = if until.timezone().is_local() { "" } else { "Z" };
            res.push(format!(
                "UNTIL={}{}",
                until.format("%Y%m%dT%H%M%S"),
                maybe_zulu
            ));
        }

        if let Some(count) = &self.count {
            res.push(format!("COUNT={}", count));
        }

        // One interval is the default, no need to expose it.
        if self.interval != 1 {
            res.push(format!("INTERVAL={}", &self.interval));
        }

        // Monday is the default, no need to expose it.
        if self.week_start != Weekday::Mon {
            res.push(format!("WKST={}", &self.week_start));
        }

        if !self.by_set_pos.is_empty() {
            res.push(format!(
                "BYSETPOS={}",
                self.by_set_pos
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }

        if !self.by_month.is_empty() {
            res.push(format!(
                "BYMONTH={}",
                self.by_month
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }

        if !self.by_month_day.is_empty() {
            res.push(format!(
                "BYMONTHDAY={}",
                self.by_month_day
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }

        if !self.by_week_no.is_empty() {
            res.push(format!(
                "BYWEEKNO={}",
                self.by_week_no
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }

        if !self.by_hour.is_empty() {
            res.push(format!(
                "BYHOUR={}",
                self.by_hour
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }

        if !self.by_minute.is_empty() {
            res.push(format!(
                "BYMINUTE={}",
                self.by_minute
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }

        if !self.by_second.is_empty() {
            res.push(format!(
                "BYSECOND={}",
                self.by_second
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }

        if !self.by_year_day.is_empty() {
            res.push(format!(
                "BYYEARDAY={}",
                self.by_year_day
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }

        if !self.by_weekday.is_empty() {
            res.push(format!(
                "BYDAY={}",
                self.by_weekday
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }

        #[cfg(feature = "by-easter")]
        if let Some(by_easter) = &self.by_easter {
            res.push(format!("BYEASTER={}", by_easter));
        }

        write!(f, "{}", res.join(";"))
    }
}

impl<S> RRule<S> {
    /// Get the frequency of the recurrence.
    #[must_use]
    pub fn get_freq(&self) -> Frequency {
        self.freq
    }

    /// Get the interval of the recurrence.
    #[must_use]
    pub fn get_interval(&self) -> u16 {
        self.interval
    }

    /// Get the count of the recurrence.
    #[must_use]
    pub fn get_count(&self) -> Option<u32> {
        self.count
    }

    /// Get the until of the recurrence.
    #[must_use]
    pub fn get_until(&self) -> Option<&DateTime> {
        self.until.as_ref()
    }

    /// Get the `by_set_pos` of the recurrence.
    #[must_use]
    pub fn get_week_start(&self) -> Weekday {
        self.week_start
    }

    /// Get the `by_month` of the recurrence.
    #[must_use]
    pub fn get_by_set_pos(&self) -> &[i32] {
        &self.by_set_pos
    }

    /// Get the `by_month` of the recurrence.
    #[must_use]
    pub fn get_by_month(&self) -> &[u8] {
        &self.by_month
    }

    /// Get the `by_month_day` of the recurrence.
    #[must_use]
    pub fn get_by_month_day(&self) -> &[i8] {
        &self.by_month_day
    }

    /// Get the `by_year_day` of the recurrence.
    #[must_use]
    pub fn get_by_year_day(&self) -> &[i16] {
        &self.by_year_day
    }

    /// Get the `by_hour` of the recurrence.
    #[must_use]
    pub fn get_by_week_no(&self) -> &[i8] {
        &self.by_week_no
    }

    /// Get the `by_hour` of the recurrence.
    #[must_use]
    pub fn get_by_weekday(&self) -> &[NWeekday] {
        &self.by_weekday
    }

    /// Get the `by_hour` of the recurrence.
    #[must_use]
    pub fn get_by_hour(&self) -> &[u8] {
        &self.by_hour
    }

    /// Get the `by_minute` of the recurrence.
    #[must_use]
    pub fn get_by_minute(&self) -> &[u8] {
        &self.by_minute
    }

    /// Get the `by_second` of the recurrence.
    #[must_use]
    pub fn get_by_second(&self) -> &[u8] {
        &self.by_second
    }

    /// Get the `by_easter` of the recurrence.
    #[cfg(feature = "by-easter")]
    #[must_use]
    pub fn get_by_easter(&self) -> Option<&i16> {
        self.by_easter.as_ref()
    }
}
