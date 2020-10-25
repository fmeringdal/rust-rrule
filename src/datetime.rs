use chrono::prelude::*;
use chrono_tz::Tz;


pub type DTime = DateTime<Tz>;

pub fn from_ordinal(ordinal: isize) -> DateTime<Utc> {
    let timestamp = ordinal * 24 * 60 * 60;
    let naive = NaiveDateTime::from_timestamp(timestamp as i64, 0);
    DateTime::from_utc(naive, Utc)
}

pub fn to_ordinal(date: &DateTime<Utc>) -> isize {
    (date.timestamp() / 60 / 60 / 24) as isize
}

#[derive(Debug)]
pub struct Time {
    pub hour: usize,
    pub minute: usize,
    pub second: usize,
    pub millisecond: usize,
}

impl Time {
    pub fn new(hour: usize, minute: usize, second: usize, millisecond: usize) -> Self {
        Self {
            hour,
            minute,
            second,
            millisecond,
        }
    }

    pub fn time(&self) -> usize {
        (self.hour * 60 * 60 + self.minute * 60 + self.second) * 1000 + self.millisecond
    }
}
