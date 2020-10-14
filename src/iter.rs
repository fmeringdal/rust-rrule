use crate::datetime::*;
use crate::iterinfo::*;
use crate::poslist::*;
use crate::yearinfo::*;
use chrono::prelude::*;
use chrono::Duration;

pub enum QueryMethodTypes {
    ALL,
    BETWEEN,
    BEFORE,
    AFTER,
}

pub struct IterArgs {
    inc: bool,
    before: DateTime<Utc>,
    after: DateTime<Utc>,
    dt: DateTime<Utc>,
    _value: Option<Vec<DateTime<Utc>>>,
}

pub struct IterResult {
    pub method: QueryMethodTypes,
    pub args: IterArgs,
    pub min_date: Option<DateTime<Utc>>,
    pub max_date: Option<DateTime<Utc>>,
    pub _result: Vec<DateTime<Utc>>,
    pub total: usize,
}

impl IterResult {
    pub fn new(method: QueryMethodTypes, args: IterArgs) -> Self {
        let (max_date, min_date) = match method {
            QueryMethodTypes::BETWEEN if args.inc => (Some(args.before), Some(args.after)),
            QueryMethodTypes::BETWEEN => (
                Some(args.before - Duration::milliseconds(1)),
                Some(args.after + Duration::milliseconds(1)),
            ),
            QueryMethodTypes::BEFORE if args.inc => (Some(args.dt), None),
            QueryMethodTypes::BEFORE => (Some(args.dt - Duration::milliseconds(1)), None),
            QueryMethodTypes::AFTER if args.inc => (None, Some(args.dt)),
            QueryMethodTypes::AFTER => (None, Some(args.dt + Duration::milliseconds(1))),
            _ => (None, None),
        };

        Self {
            method,
            args,
            min_date,
            max_date,
            total: 0,
            _result: vec![],
        }
    }

    pub fn accept(&mut self, date: DateTime<Utc>) -> bool {
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
                return false;
            }
            _ => self.add(date),
        }
    }

    pub fn add(&mut self, date: DateTime<Utc>) -> bool {
        self._result.push(date);
        return true;
    }

    pub fn get_value(&self) -> Vec<DateTime<Utc>> {
        self._result.clone()
        //match self.method {
        //QueryMethodTypes::BETWEEN => Some(self._result.clone()),
        //_ => {
        //if self._result.is_empty() {
        //return None;
        //}
        //Some(vec![self._result[self._result.len() - 1].clone()])
        //}
        //}
    }
}

pub fn iter(iter_result: &mut IterResult, options: &mut ParsedOptions) -> Vec<DateTime<Utc>> {
    if (options.count.is_some() && options.count.unwrap() == 0) || options.interval == 0 {
        return iter_result.get_value();
    }

    let mut counter_date = options.dtstart.clone();
    let mut ii = IterInfo::new(options);
    ii.rebuild(counter_date.year() as isize, counter_date.month() as usize);

    let mut timeset = make_timeset(&ii, &counter_date, options);
    let mut count = match options.count {
        Some(count) => count,
        _ => 0,
    };

    loop {
        let (dayset, start, end) = ii.getdayset(
            &options.freq,
            counter_date.year() as isize,
            counter_date.month() as usize,
            counter_date.day() as usize,
        );
        let mut dayset = dayset
            .into_iter()
            .map(|s| Some(s as isize))
            .collect::<Vec<Option<isize>>>();

        let filtered = remove_filtered_days(&mut dayset, start, end, &ii, options);

        if not_empty(&options.bysetpos) {
            let poslist = build_poslist(&options.bysetpos, &timeset, start, end, &ii, &dayset);

            for j in 0..poslist.len() {
                let res = poslist[j];
                if options.until.is_some() && res > options.until.unwrap() {
                    return iter_result.get_value();
                }

                if res >= options.dtstart {
                    //let rezoned_date = rezone_if_needed(&res, &options);
                    let rezoned_date = res.clone();
                    if !iter_result.accept(rezoned_date) {
                        return iter_result.get_value();
                    }

                    if count > 0 {
                        count -= 1;
                        if count == 0 {
                            return iter_result.get_value();
                        }
                    }
                }
            }
        } else {
            for j in start..end {
                let current_day = dayset[j];
                if current_day.is_none() {
                    continue;
                }

                let current_day = current_day.unwrap();
                let date = from_ordinal(ii.yearordinal().unwrap() + current_day);
                for k in 0..timeset.len() {
                    let res = Utc.ymd(date.year(), date.month(), date.day()).and_hms(
                        timeset[k].hour as u32,
                        timeset[k].minute as u32,
                        timeset[k].second as u32,
                    );
                    if options.until.is_some() && res > options.until.unwrap() {
                        return iter_result.get_value();
                    }
                    if res >= options.dtstart {
                        //let rezoned_date = rezone_if_needed(&res, &options);
                        let rezoned_date = res.clone();
                        if !iter_result.accept(rezoned_date) {
                            return iter_result.get_value();
                        }
                        if count > 0 {
                            count -= 1;
                            if count == 0 {
                                return iter_result.get_value();
                            }
                        }
                    }
                }
            }
        }

        if options.interval == 0 {
            return iter_result.get_value();
        }

        // Handle frequency and interval
        counter_date = increment_counter_date(counter_date, options, filtered);

        if counter_date.year() > 2200 {
            return iter_result.get_value();
        }

        if options.freq == Frequenzy::HOURLY
            || options.freq == Frequenzy::MINUTELY
            || options.freq == Frequenzy::SECONDLY
        {
            timeset = ii.gettimeset(
                &options.freq,
                counter_date.hour() as usize,
                counter_date.minute() as usize,
                counter_date.second() as usize,
                counter_date.timestamp_subsec_millis() as usize,
            );
        }

        ii.rebuild(counter_date.year() as isize, counter_date.month() as usize);
    }
}

