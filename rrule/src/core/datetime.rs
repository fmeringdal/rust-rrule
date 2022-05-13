use chrono::{Duration, NaiveTime};
use chrono_tz::Tz;

pub(crate) type DateTime = chrono::DateTime<Tz>;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u16,
}

impl Time {
    pub fn new(hour: u8, minute: u8, second: u8, millisecond: u16) -> Self {
        Self {
            hour,
            minute,
            second,
            millisecond,
        }
    }

    pub fn time(&self) -> u64 {
        (self.hour as u64 * 60 * 60 + self.minute as u64 * 60 + self.second as u64) * 1000
            + self.millisecond as u64
    }

    pub fn to_naive_time(self) -> NaiveTime {
        NaiveTime::from_hms(self.hour as u32, self.minute as u32, self.second as u32)
    }

    pub fn duration_from_midnight(&self) -> Duration {
        Duration::hours(self.hour as i64)
            + Duration::minutes(self.minute as i64)
            + Duration::seconds(self.second as i64)
    }
}

/// Generates an iCalendar date-time string format with the prefix symbols.
/// Like: ":19970714T173000Z" or ";TZID=America/New_York:19970714T133000"
/// ref: https://tools.ietf.org/html/rfc5545#section-3.3.5
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
