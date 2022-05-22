#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
#![cfg(feature = "by-easter")]

use crate::RRuleError;
use chrono::{offset::LocalResult, TimeZone, Utc};

// TODO remove this clippy `allow` flag and give variables proper names.
#[allow(clippy::many_single_char_names)]
pub(crate) fn easter(y: i32, offset: i16) -> Result<Vec<isize>, RRuleError> {
    let a = y % 19;
    let b = (y as f32 / 100_f32).floor() as i32;
    let c = y % 100;
    let d = (b as f32 / 4_f32).floor() as i32;
    let e = b % 4;
    let f = ((b + 8) as f32 / 25_f32).floor() as i32;
    let g = ((b - f + 1) as f32 / 3_f32).floor() as i32;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = (c as f32 / 4_f32).floor() as i32;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = ((a + 11 * h + 22 * l) as f32 / 451_f32).floor() as i32;
    let month = ((h + l - 7 * m + 114) as f32 / 31_f32).floor() as u32;
    let day = ((h + l - 7 * m + 114) % 31) + 1;

    let year = y as i32;
    let day = (day + i32::from(offset)) as u32;
    let date = match Utc.ymd_opt(year, month, day) {
        LocalResult::None => Err(RRuleError::new_iter_err(format!(
            "Invalid date in UTC timezone: `{}/{}/{}`",
            year, month, day
        ))),
        LocalResult::Single(date) => Ok(date),
        LocalResult::Ambiguous(date1, date2) => Err(RRuleError::new_iter_err(format!(
            "Invalid date in UTC timezone: `{}/{}/{}` \
                this date is ambiguous it can be: `{}` or `{}`",
            year, month, day, date1, date2
        ))),
    }?
    .and_hms(0, 0, 0);
    let year_start = Utc.ymd(year, 1, 1).and_hms(0, 0, 0);
    Ok(vec![
        ((date.timestamp() - year_start.timestamp()) as f32 / (60 * 60 * 24) as f32).ceil()
            as isize,
    ])
}

#[cfg(test)]
mod test_easter_masks {
    use super::*;

    #[test]
    fn easter_mask() {
        let mask = easter(1997, 0).unwrap();
        assert_eq!(mask[0], 88);
        let mask = easter(1998, 0).unwrap();
        assert_eq!(mask[0], 101);
        let mask = easter(1999, 0).unwrap();
        assert_eq!(mask[0], 93);
        let mask = easter(2000, 0).unwrap();
        assert_eq!(mask[0], 113);
    }
}
