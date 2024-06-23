use chrono::DateTime;

use super::{
    rrule_iter::{RRuleIter, WasLimited},
    MAX_ITER_LOOP,
};
use crate::{RRuleError, RRuleSet, Tz};
use std::{cmp::Ordering, iter::Peekable, str::FromStr};

#[derive(Debug, Clone)]
/// Iterator over all the dates in an [`RRuleSet`].
pub struct RRuleSetIter {
    limited: bool,
    rrule_iters: Vec<Peekable<RRuleIter>>,
    exrules: Vec<Peekable<RRuleIter>>,
    exdates: Vec<DateTime<Tz>>,
    /// Sorted additional dates in descending order
    rdates: Vec<DateTime<Tz>>,
    was_limited: bool,
}

impl RRuleSetIter {
    /// Check if a date is excluded according to exdates or exrules.
    ///
    /// Must never be called with a lesser date than the last call.
    fn is_date_excluded(&mut self, date: &DateTime<Tz>) -> bool {
        fn check_exdates(exdates: &mut Vec<DateTime<Tz>>, date: &DateTime<Tz>) -> bool {
            while let Some(exdate) = exdates.last() {
                match exdate.cmp(date) {
                    Ordering::Less => drop(exdates.pop()),
                    Ordering::Equal => return true,
                    Ordering::Greater => return false,
                }
            }
            false
        }

        fn check_exrules(exrules: &mut [Peekable<RRuleIter>], date: &DateTime<Tz>) -> bool {
            if exrules.is_empty() {
                return false;
            }
            loop {
                let Some((iter_i, exdate)) = exrules
                    .iter_mut()
                    .enumerate()
                    .filter_map(|(i, iter)| iter.peek().map(|date| (i, date)))
                    .min_by_key(|(_, date)| *date)
                else {
                    return false;
                };
                match exdate.cmp(date) {
                    Ordering::Less => drop(exrules[iter_i].next()),
                    Ordering::Equal => return true,
                    Ordering::Greater => return false,
                }
            }
        }

        check_exdates(&mut self.exdates, date) || check_exrules(&mut self.exrules, date)
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
            if !self.is_date_excluded(&next_date) {
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
        // Sort rdates in descending order.
        let mut rdates_sorted = self.rdate.clone();
        rdates_sorted.sort_by(|d1, d2| d2.cmp(d1));

        // Sort exdates in descending order.
        let mut exdates_sorted = self.exdate.clone();
        exdates_sorted.sort_by(|d1, d2| d2.cmp(d1));

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
                .map(|exrule| exrule.iter_with_ctx(self.dt_start, self.limited).peekable())
                .collect(),
            exdates: exdates_sorted,
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
