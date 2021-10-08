use super::is_some_and_not_empty;
use crate::{core::NWeekdayIdentifier, Frequency, NWeekday, Options, ParsedOptions, RRuleError};
use chrono::{Datelike, Timelike, Weekday};
use chrono_tz::{Tz, UTC};

// TODO: More validation here
pub fn parse_options(options: &Options) -> Result<ParsedOptions, RRuleError> {
    let default_partial_options = Options {
        interval: Some(1),
        freq: Some(Frequency::Yearly),
        week_start: Some(Weekday::Mon),
        ..Default::default()
    };

    let tz: Tz = match options.tz {
        Some(tz) => tz,
        None => UTC, // TODO Should this default to UTC?
    };

    let _bynweekday: Vec<Vec<isize>> = vec![];
    let mut by_n_month_day: Vec<i8> = vec![];

    let mut partial_options = Options::concat(&default_partial_options, options);

    #[cfg(feature = "by-easter")]
    if partial_options.by_easter.is_some() {
        partial_options.freq = Some(Frequency::Yearly);
    }
    let freq = partial_options.freq.unwrap_or(Frequency::Daily);

    if partial_options.dt_start.is_none() {
        return Err(RRuleError::new_parse_err("`dt_start` can not be `None`"));
    }

    if partial_options.week_start.is_none() {
        partial_options.week_start = Some(Weekday::Mon);
    }

    if let Some(by_set_pos) = &partial_options.by_set_pos {
        for pos in by_set_pos {
            if *pos == 0 || !(*pos >= -366 && *pos <= 366) {
                return Err(RRuleError::new_parse_err(
                    "Bysetpos must be between 1 and 366, or between -366 and -1",
                ));
            }
        }
    }

    let dt_start = match partial_options.dt_start {
        Some(dt_start) => Ok(dt_start),
        None => Err(RRuleError::new_parse_err("`dt_start` was not specified")),
    }?;

    // Can only be set to true if feature flag is set.
    let by_easter_is_some = if cfg!(feature = "by-easter") {
        partial_options.by_easter.is_some()
    } else {
        false
    };

    if !(partial_options.by_week_no.is_some()
        || is_some_and_not_empty(&partial_options.by_week_no)
        || is_some_and_not_empty(&partial_options.by_year_day)
        || partial_options.by_month_day.is_some()
        || is_some_and_not_empty(&partial_options.by_month_day)
        || partial_options.by_weekday.is_some()
        || by_easter_is_some)
    {
        match &freq {
            Frequency::Yearly => {
                if partial_options.by_month.is_none() {
                    partial_options.by_month = Some(vec![dt_start.month() as u8]);
                }
                partial_options.by_month_day = Some(vec![dt_start.day() as i8]);
            }
            Frequency::Monthly => {
                partial_options.by_month_day = Some(vec![dt_start.day() as i8]);
            }
            Frequency::Weekly => {
                partial_options.by_weekday = Some(vec![NWeekday::new(
                    dt_start.weekday(),
                    NWeekdayIdentifier::Every,
                )]);
            }
            _ => (),
        };
    }

    match &partial_options.by_month_day {
        None => by_n_month_day = vec![],
        Some(opts_bymonthday) => {
            use std::cmp::Ordering;

            let mut by_month_day = vec![];

            for v in opts_bymonthday {
                match v.cmp(&0) {
                    Ordering::Greater => by_month_day.push(*v),
                    Ordering::Less => by_n_month_day.push(*v),
                    Ordering::Equal => {}
                }
            }

            partial_options.by_month_day = Some(by_month_day);
        }
    }

    let mut by_weekday = vec![];
    let mut by_n_weekday: Vec<Vec<i16>> = vec![];
    // by_weekday / by_n_week_day // ! more to do here

    if let Some(opts_by_weekday) = partial_options.by_weekday {
        for weekday in opts_by_weekday {
            match weekday.n {
                NWeekdayIdentifier::Every => {
                    by_weekday.push(weekday.weekday.num_days_from_monday() as i16)
                }
                NWeekdayIdentifier::Identifier(n) => {
                    by_n_weekday.push(vec![weekday.weekday.num_days_from_monday() as i16, n]);
                }
            }
            // if weekday.n ==  {
            //     by_weekday.push(weekday.weekday);
            // } else {
            //     by_n_weekday.push(vec![weekday.weekday as isize, weekday.n]);
            // }
        }
    }

    // by_hour
    if partial_options.by_hour.is_none() && freq < Frequency::Hourly {
        partial_options.by_hour = Some(vec![dt_start.hour() as u8]);
    }

    // by_minute
    if partial_options.by_minute.is_none() && freq < Frequency::Minutely {
        partial_options.by_minute = Some(vec![dt_start.minute() as u8]);
    }

    // by_second
    if partial_options.by_second.is_none() && freq < Frequency::Secondly {
        partial_options.by_second = Some(vec![dt_start.second() as u8]);
    }

    Ok(ParsedOptions {
        freq,
        interval: partial_options.interval.unwrap(),
        count: partial_options.count,
        until: partial_options.until,
        tz,
        dt_start,
        week_start: partial_options.week_start.unwrap(),
        by_set_pos: partial_options.by_set_pos.unwrap_or_default(),
        by_month: partial_options.by_month.unwrap_or_default(),
        by_month_day: partial_options.by_month_day.unwrap_or_default(),
        by_n_month_day,
        by_year_day: partial_options.by_year_day.unwrap_or_default(),
        by_week_no: partial_options.by_week_no.unwrap_or_default(),
        by_weekday,
        by_n_weekday,
        by_hour: partial_options.by_hour.unwrap_or_default(),
        by_minute: partial_options.by_minute.unwrap_or_default(),
        by_second: partial_options.by_second.unwrap_or_default(),
        by_easter: partial_options.by_easter,
    })
}
