use crate::{core::DateTime, Frequency, NWeekday, RRule, RRuleError, RRuleProperties, RRuleSet};
use chrono::{Datelike, NaiveDate, NaiveDateTime, TimeZone, Timelike, Weekday};
use chrono_tz::{Tz, UTC};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

// Some regex used for parsing the rrule string.
lazy_static! {
    static ref DATESTR_RE: Regex =
        Regex::new(r"(?m)^(\d{4})(\d{2})(\d{2})(T(\d{2})(\d{2})(\d{2})(Z?))?$").unwrap();
    static ref DTSTART_RE: Regex =
        Regex::new(r"(?m)DTSTART(?:;TZID=([^:=]+?))?(?::|=)([^;\s]+)").unwrap();
    static ref RRULE_RE: Regex = Regex::new(r"(?m)^(?:RRULE|EXRULE):").unwrap();
    static ref PARSE_RULE_LINE_RE: Regex = Regex::new(r"(?m)^([A-Z]+?)[:;]").unwrap();
    static ref RDATE_RE: Regex = Regex::new(r"(?m)RDATE(?:;TZID=([^:=]+))?").unwrap();
    static ref EXDATE_RE: Regex = Regex::new(r"(?m)EXDATE(?:;TZID=([^:=]+))?").unwrap();
    static ref DATETIME_RE: Regex = Regex::new(r"(?m)(VALUE=DATE(-TIME)?)|(TZID=)").unwrap();
    static ref NWEEKDAY_REGEX: Regex = Regex::new(r"(?m)^([+-]?\d{1,2})([A-Z]{2})$").unwrap();
}

/// Create [`RRuleSet`] from parsing the String.
pub(crate) fn build_rruleset(s: &str) -> Result<RRuleSet, RRuleError> {
    let s = preprocess_rrule_string(s);
    let ParsedInput {
        rrule_vals,
        rdate_vals,
        exrule_vals,
        exdate_vals,
        dt_start,
        ..
    } = parse_input(&s)?;

    let mut rset = RRuleSet {
        dt_start,
        ..Default::default()
    };

    for rrule_prop in rrule_vals.iter() {
        let parsed_opts = rrule_prop.clone().dt_start(dt_start);
        let rrule = RRule::new(parsed_opts)?;
        rset.rrule(rrule);
    }

    for rdate in rdate_vals {
        rset.rdate(rdate);
    }

    for exrule_prop in exrule_vals.iter() {
        let parsed_opts = exrule_prop.clone().dt_start(dt_start);
        let exrule = RRule::new(parsed_opts)?;
        rset.exrule(exrule);
    }

    for exdate in exdate_vals {
        rset.exdate(exdate);
    }

    Ok(rset)
}

/// Create an [`RRule`] from [`String`] if input is valid.
///
/// If RRule contains invalid parts and [`RRuleError`] will be returned.
/// This should never panic, but it might be in odd cases.
/// Please report if it does panic.
pub(crate) fn parse_rrule_string_to_properties(input: &str) -> Result<RRuleProperties, RRuleError> {
    let input = preprocess_rrule_string(input);

    let ParsedInput {
        mut rrule_vals,
        dt_start,
        ..
    } = parse_input(&input)?;

    match rrule_vals.len() {
        0 => Err(RRuleError::new_parse_err("Invalid rrule string")),
        1 => {
            let rrule_opts = rrule_vals.remove(0);
            let parsed_opts = rrule_opts.dt_start(dt_start);
            Ok(parsed_opts)
        }
        _ => Err(RRuleError::new_parse_err(
            "To many rrules, please use `RRuleSet` instead.",
        )),
    }
}

/// Fill in some additional field in order to make iter work correctly.
pub(crate) fn finalize_parsed_properties(
    mut properties: RRuleProperties,
    dt_start: &DateTime,
) -> Result<RRuleProperties, RRuleError> {
    use std::cmp::Ordering;
    // TEMP: move negative months to other list
    let mut by_month_day = vec![];
    let mut by_n_month_day = properties.by_n_month_day;
    for by_month_day_item in properties.by_month_day {
        match by_month_day_item.cmp(&0) {
            Ordering::Greater => by_month_day.push(by_month_day_item),
            Ordering::Less => by_n_month_day.push(by_month_day_item),
            Ordering::Equal => {}
        }
    }
    properties.by_month_day = by_month_day;
    properties.by_n_month_day = by_n_month_day;

    // Can only be set to true if feature flag is set.
    let by_easter_is_some = if cfg!(feature = "by-easter") {
        properties.by_easter.is_some()
    } else {
        false
    };

    // Add some freq specific additional properties
    if !(!properties.by_week_no.is_empty()
        || !properties.by_year_day.is_empty()
        || !properties.by_month_day.is_empty()
        || !properties.by_n_month_day.is_empty()
        || !properties.by_weekday.is_empty()
        || by_easter_is_some)
    {
        match properties.freq {
            Frequency::Yearly => {
                if properties.by_month.is_empty() {
                    properties.by_month = vec![dt_start.month() as u8];
                }
                properties.by_month_day = vec![dt_start.day() as i8];
            }
            Frequency::Monthly => {
                properties.by_month_day = vec![dt_start.day() as i8];
            }
            Frequency::Weekly => {
                properties.by_weekday = vec![NWeekday::Every(dt_start.weekday())];
            }
            _ => (),
        };
    }

    // by_hour
    if properties.by_hour.is_empty() && properties.freq < Frequency::Hourly {
        properties.by_hour = vec![dt_start.hour() as u8];
    }

    // by_minute
    if properties.by_minute.is_empty() && properties.freq < Frequency::Minutely {
        properties.by_minute = vec![dt_start.minute() as u8];
    }

    // by_second
    if properties.by_second.is_empty() && properties.freq < Frequency::Secondly {
        properties.by_second = vec![dt_start.second() as u8];
    }
    Ok(properties)
}

