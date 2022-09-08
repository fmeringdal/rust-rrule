use chrono::Datelike;

use super::utils::{from_ordinal, pymod};
use crate::{core::Time, iter::iterinfo::IterInfo, RRuleError};

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
pub(crate) fn build_pos_list<TZ: chrono::TimeZone>(
    by_set_pos: &[i32],
    timeset: &[Time],
    start: u64,
    end: u64,
    ii: &IterInfo<TZ>,
    dayset: &[Option<i32>],
    tz: TZ,
) -> Result<Vec<chrono::DateTime<TZ>>, RRuleError> {
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

        // Get ordinal which is UTC
        let date = from_ordinal(ii.year_ordinal().unwrap() + i64::from(i));
        // Apply timezone
        let date = tz.ymd_opt(date.year(), date.month(), date.day()).unwrap();
        // Create new Date + Time combination
        // Use Date and Timezone from `date`
        // Use Time from `timeset`.
        let time = timeset[time_pos as usize].to_naive_time();
        let res = date.and_time(time).ok_or_else(|| {
            RRuleError::new_iter_err(format!(
                "Time from `timeset` invalid `{:?} + {}`",
                date, time
            ))
        })?;

        if !pos_list.iter().any(|p| *p == res) {
            pos_list.push(res);
        }
    }

    pos_list.sort_by_key(chrono::DateTime::timestamp);

    Ok(pos_list)
}
