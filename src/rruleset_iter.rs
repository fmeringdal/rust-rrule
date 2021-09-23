use crate::rruleset::RRuleSet;
use crate::{rrule_iter::RRuleIter, RRule};
use chrono::prelude::*;
use chrono_tz::Tz;
use std::collections::HashMap;
use std::iter::Iterator;

pub struct RRuleSetIter<'a> {
    pub queue: HashMap<usize, DateTime<Tz>>,
    pub rrule_iters: Vec<RRuleIter<'a>>,
    pub exrules: &'a Vec<RRule>,
    pub exdates: HashMap<i64, ()>,
    // Sorted additional dates in decreasing order
    pub rdates: Vec<DateTime<Tz>>,
}

impl<'a> Iterator for RRuleSetIter<'a> {
    type Item = DateTime<Tz>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_date: Option<(usize, DateTime<Tz>)> = None;

        for (i, rrule_iter) in self.rrule_iters.iter_mut().enumerate() {
            let rrule_queue = self.queue.remove(&i);
            let next_rrule_date;
            match rrule_queue {
                Some(d) => next_rrule_date = Some(d),
                None => {
                    // should be method on self
                    next_rrule_date = generate(rrule_iter, &mut self.exrules, &mut self.exdates);
                }
            }

            match next_rrule_date {
                Some(next_rrule_date) => match next_date {
                    None => next_date = Some((i, next_rrule_date)),
                    Some(date) => {
                        if date.1 >= next_rrule_date {
                            // Add previous date to its rrule queue
                            self.queue.insert(date.0, date.1);

                            // Update next_date
                            next_date = Some((i, next_rrule_date));
                        } else {
                            // Store for next iterations
                            self.queue.insert(i, next_rrule_date);
                        }
                    }
                },
                None => {}
            }
        }

        // TODO: RDates should be prefiltered before starting iteration
        match generate_date(&mut self.rdates, &self.exrules, &mut self.exdates) {
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

fn generate_date(
    dates: &mut Vec<DateTime<Tz>>,
    exrules: &Vec<RRule>,
    exdates: &mut HashMap<i64, ()>,
) -> Option<DateTime<Tz>> {
    if dates.is_empty() {
        return None;
    }

    let mut date = dates.remove(dates.len() - 1);
    while !accept_generated_date(&Some(date), exrules, exdates) {
        if dates.is_empty() {
            return None;
        }
        date = dates.remove(dates.len() - 1);
    }

    Some(date)
}

fn generate(
    rrule_iter: &mut RRuleIter,
    exrules: &Vec<RRule>,
    exdates: &mut HashMap<i64, ()>,
) -> Option<DateTime<Tz>> {
    let mut date = rrule_iter.next();
    while !accept_generated_date(&date, exrules, exdates) {
        date = rrule_iter.next();
    }

    date
}

fn accept_generated_date(
    date: &Option<DateTime<Tz>>,
    exrules: &Vec<RRule>,
    exdates: &mut HashMap<i64, ()>,
) -> bool {
    match date {
        None => true,
        Some(date) => {
            let dt = date.timestamp();

            if !exrules.is_empty() {
                let after = date.timezone().timestamp(dt - 1, 0);
                let before = date.timezone().timestamp(dt + 1, 0);
                for exrule in exrules {
                    for date in exrule.between(after, before, true) {
                        exdates.insert(date.timestamp(), ());
                    }
                }
            }

            if exdates.contains_key(&dt) {
                return false;
            }

            true
        }
    }
}

impl<'a> IntoIterator for &'a RRuleSet {
    type Item = DateTime<Tz>;

    type IntoIter = RRuleSetIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        // Sort in decreasing order
        let mut rdates_sorted = self.rdate.clone();
        rdates_sorted
            .sort_by(|d1, d2| d2.partial_cmp(d1).expect("Could not order dates correctly"));

        RRuleSetIter {
            queue: Default::default(),
            rrule_iters: self.rrule.iter().map(|rrule| rrule.into_iter()).collect(),
            rdates: rdates_sorted,
            exrules: &self.exrule,
            exdates: self
                .exdate
                .iter()
                .map(|exdate| (exdate.timestamp(), ()))
                .collect(),
        }
    }
}
