use crate::options::{Frequenzy, Options, ParsedOptions, RRuleParseError};
use chrono::prelude::*;
use chrono_tz::{Tz, UTC};

// TODO: Validation
pub fn parse_options(options: &Options) -> Result<ParsedOptions, RRuleParseError> {
    let mut default_partial_options = Options::new();
    default_partial_options.interval = Some(1);
    default_partial_options.freq = Some(Frequenzy::Yearly);
    default_partial_options.wkst = Some(0);

    let tzid: Tz = if options.tzid.is_some() {
        options.tzid.clone().unwrap()
    } else {
        UTC
    };

    let mut partial_options = Options::concat(&default_partial_options, options);

    if partial_options.byeaster.is_some() {
        partial_options.freq = Some(Frequenzy::Yearly);
    }
    let freq = partial_options.freq.unwrap_or(Frequenzy::Daily);

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

    let dtstart = if partial_options.dtstart.is_some() {
        partial_options.dtstart.unwrap()
    } else {
        return Err(RRuleParseError(String::from("Dtstart was not specified")));
    };

    if !(partial_options.byweekno.is_some()
        || is_some_and_not_empty(&partial_options.byweekno)
        || is_some_and_not_empty(&partial_options.byyearday)
        || partial_options.bymonthday.is_some()
        || is_some_and_not_empty(&partial_options.bymonthday)
        || partial_options.byweekday.is_some()
        || partial_options.byeaster.is_some())
    {
        match &freq {
            Frequenzy::Yearly => {
                if partial_options.bymonth.is_none() {
                    partial_options.bymonth = Some(vec![dtstart.month() as usize]);
                }
                partial_options.bymonthday = Some(vec![dtstart.day() as isize]);
            }
            Frequenzy::Monthly => {
                partial_options.bymonthday = Some(vec![dtstart.day() as isize]);
            }
            Frequenzy::Weekly => {
                partial_options.byweekday = Some(vec![dtstart.weekday() as usize]);
            }
            _ => (),
        };
    }

    match &partial_options.bymonthday {
        None => partial_options.bynmonthday = None,
        Some(opts_bymonthday) => {
            let mut bymonthday = vec![];
            let mut bynmonthday = vec![];

            for v in opts_bymonthday {
                if *v > 0 {
                    bymonthday.push(*v);
                } else if *v < 0 {
                    bynmonthday.push(*v);
                }
            }

            partial_options.bymonthday = Some(bymonthday);
            partial_options.bynmonthday = Some(bynmonthday);
        }
    }

    // byweekday / bynweekday
    if partial_options.byweekday.is_some() {
        partial_options.bynweekday = None;
    }

    // byhour
    if partial_options.byhour.is_none() && freq < Frequenzy::Hourly {
        partial_options.byhour = Some(vec![dtstart.hour() as usize]);
    }

    // byminute
    if partial_options.byminute.is_none() && freq < Frequenzy::Minutely {
        partial_options.byminute = Some(vec![dtstart.minute() as usize]);
    }

    // bysecond
    if partial_options.bysecond.is_none() && freq < Frequenzy::Secondly {
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
        bysetpos: partial_options.bysetpos.unwrap_or(vec![]),
        bymonth: partial_options.bymonth.unwrap_or(vec![]),
        bymonthday: partial_options.bymonthday.unwrap_or(vec![]),
        bynmonthday: partial_options.bynmonthday.unwrap_or(vec![]),
        byyearday: partial_options.byyearday.unwrap_or(vec![]),
        byweekno: partial_options.byweekno.unwrap_or(vec![]),
        byweekday: partial_options.byweekday.unwrap_or(vec![]),
        byhour: partial_options.byhour.unwrap_or(vec![]),
        byminute: partial_options.byminute.unwrap_or(vec![]),
        bysecond: partial_options.bysecond.unwrap_or(vec![]),
        bynweekday: partial_options.bynweekday.unwrap_or(vec![]),
        byeaster: partial_options.byeaster,
    })
}

fn is_some_and_not_empty<T>(v: &Option<Vec<T>>) -> bool {
    match v {
        Some(v) => !v.is_empty(),
        None => false,
    }
}
