use super::RRuleIterError;
use crate::options::*;
use crate::utils::pymod;

#[derive(Debug)]
pub struct MonthInfo {
    pub lastyear: isize,
    pub lastmonth: usize,
    pub nwdaymask: Vec<isize>,
}

pub fn rebuild_month(
    year: isize,
    month: usize,
    yearlen: usize,
    mrange: &[usize],
    wdaymask: &[usize],
    options: &ParsedOptions,
) -> Result<MonthInfo, RRuleIterError> {
    let mut result = MonthInfo {
        lastyear: year,
        lastmonth: month,
        nwdaymask: vec![],
    };

    // Build up `ranges`
    let mut ranges: Vec<(isize, isize)> = vec![];
    if options.freq == Frequency::Yearly {
        if options.bymonth.is_empty() {
            ranges = vec![(0, yearlen as isize - 1)];
        } else {
            for month in &options.bymonth {
                if *month == 0 {
                    return Err(RRuleIterError(
                        "Month `0` does not exists, 1-12 expected".to_owned(),
                    ));
                }
                let first = *mrange
                    .get(month - 1)
                    .ok_or_else(|| RRuleIterError("Index out of bounds `mrange`".to_owned()))?
                    as isize;
                let last = *mrange
                    .get(*month)
                    .ok_or_else(|| RRuleIterError("Index out of bounds `mrange`".to_owned()))?
                    as isize;
                ranges.push((first, last - 1))
            }
        }
    } else if options.freq == Frequency::Monthly {
        if month == 0 {
            return Err(RRuleIterError(
                "Month `0` does not exists, 1-12 expected".to_owned(),
            ));
        }
        let first = *mrange
            .get(month - 1)
            .ok_or_else(|| RRuleIterError("Index out of bounds `mrange`".to_owned()))?
            as isize;
        let last = *mrange
            .get(month)
            .ok_or_else(|| RRuleIterError("Index out of bounds `mrange`".to_owned()))?
            as isize;
        ranges.push((first, last - 1));
    }

    if ranges.is_empty() {
        return Ok(result);
    }

    // Weekly frequency won't get here, so we may not
    // care about cross-year weekly periods.
    result.nwdaymask = vec![0; yearlen];

    // Loop over `ranges`
    for (first, last) in ranges {
        for bynweekday in &options.bynweekday {
            let mut i: isize;
            let wday = *bynweekday
                .get(0)
                .ok_or_else(|| RRuleIterError("Index out of bounds `bynweekday`".to_owned()))?;
            let n = *bynweekday
                .get(1)
                .ok_or_else(|| RRuleIterError("Index out of bounds `bynweekday`".to_owned()))?;
            if n < 0 {
                i = last + (n + 1) * 7;
                let wday_from_mask: isize = *wdaymask
                    .get(i as usize)
                    .ok_or_else(|| RRuleIterError("Index out of bounds `wdaymask`".to_owned()))?
                    as isize;
                i -= pymod(wday_from_mask - wday, 7);
            } else {
                i = first + (n - 1) * 7;
                let wday_from_mask: isize = *wdaymask
                    .get(i as usize)
                    .ok_or_else(|| RRuleIterError("Index out of bounds `wdaymask`".to_owned()))?
                    as isize;
                i += pymod(7 - wday_from_mask + wday, 7);
            }
            if first <= i && i <= last {
                result.nwdaymask[i as usize] = 1;
            }
        }
    }

    Ok(result)
}