fn parse_datestring_bit<T: FromStr>(
    bits: &regex::Captures,
    i: usize,
    dt: &str,
) -> Result<T, RRuleError> {
    match bits.get(i) {
        Some(bit) => match bit.as_str().parse::<T>() {
            Err(_) => Err(RRuleError::new_parse_err(format!(
                "Invalid datetime: `{}`",
                dt
            ))),
            Ok(val) => Ok(val),
        },
        _ => Err(RRuleError::new_parse_err(format!(
            "Invalid datetime: `{}`",
            dt
        ))),
    }
}

fn parse_timezone(tz: &str) -> Result<Tz, RRuleError> {
    Tz::from_str(tz)
        .map_err(|_err| RRuleError::new_parse_err(format!("Invalid timezone: `{}`", tz)))
}

fn create_date(dt: &str, year: i32, month: u32, day: u32) -> Result<NaiveDate, RRuleError> {
    match NaiveDate::from_ymd_opt(year, month, day) {
        Some(date) => Ok(date),
        None => Err(RRuleError::new_parse_err(format!(
            "Invalid date in: `{}`",
            dt
        ))),
    }
}

fn create_datetime(
    dt: &str,
    date: &NaiveDate,
    hour: u32,
    min: u32,
    sec: u32,
) -> Result<NaiveDateTime, RRuleError> {
    match date.and_hms_opt(hour, min, sec) {
        Some(datetime) => Ok(datetime),
        None => Err(RRuleError::new_parse_err(format!(
            "Invalid time in: `{}`",
            dt
        ))),
    }
}

fn datestring_to_date(dt: &str, tz: &Option<Tz>) -> Result<DateTime, RRuleError> {
    let bits = DATESTR_RE.captures(dt);
    if bits.is_none() {
        return Err(RRuleError::new_parse_err(format!(
            "Invalid datetime: `{}`",
            dt
        )));
    }
    let bits = bits.expect("This is checked in the lines above.");
    if bits.len() < 3 {
        return Err(RRuleError::new_parse_err(format!(
            "Invalid datetime: `{}`",
            dt
        )));
    }

    // Combine parts to create data time.
    let date = create_date(
        dt,
        parse_datestring_bit(&bits, 1, dt)?,
        parse_datestring_bit(&bits, 2, dt)?,
        parse_datestring_bit(&bits, 3, dt)?,
    )?;
    // Spec defines this is a date-time OR date
    // So the time can will be set to 0:0:0 if only a date is given.
    // https://icalendar.org/iCalendar-RFC-5545/3-8-2-4-date-time-start.html
    let datetime = create_datetime(
        dt,
        &date,
        parse_datestring_bit(&bits, 5, dt).unwrap_or_default(),
        parse_datestring_bit(&bits, 6, dt).unwrap_or_default(),
        parse_datestring_bit(&bits, 7, dt).unwrap_or_default(),
    )?;
    // Apply timezone appended to the datetime before converting to UTC.
    // For more info https://icalendar.org/iCalendar-RFC-5545/3-3-5-date-time.html
    let zulu_timezone_set = match bits.get(8) {
        Some(part) => part.as_str() == "Z",
        None => false,
    };
    let datetime: chrono::DateTime<chrono::Utc> = if zulu_timezone_set {
        // If a `Z` is present, UTC should be used.
        chrono::DateTime::<_>::from_utc(datetime, chrono::Utc)
    } else {
        // If no `Z` is present, local time should be used.
        use chrono::offset::LocalResult;
        // Get datetime in local time or machine local time.
        // So this also takes into account daylight or standard time (summer/winter).
        match tz {
            Some(tz) => {
                // Use the timezone specified in the `tz`
                match tz.from_local_datetime(&datetime) {
                    LocalResult::None => Err(RRuleError::new_parse_err(format!(
                        "Invalid datetime in local timezone: `{}`",
                        dt
                    ))),
                    LocalResult::Single(date) => Ok(date),
                    LocalResult::Ambiguous(date1, date2) => {
                        Err(RRuleError::new_parse_err(format!(
                            "Invalid datetime in local timezone: `{}` \
                        this datetime is ambiguous it can be: `{}` or `{}`",
                            dt, date1, date2
                        )))
                    }
                }?
                .with_timezone(&chrono::Utc)
            }
            None => {
                // Use current system timezone
                // TODO Add option to always use UTC when this is executed on a server.
                let local = chrono::Local;
                match local.from_local_datetime(&datetime) {
                    LocalResult::None => Err(RRuleError::new_parse_err(format!(
                        "Invalid datetime in local timezone: `{}`",
                        dt
                    ))),
                    LocalResult::Single(date) => Ok(date),
                    LocalResult::Ambiguous(date1, date2) => {
                        Err(RRuleError::new_parse_err(format!(
                            "Invalid datetime in local timezone: `{}` \
                        this datetime is ambiguous it can be: `{}` or `{}`",
                            dt, date1, date2
                        )))
                    }
                }?
                .with_timezone(&chrono::Utc)
            }
        }
    };
    let datetime_with_timezone = if let Some(tz) = tz {
        // Apply timezone from `TZID=` part (if any)
        datetime.with_timezone(tz)
    } else {
        // If no timezone is give, set datetime as UTC
        datetime.with_timezone(&UTC)
    };

    Ok(datetime_with_timezone)
}

fn parse_dtstart(s: &str) -> Result<DateTime, RRuleError> {
    let caps = DTSTART_RE.captures(s);

    match caps {
        Some(caps) => {
            let tz: Option<Tz> = match caps.get(1) {
                Some(tz) => Some(parse_timezone(tz.as_str())?),
                None => None,
            };

            let dt_start_str = match caps.get(2) {
                Some(dt) => dt.as_str(),
                None => {
                    return Err(RRuleError::new_parse_err(format!(
                        "Invalid datetime: `{}`",
                        s
                    )))
                }
            };

            datestring_to_date(dt_start_str, &tz)
        }
        None => Err(RRuleError::new_parse_err(format!(
            "Invalid datetime: {}",
            s
        ))),
    }
}

