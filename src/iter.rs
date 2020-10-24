use crate::datetime::*;
use crate::iter_set::IterResult;
use crate::iterinfo::*;
use crate::options::*;
use crate::poslist::*;
use crate::yearinfo::*;
use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::Tz;

pub enum QueryMethodTypes {
    ALL,
    BETWEEN,
    BEFORE,
    AFTER,
}

pub struct IterArgs {
    pub inc: bool,
    pub before: Option<DateTime<Tz>>,
    pub after: Option<DateTime<Tz>>,
    pub dt: Option<DateTime<Tz>>,
}

pub struct RRuleIterRes {
    pub method: QueryMethodTypes,
    pub args: IterArgs,
    pub min_date: Option<DateTime<Tz>>,
    pub max_date: Option<DateTime<Tz>>,
    pub result: Vec<DateTime<Tz>>,
    pub total: usize,
}

impl RRuleIterRes {
    pub fn new(method: QueryMethodTypes, args: IterArgs) -> Self {
        let (max_date, min_date) = match method {
            QueryMethodTypes::BETWEEN if args.inc => {
                (Some(args.before.unwrap()), Some(args.after.unwrap()))
            }
            QueryMethodTypes::BETWEEN => (
                Some(args.before.unwrap() - Duration::milliseconds(1)),
                Some(args.after.unwrap() + Duration::milliseconds(1)),
            ),
            QueryMethodTypes::BEFORE if args.inc => (Some(args.dt.unwrap()), None),
            QueryMethodTypes::BEFORE => (Some(args.dt.unwrap() - Duration::milliseconds(1)), None),
            QueryMethodTypes::AFTER if args.inc => (None, Some(args.dt.unwrap())),
            QueryMethodTypes::AFTER => (None, Some(args.dt.unwrap() + Duration::milliseconds(1))),
            _ => (None, None),
        };

        Self {
            method,
            args,
            min_date,
            max_date,
            total: 0,
            result: vec![],
        }
    }

    pub fn add(&mut self, date: DateTime<Tz>) -> bool {
        self.result.push(date);
        true
    }
}

impl IterResult for RRuleIterRes {
    fn accept(&mut self, date: DateTime<Tz>) -> bool {
        self.total += 1;
        let too_early = self.min_date.is_some() && date < self.min_date.unwrap();
        let too_late = self.max_date.is_some() && date > self.max_date.unwrap();

        match self.method {
            QueryMethodTypes::BETWEEN if too_early => true,
            QueryMethodTypes::BETWEEN if too_late => false,
            QueryMethodTypes::BEFORE if too_late => false,
            QueryMethodTypes::AFTER if too_early => true,
            QueryMethodTypes::AFTER => {
                self.add(date);
                false
            }
            _ => self.add(date),
        }
    }

    /// before and after returns only one date whereas all and between an array
    fn get_value(&self) -> Vec<DateTime<Tz>> {
        match self.method {
            QueryMethodTypes::BETWEEN | QueryMethodTypes::ALL => self.result.clone(),
            _ => {
                if self.result.is_empty() {
                    return vec![];
                }
                vec![self.result[self.result.len() - 1].clone()]
            }
        }
    }
}

pub fn increment_counter_date(
    counter_date: DateTime<Utc>,
    options: &ParsedOptions,
    filtered: bool,
) -> DateTime<Utc> {
    match options.freq {
        Frequenzy::Yearly => counter_date
            .with_year(counter_date.year() + options.interval as i32)
            .unwrap(),
        Frequenzy::Monthly => {
            let new_month = counter_date.month() + options.interval as u32;
            if new_month > 12 {
                let mut year_div = new_month / 12;
                let mut new_month = new_month % 12;
                if new_month == 0 {
                    new_month = 12;
                    year_div -= 1;
                }
                let new_year = counter_date.year() + year_div as i32;
                counter_date
                    .with_month(new_month)
                    .unwrap()
                    .with_year(new_year)
                    .unwrap()
            } else {
                counter_date.with_month(new_month).unwrap()
            }
        }
        Frequenzy::Weekly => {
            let mut day_delta = 0;
            let weekday = get_weekday_val(&counter_date.weekday());
            if options.wkst > weekday {
                day_delta += -((weekday + 1 + (6 - options.wkst)) as isize)
                    + (options.interval as isize) * 7;
            } else {
                day_delta += -((weekday - options.wkst) as isize) + (options.interval as isize) * 7;
            }
            counter_date + Duration::days(day_delta as i64)
        }
        Frequenzy::Daily => counter_date + Duration::days(options.interval as i64),
        Frequenzy::Hourly => {
            let mut new_hours = counter_date.hour() as usize;
            if filtered {
                new_hours += ((23 - new_hours) as f32 / options.interval as f32).floor() as usize
                    * options.interval;
            }

            loop {
                new_hours += options.interval;
                if options.byhour.is_empty()
                    || options
                        .byhour
                        .iter()
                        .any(|bh| *bh == (new_hours % 24) as usize)
                {
                    break;
                }
            }
            counter_date.with_hour(0).unwrap() + Duration::hours(new_hours as i64)
        }
        Frequenzy::Minutely => counter_date + Duration::minutes(options.interval as i64),
        Frequenzy::Secondly => counter_date + Duration::seconds(options.interval as i64),
    }
}

