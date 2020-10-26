use chrono::prelude::*;
use crate::options::{ParsedOptions, Frequenzy, PartialOptions};
use chrono_tz::{Tz, UTC};

// TODO: Validation
pub fn parse_options(options: &PartialOptions) -> ParsedOptions {
    let mut default_partial_options = PartialOptions::new();
    default_partial_options.interval = Some(1);
    default_partial_options.freq = Some(Frequenzy::Yearly);
    default_partial_options.wkst = Some(0);

    let tzid: Tz = if options.tzid.is_some() {
        options.tzid.clone().unwrap()
    } else {
        UTC
    };

    let mut partial_options = PartialOptions::concat(&default_partial_options, options);

    if partial_options.byeaster.is_some() {
        partial_options.freq = Some(Frequenzy::Yearly);
    }
    let freq = partial_options.freq.unwrap();

    if partial_options.dtstart.is_none() {
        panic!("Dtstart can not be None");
    }

    if partial_options.wkst.is_none() {
        partial_options.wkst = Some(0);
    }

    if let Some(bysetpos) = &partial_options.bysetpos {
        for pos in bysetpos {
            if *pos == 0 || !(*pos >= -366 && *pos <= 366) {
                panic!("bysetpos must be between 1 and 366, or between -366 and -1");
            }
        }
    }

    if !(
        partial_options.byweekno.is_some() ||
        is_some_and_not_empty(&partial_options.byweekno) ||
        is_some_and_not_empty(&partial_options.byyearday) ||
        partial_options.bymonthday.is_some() ||
        is_some_and_not_empty(&partial_options.bymonthday) ||
        partial_options.byweekday.is_some() ||
        partial_options.byeaster.is_some()
    ) { 
        match &freq {
            Frequenzy::Yearly => {
                if partial_options.bymonth.is_none() {
                    partial_options.bymonth = Some(vec![partial_options.dtstart.unwrap().month() as usize]);
                }
                partial_options.bymonthday = Some(vec![partial_options.dtstart.unwrap().day() as isize]);
            },
            Frequenzy::Monthly => {
                partial_options.bymonthday = Some(vec![partial_options.dtstart.unwrap().day() as isize]); 
            },
            Frequenzy::Weekly => {
                partial_options.byweekday = Some(vec![partial_options.dtstart.unwrap().weekday() as usize]);
            },
            _ => ()
        };
    }

    if partial_options.bymonthday.is_none() {
        partial_options.bynmonthday = None;
    } else {
        let mut bymonthday = vec![];
        let mut bynmonthday = vec![];

      for v in &partial_options.bymonthday.unwrap() {
          if *v > 0 {
              bymonthday.push(*v);
          } else if *v < 0 {
              bynmonthday.push(*v);
          }
      }

      partial_options.bymonthday = Some(bymonthday);
      partial_options.bynmonthday = Some(bynmonthday);
    }

    // byweekday / bynweekday
    if partial_options.byweekday.is_some() {
        partial_options.bynweekday = None;
    }

  // byhour
  if partial_options.byhour.is_none() && freq < Frequenzy::Hourly {
        partial_options.byhour = Some(vec![partial_options.dtstart.unwrap().hour() as usize]);
  }

  // byminute
  if partial_options.byminute.is_none() && freq < Frequenzy::Minutely {
    partial_options.byminute = Some(vec![partial_options.dtstart.unwrap().minute() as usize]);
}


  // bysecond
  if partial_options.bysecond.is_none() && freq < Frequenzy::Secondly {
    partial_options.bysecond = Some(vec![partial_options.dtstart.unwrap().second() as usize]);
  }


  ParsedOptions {
    freq,
    interval: partial_options.interval.unwrap(),
    count: partial_options.count,
    until: partial_options.until,
    tzid,
    dtstart: partial_options.dtstart.unwrap(),
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
  }
}

fn is_some_and_not_empty<T>(v: &Option<Vec<T>>) -> bool {
    match v {
        Some(v) => !v.is_empty(),
        None => false
    }
}