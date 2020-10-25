use chrono::prelude::*;

pub fn easter(y: isize, offset: isize) -> Vec<isize> {
    let a = y % 19;
    let b = (y as f32 / 100_f32).floor() as isize;
    let c = y % 100;
    let d = (b as f32 / 4_f32).floor() as isize;
    let e = b % 4;
    let f = ((b + 8) as f32 / 25_f32).floor() as isize;
    let g = ((b - f + 1) as f32 / 3_f32).floor() as isize;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = (c as f32 / 4_f32).floor() as isize;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = ((a + 11 * h + 22 * l) as f32 / 451_f32).floor() as isize;
    let month = ((h + l - 7 * m + 114) as f32 / 31_f32).floor() as u32;
    let day = ((h + l - 7 * m + 114) % 31) + 1;
    let date = Utc
        .ymd(y as i32, month, (day + offset) as u32)
        .and_hms(0, 0, 0);
    let year_start = Utc.ymd(y as i32, 1, 1).and_hms(0, 0, 0);
    return vec![
        ((date.timestamp() - year_start.timestamp()) as f32 / (60 * 60 * 24) as f32).ceil()
            as isize,
    ];
}

#[cfg(test)]
mod test_easter_masks {
    use super::*;

    #[test]
    fn easter_mask() {
        let mask = easter(1997, 0);
        assert_eq!(mask[0], 88);
        let mask = easter(1998, 0);
        assert_eq!(mask[0], 101);
        let mask = easter(1999, 0);
        assert_eq!(mask[0], 93);
        let mask = easter(2000, 0);
        assert_eq!(mask[0], 113);
    }
}
