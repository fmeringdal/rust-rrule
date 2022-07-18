use chrono::{Datelike, Duration, NaiveTime, Timelike};
use chrono_tz::Tz;

pub(crate) type DateTime = chrono::DateTime<Tz>;

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
    if tz == Tz::UTC {
        tz_postfix = "Z".to_string();
    } else {
        tz_prefix = format!(";TZID={}", tz.name());
    };

    let dt = dt.format("%Y%m%dT%H%M%S");
    format!("{}:{}{}", tz_prefix, dt, tz_postfix)
}
