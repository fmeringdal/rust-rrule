use super::utils::{from_ordinal, pymod};
use crate::{core::DateTime, iter::iterinfo::IterInfo, RRuleError};
use chrono::{Datelike, LocalResult, NaiveTime, TimeZone};
use chrono_tz::Tz;

pub(crate) fn build_pos_list(
    by_set_pos: &[i32],
    dayset: &[usize],
    timeset: &[NaiveTime],
    ii: &IterInfo,
    tz: Tz,
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
    let timeset_len_int = i32::try_from(timeset.len())
        .map_err(|_| RRuleError::new_iter_err("timeset length is too large"))?;
    for pos in by_set_pos {
        let day_pos;
        let time_pos;
        if *pos < 0 {
            day_pos = (f64::from(*pos) / timeset_len_float).floor() as isize;
            time_pos = usize::try_from(pymod(*pos, timeset_len_int))
                .expect("modulus is a positive number and within range of usize");
        } else {
            day_pos = ((f64::from(*pos) - 1.) / timeset_len_float) as isize;
            time_pos = usize::try_from(pymod(pos - 1, timeset_len_int))
                .expect("modulus is a positive number and within range of usize");
        }

        let day_idx = if day_pos < 0 {
            let dayset_len = isize::try_from(dayset.len())
                .expect("dayset is controlled by us and is never more than isize::MAX");
            let index = dayset_len + day_pos;
            usize::try_from(index)
                .map_err(|_| RRuleError::new_iter_err("Index out of bounds `tmp`"))?
        } else {
            usize::try_from(day_pos)
                .map_err(|_| RRuleError::new_iter_err("Index out of bounds `tmp`"))?
        };
        let day = dayset
            .get(day_idx)
            .ok_or_else(|| RRuleError::new_iter_err("Index out of bounds `tmp`"))?;
        let day = i64::try_from(*day)
            .expect("dayset is controlled by us and all elements are within range of i64");

        // Get ordinal which is UTC
        let date = from_ordinal(ii.year_ordinal() + day);
        // Apply timezone
        let date = match tz.ymd_opt(date.year(), date.month(), date.day()) {
            LocalResult::Single(date) => date,
            e => {
                return Err(RRuleError::new_iter_err(format!(
                    "Invalid date encountered. Error: {:?}",
                    e
                )))
            }
        };
        // Create new Date + Time combination
        // Use Date and Timezone from `date`
        // Use Time from `timeset`.
        let time = timeset[time_pos];
        let res = date.and_time(time).ok_or_else(|| {
            RRuleError::new_iter_err(format!("Time from `timeset` invalid `{} + {}`", date, time))
        })?;

        if !pos_list.contains(&res) {
            pos_list.push(res);
        }
    }

    pos_list.sort_by_key(DateTime::timestamp);

    Ok(pos_list)
}