pub fn increment_counter_date(
    counter_date: DateTime<Utc>,
    options: &ParsedOptions,
    filtered: bool,
) -> DateTime<Utc> {
    match options.freq {
        Frequenzy::YEARLY => counter_date
            .with_year(counter_date.year() + options.interval as i32)
            .unwrap(),
        Frequenzy::MONTHLY => {
            let new_month = counter_date.month() + options.interval as u32;
            if new_month > 12 {
                let mut year_div = new_month / 12;
                let mut new_month = new_month % 12;
                if new_month == 0 {
                    new_month = 12;
                    year_div -= 1;
                }
                let new_year = counter_date.year() + year_div as i32;
                return counter_date
                    .with_month(new_month)
                    .unwrap()
                    .with_year(new_year)
                    .unwrap();
            } else {
                return counter_date.with_month(new_month).unwrap();
            }
        }
        Frequenzy::WEEKLY => {
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
        Frequenzy::DAILY => counter_date + Duration::days(options.interval as i64),
        _ => panic!("hfoashfosa"),
    }
}

pub fn includes<T>(v: &Vec<T>, el: &T) -> bool
where
    T: PartialEq,
{
    v.iter().any(|ve| ve == el)
}

pub fn not_empty<T>(v: &Vec<T>) -> bool {
    return v.len() > 0;
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
        || ((not_empty(&options.bymonthday) || not_empty(&options.bynmonthday))
            && !includes(&options.bymonthday, &ii.mdaymask().unwrap()[current_day])
            && !includes(&options.bynmonthday, &ii.nmdaymask().unwrap()[current_day]))
        || (not_empty(&options.byyearday)
            && ((current_day < ii.yearlen().unwrap()
                && !includes(&options.byyearday, &(current_day + 1))
                && !includes(
                    &options.byyearday.iter().map(|v| *v as isize).collect(),
                    &(-(ii.yearlen().unwrap() as isize) + current_day as isize),
                ))
                || (current_day >= ii.yearlen().unwrap()
                    && !includes(
                        &options.byyearday,
                        &(current_day + 1 - ii.yearlen().unwrap()),
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

    if !(options.freq == Frequenzy::DAILY
        || options.freq == Frequenzy::MONTHLY
        || options.freq == Frequenzy::WEEKLY
        || options.freq == Frequenzy::YEARLY)
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
    if options.freq == Frequenzy::DAILY
        || options.freq == Frequenzy::MONTHLY
        || options.freq == Frequenzy::WEEKLY
        || options.freq == Frequenzy::YEARLY
    {
        return build_timeset(options);
    }

    if (options.freq >= Frequenzy::HOURLY
        && options.byhour.len() > 0
        && !options
            .byhour
            .iter()
            .any(|&h| h == counter_date.hour() as usize))
        || (options.freq >= Frequenzy::MINUTELY
            && options.byminute.len() > 0
            && !options
                .byminute
                .iter()
                .any(|&m| m == counter_date.minute() as usize))
        || (options.freq >= Frequenzy::SECONDLY
            && options.bysecond.len() > 0
            && !options
                .bysecond
                .iter()
                .any(|&s| s == counter_date.second() as usize))
    {
        return vec![];
    }

    return ii.gettimeset(
        &options.freq,
        counter_date.hour() as usize,
        counter_date.minute() as usize,
        counter_date.second() as usize,
        counter_date.timestamp_subsec_millis() as usize,
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn int_works() {
        let iter_args = IterArgs {
            inc: true,
            before: Utc::now(),
            after: Utc::now(),
            dt: Utc::now(),
            _value: Some(vec![]),
        };
        let mut iter_res = IterResult::new(QueryMethodTypes::ALL, iter_args);
        let mut options = ParsedOptions {
            freq: Frequenzy::DAILY,
            dtstart: Utc.ymd(2012, 1, 1).and_hms(10, 30, 0),
            until: Some(Utc.ymd(2012, 12, 31).and_hms(10, 30, 0)),
            tzid: None,
            interval: 1,
            wkst: 0,
            count: None,
            bysecond: vec![0],
            byminute: vec![30],
            byhour: vec![10],
            bymonth: vec![],
            bymonthday: vec![],
            bysetpos: vec![],
            byweekno: vec![],
            byyearday: vec![],
            byweekday: vec![0, 1, 2, 3, 4, 5, 6],
            bynweekday: vec![],
            bynmonthday: vec![],
        };
        let mut options_2 = ParsedOptions {
            freq: Frequenzy::WEEKLY,
            dtstart: Utc.ymd(2012, 1, 1).and_hms(10, 30, 0),
            until: None,
            //until: Some(Utc.ymd(2012, 12, 31).and_hms(10, 30, 0)),
            tzid: None,
            interval: 5,
            wkst: 0,
            count: Some(5),
            bysecond: vec![0],
            byminute: vec![30],
            byhour: vec![10],
            bymonth: vec![6],
            bymonthday: vec![],
            bysetpos: vec![],
            byweekno: vec![],
            byyearday: vec![],
            byweekday: vec![0, 4],
            bynweekday: vec![],
            bynmonthday: vec![],
        };
        let res = iter(&mut iter_res, &mut options);
        println!("Res: {:?}", res);
    }
}