fn from_str_to_freq(s: &str) -> Option<Frequency> {
    match s.to_uppercase().as_str() {
        "YEARLY" => Some(Frequency::Yearly),
        "MONTHLY" => Some(Frequency::Monthly),
        "WEEKLY" => Some(Frequency::Weekly),
        "DAILY" => Some(Frequency::Daily),
        "HOURLY" => Some(Frequency::Hourly),
        "MINUTELY" => Some(Frequency::Minutely),
        "SECONDLY" => Some(Frequency::Secondly),
        _ => None,
    }
}

fn weekday_from_str(val: &str) -> Result<Weekday, String> {
    match val {
        "MO" => Ok(Weekday::Mon),
        "TU" => Ok(Weekday::Tue),
        "WE" => Ok(Weekday::Wed),
        "TH" => Ok(Weekday::Thu),
        "FR" => Ok(Weekday::Fri),
        "SA" => Ok(Weekday::Sat),
        "SU" => Ok(Weekday::Sun),
        _ => Err(format!("Invalid weekday: {}", val)),
    }
}

fn stringval_to_int<T: FromStr>(val: &str, err_msg: String) -> Result<T, RRuleError> {
    if let Ok(val) = val.parse() {
        Ok(val)
    } else {
        Err(RRuleError::new_parse_err(err_msg))
    }
}

fn stringval_to_intvec<T: FromStr + Ord + PartialEq + Copy, F: Fn(T) -> bool>(
    val: &str,
    accept: F,
    err_msg: String,
) -> Result<Vec<T>, RRuleError> {
    let mut parsed_vals = vec![];
    for val in val.split(',') {
        let val = stringval_to_int(val, err_msg.clone())?;
        if accept(val) {
            parsed_vals.push(val);
        } else {
            return Err(RRuleError::new_parse_err(err_msg));
        }
    }

    parsed_vals.sort();
    parsed_vals.dedup();

    Ok(parsed_vals)
}

