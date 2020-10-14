use crate::masks::Masks;
use crate::yearinfo::*;
use chrono::prelude::*;
use chrono::{DateTime, Duration};

struct RRule {}

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
    mrange: &Vec<usize>,
    wdaymask: &Vec<usize>,
    options: &ParsedOptions,
) -> MonthInfo {
    let mut result = MonthInfo {
        lastyear: year,
        lastmonth: month,
        nwdaymask: vec![],
    };

    let mut ranges: Vec<(isize, isize)> = vec![];
    if options.freq == Frequenzy::YEARLY {
        if options.bymonth.is_empty() {
            ranges = vec![(0, year as isize)];
        } else {
            for j in 0..options.bymonth.len() {
                let m = options.bymonth[j];
                ranges.push((mrange[m - 1] as isize, mrange[m] as isize))
            }
        }
    } else if options.freq == Frequenzy::MONTHLY {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let options = ParsedOptions {
            freq: Frequenzy::YEARLY,
            interval: 1,
            count: Some(10),
            until: None,
            tzid: None,
            dtstart: Utc.ymd(2020, 1, 1).and_hms(0, 0, 0),
            wkst: 0,
            bysetpos: vec![],
            bymonth: vec![],
            bymonthday: vec![],
            bynmonthday: vec![],
            byyearday: vec![],
            byweekno: vec![1],
            byweekday: vec![],
            byhour: vec![],
            byminute: vec![],
            bysecond: vec![],
            bynweekday: vec![],
        };
        let res = rebuild_month(2020, 1, 366, &vec![], &vec![], &options);
        println!("Res: {:?}", res);
        assert_eq!(2 + 2, 4);
    }
}
