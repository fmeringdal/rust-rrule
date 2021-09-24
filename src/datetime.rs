use chrono::prelude::*;
use chrono::Utc;
use chrono_tz::Tz;

pub type DateTime = chrono::DateTime<Tz>;

/// Convert number of days since unix epoch back to `DataTime`
pub fn from_ordinal(ordinal: i64, tz: &Tz) -> DateTime {
    let timestamp = ordinal * 24 * 60 * 60;
    tz.timestamp(timestamp as i64, 0)
}

/// Return number of days since unix epoch (rounded down)
pub fn to_ordinal(date: &chrono::DateTime<Utc>) -> i64 {
    // Number of seconds since Unix epoch
    // sec / 60 = min
    // min / 60 = hours
    // hours / 24 = days
    date.timestamp() / 60 / 60 / 24
}

/// Return true if given year is a leap year
pub fn is_leap_year(year: i32) -> bool {
    // Every 4 years, and every 100 years
    // but not if dividable by 400.
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Return amount of days in year,
/// So 365 or 366 depending on the year
pub fn get_year_len(year: i32) -> u16 {
    if is_leap_year(year) {
        return 366;
    }
    365
}

/// Get day of the week,
/// Mondays = 0,
/// Sunday = 6
pub fn get_weekday_val(wk: &Weekday) -> u8 {
    match wk {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    }
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
