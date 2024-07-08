use std::ops;

use crate::core::{duration_from_midnight, Tz};
use chrono::{NaiveDate, NaiveTime, Utc};

const DAY_SECS: i64 = 24 * 60 * 60;

/// Converts number of days since unix epoch to a (naive) date.
pub(crate) fn date_from_ordinal(ordinal: i64) -> NaiveDate {
    chrono::DateTime::<Utc>::from_timestamp(ordinal * DAY_SECS, 0)
        .unwrap()
        .date_naive()
}

/// Returns number of days since unix epoch (rounded down)
pub(crate) fn days_since_unix_epoch(date: &chrono::DateTime<Utc>) -> i64 {
    date.timestamp() / DAY_SECS
}

/// Returns true if given year is a leap year
pub(crate) fn is_leap_year(year: i32) -> bool {
    // Every 4 years, and every 100 years,
    // but not if dividable by 400.
    year.trailing_zeros() >= 2 && (year % 25 != 0 || year.trailing_zeros() >= 4)
}

/// Returns number of days in year,
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

pub(crate) fn add_time_to_date(
    tz: Tz,
    date: NaiveDate,
    time: NaiveTime,
) -> Option<chrono::DateTime<Tz>> {
    if let Some(dt) = date.and_time(time).and_local_timezone(tz).single() {
        return Some(dt);
    }
    // If the day is a daylight saving time, the above code might not work, and we
    // can try to get a valid datetime by adding the `time` as a duration instead.
    let dt = date.and_hms_opt(0, 0, 0)?.and_local_timezone(tz).single()?;
    let day_duration = duration_from_midnight(time);
    dt.checked_add_signed(day_duration)
}

#[cfg(test)]
mod test {

    use chrono::{Duration, TimeZone};

    use super::*;

    #[test]
    fn naive_date_from_ordinal() {
        let tests = [
            (-1, NaiveDate::from_ymd_opt(1969, 12, 31).unwrap()),
            (0, NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
            (1, NaiveDate::from_ymd_opt(1970, 1, 2).unwrap()),
            (10, NaiveDate::from_ymd_opt(1970, 1, 11).unwrap()),
            (365, NaiveDate::from_ymd_opt(1971, 1, 1).unwrap()),
            (19877, NaiveDate::from_ymd_opt(2024, 6, 3).unwrap()),
        ];

        for (days, expected) in tests {
            assert_eq!(date_from_ordinal(days), expected, "seconds: {}", days);
        }
    }

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
        let tests = [
            (
                Tz::UTC,
                NaiveDate::from_ymd_opt(2017, 1, 1).unwrap(),
                NaiveTime::from_hms_opt(1, 15, 30).unwrap(),
                Some(Tz::UTC.with_ymd_and_hms(2017, 1, 1, 1, 15, 30).unwrap()),
            ),
            (
                Tz::America__Vancouver,
                NaiveDate::from_ymd_opt(2021, 3, 14).unwrap(),
                NaiveTime::from_hms_opt(2, 22, 10).unwrap(),
                Some(
                    Tz::America__Vancouver
                        .with_ymd_and_hms(2021, 3, 14, 0, 0, 0)
                        .unwrap()
                        + Duration::hours(2)
                        + Duration::minutes(22)
                        + Duration::seconds(10),
                ),
            ),
            (
                Tz::America__New_York,
                NaiveDate::from_ymd_opt(1997, 10, 26).unwrap(),
                NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                Some(
                    Tz::America__New_York
                        .with_ymd_and_hms(1997, 10, 26, 9, 0, 0)
                        .unwrap(),
                ),
            ),
        ];

        for (tz, date, time, expected_output) in tests {
            let res = add_time_to_date(tz, date, time);
            assert_eq!(res, expected_output);
        }
    }
}