fn parse_rrule(line: &str, mut dt_start: Option<DateTime>) -> Result<RRuleProperties, RRuleError> {
    // Store all parts independently, so we can see if things are double set or missing.
    let mut freq = None;
    let mut interval = None;
    let mut count = None;
    let mut until = None;
    let mut tz = None;
    // let mut dt_start = None;
    let mut week_start = None;
    let mut by_set_pos = None;
    let mut by_month = None;
    let mut by_month_day = None;
    let mut by_year_day = None;
    let mut by_week_no = None;
    let mut by_weekday = None;
    let mut by_hour = None;
    let mut by_minute = None;
    let mut by_second = None;
    #[allow(unused_mut)]
    let mut by_easter = None;

    let attrs = RRULE_RE.replace(line, "");
    let attrs = attrs.split(';');

    for attr in attrs {
        let l: Vec<&str> = attr.split('=').collect();

        let key = l[0];
        let mut value = "";
        if l.len() > 1 {
            value = l[1];
        }

        match key.to_uppercase().as_str() {
            "FREQ" => match from_str_to_freq(value) {
                Some(new_freq) => {
                    if freq.is_none() {
                        freq = Some(new_freq)
                    } else {
                        return Err(RRuleError::new_parse_err(format!(
                            "`FREQ` was found twice in `{}`",
                            line
                        )));
                    }
                }
                None => {
                    return Err(RRuleError::new_parse_err(format!(
                        "Invalid frequency: `{}`",
                        value
                    )))
                }
            },
            "INTERVAL" => {
                let new_interval = stringval_to_int(value, "Invalid interval".to_owned())?;
                if interval.is_none() {
                    interval = Some(new_interval)
                } else {
                    return Err(RRuleError::new_parse_err(format!(
                        "`INTERVAL` was found twice in `{}`",
                        line
                    )));
                }
            }
            "COUNT" => {
                let new_count = stringval_to_int(value, "Invalid count".to_owned())?;
                if count.is_none() {
                    count = Some(new_count)
                } else {
                    return Err(RRuleError::new_parse_err(format!(
                        "`COUNT` was found twice in `{}`",
                        line
                    )));
                }
            }
            "UNTIL" => {
                // Until is always in UTC
                // TODO: Comment above is not true because of:
                // > [...]
                // > Furthermore, if the "DTSTART" property is specified as a date with local time,
                // > then the UNTIL rule part MUST also be specified as a date with local time.
                //
                // Thus This can be in local time
                if until.is_none() {
                    until = Some(datestring_to_date(value, &Some(UTC))?)
                } else {
                    return Err(RRuleError::new_parse_err(format!(
                        "`WKST` was found twice in `{}`",
                        line
                    )));
                }
            }
            "DTSTART" | "TZID" => {
                // for backwards compatibility
                let dtstart_opts = parse_dtstart(line)?;
                tz = Some(dtstart_opts.timezone());
                dt_start = Some(dtstart_opts);
            }
            "WKST" => match weekday_from_str(value) {
                Ok(new_weekday) => {
                    if week_start.is_none() {
                        week_start = Some(new_weekday)
                    } else {
                        return Err(RRuleError::new_parse_err(format!(
                            "`WKST` was found twice in `{}`",
                            line
                        )));
                    }
                }
                Err(e) => {
                    return Err(RRuleError::new_parse_err(e));
                }
            },
            "BYSETPOS" => {
                let new_by_set_pos =
                    stringval_to_intvec(value, |_pos| true, "Invalid by_set_pos value".to_owned())?;
                if by_set_pos.is_none() {
                    by_set_pos = Some(new_by_set_pos)
                } else {
                    return Err(RRuleError::new_parse_err(format!(
                        "`BYSETPOS` was found twice in `{}`",
                        line
                    )));
                }
            }
            "BYMONTH" => {
                let new_by_month = stringval_to_intvec(
                    value,
                    |month| (1..=12).contains(&month),
                    "Invalid by_month value".to_owned(),
                )?;
                if by_month.is_none() {
                    by_month = Some(new_by_month)
                } else {
                    return Err(RRuleError::new_parse_err(format!(
                        "`BYMONTH` was found twice in `{}`",
                        line
                    )));
                }
            }
            "BYMONTHDAY" => {
                let new_by_month_day = stringval_to_intvec(
                    value,
                    |monthday| (-31..=31).contains(&monthday),
                    "Invalid by_month_day value".to_owned(),
                )?;
                if by_month_day.is_none() {
                    by_month_day = Some(new_by_month_day)
                } else {
                    return Err(RRuleError::new_parse_err(format!(
                        "`BYMONTHDAY` was found twice in `{}`",
                        line
                    )));
                }
            }
            "BYYEARDAY" => {
                let new_by_year_day = stringval_to_intvec(
                    value,
                    |yearday| (-366..=366).contains(&yearday),
                    "Invalid by_year_day value".to_owned(),
                )?;
                if by_year_day.is_none() {
                    by_year_day = Some(new_by_year_day)
                } else {
                    return Err(RRuleError::new_parse_err(format!(
                        "`BYYEARDAY` was found twice in `{}`",
                        line
                    )));
                }
            }
            "BYWEEKNO" => {
                let new_by_week_no = stringval_to_intvec(
                    value,
                    |weekno| (-53..=53).contains(&weekno),
                    "Invalid by_week_no value".to_owned(),
                )?;
                if by_week_no.is_none() {
                    by_week_no = Some(new_by_week_no)
                } else {
                    return Err(RRuleError::new_parse_err(format!(
                        "`BYWEEKNO` was found twice in `{}`",
                        line
                    )));
                }
            }
            "BYHOUR" => {
                let new_by_hour = stringval_to_intvec(
                    value,
                    |hour| hour < 24,
                    "Invalid by_hour value".to_owned(),
                )?;
                if by_hour.is_none() {
                    by_hour = Some(new_by_hour)
                } else {
                    return Err(RRuleError::new_parse_err(format!(
                        "`BYHOUR` was found twice in `{}`",
                        line
                    )));
                }
            }
            "BYMINUTE" => {
                let new_by_minute = stringval_to_intvec(
                    value,
                    |minute| minute < 60,
                    "Invalid by_minute value".to_owned(),
                )?;
                if by_minute.is_none() {
                    by_minute = Some(new_by_minute)
                } else {
                    return Err(RRuleError::new_parse_err(format!(
                        "`BYMINUTE` was found twice in `{}`",
                        line
                    )));
                }
            }
            "BYSECOND" => {
                let new_by_second = stringval_to_intvec(
                    value,
                    |sec| sec < 60,
                    "Invalid by_second value".to_owned(),
                )?;
                if by_second.is_none() {
                    by_second = Some(new_by_second)
                } else {
                    return Err(RRuleError::new_parse_err(format!(
                        "`BYSECOND` was found twice in `{}`",
                        line
                    )));
                }
            }
            "BYWEEKDAY" | "BYDAY" => {
                let new_by_weekday = parse_weekday(value)?;

                if by_weekday.is_none() {
                    by_weekday = Some(new_by_weekday)
                } else {
                    return Err(RRuleError::new_parse_err(format!(
                        "`BYWEEKDAY`/`BYDAY` was found twice in `{}`",
                        line
                    )));
                }
            }
            #[cfg(feature = "by-easter")]
            "BYEASTER" => {
                let new_by_easter =
                    stringval_to_int(value, format!("Invalid by_easter val: {}", value))?;
                if by_easter.is_none() {
                    by_easter = Some(new_by_easter)
                } else {
                    return Err(RRuleError::new_parse_err(format!(
                        "`BYEASTER` was found twice in `{}`",
                        line
                    )));
                }
            }
            _ => {
                return Err(RRuleError::new_parse_err(format!(
                    "Invalid property: {}",
                    key
                )))
            }
        };
    }

    // Check if manditory fields are set
    Ok(RRuleProperties {
        freq: freq.ok_or_else(|| {
            RRuleError::new_parse_err(format!("Property `FREQ` was missing in `{}`", line))
        })?,
        // `1` is default value according to spec.
        interval: interval.unwrap_or(1),
        count,
        until,
        tz: tz.unwrap_or(UTC), // TODO check, I think this should be local timezone
        dt_start: dt_start.unwrap_or_else(|| UTC.ymd(1970, 1, 1).and_hms(0, 0, 0)), // Unix Epoch
        // dt_start: dt_start.ok_or_else(|| {
        //     RRuleError::new_parse_err(format!("Property `DTSTART` was missing in `{}`", line))
        // })?,
        week_start: week_start.unwrap_or(Weekday::Mon),
        by_set_pos: by_set_pos.unwrap_or_default(),
        by_month: by_month.unwrap_or_default(),
        by_month_day: by_month_day.unwrap_or_default(),
        by_n_month_day: vec![],
        by_year_day: by_year_day.unwrap_or_default(),
        by_week_no: by_week_no.unwrap_or_default(),
        by_weekday: by_weekday.unwrap_or_default(),
        by_hour: by_hour.unwrap_or_default(),
        by_minute: by_minute.unwrap_or_default(),
        by_second: by_second.unwrap_or_default(),
        by_easter,
    })
}

fn str_to_weekday(d: &str) -> Result<Weekday, RRuleError> {
    match d.to_uppercase().as_str() {
        "MO" => Ok(Weekday::Mon),
        "TU" => Ok(Weekday::Tue),
        "WE" => Ok(Weekday::Wed),
        "TH" => Ok(Weekday::Thu),
        "FR" => Ok(Weekday::Fri),
        "SA" => Ok(Weekday::Sat),
        "SU" => Ok(Weekday::Sun),
        _ => Err(RRuleError::new_parse_err(format!("Invalid weekday: {}", d))),
    }
}

