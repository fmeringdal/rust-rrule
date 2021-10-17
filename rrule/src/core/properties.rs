use super::datetime::DateTime;
use chrono::{Month, TimeZone, Utc, Weekday};
use chrono_tz::{Tz, UTC};
use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Frequency {
    Yearly = 0,
    Monthly = 1,
    Weekly = 2,
    Daily = 3,
    Hourly = 4,
    Minutely = 5,
    Secondly = 6,
}

impl Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NWeekday {
    Every(Weekday),
    /// Value from -366 to -1 and 1 to 366 depending on frequency
    Nth(i16, Weekday),
}

impl NWeekday {
    /// Create new week occurrence
    pub fn new(number: Option<i16>, weekday: Weekday) -> Self {
        match number {
            Some(number) => Self::Nth(number, weekday),
            None => Self::Every(weekday),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RRuleProperties {
    /// The frequency of the rule.
    /// For example: yearly, weekly, hourly
    pub freq: Frequency,
    /// The interval between each frequency iteration.
    /// For example:
    /// - A yearly frequency with an interval of `2` creates 1 event every two years.
    /// - An hourly frequency with an interval of `2` created 1 event every two hours.
    pub interval: u16,
    /// How many occurrences will be generated.
    pub count: Option<u32>,
    /// The end date after which new events will no longer be generated.
    /// If the `DateTime` is equal to an instance of the event it will be the last event.
    pub until: Option<DateTime>,
    /// The timezone used during the creation of the events.
    pub tz: Tz,
    /// The start datetime of the recurring event.
    // TODO: add info about how timezone is used.
    pub dt_start: DateTime,
    /// The start day of the week.
    /// This will affect recurrences based on weekly periods.
    pub week_start: Weekday,
    /// Occurrence number corresponding to the frequency period.
    /// For example:
    /// - A monthly frequency with an `by_set_pos` of `-1` meaning the last day of the month.
    /// - An hourly frequency with an `by_set_pos` of `2` meaning the 2nd hour. (TODO Check)
    pub by_set_pos: Vec<i32>,
    /// The months to apply the recurrence to.
    /// Can be a value from 1 to 12.
    pub by_month: Vec<u8>,
    /// The month days to apply the recurrence to.
    /// Can be a value from -31 to -1 and 1 to 31.
    pub by_month_day: Vec<i8>,
    pub by_n_month_day: Vec<i8>,
    /// The year days to apply the recurrence to.
    /// Can be a value from -366 to -1 and 1 to 366.
    pub by_year_day: Vec<i16>,
    /// The week numbers to apply the recurrence to.
    /// Week numbers have the meaning described in ISO8601, that is,
    /// the first week of the year is that containing at least four days of the new year.
    /// Week day starts counting on from `week_start` value.
    /// Can be a value from -53 to -1 and 1 to 53.
    pub by_week_no: Vec<i8>,
    /// The days of the week the rules should be recurring.
    /// Should be a value of `Weekday` and optionally with a prefix of -366 to 366 depending on frequency.
    /// Corresponds with `BYDAY` field.
    pub by_weekday: Vec<NWeekday>,
    /// The hours to apply the recurrence to.
    /// Can be a value from 0 to 23.
    pub by_hour: Vec<u8>,
    /// The minutes to apply the recurrence to.
    /// Can be a value from 0 to 59.
    pub by_minute: Vec<u8>,
    /// The seconds to apply the recurrence to.
    /// Can be a value from 0 to 59.
    pub by_second: Vec<u8>,
    /// Extension, not part of RFC spec.
    /// Amount of days/months from Easter Sunday itself.
    /// Can be a value from -366 to 366.
    /// Note: Only used when `by-easter` feature flag is set. Otherwise, it is ignored.
    pub by_easter: Option<i16>,
}

impl Default for RRuleProperties {
    fn default() -> Self {
        Self {
            freq: Frequency::Yearly,
            interval: 1,
            count: None,
            until: None,
            tz: UTC,
            dt_start: UTC.ymd(1970, 1, 1).and_hms(0, 0, 0), // Unix Epoch
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
        }
    }
}

impl RRuleProperties {
    pub fn new(freq: Frequency, dt_start: DateTime) -> Self {
        Self {
            freq,
            tz: dt_start.timezone(),
            dt_start,
            ..Default::default()
        }
    }

    /// The FREQ rule part identifies the type of recurrence rule.
    pub fn freq(mut self, freq: Frequency) -> Self {
        self.freq = freq;
        self
    }

    /// The interval between each freq iteration.
    pub fn interval(mut self, interval: u16) -> Self {
        self.interval = interval;
        self
    }

    /// If given, this determines how many occurrences will be generated.
    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    /// If given, this must be a datetime instance specifying the
    /// upper-bound limit of the recurrence.
    pub fn until(mut self, until: chrono::DateTime<Utc>) -> Self {
        self.until = Some(until.with_timezone(&UTC));
        self
    }

    /// The recurrence start. Recurrences generated by the rrule will
    /// be in the same time zone as the start date.
    pub fn dt_start(mut self, dt_start: DateTime) -> Self {
        self.dt_start = dt_start;
        self.tz = dt_start.timezone();
        self
    }

    /// The week start day. This will affect recurrences based on weekly periods.
    /// The default week start is [`Weekday::Mon`].
    pub fn week_start(mut self, week_start: Weekday) -> Self {
        self.week_start = week_start;
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, positive or negative.
    /// Each given integer will specify an occurrence number, corresponding to the nth occurrence
    /// of the rule inside the frequency period. For example, a by_set_pos of -1 if combined with
    /// a MONTHLY frequency, and a by_weekday of (MO, TU, WE, TH, FR), will result in the last
    /// work day of every month.
    pub fn by_set_pos(mut self, by_set_pos: Vec<i32>) -> Self {
        self.by_set_pos = by_set_pos;
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, meaning
    /// the months to apply the recurrence to.
    pub fn by_month(mut self, by_month: Vec<Month>) -> Self {
        self.by_month = by_month
            .iter()
            .map(|month| month.number_from_month() as u8)
            .collect();
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, meaning
    /// the month days to apply the recurrence to.
    pub fn by_month_day(mut self, by_month_day: Vec<i8>) -> Self {
        self.by_month_day = by_month_day;
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, meaning
    /// the year days to apply the recurrence to.
    pub fn by_year_day(mut self, by_year_day: Vec<i16>) -> Self {
        self.by_year_day = by_year_day;
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, meaning
    /// the week numbers to apply the recurrence to. Week numbers have the meaning
    /// described in ISO8601, that is, the first week of the year is that containing
    /// at least four days of the new year.
    pub fn by_week_no(mut self, by_week_no: Vec<i8>) -> Self {
        self.by_week_no = by_week_no;
        self
    }

    /// If given, it must be either an integer (0 == MO), a sequence of integers, one
    /// of the weekday constants (MO, TU, etc.), or a sequence of these constants.
    /// When given, these variables will define the weekdays where the recurrence
    /// will be applied.
    /// A nth occurrence prefix can be given.
    pub fn by_weekday(mut self, by_weekday: Vec<NWeekday>) -> Self {
        self.by_weekday = by_weekday;
        self
    }

    /// If given, it must be either an integer, or a sequence of integers,
    /// meaning the hours to apply the recurrence to.
    pub fn by_hour(mut self, by_hour: Vec<u8>) -> Self {
        self.by_hour = by_hour;
        self
    }

    /// If given, it must be either an integer, or a sequence of integers,
    /// meaning the minutes to apply the recurrence to.
    pub fn by_minute(mut self, by_minute: Vec<u8>) -> Self {
        self.by_minute = by_minute;
        self
    }

    /// If given, it must be either an integer, or a sequence of integers,
    /// meaning the seconds to apply the recurrence to.
    pub fn by_second(mut self, by_second: Vec<u8>) -> Self {
        self.by_second = by_second;
        self
    }

    /// If given, it must be either an integer, or a sequence of integers, positive or negative.
    /// Each integer will define an offset from the Easter Sunday. Passing the offset 0 to
    /// by_easter will yield the Easter Sunday itself. This is an extension to the RFC specification.
    #[cfg(feature = "by-easter")]
    pub fn by_easter(mut self, by_easter: i16) -> Self {
        self.by_easter = Some(by_easter);
        self
    }
}
