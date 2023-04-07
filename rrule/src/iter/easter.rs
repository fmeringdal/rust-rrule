#![cfg(feature = "by-easter")]

use super::masks::MASKS;
use super::utils::is_leap_year;

/// Implementation of the Gauss Easter Algorithm.
#[allow(clippy::many_single_char_names)]
pub(crate) fn easter(year: i32, offset: i16) -> i32 {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = usize::try_from((h + l - 7 * m + 114) / 31)
        .expect("the algorithm makes sure this is between 1-12 which is covered by usize");
    let day = ((h + l - 7 * m + 114) % 31) + 1 + i32::from(offset);

    let month_range_mask = if is_leap_year(year) {
        &MASKS.month_366_range
    } else {
        &MASKS.month_365_range
    };

    i32::from(month_range_mask[month - 1]) + day - 1
}

#[cfg(test)]
mod test_easter_masks {
    use super::*;

    #[test]
    fn easter_mask() {
        let easter_day = easter(1997, 0);
        assert_eq!(easter_day, 88);
        let easter_day = easter(1998, 0);
        assert_eq!(easter_day, 101);
        let easter_day = easter(1999, 0);
        assert_eq!(easter_day, 93);
        let easter_day = easter(2000, 0);
        assert_eq!(easter_day, 113);
    }
}
