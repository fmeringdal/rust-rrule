use chrono::DateTime;

use super::rrule_iter::WasLimited;
use super::{rrule_iter::RRuleIter, MAX_ITER_LOOP};
use crate::RRuleSet;
use crate::{RRuleError, Tz};
use std::{collections::BTreeSet, iter::Peekable, str::FromStr};

#[derive(Debug, Clone)]
/// Iterator over all the dates in an [`RRuleSet`].
pub struct RRuleSetIter {
    limited: bool,
    rrule_iters: Vec<Peekable<RRuleIter>>,
    exrules: Vec<RRuleIter>,
    exdates: BTreeSet<i64>,
    /// Sorted additional dates in descending order
    rdates: Vec<DateTime<Tz>>,
    was_limited: bool,
}

impl RRuleSetIter {
    fn is_date_excluded(
        date: &DateTime<Tz>,
        exrules: &mut [RRuleIter],
        exdates: &mut BTreeSet<i64>,
    ) -> bool {
        for exrule in exrules {
            for exdate in exrule {
                exdates.insert(exdate.timestamp());
                if exdate > *date {
                    break;
                }
            }
        }

        exdates.contains(&date.timestamp())
    }
}

impl Iterator for RRuleSetIter {
    type Item = DateTime<Tz>;

    fn next(&mut self) -> Option<Self::Item> {
        // If there already was an error, return the error again.
        if self.was_limited {
            return None;
        }

        let mut exdate_limit_counter: u32 = 0;
        loop {
            // Peek all rrule iterators and find the one holding the earliest date.
            let rrule_date = self
                .rrule_iters
                .iter_mut()
                .enumerate()
                .filter_map(|(i, iter)| iter.peek().copied().map(|date| (i, date)))
                .min_by_key(|(_, date)| *date);

            // Peek next rdate.
            let rdate = self.rdates.last().copied();

            let next_date = match (rrule_date, rdate) {
                (None, None) => {
                    // No rrule date and no rdate. End of iterator.
                    return None;
                }
                (None, Some(rdate)) => {
                    // No rrule date, only rdate. Pop it of the list and use it.
                    self.rdates.pop();
                    rdate
                }
                (Some((iter_i, rrule_date)), None) => {
                    // No rdate, only rrule. Step the corresponding rrule
                    // iterator and use the date.
                    self.rrule_iters[iter_i].next();
                    rrule_date
                }
                (Some((iter_i, rrule_date)), Some(next_rdate)) => {
                    // Both rrule and rdate available. Check which one of
                    // them is the earliest and use it.
                    if rrule_date < next_rdate {
                        self.rrule_iters[iter_i].next();
                        rrule_date
                    } else {
                        self.rdates.pop();
                        next_rdate
                    }
                }
            };

            // Check if the date should be excluded.
            // If the date is excluded then we just loop another round find
            // the next earliest date from rrules and rdates.
            if !Self::is_date_excluded(&next_date, &mut self.exrules, &mut self.exdates) {
                return Some(next_date);
            } else if self.limited {
                exdate_limit_counter += 1;
                if exdate_limit_counter >= MAX_ITER_LOOP {
                    log::warn!(
                        "Reached max loop counter (`{MAX_ITER_LOOP}`). \
                        See 'validator limits' in docs for more info.",
                    );
                    self.was_limited = true;
                    return None;
                }
            }
        }
    }
}

impl IntoIterator for &RRuleSet {
    type Item = DateTime<Tz>;

    type IntoIter = RRuleSetIter;

    fn into_iter(self) -> Self::IntoIter {
        let mut rdates_sorted = self.rdate.clone();

        // Sort rdates in descending order.
        rdates_sorted.sort_by(|d1, d2| d2.cmp(d1));

        RRuleSetIter {
            limited: self.limited,
            rrule_iters: self
                .rrule
                .iter()
                .map(|rrule| rrule.iter_with_ctx(self.dt_start, self.limited).peekable())
                .collect(),
            rdates: rdates_sorted,
            exrules: self
                .exrule
                .iter()
                .map(|exrule| exrule.iter_with_ctx(self.dt_start, self.limited))
                .collect(),
            exdates: self.exdate.iter().map(DateTime::timestamp).collect(),
            was_limited: false,
        }
    }
}

impl WasLimited for RRuleSetIter {
    fn was_limited(&self) -> bool {
        self.was_limited
    }
}

impl FromStr for RRuleSetIter {
    type Err = RRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RRuleSet::from_str(s)?.into_iter())
    }
}
