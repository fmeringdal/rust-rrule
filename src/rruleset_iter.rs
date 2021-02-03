use crate::rruleset::RRuleSet;
use crate::{iter::RRuleIter, RRule};
use chrono::prelude::*;
use chrono_tz::Tz;
use std::collections::HashMap;
use std::iter::Iterator;

pub struct RRuleIterSet {
    pub queue: HashMap<usize, DateTime<Tz>>,
    pub rrule_iters: Vec<RRuleIter>,
    pub exrules: Vec<RRule>,
    pub exdates: HashMap<i64, ()>,
}

impl Iterator for RRuleIterSet {
    type Item = DateTime<Tz>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_date: Option<(usize, DateTime<Tz>)> = None;

        for (i, rrule_iter) in self.rrule_iters.iter_mut().enumerate() {
            let rrule_queue = self.queue.remove(&i);
            let mut next_rrule_date = None;
            match rrule_queue {
                Some(d) => next_rrule_date = Some(d),
                None => {
                    // should be generated
                    next_rrule_date = generate(rrule_iter, &mut self.exrules, &mut self.exdates);
                }
            }

            match next_rrule_date {
                Some(next_rrule_date) => match next_date {
                    None => next_date = Some((i, next_rrule_date)),
                    Some(date) => {
                        if date.1 < next_rrule_date {
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

        next_date.map(|d| d.1)
    }
}

fn generate(
    rrule_iter: &mut RRuleIter,
    exrules: &mut Vec<RRule>,
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
    exrules: &mut Vec<RRule>,
    exdates: &mut HashMap<i64, ()>,
) -> bool {
    match date {
        None => true,
        Some(date) => {
            let dt = date.timestamp();

            if !exrules.is_empty() {
                let after = date.timezone().timestamp(dt - 1, 0);
                let before = date.timezone().timestamp(dt + 1, 0);
                for exrule in exrules.iter_mut() {
                    for date in exrule.between(after, before, true) {
                        exdates.insert(date.timestamp(), ());
                    }
                }
            }

            if !exdates.contains_key(&dt) {
                return false;
            }

            true
        }
    }
}

impl IntoIterator for RRuleSet {
    type Item = DateTime<Tz>;

    type IntoIter = RRuleIterSet;

    fn into_iter(self) -> Self::IntoIter {
        RRuleIterSet {
            queue: Default::default(),
            rrule_iters: self
                .rrule
                .into_iter()
                .map(|rrule| rrule.into_iter())
                .collect(),
            exrules: self.exrule,
            exdates: self
                .exdate
                .iter()
                .map(|exdate| (exdate.timestamp(), ()))
                .collect(),
        }
    }
}
