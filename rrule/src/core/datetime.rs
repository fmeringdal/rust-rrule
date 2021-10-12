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
