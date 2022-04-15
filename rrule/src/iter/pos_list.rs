use super::utils::{from_ordinal, pymod};
use crate::{
    core::{DateTime, Time},
    iter::iterinfo::IterInfo,
    RRuleError,
};
use chrono_tz::Tz;

pub(crate) fn build_pos_list(
    by_set_pos: &[i32],
    timeset: &[Time],
    start: u64,
    end: u64,
    ii: &IterInfo,
    dayset: &[Option<i32>],
    tz: &Tz,
) -> Result<Vec<DateTime>, RRuleError> {
    let mut pos_list = vec![];
    if timeset.is_empty() {
        // This will prevent the divide by 0 and out of bounds in this function.
        return Err(RRuleError::new_iter_err("`timeset` can not be empty"));
    }

    for pos in by_set_pos {
        let day_pos;
        let time_pos;
        if *pos < 0 {
            day_pos = (*pos as f32 / timeset.len() as f32).floor() as isize;
            time_pos = pymod(*pos as isize, timeset.len() as isize);
        } else {
            day_pos = ((*pos - 1) as f32 / timeset.len() as f32) as isize;
            time_pos = pymod(*pos as isize - 1, timeset.len() as isize);
        }

        let mut tmp = vec![];
        for k in start..end {
            let val = dayset
                .get(k as usize)
                .ok_or_else(|| RRuleError::new_iter_err("Index out of bounds `dayset`"))?;
            match val {
                Some(v) => tmp.push(v),
                None => (),
            }
        }

        let i = if day_pos < 0 {
            let index = tmp.len() as isize + day_pos;
            **tmp
                .get(index as usize)
                .ok_or_else(|| RRuleError::new_iter_err("Index out of bounds `tmp`"))?
        } else {
            **tmp
                .get(day_pos as usize)
                .ok_or_else(|| RRuleError::new_iter_err("Index out of bounds `tmp`"))?
        };

        let date = from_ordinal(ii.year_ordinal().unwrap() + i as i64, tz);
        // Create new Date + Time combination
        // Use Date and Timezone from `date`
        // Use Time from `timeset`.
        let time = timeset[time_pos as usize].to_naive_time();
        let res = date.date().and_time(time).ok_or_else(|| {
            RRuleError::new_iter_err(format!(
                "Time from `timeset` invalid `{} + {}`",
                date.date(),
                time
            ))
        })?;

        if !pos_list.iter().any(|&p| p == res) {
            pos_list.push(res);
        }
    }

    pos_list.sort_by_key(|a| a.timestamp());

    Ok(pos_list)
}
