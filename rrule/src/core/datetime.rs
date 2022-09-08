use chrono::{Duration, NaiveTime};
use chrono_tz::Tz;

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

    pub fn time(self) -> u64 {
        (u64::from(self.hour) * 60 * 60 + u64::from(self.minute) * 60 + u64::from(self.second))
            * 1000
            + u64::from(self.millisecond)
    }

    pub fn to_naive_time(self) -> NaiveTime {
        NaiveTime::from_hms(
            u32::from(self.hour),
            u32::from(self.minute),
            u32::from(self.second),
        )
    }

    pub fn duration_from_midnight(self) -> Duration {
        Duration::hours(i64::from(self.hour))
            + Duration::minutes(i64::from(self.minute))
            + Duration::seconds(i64::from(self.second))
    }
}

/// Generates an iCalendar date-time string format with the prefix symbols.
/// Like: `:19970714T173000Z` or `;TZID=America/New_York:19970714T133000`
/// ref: <https://tools.ietf.org/html/rfc5545#section-3.3.5>
pub(crate) fn datetime_to_ical_format(dt: &chrono::DateTime<chrono_tz::Tz>) -> String {
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
