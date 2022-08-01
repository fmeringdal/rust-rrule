use super::utils::{add_time_to_date, from_ordinal, pymod};
use crate::{
    core::{DateTime, RRuleTimeZone},
    RRuleError,
};
use chrono::NaiveTime;

pub(crate) fn build_pos_list(
    by_set_pos: &[i32],
    dayset: &[usize],
    timeset: &[NaiveTime],
    year_ordinal: i64,
    tz: RRuleTimeZone,
) -> Result<Vec<DateTime>, RRuleError> {
    let mut pos_list = vec![];
    if timeset.is_empty() {
        // This will prevent the divide by 0 and out of bounds in this function.
        return Err(RRuleError::new_iter_err("`timeset` can not be empty"));
    }

    let timeset_len = u32::try_from(timeset.len())
        .map_err(|_| RRuleError::new_iter_err("timeset length is too large"))?;
    let timeset_len_float = f64::try_from(timeset_len)
        .map_err(|_| RRuleError::new_iter_err("timeset length is too large"))?;
    let timeset_len_int = i32::try_from(timeset_len)
        .map_err(|_| RRuleError::new_iter_err("timeset length is too large"))?;
    for pos in by_set_pos {
        let pos = if *pos > 0 {
            pos.checked_sub(1)
                .ok_or_else(|| RRuleError::new_iter_err("invalid BYSETPOS values encountered"))?
        } else {
            *pos
        };
        let day_pos = (f64::from(pos) / timeset_len_float).floor() as isize;
        let time_pos = usize::try_from(pymod(pos, timeset_len_int))
            .expect("modulus is a positive number and within range of usize");

        let day_idx = if day_pos < 0 {
            let dayset_len = isize::try_from(dayset.len())
                .expect("dayset is controlled by us and is never more than isize::MAX");
            let index = dayset_len + day_pos;
            usize::try_from(index)
                .map_err(|_| RRuleError::new_iter_err("`day_pos` overflowed `dayset_len`"))?
        } else {
            usize::try_from(day_pos).expect("a non-negative isize fits within a usize")
        };
        let day = dayset
            .get(day_idx)
            .ok_or_else(|| RRuleError::new_iter_err("Computed day index is not in the `dayset`"))?;
        let day = i64::try_from(*day)
            .expect("dayset is controlled by us and all elements are within range of i64");

        // Get ordinal which is UTC and apply timezone
        let date = from_ordinal(year_ordinal + day);
        // Create new Date + Time combination
        // Use Date and Timezone from `date`
        // Use Time from `timeset`.
        let time = timeset[time_pos];
        let res = match tz.datetime(date.year(), date.month(), date.day(), time) {
            Some(date) => date,
            None => continue,
        };

        if !pos_list.contains(&res) {
            pos_list.push(res);
        }
    }

    pos_list.sort();

    Ok(pos_list)
}
