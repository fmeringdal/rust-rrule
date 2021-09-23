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
) -> MonthInfo {
    let mut result = MonthInfo {
        lastyear: year,
        lastmonth: month,
        nwdaymask: vec![],
    };

    let mut ranges: Vec<(isize, isize)> = vec![];
    if options.freq == Frequency::Yearly {
        if options.bymonth.is_empty() {
            ranges = vec![(0, yearlen as isize)];
        } else {
            for j in 0..options.bymonth.len() {
                let m = options.bymonth[j];
                ranges.push((mrange[m - 1] as isize, mrange[m] as isize))
            }
        }
    } else if options.freq == Frequency::Monthly {
        ranges.push((mrange[month - 1] as isize, mrange[month] as isize));
    }

    if ranges.is_empty() {
        return result;
    }

    // Weekly frequency won't get here, so we may not
    // care about cross-year weekly periods.
    result.nwdaymask = vec![0; yearlen];

    for j in 0..ranges.len() {
        let rang = ranges[j];
        let first = rang.0;
        let last = rang.1 - 1;

        for k in 0..options.bynweekday.len() {
            let mut i: isize;
            let wday = options.bynweekday[k][0];
            let n = options.bynweekday[k][1];
            if n < 0 {
                i = last + (n + 1) * 7;
                i -= pymod(wdaymask[i as usize] as isize - wday, 7);
            } else {
                i = first + (n - 1) * 7;
                i += pymod(7 - wdaymask[i as usize] as isize + wday, 7);
            }
            if first <= i && i <= last {
                result.nwdaymask[i as usize] = 1;
            }
        }
    }

    result
}
