use std::ops;

use crate::core::{duration_from_midnight, Tz};
use chrono::{Date, NaiveTime, TimeZone, Utc};

const UTC: Tz = Tz::UTC;

const DAY_SECS: i64 = 24 * 60 * 60;

/// Converts number of days since unix epoch back to `DataTime`
pub(crate) fn from_ordinal(ordinal: i64) -> chrono::DateTime<crate::Tz> {
    let timestamp = ordinal * DAY_SECS;
    UTC.timestamp(timestamp, 0)
}

/// Returns number of days since unix epoch (rounded down)
pub(crate) fn days_since_unix_epoch(date: &chrono::DateTime<Utc>) -> i64 {
    date.timestamp() / DAY_SECS
}

/// Returns true if given year is a leap year
pub(crate) fn is_leap_year(year: i32) -> bool {
    // Every 4 years, and every 100 years
    // but not if dividable by 400.
    year.trailing_zeros() >= 2 && (year % 25 != 0 || year.trailing_zeros() >= 4)
}

/// Returns amount of days in year,
/// So 365 or 366 depending on the year
pub(crate) fn get_year_len(year: i32) -> u16 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

pub(crate) trait DifferentSigns {
    fn different_sign(a: Self, b: Self) -> bool;
}

macro_rules! impl_different_signs {
    ($($ty:ty),*) => {
        $(
        impl DifferentSigns for $ty {
            fn different_sign(a: Self, b: Self) -> bool {
                a > 0 && b < 0 || a < 0 && b > 0
            }
        })*
    };
    (@unsigned, $($ty:ty),*) => {
        $(
        impl DifferentSigns for $ty {
            fn different_sign(_: Self, _: Self) -> bool {
                false
            }
        })*
    };
}
impl_different_signs!(isize, i32, i16);
impl_different_signs!(@unsigned, usize, u32, u16, u8);

pub(crate) fn pymod<T>(a: T, b: T) -> T
where
    T: DifferentSigns + Copy + ops::Rem<Output = T> + ops::Add<Output = T>,
{
    let r = a % b;
    // If r and b differ in sign, add b to wrap the result to the correct sign.
    if T::different_sign(r, b) {
        r + b
    } else {
        r
    }
}

pub(crate) fn add_time_to_date<TZ: chrono::TimeZone>(
    date: Date<TZ>,
    time: NaiveTime,
) -> Option<chrono::DateTime<TZ>> {
    if let Some(dt) = date.and_time(time) {
        return Some(dt);
    }
    // If the day is a daylight saving time, the above code might now work and we
    // can try to get a valid datetime by adding the `time` as a duration instead.
    let dt = date.and_hms_opt(0, 0, 0)?;
    let day_duration = duration_from_midnight(time);
    dt.checked_add_signed(day_duration)
}

#[cfg(test)]
mod test {

    use chrono::Duration;

    use super::*;

    #[test]
    fn python_mod() {
        assert_eq!(pymod(2, -3), -1);
        assert_eq!(pymod(-2, 3), 1);
        assert_eq!(pymod(-2, -3), -2);
        assert_eq!(pymod(-3, -3), 0);
        assert_eq!(pymod(0, 3), 0);
        assert_eq!(pymod(1, 3), 1);
        assert_eq!(pymod(2, 3), 2);
        assert_eq!(pymod(3, 3), 0);
        assert_eq!(pymod(4, 3), 1);
        assert_eq!(pymod(6, 3), 0);
        assert_eq!(pymod(-6, 3), 0);
        assert_eq!(pymod(-6, -3), 0);
        assert_eq!(pymod(6, -3), 0);
    }

    #[test]
    fn leap_year() {
        let tests = [
            (2015, false),
            (2016, true),
            (2017, false),
            (2018, false),
            (2019, false),
            (2020, true),
            (2021, false),
        ];

        for (year, expected_output) in tests {
            let res = is_leap_year(year);
            assert_eq!(res, expected_output);
        }
    }

    #[test]
    fn year_length() {
        let tests = [(2015, 365), (2016, 366)];

        for (year, expected_output) in tests {
            let res = get_year_len(year);
            assert_eq!(res, expected_output);
        }
    }

    #[test]
    fn adds_time_to_date() {
        const AMERICA_NEW_YORK: Tz = Tz::America__New_York;
        const AMERICA_VANCOUVER: Tz = Tz::America__Vancouver;

        let tests = [
            (
                UTC.ymd(2017, 1, 1),
                NaiveTime::from_hms(1, 15, 30),
                Some(UTC.ymd(2017, 1, 1).and_hms(1, 15, 30)),
            ),
            (
                AMERICA_VANCOUVER.ymd(2021, 3, 14),
                NaiveTime::from_hms(2, 22, 10),
                Some(
                    AMERICA_VANCOUVER.ymd(2021, 3, 14).and_hms(0, 0, 0)
                        + Duration::hours(2)
                        + Duration::minutes(22)
                        + Duration::seconds(10),
                ),
            ),
            (
                AMERICA_NEW_YORK.ymd(1997, 10, 26),
                NaiveTime::from_hms(9, 0, 0),
                Some(AMERICA_NEW_YORK.ymd(1997, 10, 26).and_hms(9, 0, 0)),
            ),
        ];

        for (date, time, expected_output) in tests {
            let res = add_time_to_date(date, time);
            assert_eq!(res, expected_output);
        }
    }
}