pub fn includes<T>(v: &Vec<T>, el: &T) -> bool
where
    T: PartialEq,
{
    v.iter().any(|ve| ve == el)
}

pub fn not_empty<T>(v: &Vec<T>) -> bool {
    !v.is_empty()
}

pub fn is_filtered(ii: &IterInfo, current_day: usize, options: &ParsedOptions) -> bool {
    return (not_empty(&options.bymonth)
        && !includes(&options.bymonth, &ii.mmask().unwrap()[current_day]))
        || (not_empty(&options.byweekno) && (ii.wnomask().unwrap()[current_day]) == 0)
        || (not_empty(&options.byweekday)
            && !includes(&options.byweekday, &ii.wdaymask().unwrap()[current_day]))
        || (ii.nwdaymask().is_some()
            && not_empty(ii.nwdaymask().unwrap())
            && (ii.nwdaymask().unwrap()[current_day]) == 0)
        || (options.byeaster.is_some()
            && !(includes(ii.eastermask().unwrap(), &(current_day as isize))))
        || ((not_empty(&options.bymonthday) || not_empty(&options.bynmonthday))
            && !includes(&options.bymonthday, &ii.mdaymask().unwrap()[current_day])
            && !includes(&options.bynmonthday, &ii.nmdaymask().unwrap()[current_day]))
        || (not_empty(&options.byyearday)
            && ((current_day < ii.yearlen().unwrap()
                && !includes(&options.byyearday, &(current_day as isize + 1))
                && !includes(
                    &options.byyearday.iter().map(|v| *v as isize).collect(),
                    &(-(ii.yearlen().unwrap() as isize) + current_day as isize),
                ))
                || (current_day >= ii.yearlen().unwrap()
                    && !includes(
                        &options.byyearday,
                        &((current_day + 1 - ii.yearlen().unwrap()) as isize),
                    )
                    && !includes(
                        &options.byyearday.iter().map(|v| *v as isize).collect(),
                        &(-(ii.nextyearlen().unwrap() as isize) + current_day as isize
                            - ii.yearlen().unwrap() as isize),
                    ))));
}

pub fn remove_filtered_days(
    dayset: &mut Vec<Option<isize>>,
    start: usize,
    end: usize,
    ii: &IterInfo,
    options: &ParsedOptions,
) -> bool {
    let mut filtered = false;

    for daycounter in start..end {
        let current_day = dayset[daycounter];
        if current_day.is_none() {
            continue;
        }

        filtered = is_filtered(ii, current_day.unwrap() as usize, options);
        if filtered {
            dayset[daycounter] = None;
        }
    }
    filtered
}

pub fn build_timeset(options: &ParsedOptions) -> Vec<Time> {
    let millisecond_mod = (options.dtstart.timestamp_millis() & 1000) as usize;

    if !(options.freq == Frequenzy::Daily
        || options.freq == Frequenzy::Monthly
        || options.freq == Frequenzy::Weekly
        || options.freq == Frequenzy::Yearly)
    {
        return vec![];
    }

    let mut timeset = vec![];
    for hour in &options.byhour {
        for minute in &options.byminute {
            for second in &options.bysecond {
                timeset.push(Time::new(*hour, *minute, *second, millisecond_mod));
            }
        }
    }

    timeset
}

pub fn make_timeset(
    ii: &IterInfo,
    counter_date: &DateTime<Utc>,
    options: &ParsedOptions,
) -> Vec<Time> {
    if options.freq == Frequenzy::Daily
        || options.freq == Frequenzy::Monthly
        || options.freq == Frequenzy::Weekly
        || options.freq == Frequenzy::Yearly
    {
        return build_timeset(options);
    }

    if (options.freq >= Frequenzy::Hourly
        && !options.byhour.is_empty()
        && !options
            .byhour
            .iter()
            .any(|&h| h == counter_date.hour() as usize))
        || (options.freq >= Frequenzy::Minutely
            && !options.byminute.is_empty()
            && !options
                .byminute
                .iter()
                .any(|&m| m == counter_date.minute() as usize))
        || (options.freq >= Frequenzy::Secondly
            && !options.bysecond.is_empty()
            && !options
                .bysecond
                .iter()
                .any(|&s| s == counter_date.second() as usize))
    {
        return vec![];
    }

    ii.gettimeset(
        &options.freq,
        counter_date.hour() as usize,
        counter_date.minute() as usize,
        counter_date.second() as usize,
        counter_date.timestamp_subsec_millis() as usize,
    )
}
