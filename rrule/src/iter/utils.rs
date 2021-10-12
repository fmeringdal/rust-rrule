use crate::core::DateTime;
use chrono::{TimeZone, Utc};
use chrono_tz::Tz;

/// Convert number of days since unix epoch back to `DataTime`
pub(crate) fn from_ordinal(ordinal: i64, tz: &Tz) -> DateTime {
    let timestamp = ordinal * 24 * 60 * 60;
    tz.timestamp(timestamp as i64, 0)
}

/// Return number of days since unix epoch (rounded down)
pub(crate) fn to_ordinal(date: &chrono::DateTime<Utc>) -> i64 {
    // Number of seconds since Unix epoch
    // sec / 60 = min
    // min / 60 = hours
    // hours / 24 = days
    // TODO can be replaced with `ordinal` or `ordinal0`
    // https://docs.rs/chrono/0.4.19/chrono/trait.Datelike.html#tymethod.ordinal
    date.timestamp() / 60 / 60 / 24
}

/// Return true if given year is a leap year
pub(crate) fn is_leap_year(year: i32) -> bool {
    // Every 4 years, and every 100 years
    // but not if dividable by 400.
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Return amount of days in year,
/// So 365 or 366 depending on the year
pub(crate) fn get_year_len(year: i32) -> u16 {
    if is_leap_year(year) {
        return 366;
    }
    365
}

pub(crate) fn pymod(a: isize, b: isize) -> isize {
    let r = a % b;
    // If r and b differ in sign, add b to wrap the result to the correct sign.
    if (r > 0 && b < 0) || (r < 0 && b > 0) {
        return r + b;
    }
    r
}

pub(crate) fn includes<T>(v: &[T], el: &T) -> bool
where
    T: PartialEq,
{
    v.iter().any(|ve| ve == el)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn python_mod() {
        assert_eq!(pymod(2, -3), -1);
        assert_eq!(pymod(-2, 3), 1);
        assert_eq!(pymod(-2, -3), -2);
        assert_eq!(pymod(-3, -3), 0);
        assert_eq!(pymod(3, 3), 0);
        assert_eq!(pymod(2, 3), 2);
        assert_eq!(pymod(4, 3), 1);
        assert_eq!(pymod(3, 3), 0);
        assert_eq!(pymod(6, 3), 0);
        assert_eq!(pymod(-6, 3), 0);
        assert_eq!(pymod(-6, -3), 0);
        assert_eq!(pymod(6, -3), 0);
    }

    #[test]
    fn includes_works() {
        assert!(!includes(&vec![], &0));
        assert!(includes(&vec![1], &1));
        assert!(includes(&vec![1, 2, 3, 4], &3));
        assert!(!includes(&vec![1, 2, 3, 4], &5));
    }
}
