use crate::options::{
    Frequency, NWeekday, NWeekdayIdentifier, Options, ParsedOptions, RRuleParseError,
};
use crate::utils::is_some_and_not_empty;
use chrono::prelude::*;
use chrono_tz::{Tz, UTC};

// TODO: More validation here
pub fn parse_options(options: &Options) -> Result<ParsedOptions, RRuleParseError> {
    let default_partial_options = Options {
        interval: Some(1),
        freq: Some(Frequency::Yearly),
        wkst: Some(0),
        ..Default::default()
    };

    let tzid: Tz = match options.tzid {
        Some(tz) => tz,
        None => UTC, // TODO Should this default to UTC?
    };

    let _bynweekday: Vec<Vec<isize>> = vec![];
    let mut bynmonthday: Vec<isize> = vec![];

    let mut partial_options = Options::concat(&default_partial_options, options);

    if partial_options.byeaster.is_some() {
        partial_options.freq = Some(Frequency::Yearly);
    }
    let freq = partial_options.freq.unwrap_or(Frequency::Daily);

    if partial_options.dtstart.is_none() {
        return Err(RRuleParseError(String::from("Dtstart can not be None")));
    }

    if partial_options.wkst.is_none() {
        partial_options.wkst = Some(0);
    }

    if let Some(bysetpos) = &partial_options.bysetpos {
        for pos in bysetpos {
            if *pos == 0 || !(*pos >= -366 && *pos <= 366) {
                return Err(RRuleParseError(String::from(
                    "Bysetpos must be between 1 and 366, or between -366 and -1",
                )));
            }
        }
    }

    let dtstart = match partial_options.dtstart {
        Some(dtstart) => Ok(dtstart),
        None => Err(RRuleParseError(String::from("Dtstart was not specified"))),
    }?;

    if !(partial_options.byweekno.is_some()
        || is_some_and_not_empty(&partial_options.byweekno)
        || is_some_and_not_empty(&partial_options.byyearday)
        || partial_options.bymonthday.is_some()
        || is_some_and_not_empty(&partial_options.bymonthday)
        || partial_options.byweekday.is_some()
        || partial_options.byeaster.is_some())
    {
        match &freq {
            Frequency::Yearly => {
                if partial_options.bymonth.is_none() {
                    partial_options.bymonth = Some(vec![dtstart.month() as usize]);
                }
                partial_options.bymonthday = Some(vec![dtstart.day() as isize]);
            }
            Frequency::Monthly => {
                partial_options.bymonthday = Some(vec![dtstart.day() as isize]);
            }
            Frequency::Weekly => {
                partial_options.byweekday = Some(vec![NWeekday::new(
                    dtstart.weekday() as u8,
                    NWeekdayIdentifier::Every,
                )]);
            }
            _ => (),
        };
    }

    match &partial_options.bymonthday {
        None => bynmonthday = vec![],
        Some(opts_bymonthday) => {
            use std::cmp::Ordering;

            let mut bymonthday = vec![];

            for v in opts_bymonthday {
                match v.cmp(&0) {
                    Ordering::Greater => bymonthday.push(*v),
                    Ordering::Less => bynmonthday.push(*v),
                    Ordering::Equal => {}
                }
            }

            partial_options.bymonthday = Some(bymonthday);
        }
    }

    let mut byweekday = vec![];
    let mut bynweekday: Vec<Vec<isize>> = vec![];
    // byweekday / bynweekday // ! more to do here

    if let Some(opts_byweekday) = partial_options.byweekday {
        for wday in opts_byweekday {
            match wday.n {
                NWeekdayIdentifier::Every => byweekday.push(wday.weekday),
                NWeekdayIdentifier::Identifier(n) => {
                    bynweekday.push(vec![wday.weekday as isize, n]);
                }
            }
            // if wday.n ==  {
            //     byweekday.push(wday.weekday);
            // } else {
            //     bynweekday.push(vec![wday.weekday as isize, wday.n]);
            // }
        }
    }

    // byhour
    if partial_options.byhour.is_none() && freq < Frequency::Hourly {
        partial_options.byhour = Some(vec![dtstart.hour() as usize]);
    }

    // byminute
    if partial_options.byminute.is_none() && freq < Frequency::Minutely {
        partial_options.byminute = Some(vec![dtstart.minute() as usize]);
    }

    // bysecond
    if partial_options.bysecond.is_none() && freq < Frequency::Secondly {
        partial_options.bysecond = Some(vec![dtstart.second() as usize]);
    }

    Ok(ParsedOptions {
        freq,
        interval: partial_options.interval.unwrap(),
        count: partial_options.count,
        until: partial_options.until,
        tzid,
        dtstart,
        wkst: partial_options.wkst.unwrap(),
        bysetpos: partial_options.bysetpos.unwrap_or_default(),
        bymonth: partial_options.bymonth.unwrap_or_default(),
        bymonthday: partial_options.bymonthday.unwrap_or_default(),
        bynmonthday,
        byyearday: partial_options.byyearday.unwrap_or_default(),
        byweekno: partial_options.byweekno.unwrap_or_default(),
        byweekday,
        bynweekday,
        byhour: partial_options.byhour.unwrap_or_default(),
        byminute: partial_options.byminute.unwrap_or_default(),
        bysecond: partial_options.bysecond.unwrap_or_default(),
        byeaster: partial_options.byeaster,
    })
}