/// Parse the "BYWEEKDAY" and "BYDAY" values
/// Example: `SU,MO,TU,WE,TH,FR` or `4MO` or `-1WE`
/// > For example, within a MONTHLY rule, +1MO (or simply 1MO) represents the first Monday
/// > within the month, whereas -1MO represents the last Monday of the month.
fn parse_weekday(val: &str) -> Result<Vec<NWeekday>, RRuleError> {
    let mut wdays = vec![];
    // Separate all days
    for day in val.split(',') {
        // Each day is 2 characters long
        if day.len() == 2 {
            // MO, TU, ...
            let wday = str_to_weekday(day)?;
            wdays.push(NWeekday::new(None, wday));
        } else {
            // When a day has values in front or behind it
            // Parse `4MO` and `-1WE`
            match NWEEKDAY_REGEX.captures(day) {
                Some(parts) => {
                    // Will only panic when regex is incorrect
                    let number = parts.get(1).unwrap().as_str().parse().unwrap();
                    let wdaypart = parts.get(2).unwrap();
                    let wday = str_to_weekday(wdaypart.as_str())?;
                    wdays.push(NWeekday::new(Some(number), wday));
                }
                None => {
                    return Err(RRuleError::new_parse_err(format!(
                        "Invalid weekday selection: {}",
                        day
                    )));
                }
            }
        }
    }
    Ok(wdays)
}

fn parse_rule_line(rfc_string: &str) -> Result<Option<RRuleProperties>, RRuleError> {
    let rfc_string = rfc_string.trim();
    // If this part is empty return
    if rfc_string.is_empty() {
        return Ok(None);
    }

    let rfc_string_upper = rfc_string.to_uppercase();
    // Get header, `RRULE:` or `EXRULE;` part.
    let header = PARSE_RULE_LINE_RE.captures(&rfc_string_upper);

    if let Some(header) = header {
        let key = match header.get(1) {
            Some(k) => k.as_str(),
            None => {
                return Err(RRuleError::new_parse_err(format!(
                    "Invalid rule line prefix: {}",
                    rfc_string
                )))
            }
        };

        match key {
            "EXRULE" | "RRULE" => Ok(Some(parse_rrule(rfc_string, None)?)),
            "DTSTART" => Ok(Some(RRuleProperties::new(
                Frequency::Yearly, // TODO this value should not be a default
                parse_dtstart(rfc_string)?,
            ))),
            _ => Err(RRuleError::new_parse_err(format!(
                "Unsupported RFC prop {} in {}",
                key, &rfc_string
            ))),
        }
    } else {
        // If no header is set, we can parse it as `RRULE`
        Ok(Some(parse_rrule(rfc_string, None)?))
    }
}

#[derive(Debug)]
struct ParsedLine {
    name: String,
    params: Vec<String>,
    value: String,
}

fn break_down_line(line: &str) -> ParsedLine {
    let parsed_line_name = extract_name(String::from(line));
    let params: Vec<&str> = parsed_line_name.name.split(';').collect();

    ParsedLine {
        name: params[0].to_uppercase(),
        params: params[1..].iter().map(|s| String::from(*s)).collect(),
        value: parsed_line_name.value,
    }
}

struct LineName {
    name: String,
    value: String,
}

fn extract_name(line: String) -> LineName {
    if !line.contains(':') {
        return LineName {
            name: String::from("RRULE"),
            value: line,
        };
    }

    let parts: Vec<&str> = line.split(':').collect();
    let name = parts[0];
    let value = parts[1..].join("");

    LineName {
        name: String::from(name),
        value,
    }
}

fn parse_rule(rfc_string: &str) -> Result<RRuleProperties, RRuleError> {
    let mut option = None;
    for line in rfc_string.split('\n') {
        let parsed_line = parse_rule_line(line)?;
        if let Some(parsed_line) = parsed_line {
            if option.is_none() {
                option = Some(parsed_line);
            } else {
                return Err(RRuleError::new_parse_err(format!(
                    "Found to many RRule lines in `{}`.",
                    rfc_string
                )));
            }
        }
    }

    if let Some(option) = option {
        Ok(option)
    } else {
        Err(RRuleError::new_parse_err(format!(
            "String is not a valid RRule: `{}`.",
            rfc_string
        )))
    }
}

#[derive(Debug)]
struct ParsedInput {
    rrule_vals: Vec<RRuleProperties>,
    rdate_vals: Vec<DateTime>,
    exrule_vals: Vec<RRuleProperties>,
    exdate_vals: Vec<DateTime>,
    dt_start: DateTime,
    // TODO: Why is this field never used?
    #[allow(dead_code)]
    tz: Tz,
}

