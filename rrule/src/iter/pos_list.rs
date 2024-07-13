use super::utils::{add_time_to_date, date_from_ordinal, pymod};
use crate::core::Tz;
use chrono::NaiveTime;

pub(crate) fn build_pos_list(
    by_set_pos: &[i32],
    dayset: &[usize],
    timeset: &[NaiveTime],
    year_ordinal: i32,
    tz: Tz,
) -> Vec<chrono::DateTime<Tz>> {
    let mut pos_list = vec![];

    if timeset.is_empty() {
        return vec![];
    }

    let timeset_len = u32::try_from(timeset.len())
        .expect("timeset length is a maximum of 24 * 60 * 60 which is covered by u32");
    let timeset_len_float = f64::from(timeset_len);
    let timeset_len_int = i32::try_from(timeset_len)
        .expect("timeset length is a maximum of 24 * 60 * 60 which is covered by i32");
    for pos in by_set_pos {
        let pos = if *pos > 0 { pos - 1 } else { *pos };
        let day_pos = (f64::from(pos) / timeset_len_float).floor() as isize;
        let time_pos = usize::try_from(pymod(pos, timeset_len_int))
            .expect("modulus is a positive number and within the range of usize");

        let day_idx = if day_pos < 0 {
            let dayset_len = isize::try_from(dayset.len())
                .expect("dayset is controlled by us and is never more than isize::MAX");
            let index = dayset_len + day_pos;
            match usize::try_from(index) {
                Ok(day_idx) => day_idx,
                Err(_) => continue,
            }
        } else {
            usize::try_from(day_pos).expect("a non-negative isize fits within a usize")
        };
        let day = match dayset.get(day_idx) {
            Some(day) => day,
            None => continue,
        };
        let day = i32::try_from(*day)
            .expect("dayset is controlled by us and all elements are within range of i32");

        // Get ordinal which is UTC
        let date = date_from_ordinal(year_ordinal + day);
        // Create new Date + Time combination
        // Use Time from `timeset`.
        let time = timeset[time_pos];
        let res = match add_time_to_date(tz, date, time) {
            Some(date) => date,
            None => continue,
        };

        if !pos_list.contains(&res) {
            pos_list.push(res);
        }
    }

    pos_list.sort();

    pos_list
}
