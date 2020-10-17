use crate::masks::MASKS;
use crate::options::*;
use chrono::prelude::*;
use chrono::DateTime;

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn get_year_len(year: i32) -> usize {
    if is_leap_year(year) {
        return 366;
    }
    365
}

pub fn to_ordinal(date: &DateTime<Utc>) -> isize {
    (date.timestamp() / 60 / 60 / 24) as isize
}

pub fn get_weekday_val(wk: &Weekday) -> usize {
    match wk {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    }
}

pub struct BaseMasks {
    mmask: Vec<usize>,
    mdaymask: Vec<isize>,
    nmdaymask: Vec<isize>,
    wdaymask: Vec<usize>,
    mrange: Vec<usize>,
}

fn base_year_masks(year: i32) -> BaseMasks {
    let masks = MASKS.clone();
    let firstyday = Utc.ymd(year, 1, 1).and_hms_milli(0, 0, 0, 0);
    let yearlen = get_year_len(year);
    let wday = get_weekday_val(&firstyday.weekday()) as usize;

    if yearlen == 365 {
        return BaseMasks {
            mmask: masks.m365,
            mdaymask: masks.mday365,
            nmdaymask: masks.nmday365,
            mrange: masks.m365range,
            wdaymask: Vec::from(&masks.wday[wday..]),
        };
    }

    BaseMasks {
        mmask: masks.m366,
        mdaymask: masks.mday366,
        nmdaymask: masks.nmday366,
        mrange: masks.m366range,
        wdaymask: Vec::from(&masks.wday[wday..]),
    }
}

pub fn pymod(a: isize, b: isize) -> isize {
    let r = a % b;
    // If r and b differ in sign, add b to wrap the result to the correct sign.
    if (r > 0 && b < 0) || (r < 0 && b > 0) {
        return r + b;
    }
    r
}

pub fn rebuild_year(year: i32, options: &ParsedOptions) -> YearInfo {
    let firstyday = Utc.ymd(year, 1, 1).and_hms_milli(0, 0, 0, 0);

    let yearlen = get_year_len(year);
    let nextyearlen = get_year_len(year + 1);
    let yearordinal = to_ordinal(&firstyday);
    let yearweekday = get_weekday_val(&firstyday.weekday());

    let base_masks = base_year_masks(year);

    let mut result = YearInfo {
        yearlen,
        nextyearlen,
        yearordinal,
        yearweekday,
        wnomask: None,
        mmask: base_masks.mmask,
        mdaymask: base_masks.mdaymask,
        nmdaymask: base_masks.nmdaymask,
        mrange: base_masks.mrange,
        wdaymask: base_masks.wdaymask,
    };

    if options.byweekno.is_empty() {
        return result;
    }

    let mut wnomask = vec![0; yearlen + 7];
    let wyearlen;
    let mut no1wkst = pymod((7 - yearweekday + options.wkst) as isize, 7);
    let firstwkst = no1wkst;
    if no1wkst >= 4 {
        no1wkst = 0;
        // Number of days in the year, plus the days we got
        // from last year.
        wyearlen = result.yearlen as isize + pymod(yearweekday as isize - options.wkst as isize, 7);
    } else {
        // Number of days in the year, minus the days we
        // left in last year.
        wyearlen = yearlen as isize - no1wkst;
    }

    let div = (wyearlen as f32 / 7.).floor() as isize;
    let year_mod = pymod(wyearlen, 7);
    //const numweeks = Math.floor(div + mod / 4)
    let numweeks = div + (year_mod / 4);

    for j in 0..options.byweekno.len() {
        let mut n = options.byweekno[j];
        if n < 0 {
            n += (numweeks + 1) as isize;
        }
        if !(n > 0 && n <= numweeks) {
            continue;
        }
        let mut i;
        if n > 1 {
            i = no1wkst + (n - 1) * 7;
            if no1wkst != firstwkst {
                i -= 7 - firstwkst;
            }
        } else {
            i = no1wkst;
        }

        for _ in 0..7 {
            wnomask[i as usize] = 1;
            i += 1;
            if result.wdaymask[i as usize] as usize == options.wkst {
                break;
            }
        }
    }

    if options.byweekno.iter().any(|&wkno| wkno == 1) {
        // Check week number 1 of next year as well
        // orig-TODO : Check -numweeks for next year.
        let mut i = no1wkst + numweeks * 7;
        if no1wkst != firstwkst {
            i -= 7 - firstwkst;
        }
        if i < yearlen as isize {
            // If week starts in next year, we
            // don't care about it.
            for _ in 0..7 {
                wnomask[i as usize] = 1;
                i += 1;
                if result.wdaymask[i as usize] as usize == options.wkst {
                    break;
                }
            }
        }
    }

    if no1wkst > 0 {
        // Check last week number of last year as
        // well. If no1wkst is 0, either the year
        // started on week start, or week number 1
        // got days from last year, so there are no
        // days from last year's last week number in
        // this year.
        let lnumweeks;
        if !options.byweekno.iter().any(|&weekno| weekno == -1) {
            let lyearweekday = get_weekday_val(&Utc.ymd(year - 1, 1, 1).weekday());

            let lno1wkst = pymod((7 - lyearweekday + options.wkst) as isize, 7);

            let lyearlen = get_year_len(year - 1);
            let weekst;
            if lno1wkst >= 4 {
                //lno1wkst = 0;
                weekst = lyearlen as isize + pymod((lyearweekday - options.wkst) as isize, 7);
            } else {
                weekst = yearlen as isize - no1wkst;
            }

            lnumweeks = 52 + (pymod(weekst, 7) / 4) as isize;
        } else {
            lnumweeks = -1 as isize;
        }

        if options.byweekno.iter().any(|&weekno| weekno == lnumweeks) {
            for i in 0..no1wkst {
                wnomask[i as usize] = 1;
            }
        }
    }

    result.wnomask = Some(wnomask);

    result
}
