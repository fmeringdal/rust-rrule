use super::rrule_iter::WasLimited;
use super::{rrule_iter::RRuleIter, MAX_ITER_LOOP};
use crate::{core::DateTime, RRuleSet};
use std::collections::BTreeSet;
use std::{collections::HashMap, iter::Iterator};

#[derive(Debug, Clone)]
/// Iterator over all the dates in an [`RRuleSet`].
pub struct RRuleSetIter<'a> {
    queue: HashMap<usize, DateTime>,
    limited: bool,
    rrule_iters: Vec<RRuleIter<'a>>,
    exrules: Vec<RRuleIter<'a>>,
    exdates: BTreeSet<i64>,
    /// Sorted additional dates in descending order
    rdates: Vec<DateTime>,
    was_limited: bool,
}

impl<'a> RRuleSetIter<'a> {
    fn generate_date(
        dates: &mut Vec<DateTime>,
        exrules: &mut [RRuleIter],
        exdates: &mut BTreeSet<i64>,
        limited: bool,
    ) -> (Option<DateTime>, bool) {
        if dates.is_empty() {
            return (None, false);
        }

        let mut date = dates.remove(dates.len() - 1);
        let mut loop_counter: u32 = 0;
        while Self::is_date_excluded(&date, exrules, exdates) {
            if dates.is_empty() {
                return (None, false);
            }
            // Prevent infinite loops
            if limited {
                loop_counter += 1;
                if loop_counter >= MAX_ITER_LOOP {
                    log::warn!(
                        "Reached max loop counter (`{}`). \
                See 'validator limits' in docs for more info.",
                        MAX_ITER_LOOP
                    );
                    return (None, true);
                }
            }
            date = dates.remove(dates.len() - 1);
        }

        (Some(date), false)
    }

    fn generate(
        rrule_iter: &mut RRuleIter,
        exrules: &mut [RRuleIter],
        exdates: &mut BTreeSet<i64>,
        limited: bool,
    ) -> (Option<DateTime>, bool) {
        let mut date = match rrule_iter.next() {
            Some(d) => d,
            None => return (None, false),
        };
        let mut loop_counter: u32 = 0;
        while Self::is_date_excluded(&date, exrules, exdates) {
            // Prevent infinite loops
            if limited {
                loop_counter += 1;
                if loop_counter >= MAX_ITER_LOOP {
                    log::warn!(
                        "Reached max loop counter (`{}`). \
                    See 'validator limits' in docs for more info.",
                        MAX_ITER_LOOP
                    );
                    return (None, true);
                }
            }

            date = match rrule_iter.next() {
                Some(d) => d,
                None => return (None, false),
            };
        }

        (Some(date), false)
    }

    fn is_date_excluded(
        date: &DateTime,
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

impl<'a> Iterator for RRuleSetIter<'a> {
    type Item = DateTime;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_date: Option<(usize, DateTime)> = None;

        // If there already was an error, return the error again.
        if self.was_limited {
            return None;
        }

        for (i, rrule_iter) in self.rrule_iters.iter_mut().enumerate() {
            let rrule_queue = self.queue.remove(&i);
            let next_rrule_date = if let Some(d) = rrule_queue {
                Some(d)
            } else {
                // should be method on self
                let (date, was_limited) = Self::generate(
                    rrule_iter,
                    &mut self.exrules,
                    &mut self.exdates,
                    self.limited,
                );

                if was_limited {
                    self.was_limited = true;
                    return None;
                }

                date
            };

            if let Some(next_rrule_date) = next_rrule_date {
                match next_date {
                    None => next_date = Some((i, next_rrule_date)),
                    Some((idx, date)) => {
                        if date >= next_rrule_date {
                            // Add previous date to its rrule queue
                            self.queue.insert(idx, date);

                            // Update next_date
                            next_date = Some((i, next_rrule_date));
                        } else {
                            // Store for next iterations
                            self.queue.insert(i, next_rrule_date);
                        }
                    }
                }
            }
        }

        let (generated_date, was_limited) = Self::generate_date(
            &mut self.rdates,
            &mut self.exrules,
            &mut self.exdates,
            self.limited,
        );
        if was_limited {
            self.was_limited = true;
            return None;
        }

        match generated_date {
            Some(first_rdate) => {
                let next_date = match next_date {
                    Some(next_date) => {
                        if next_date.1 >= first_rdate {
                            // Add previous date to its rrule queue
                            self.queue.insert(next_date.0, next_date.1);

                            first_rdate
                        } else {
                            // add rdate back
                            self.rdates.push(first_rdate);

                            next_date.1
                        }
                    }
                    None => first_rdate,
                };
                Some(next_date)
            }
            None => next_date.map(|d| d.1),
        }
    }
}

impl<'a> IntoIterator for &'a RRuleSet {
    type Item = DateTime;

    type IntoIter = RRuleSetIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        // Sort in decreasing order
        let mut rdates_sorted = self.rdate.clone();
        rdates_sorted
            .sort_by(|d1, d2| d2.partial_cmp(d1).expect("Could not order dates correctly"));

        let limited = self.limited;

        RRuleSetIter {
            queue: HashMap::new(),
            limited,
            rrule_iters: self
                .rrule
                .iter()
                .map(|rrule| rrule.iter_with_ctx(self.dt_start, limited))
                .collect(),
            rdates: rdates_sorted,
            exrules: self
                .exrule
                .iter()
                .map(|exrule| exrule.iter_with_ctx(self.dt_start, limited))
                .collect(),
            exdates: self.exdate.iter().map(DateTime::timestamp).collect(),
            was_limited: false,
        }
    }
}

impl<'a> WasLimited for RRuleSetIter<'a> {
    fn was_limited(&self) -> bool {
        self.was_limited
    }
}
