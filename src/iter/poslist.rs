use super::RRuleIterError;
use crate::datetime::from_ordinal;
use crate::datetime::{DateTime, Time};
use crate::iter::iterinfo::IterInfo;
use crate::utils::pymod;
use chrono_tz::Tz;

pub fn build_poslist(
    bysetpost: &Vec<isize>,
    timeset: &Vec<Time>,
    start: usize,
    end: usize,
    ii: &IterInfo,
    dayset: &Vec<Option<isize>>,
    tz: &Tz,
) -> Result<Vec<DateTime>, RRuleIterError> {
    let mut poslist = vec![];
    if timeset.is_empty() {
        // This will prevent the divide by 0 and out of bounds in this function.
        return Err(RRuleIterError("`timeset` can not be empty".to_owned()));
    }

    for pos in bysetpost {
        let daypos;
        let timepos;
        if *pos < 0 {
            daypos = (*pos as f32 / timeset.len() as f32).floor() as isize;
            timepos = pymod(*pos as isize, timeset.len() as isize);
        } else {
            daypos = ((*pos - 1) as f32 / timeset.len() as f32) as isize;
            timepos = pymod(*pos as isize - 1, timeset.len() as isize);
        }

        let mut tmp = vec![];
        for k in start..end {
            let val = dayset
                .get(k)
                .ok_or_else(|| RRuleIterError("Index out of bounds `dayset`".to_owned()))?;
            match val {
                Some(v) => tmp.push(v),
                None => (),
            }
        }

        let i;
        if daypos < 0 {
            let index = tmp.len() as isize + daypos;
            i = **tmp
                .get(index as usize)
                .ok_or_else(|| RRuleIterError("Index out of bounds `tmp`".to_owned()))?;
        } else {
            i = **tmp
                .get(daypos as usize)
                .ok_or_else(|| RRuleIterError("Index out of bounds `tmp`".to_owned()))?;
        }

        let date = from_ordinal(ii.yearordinal().unwrap() + i as i64, tz);
        // Create new Date + Time combination
        // Use Date and Timezone from `date`
        // Use Time from `timeset`.
        let time = timeset[timepos as usize].to_naive_time();
        let res = date.date().and_time(time).ok_or_else(|| {
            RRuleIterError(format!(
                "Time from `timeset` invalid `{} + {}`",
                date.date(),
                time
            ))
        })?;

        if !poslist.iter().any(|&p| p == res) {
            poslist.push(res);
        }
    }

    poslist.sort_by_key(|a| a.timestamp());

    Ok(poslist)
}