fn parse_input(s: &str) -> Result<ParsedInput, RRuleError> {
    let mut rrule_vals = vec![];
    let mut rdate_vals = vec![];
    let mut exrule_vals = vec![];
    let mut exdate_vals = vec![];
    let dt_start = parse_dtstart(s)?;
    let tz = dt_start.timezone();

    let lines: Vec<&str> = s.split('\n').collect();
    for line in &lines {
        let parsed_line = break_down_line(line);
        match parsed_line.name.to_uppercase().as_str() {
            "RRULE" => {
                if !parsed_line.params.is_empty() {
                    return Err(RRuleError::new_parse_err(
                        "Unsupported RRULE value".to_owned(),
                    ));
                }
                if parsed_line.value.is_empty() {
                    continue;
                }

                rrule_vals.push(finalize_parsed_properties(parse_rule(line)?, &dt_start)?);
            }
            "EXRULE" => {
                if !parsed_line.params.is_empty() {
                    return Err(RRuleError::new_parse_err(
                        "Unsupported EXRULE value".to_owned(),
                    ));
                }
                if parsed_line.value.is_empty() {
                    continue;
                }
                // TODO: why is it parsed_line.value here and line for RRULE ?? Do some testing
                exrule_vals.push(finalize_parsed_properties(
                    parse_rule(&parsed_line.value)?,
                    &dt_start,
                )?);
            }
            "RDATE" => {
                let matches = match RDATE_RE.captures(line) {
                    Some(m) => m,
                    None => {
                        return Err(RRuleError::new_parse_err(
                            "Invalid RDATE specified".to_owned(),
                        ))
                    }
                };
                let tz: Option<Tz> = match matches.get(1) {
                    Some(tz_str) => Some(parse_timezone(tz_str.as_str())?),
                    None => None,
                };

                rdate_vals.append(&mut parse_rdate(
                    &parsed_line.value,
                    parsed_line.params,
                    &tz,
                )?);
            }
            "EXDATE" => {
                let matches = match EXDATE_RE.captures(line) {
                    Some(m) => m,
                    None => {
                        return Err(RRuleError::new_parse_err(
                            "Invalid EXDATE specified".to_owned(),
                        ))
                    }
                };
                let tz: Option<Tz> = match matches.get(1) {
                    Some(tz_str) => Some(parse_timezone(tz_str.as_str())?),
                    None => None,
                };
                exdate_vals.append(&mut parse_rdate(
                    &parsed_line.value,
                    parsed_line.params,
                    &tz,
                )?);
            }
            "DTSTART" => (),
            _ => {
                return Err(RRuleError::new_parse_err(format!(
                    "Unsupported property: {}",
                    parsed_line.name
                )))
            }
        }
    }

    Ok(ParsedInput {
        dt_start,
        tz,
        rrule_vals,
        rdate_vals,
        exrule_vals,
        exdate_vals,
    })
}

fn validate_date_param(params: Vec<&str>) -> Result<(), RRuleError> {
    for param in &params {
        match DATETIME_RE.captures(param) {
            Some(caps) if caps.len() > 0 => (),
            _ => {
                return Err(RRuleError::new_parse_err(format!(
                    "Unsupported RDATE/EXDATE parm: {}",
                    param
                )))
            }
        }
    }
    Ok(())
}

fn parse_rdate(
    rdateval: &str,
    params: Vec<String>,
    tz: &Option<Tz>,
) -> Result<Vec<DateTime>, RRuleError> {
    let params: Vec<&str> = params.iter().map(|p| p.as_str()).collect();
    validate_date_param(params)?;

    let mut rdatevals = vec![];
    for datestr in rdateval.split(',') {
        rdatevals.push(datestring_to_date(datestr, tz)?);
    }

    Ok(rdatevals)
}

