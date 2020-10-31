use crate::datetime::from_ordinal;
use crate::datetime::{DTime, Time};
use crate::iter::iterinfo::IterInfo;
use crate::iter::utils::pymod;
use chrono::prelude::*;
use chrono_tz::Tz;

pub fn build_poslist(
    bysetpost: &Vec<isize>,
    timeset: &Vec<Time>,
    start: usize,
    end: usize,
    ii: &IterInfo,
    dayset: &Vec<Option<isize>>,
    tz: &Tz,
) -> Vec<DTime> {
    let mut poslist = vec![];

    for j in 0..bysetpost.len() {
        let daypos;
        let timepos;
        let pos = bysetpost[j];
        if pos < 0 {
            daypos = (pos as f32 / timeset.len() as f32).floor() as isize;
            timepos = pymod(pos as isize, timeset.len() as isize);
        } else {
            daypos = ((pos - 1) as f32 / timeset.len() as f32) as isize;
            timepos = pymod(pos as isize - 1, timeset.len() as isize);
        }

        let mut tmp = vec![];
        for k in start..end {
            let val = dayset[k];
            match val {
                Some(v) => tmp.push(v),
                None => (),
            }
        }

        let i;
        if daypos < 0 {
            let index = tmp.len() as isize + daypos;
            i = &tmp[index as usize];
        } else {
            i = &tmp[daypos as usize];
        }

        let date = from_ordinal(ii.yearordinal().unwrap() + i, tz);
        let res = tz.ymd(date.year(), date.month(), date.day()).and_hms(
            timeset[timepos as usize].hour as u32,
            timeset[timepos as usize].minute as u32,
            timeset[timepos as usize].second as u32,
        );
        // XXX: can this ever be in the array?
        // - compare the actual date instead?
        if !poslist.iter().any(|&p| p == res) {
            poslist.push(res);
        }
    }

    poslist.sort_by_key(|a| a.timestamp());

    poslist
}
