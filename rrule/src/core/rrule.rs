use super::datetime::DateTime;
use crate::core::utils::check_str_validity;
use crate::iter::iterinfo::IterInfo;
use crate::parser::parse_rule;
use crate::parser::ParseError;
use crate::validator::check_limits;
use crate::validator::validate_rrule;
use crate::{RRuleError, RRuleIter, RRuleSet, Unvalidated, Validated};
use chrono::{Datelike, Month, Timelike, Weekday};
use chrono_tz::Tz;
#[cfg(feature = "serde")]
use serde_with::{serde_as, DeserializeFromStr, SerializeDisplay};
use std::collections::VecDeque;
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
        check_str_validity(value)?;

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
    /// let nth_weekday = NWeekday::nth(1, Weekday::Mon);
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

    /// Generates an [`NWeekday`] from a string
    ///
    /// # Examples
    /// ```
    /// use chrono::Weekday;
    /// use rrule::NWeekday;
    ///
    /// assert_eq!("1MO".parse::<NWeekday>().unwrap(), NWeekday::Nth(1, Weekday::Mon));
    /// assert_eq!("-1MO".parse::<NWeekday>().unwrap(), NWeekday::Nth(-1, Weekday::Mon));
    /// assert_eq!("MO".parse::<NWeekday>().unwrap(), NWeekday::Every(Weekday::Mon));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid [`NWeekday`]
    /// ```
    /// use rrule::NWeekday;
    ///
    /// assert_eq!("1".parse::<NWeekday>(), Err(rrule::ParseError::InvalidNWeekday("1".to_string())));
    /// assert_eq!("0MO".parse::<NWeekday>(), Err(rrule::ParseError::InvalidNWeekday("0MO".to_string())));
    /// ```
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let length = value.len();

        if check_str_validity(value).is_err() || length < 2 {
            return Err(ParseError::InvalidWeekday(value.to_string()));
        }

        // it doesn't have any issue, because we checked the string is ASCII above
        let wd = str_to_weekday(&value[(length - 2)..])?;
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
    /// assert_eq!(format!("{}", NWeekday::Nth(1, Weekday::Mon)), "1MO");
    /// assert_eq!(format!("{}", NWeekday::Every(Weekday::Mon)), "MO");
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

fn str_to_weekday(d: &str) -> Result<Weekday, ParseError> {
    let day = match &d.to_uppercase()[..] {
        "MO" => Weekday::Mon,
        "TU" => Weekday::Tue,
        "WE" => Weekday::Wed,
        "TH" => Weekday::Thu,
        "FR" => Weekday::Fri,
        "SA" => Weekday::Sat,
        "SU" => Weekday::Sun,
        _ => return Err(ParseError::InvalidWeekday(d.to_string())),
    };
    Ok(day)
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
/// - `Validated`, which is when the RRule has been parsed and validated, based on the start date
#[cfg_attr(feature = "serde", serde_as)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(DeserializeFromStr, SerializeDisplay))]
pub struct RRule<Stage = Validated> {
    /// The frequency of the rrule.
    /// For example: yearly, weekly, hourly
    pub(crate) freq: Frequency,
    /// The interval between each frequency iteration.
    /// For example:
    /// - A yearly frequency with an interval of `2` creates 1 event every two years.
    /// - An hourly frequency with an interval of `2` created 1 event every two hours.
    pub(crate) interval: u16,
    /// How many occurrences will be generated.
    pub(crate) count: Option<u32>,
    /// The end date after which new events will no longer be generated.
    /// If the `DateTime` is equal to an instance of the event it will be the last event.
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
    /// the first week of the year is that containing at least four days of the new year.
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
            ..Default::default()
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
    pub fn until(mut self, until: chrono::DateTime<Tz>) -> Self {
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
    #[allow(clippy::cast_possible_truncation)]
    pub fn by_month(mut self, by_month: &[Month]) -> Self {
        self.by_month = by_month
            .iter()
            .map(|month| month.number_from_month() as u8)
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
    /// described in ISO8601, that is, the first week of the year is that containing
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
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn finalize_parsed_rrule(mut self, dt_start: &DateTime) -> RRule<Unvalidated> {
        use std::cmp::Ordering;
        // TEMP: move negative months to other list
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

        // Can only be set to true if feature flag is set.
        let by_easter_is_some = if cfg!(feature = "by-easter") {
            self.by_easter.is_some()
        } else {
            false
        };

        // Add some freq specific additional properties
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
                        self.by_month = vec![dt_start.month() as u8];
                    }
                    self.by_month_day = vec![dt_start.day() as i8];
                }
                Frequency::Monthly => {
                    self.by_month_day = vec![dt_start.day() as i8];
                }
                Frequency::Weekly => {
                    self.by_weekday = vec![NWeekday::Every(dt_start.weekday())];
                }
                _ => (),
            };
        }

        // by_hour
        if self.by_hour.is_empty() && self.freq < Frequency::Hourly {
            self.by_hour = vec![dt_start.hour() as u8];
        }

        // by_minute
        if self.by_minute.is_empty() && self.freq < Frequency::Minutely {
            self.by_minute = vec![dt_start.minute() as u8];
        }

        // by_second
        if self.by_second.is_empty() && self.freq < Frequency::Secondly {
            self.by_second = vec![dt_start.second() as u8];
        }

        self
    }

    /// Validates the [`RRule`] with the given `dt_start`.
    ///
    /// # Errors
    ///
    /// If the properties are not valid it will return [`RRuleError`].
    pub fn validate(self, dt_start: DateTime) -> Result<RRule<Validated>, RRuleError> {
        let rrule = self.finalize_parsed_rrule(&dt_start);

        // Validate required checks (defined by RFC 5545)
        validate_rrule::validate_rrule_forced(&rrule, &dt_start)?;
        // Validate (optional) sanity checks. (arbitrary limits)
        // Can be disabled by `no-validation-limits` feature flag, see README.md for more info.
        check_limits::check_limits(&rrule, &dt_start)?;

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
        let rrule_set = RRuleSet::new(dt_start).set_rrules(vec![self.validate(dt_start)?]);
        Ok(rrule_set)
    }
}

impl RRule {
    pub(crate) fn iter_with_ctx(&self, dt_start: DateTime) -> RRuleIter {
        match RRuleIter::new(self, &dt_start) {
            Ok(iter) => iter,
            Err(err) => {
                // Print error and create iterator that will ways return the error if used.
                log::error!("{:?}", err);
                let error = Some(err);
                // This is mainly a dummy object, as it will ways return the error when called.
                RRuleIter {
                    counter_date: dt_start,
                    ii: IterInfo::new_no_rebuild(self),
                    timeset: vec![],
                    dt_start,
                    buffer: VecDeque::new(),
                    finished: false,
                    count: None,
                    error,
                }
            }
        }
    }
}

impl FromStr for RRule<Unvalidated> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        check_str_validity(s)?;

        parse_rule(s)
    }
}

impl<S> Display for RRule<S> {
    /// Generates a string based on the [iCalendar RRULE spec](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.3).
    /// It doesn't prepend "RRULE:" to the string.
    /// When you call this function on [`RRule<Unvalidated>`], it can generate an invalid string, like 'FREQ=YEARLY;INTERVAL=-1'
    /// But it supposed to always generate a valid string on [`RRule<Validated>`].
    /// So if you want a valid string, it's smarter to always use `rrule.validate(ds_start)?.to_string()`.
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = Vec::with_capacity(15);
        res.push(format!("FREQ={}", &self.freq));

        if let Some(until) = &self.until {
            res.push(format!("UNTIL={}", until.format("%Y%m%dT%H%M%SZ")));
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
