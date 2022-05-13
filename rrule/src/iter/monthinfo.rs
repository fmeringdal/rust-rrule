use super::utils::pymod;
use crate::{Frequency, NWeekday, RRule, RRuleError};

#[derive(Debug, Clone)]
pub(crate) struct MonthInfo {
    pub last_year: i32,
    pub last_month: u8,
    // TODO: Only ever set to 0 and 1
    pub neg_weekday_mask: Vec<i8>,
}

pub(crate) fn rebuild_month(
    year: i32,
    month: u8,
    year_len: u32,
    month_range: &[u16],
    weekday_mask: &[u8],
    rrule: &RRule,
) -> Result<MonthInfo, RRuleError> {
    let mut result = MonthInfo {
        last_year: year,
        last_month: month,
        neg_weekday_mask: vec![],
    };

    // Build up `ranges`
    let mut ranges: Vec<(isize, isize)> = vec![];
    if rrule.freq == Frequency::Yearly {
        if rrule.by_month.is_empty() {
            ranges = vec![(0, year_len as isize - 1)];
        } else {
            for month in &rrule.by_month {
                if month == &0 {
                    return Err(RRuleError::new_iter_err(
                        "Month `0` does not exist, 1-12 expected",
                    ));
                }
                let first = *month_range
                    .get(*month as usize - 1)
                    .ok_or_else(|| RRuleError::new_iter_err("Index out of bounds `month_range`"))?
                    as isize;
                let last = *month_range
                    .get(*month as usize)
                    .ok_or_else(|| RRuleError::new_iter_err("Index out of bounds `month_range`"))?
                    as isize;
                ranges.push((first, last - 1))
            }
        }
    } else if rrule.freq == Frequency::Monthly {
        if month == 0 {
            return Err(RRuleError::new_iter_err(
                "Month `0` does not exist, 1-12 expected",
            ));
        }
        let first = *month_range
            .get(month as usize - 1)
            .ok_or_else(|| RRuleError::new_iter_err("Index out of bounds `month_range`"))?
            as isize;
        let last = *month_range
            .get(month as usize)
            .ok_or_else(|| RRuleError::new_iter_err("Index out of bounds `month_range`"))?
            as isize;
        ranges.push((first, last - 1));
    }

    if ranges.is_empty() {
        return Ok(result);
    }

    // Weekly frequency won't get here, so we may not
    // care about cross-year weekly periods.
    result.neg_weekday_mask = vec![0; year_len as usize];

    // Loop over `ranges`
    for (first, last) in ranges {
        for by_weekday in &rrule.by_weekday {
            // Only check Nth occurrences here
            if let NWeekday::Nth(number, weekday) = by_weekday {
                let mut i: isize;
                if *number < 0 {
                    i = last + (*number as isize + 1) * 7;
                    let weekday_from_mask: isize =
                        *weekday_mask.get(i as usize).ok_or_else(|| {
                            RRuleError::new_iter_err("Index out of bounds `weekday_from_mask`")
                        })? as isize;
                    i -= pymod(
                        weekday_from_mask - weekday.num_days_from_monday() as isize,
                        7,
                    );
                } else {
                    i = first + (*number as isize - 1) * 7;
                    let weekday_from_mask: isize =
                        *weekday_mask.get(i as usize).ok_or_else(|| {
                            RRuleError::new_iter_err("Index out of bounds `weekday_from_mask`")
                        })? as isize;
                    i += pymod(
                        7 - weekday_from_mask + weekday.num_days_from_monday() as isize,
                        7,
                    );
                }
                if first <= i && i <= last {
                    result.neg_weekday_mask[i as usize] = 1;
                }
            }
        }
    }

    Ok(result)
}
