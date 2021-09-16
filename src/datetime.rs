use chrono::prelude::*;
use chrono::Utc;
use chrono_tz::Tz;

pub type DTime = DateTime<Tz>;

pub fn from_ordinal(ordinal: isize, tz: &Tz) -> DTime {
    let timestamp = ordinal * 24 * 60 * 60;
    tz.timestamp(timestamp as i64, 0)
}

pub fn to_ordinal(date: &DateTime<Utc>) -> isize {
    (date.timestamp() / 60 / 60 / 24) as isize
}

pub fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

pub fn get_year_len(year: i32) -> usize {
    if is_leap_year(year) {
        return 366;
    }
    365
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
