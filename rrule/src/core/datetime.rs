use super::timezone::Tz;
use chrono::{Datelike, Duration, NaiveTime, Timelike};

pub(crate) fn duration_from_midnight(time: NaiveTime) -> Duration {
    Duration::hours(i64::from(time.hour()))
        + Duration::minutes(i64::from(time.minute()))
        + Duration::seconds(i64::from(time.second()))
}

pub(crate) fn get_month(dt: &chrono::DateTime<Tz>) -> u8 {
    u8::try_from(dt.month()).expect("month is between 1-12 which is covered by u8")
}

pub(crate) fn get_day(dt: &chrono::DateTime<Tz>) -> i8 {
    i8::try_from(dt.day()).expect("day is between 1-31 which is covered by i8")
}

pub(crate) fn get_hour(dt: &chrono::DateTime<Tz>) -> u8 {
    u8::try_from(dt.hour()).expect("hour is between 0-23 which is covered by u8")
}

pub(crate) fn get_minute(dt: &chrono::DateTime<Tz>) -> u8 {
    u8::try_from(dt.minute()).expect("minute is between 0-59 which is covered by u8")
}

pub(crate) fn get_second(dt: &chrono::DateTime<Tz>) -> u8 {
    u8::try_from(dt.second()).expect("second is between 0-59 which is covered by u8")
}

/// Generates an iCalendar date-time string format with the prefix symbols.
/// Like: `:19970714T173000Z` or `;TZID=America/New_York:19970714T133000`
/// ref: <https://tools.ietf.org/html/rfc5545#section-3.3.5>
pub(crate) fn datetime_to_ical_format(dt: &chrono::DateTime<Tz>) -> String {
    let mut tz_prefix = String::new();
    let mut tz_postfix = String::new();
    let tz = dt.timezone();
    match tz {
        Tz::Local(_) => {}
        Tz::Tz(tz) => match tz {
            chrono_tz::UTC => {
                tz_postfix = "Z".to_string();
            }
            tz => {
                tz_prefix = format!(";TZID={}", tz.name());
            }
        },
    }

    let dt = dt.format("%Y%m%dT%H%M%S");
    format!("{}:{}{}", tz_prefix, dt, tz_postfix)
}
