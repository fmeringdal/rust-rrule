use crate::datetime::from_ordinal;
use crate::datetime::{DTime, Time};
use crate::iter::iterinfo::IterInfo;
use crate::utils::pymod;
use chrono::prelude::*;
use chrono_tz::Tz;

pub fn build_poslist(
    bysetpost: &[isize],
    timeset: &[Time],
    start: usize,
    end: usize,
    ii: &IterInfo,
    dayset: &[Option<isize>],
    tz: &Tz,
) -> Vec<DTime> {
    let mut poslist = Vec::new();

    for pos in bysetpost.iter().copied() {
        let daypos;
        let timepos;
        if pos < 0 {
            daypos = (pos as f32 / timeset.len() as f32).floor() as isize;
            timepos = pymod(pos as isize, timeset.len() as isize);
        } else {
            daypos = ((pos - 1) as f32 / timeset.len() as f32) as isize;
            timepos = pymod(pos as isize - 1, timeset.len() as isize);
        }

        let tmp = dayset
            .iter()
            .take(end)
            .skip(start)
            .copied()
            .flatten()
            .collect::<Vec<_>>();

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

        if !poslist.iter().any(|&p| p == res) {
            poslist.push(res);
        }
    }

    poslist.sort_by_key(|a| a.timestamp());

    poslist
}