fn preprocess_rrule_string(s: &str) -> String {
    s.replace("DTSTART;VALUE=DATETIME", "DTSTART")
        .replace("DTSTART;VALUE=DATE", "DTSTART")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::DateFilter;

    /// Print and compare 2 lists of dates and panic it they are not the same.
    fn check_occurrences(occurrences: Vec<DateTime>, expected: Vec<&str>) {
        let formater = |dt: &DateTime| -> String { format!("    \"{}\",\n", dt.to_rfc3339()) };
        println!(
            "Given: [\n{}]\nExpected: {:#?}",
            occurrences
                .iter()
                .map(formater)
                .collect::<Vec<_>>()
                .join(""),
            expected
        );
        assert_eq!(occurrences.len(), expected.len(), "List sizes don't match");
        for (given, expected) in occurrences.iter().zip(expected.iter()) {
            let exp_datetime = chrono::DateTime::parse_from_rfc3339(expected).unwrap();
            // Compare items and check if in the same offset/timezone
            assert_eq!(
                given.to_rfc3339(),
                exp_datetime.to_rfc3339(),
                "Dates not in same timezone"
            );
        }
    }

    #[test]
    fn it_works_1() {
        let res = build_rruleset("DTSTART:19970902T090000Z\nRRULE:FREQ=YEARLY;COUNT=3\n");
        assert!(res.is_ok());
    }

    #[test]
    fn it_works_2() {
        let res = parse_rrule_string_to_properties("DTSTART:20120201T093000Z\nRRULE:FREQ=WEEKLY;INTERVAL=5;UNTIL=20130130T230000Z;BYDAY=MO,FR");
        assert!(res.is_ok());
    }

    #[test]
    fn it_works_3() {
        let res = build_rruleset("RRULE:UNTIL=19990404T110000Z;DTSTART;TZID=America/Denver:19990104T110000Z;FREQ=WEEKLY;BYDAY=TU,WE");
        assert!(res.is_ok());
    }

    #[test]
    fn it_works_4() {
        let res = build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5\nEXDATE;TZID=Europe/Berlin:20120202T130000Z,20120203T130000Z");
        assert!(res.is_ok());
    }

    #[test]
    fn rrule() {
        let res = build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5");
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.rrule.len(), 1);
        assert_eq!(res.rrule[0].get_properties().interval, 1);
        assert_eq!(res.rrule[0].get_properties().count.unwrap(), 5);
        assert_eq!(res.rrule[0].get_properties().freq, Frequency::Daily);
    }

    #[test]
    fn exrule() {
        let res = build_rruleset(
            "DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5\nEXRULE:FREQ=WEEKLY;INTERVAL=2",
        );
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.exrule.len(), 1);
        assert_eq!(res.exrule[0].get_properties().interval, 2);
        assert_eq!(res.exrule[0].get_properties().freq, Frequency::Weekly);
    }

    ////////////////////////////////////////////////////
    // Invalid stuff
    ////////////////////////////////////////////////////
    #[test]
    fn garbage_strings_rrule() {
        let test_cases = vec![
            "",
            "!",
            "1",
            "fioashfoias!?",
            "        ",
            "helloworld",
            "foo bar",
            "hello\nworld",
            "RRUle:test",
        ];
        for test_case in &test_cases {
            let res = parse_rrule_string_to_properties(test_case);
            assert!(res.is_err());
        }
    }

    #[test]
    fn garbage_strings_rrule_set() {
        let test_cases = vec!["helloworld", "foo bar", "hello\nworld", "RRUle:test"];
        for test_case in &test_cases {
            let res = build_rruleset(test_case);
            assert!(res.is_err());
        }
    }

    #[test]
    fn invalid_dtstart() {
        let res = build_rruleset("DTSTART:20120201120000Z\nRRULE:FREQ=DAILY;COUNT=5");
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            RRuleError::new_parse_err("Invalid datetime: `20120201120000Z`")
        );
    }

    #[test]
    fn invalid_freq() {
        let res = build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAIL;COUNT=5");
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            RRuleError::new_parse_err("Invalid frequency: `DAIL`")
        );
    }

    #[test]
    fn invalid_byhour() {
        let res = build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYHOUR=24");
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            RRuleError::new_parse_err("Invalid by_hour value")
        );

        let res =
            build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYHOUR=5,6,25");
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            RRuleError::new_parse_err("Invalid by_hour value")
        );
    }

    #[test]
    fn invalid_byminute() {
        let res = build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYMINUTE=60");
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            RRuleError::new_parse_err("Invalid by_minute value")
        );

        let res =
            build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYMINUTE=4,5,64");
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            RRuleError::new_parse_err("Invalid by_minute value")
        );
    }

    #[test]
    fn parses_dtstart_when_just_date() {
        let res = build_rruleset("DTSTART;VALUE=DATE:20200812\nRRULE:FREQ=WEEKLY;UNTIL=20210511T220000Z;INTERVAL=1;BYDAY=WE;WKST=MO");
        assert!(res.is_ok());
    }

    #[test]
    fn parses_byday_as_nweekday_when_n_is_first() {
        let res = parse_rrule_string_to_properties("DTSTART;VALUE=DATE:20200701\nRRULE:FREQ=MONTHLY;UNTIL=20210303T090000Z;INTERVAL=1;BYDAY=1WE").unwrap();
        assert_eq!(res.by_weekday, vec![NWeekday::new(Some(1), Weekday::Wed)]);
    }

    #[test]
    fn parses_byday_with_n() {
        let cases = vec![
            "DTSTART:20200901T174500\nRRULE:FREQ=MONTHLY;UNTIL=20210504T154500Z;INTERVAL=1;BYDAY=1TU",
            "DTSTART;VALUE=DATE:20200902\nRRULE:FREQ=MONTHLY;UNTIL=20210504T220000Z;INTERVAL=1;BYDAY=1WE",
            "DTSTART:20200902T100000\nRRULE:FREQ=MONTHLY;UNTIL=20210505T080000Z;INTERVAL=1;BYDAY=1WE",
            "DTSTART;VALUE=DATE:20200812\nRRULE:FREQ=MONTHLY;UNTIL=20210524T090000Z;INTERVAL=1;BYDAY=4MO"
        ];
        for case in &cases {
            let res = build_rruleset(case);
            assert!(res.is_ok());
        }
        let cases = vec![
            "RRULE:FREQ=MONTHLY;UNTIL=20210504T154500Z;INTERVAL=1;BYDAY=1TU",
            "RRULE:FREQ=MONTHLY;UNTIL=20210504T220000Z;INTERVAL=1;BYDAY=1WE",
            "RRULE:FREQ=MONTHLY;UNTIL=20210505T080000Z;INTERVAL=1;BYDAY=-1WE",
            "RRULE:FREQ=MONTHLY;UNTIL=20210505T080000Z;INTERVAL=1;BYDAY=12SU",
            "RRULE:FREQ=MONTHLY;UNTIL=20210524T090000Z;INTERVAL=1;BYDAY=+4MO",
        ];
        let opts = vec![
            vec![NWeekday::new(Some(1), Weekday::Tue)],
            vec![NWeekday::new(Some(1), Weekday::Wed)],
            vec![NWeekday::new(Some(-1), Weekday::Wed)],
            vec![NWeekday::new(Some(12), Weekday::Sun)],
            vec![NWeekday::new(Some(4), Weekday::Mon)],
        ];
        for i in 0..cases.len() {
            let opts_or_err = parse_rule(cases[i]);
            assert!(opts_or_err.is_ok());
            assert_eq!(opts_or_err.unwrap().by_weekday, opts[i]);
        }
    }

    #[test]
    // #[ignore = "Only for benching"]
    fn bench() {
        let now = std::time::SystemTime::now();
        for _ in 0..1000 {
            // let res = build_rruleset("RRULE:UNTIL=19990404T110000Z;\
            // DTSTART;TZID=America/New_York:19990104T110000Z;FREQ=WEEKLY;BYDAY=TU,WE").unwrap();
            let res = build_rruleset(
                "RRULE:UNTIL=20100404T110000Z;\
                DTSTART;TZID=America/New_York:19990104T110000Z;FREQ=WEEKLY;BYDAY=TU,WE",
            )
            .unwrap();

            // println!("Parsing took: {:?}", now.elapsed().unwrap().as_millis());
            let tmp_now = std::time::SystemTime::now();

            // res.all(50);
            res.all_between(
                UTC.timestamp_millis(915321600000),
                UTC.timestamp_millis(920505600000),
                true,
            )
            .unwrap();
            println!("All took: {:?}", tmp_now.elapsed().unwrap().as_nanos());
        }
        println!("Time took: {:?}", now.elapsed().unwrap().as_millis());
    }

    #[test]
    #[ignore = "`dt_start` should be set, although error message is incorrect."]
    fn parses_rrule_without_dtstart() {
        let res = parse_rrule_string_to_properties("FREQ=DAILY;COUNT=7");
        println!("Res: {:?}", res);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.count, Some(7));
        assert_eq!(res.freq, Frequency::Daily);
        assert!(chrono::Utc::now().timestamp() - res.dt_start.timestamp() < 2);

        let res = build_rruleset("FREQ=DAILY;COUNT=7");
        assert!(res.is_ok());
        let occurrences = res.unwrap().all(50).unwrap();
        assert_eq!(occurrences.len(), 7);
        assert!(chrono::Utc::now().timestamp() - occurrences[0].timestamp() < 2);
    }

    #[test]
    fn avoids_infinite_loop() {
        let rrule = "DTSTART:20200427T090000\n\
            FREQ=WEEKLY;UNTIL=20200506T035959Z;BYDAY=FR,MO,TH,TU,WE"
            .parse::<RRule>()
            .unwrap();
        let instances = rrule
            .into_iter()
            .skip_while(|d| *d < chrono::Local::now())
            .take(2);
        assert_eq!(instances.count(), 0);
    }

    #[test]
    fn daytime_savings() {
        let rrule: RRule =
            "DTSTART;TZID=America/Vancouver:20210301T022210\nRRULE:FREQ=DAILY;COUNT=30"
                .parse()
                .unwrap();

        let (dates, error) = rrule.all_with_error(60);
        check_occurrences(
            dates,
            vec![
                "2021-03-01T02:22:10-08:00",
                "2021-03-02T02:22:10-08:00",
                "2021-03-03T02:22:10-08:00",
                "2021-03-04T02:22:10-08:00",
                "2021-03-05T02:22:10-08:00",
                "2021-03-06T02:22:10-08:00",
                "2021-03-07T02:22:10-08:00",
                "2021-03-08T02:22:10-08:00",
                "2021-03-09T02:22:10-08:00",
                "2021-03-10T02:22:10-08:00",
                "2021-03-11T02:22:10-08:00",
                "2021-03-12T02:22:10-08:00",
                "2021-03-13T02:22:10-08:00",
                "2021-03-14T03:22:10-07:00",
                "2021-03-15T02:22:10-07:00",
                "2021-03-16T02:22:10-07:00",
                "2021-03-17T02:22:10-07:00",
                "2021-03-18T02:22:10-07:00",
                "2021-03-19T02:22:10-07:00",
                "2021-03-20T02:22:10-07:00",
                "2021-03-21T02:22:10-07:00",
                "2021-03-22T02:22:10-07:00",
                "2021-03-23T02:22:10-07:00",
                "2021-03-24T02:22:10-07:00",
                "2021-03-25T02:22:10-07:00",
                "2021-03-26T02:22:10-07:00",
                "2021-03-27T02:22:10-07:00",
                "2021-03-28T02:22:10-07:00",
                "2021-03-29T02:22:10-07:00",
                "2021-03-30T02:22:10-07:00",
            ],
        );
        assert!(error.is_none());
    }

    #[test]
    fn rrule_all_fails_with_panic() {
        let res = "DTSTART;VALUE=DATE:20201230T130000\n\
        RRULE:FREQ=MONTHLY;UNTIL=20210825T120000Z;INTERVAL=1;BYDAY=-1WE"
            .parse::<RRuleSet>()
            .unwrap()
            .all(50);
        println!("Res {:?}", res);
    }

    #[test]
    fn rrule_generates_recurring_filter() {
        let dates = "DTSTART;TZID=Europe/Paris:20201214T093000\n\
        RRULE:FREQ=WEEKLY;UNTIL=20210308T083000Z;INTERVAL=2;BYDAY=MO;WKST=MO\n\
        EXDATE;TZID=Europe/Paris:20201228T093000,20210125T093000,20210208T093000"
            .parse::<RRuleSet>()
            .unwrap()
            .all(50)
            .unwrap();
        // This results in following set (minus exdate)
        // [
        //     2020-12-14T09:30:00CET,
        //     2020-12-28T09:30:00CET, // Removed because of exdate
        //     2021-01-11T09:30:00CET,
        //     2021-01-25T09:30:00CET, // Removed because of exdate
        //     2021-02-08T09:30:00CET, // Removed because of exdate
        //     2021-02-22T09:30:00CET,
        //     2021-03-08T09:30:00CET, // same as `UNTIL` but different timezones
        // ]
        check_occurrences(
            dates,
            vec![
                "2020-12-14T09:30:00+01:00",
                "2021-01-11T09:30:00+01:00",
                "2021-02-22T09:30:00+01:00",
                "2021-03-08T09:30:00+01:00",
            ],
        )
    }

    #[test]
    fn test_zulu() {
        let rrule_str = "DTSTART:20210405T150000Z\nRRULE:FREQ=WEEKLY;INTERVAL=1;BYDAY=MO";
        let rrule: RRule = rrule_str.parse().unwrap();
        assert_eq!(rrule.get_properties().freq, Frequency::Weekly);
        assert_eq!(
            rrule.get_properties().by_weekday,
            vec![NWeekday::new(None, Weekday::Mon)]
        );
        assert_eq!(rrule.get_properties().interval, 1);
        assert_eq!(
            rrule.get_properties().dt_start,
            UTC.ymd(2021, 4, 5).and_hms(15, 0, 0)
        );
    }

    #[test]
    fn rrule_daylight_savings() {
        let dates = "DTSTART;TZID=Europe/Paris:20210214T093000\n\
        RRULE:FREQ=WEEKLY;UNTIL=20210508T083000Z;INTERVAL=2;BYDAY=MO;WKST=MO"
            .parse::<RRuleSet>()
            .unwrap()
            .all(50)
            .unwrap();
        check_occurrences(
            dates,
            vec![
                "2021-02-22T09:30:00+01:00",
                "2021-03-08T09:30:00+01:00",
                "2021-03-22T09:30:00+01:00",
                "2021-04-05T09:30:00+02:00", // Switching to daylight saving time.
                "2021-04-19T09:30:00+02:00",
                "2021-05-03T09:30:00+02:00",
            ],
        )
    }
}
