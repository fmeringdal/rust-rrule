use std::fmt::Display;

use chrono::{Datelike, Duration, Local, NaiveTime, TimeZone, Timelike, Weekday};
use chrono_tz::Tz;

use crate::iter::add_time_to_date;

pub(crate) fn duration_from_midnight(time: NaiveTime) -> Duration {
    Duration::hours(i64::from(time.hour()))
        + Duration::minutes(i64::from(time.minute()))
        + Duration::seconds(i64::from(time.second()))
}

pub(crate) fn get_month(dt: &DateTime) -> u8 {
    u8::try_from(dt.month()).expect("month is between 1-12 which is covered by u8")
}

pub(crate) fn get_day(dt: &DateTime) -> i8 {
    i8::try_from(dt.day()).expect("day is between 1-31 which is covered by i8")
}

pub(crate) fn get_hour(dt: &DateTime) -> u8 {
    u8::try_from(dt.hour()).expect("hour is between 0-23 which is covered by u8")
}

pub(crate) fn get_minute(dt: &DateTime) -> u8 {
    u8::try_from(dt.minute()).expect("minute is between 0-59 which is covered by u8")
}

pub(crate) fn get_second(dt: &DateTime) -> u8 {
    u8::try_from(dt.second()).expect("second is between 0-59 which is covered by u8")
}

/// Generates an iCalendar date-time string format with the prefix symbols.
/// Like: `:19970714T173000Z` or `;TZID=America/New_York:19970714T133000`
/// ref: <https://tools.ietf.org/html/rfc5545#section-3.3.5>
pub(crate) fn datetime_to_ical_format(dt: &DateTime) -> String {
    let mut tz_prefix = String::new();
    let mut tz_postfix = String::new();
    let tz = dt.timezone();
    match tz {
        RRuleTimeZone::Local => {}
        RRuleTimeZone::Tz(tz) => match tz {
            Tz::UTC => {
                tz_postfix = "Z".to_string();
            }
            _ => {
                tz_prefix = format!(";TZID={}", tz.name());
            }
        },
    }

    let dt = dt.format("%Y%m%dT%H%M%S");
    format!("{}:{}{}", tz_prefix, dt, tz_postfix)
}

#[derive(Debug, PartialEq)]
pub enum RRuleTimeZone {
    Local,
    Tz(chrono_tz::Tz),
}

impl RRuleTimeZone {
    pub fn name(&self) -> String {
        match self {
            RRuleTimeZone::Local => "Local".into(),
            RRuleTimeZone::Tz(tz) => tz.name().into(),
        }
    }

    pub fn datetime(&self, year: i32, month: u32, day: u32, time: NaiveTime) -> Option<DateTime> {
        match self {
            RRuleTimeZone::Local => {
                let date = Local.ymd(year, month, day);
                add_time_to_date(date, time)
            }
            RRuleTimeZone::Tz(tz) => {
                let date = tz.ymd(year, month, day);
                add_time_to_date(date, time)
            }
        }
    }
}

/// DateTime that is able to represent datetimes in Local and chrono_tz::Tz
/// timezones.
#[derive(Debug, Clone, Eq, Ord, Copy)]
pub enum DateTime {
    /// Local timezone
    Local(chrono::DateTime<chrono::Local>),
    /// Specific non-local timezone
    Tz(chrono::DateTime<chrono_tz::Tz>),
}

impl PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Local(l0), Self::Local(r0)) => l0 == r0,
            (Self::Tz(l0), Self::Tz(r0)) => l0 == r0,
            (Self::Tz(l0), Self::Local(r0)) => l0 == r0,
            (Self::Local(l0), Self::Tz(r0)) => l0 == r0,
        }
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Local(l0), Self::Local(r0)) => l0.partial_cmp(r0),
            (Self::Tz(l0), Self::Tz(r0)) => l0.partial_cmp(r0),
            (Self::Tz(l0), Self::Local(r0)) => l0.partial_cmp(r0),
            (Self::Local(l0), Self::Tz(r0)) => l0.partial_cmp(r0),
        }
    }
}

impl DateTime {
    pub fn with_timezone<T: TimeZone>(&self, tz: &T) -> chrono::DateTime<T> {
        match self {
            Self::Local(dt) => dt.with_timezone(tz),
            Self::Tz(dt) => dt.with_timezone(tz),
        }
    }

    pub fn timezone(&self) -> RRuleTimeZone {
        match self {
            Self::Local(_) => RRuleTimeZone::Local,
            Self::Tz(dt) => RRuleTimeZone::Tz(dt.timezone()),
        }
    }

    pub fn year(&self) -> i32 {
        match self {
            Self::Local(dt) => dt.year(),
            Self::Tz(dt) => dt.year(),
        }
    }

    pub fn month(&self) -> u32 {
        match self {
            Self::Local(dt) => dt.month(),
            Self::Tz(dt) => dt.month(),
        }
    }

    pub fn weekday(&self) -> Weekday {
        match self {
            Self::Local(dt) => dt.weekday(),
            Self::Tz(dt) => dt.weekday(),
        }
    }

    pub fn day(&self) -> u32 {
        match self {
            Self::Local(dt) => dt.day(),
            Self::Tz(dt) => dt.day(),
        }
    }

    pub fn hour(&self) -> u32 {
        match self {
            Self::Local(dt) => dt.hour(),
            Self::Tz(dt) => dt.hour(),
        }
    }

    pub fn minute(&self) -> u32 {
        match self {
            Self::Local(dt) => dt.minute(),
            Self::Tz(dt) => dt.minute(),
        }
    }

    pub fn second(&self) -> u32 {
        match self {
            Self::Local(dt) => dt.second(),
            Self::Tz(dt) => dt.second(),
        }
    }

    pub fn timestamp(&self) -> i64 {
        match self {
            Self::Local(dt) => dt.timestamp(),
            Self::Tz(dt) => dt.timestamp(),
        }
    }

    pub fn to_rfc3339(&self) -> String {
        match self {
            Self::Local(dt) => dt.to_rfc3339(),
            Self::Tz(dt) => dt.to_rfc3339(),
        }
    }

    pub fn format<'a>(
        &self,
        fmt: &'a str,
    ) -> chrono::format::DelayedFormat<chrono::format::StrftimeItems<'a>> {
        match self {
            Self::Local(dt) => dt.format(fmt),
            Self::Tz(dt) => dt.format(fmt),
        }
    }
}

impl From<chrono::DateTime<chrono_tz::Tz>> for DateTime {
    fn from(dt: chrono::DateTime<chrono_tz::Tz>) -> Self {
        Self::Tz(dt)
    }
}

impl From<&chrono::DateTime<chrono_tz::Tz>> for DateTime {
    fn from(dt: &chrono::DateTime<chrono_tz::Tz>) -> Self {
        Self::Tz(*dt)
    }
}

impl From<chrono::DateTime<chrono::Local>> for DateTime {
    fn from(dt: chrono::DateTime<chrono::Local>) -> Self {
        Self::Local(dt)
    }
}

impl From<chrono::DateTime<chrono::Utc>> for DateTime {
    fn from(dt: chrono::DateTime<chrono::Utc>) -> Self {
        Self::Tz(dt.with_timezone(&chrono_tz::UTC))
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local(dt) => dt.fmt(f),
            Self::Tz(dt) => dt.fmt(f),
        }
    }
}
