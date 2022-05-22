use super::ParseError;
use crate::core::Unvalidated;
use crate::{core::DateTime, Frequency, NWeekday, RRule, RRuleError, RRuleSet};
use chrono::{Datelike, NaiveDate, TimeZone, Timelike, Weekday};
use chrono_tz::{Tz, UTC};
use lazy_static::lazy_static;
use regex::Regex;
use std::marker::PhantomData;
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
}

/// Creates [`RRuleSet`] from parsing the String.
pub(crate) fn build_rruleset(s: &str) -> Result<RRuleSet, RRuleError> {
    let s = preprocess_rrule_string(s);
    let ParsedInput {
        rrule_vals,
        rdate_vals,
        exrule_vals,
        exdate_vals,
        dt_start,
    } = parse_input(&s)?;

    let mut rset = RRuleSet::new(dt_start);

    for rrule_prop in rrule_vals {
        rset.rrule(rrule_prop.validate(dt_start)?);
    }

    for rdate in rdate_vals {
        rset.rdate(rdate);
    }

    for exrule_prop in exrule_vals {
        let exrule = exrule_prop.validate(dt_start)?;
        #[allow(deprecated)]
        rset.exrule(exrule);
    }

    for exdate in exdate_vals {
        rset.exdate(exdate);
    }

    Ok(rset)
}

/// Fills in some additional fields in order to make iter work correctly.
#[allow(clippy::cast_possible_truncation)]
pub(crate) fn finalize_parsed_rrule(
    mut rrule: RRule<Unvalidated>,
    dt_start: &DateTime,
) -> RRule<Unvalidated> {
    use std::cmp::Ordering;
    // TEMP: move negative months to other list
    let mut by_month_day = vec![];
    let mut by_n_month_day = rrule.by_n_month_day;
    for by_month_day_item in rrule.by_month_day {
        match by_month_day_item.cmp(&0) {
            Ordering::Greater => by_month_day.push(by_month_day_item),
            Ordering::Less => by_n_month_day.push(by_month_day_item),
            Ordering::Equal => {}
        }
    }
    rrule.by_month_day = by_month_day;
    rrule.by_n_month_day = by_n_month_day;

    // Can only be set to true if feature flag is set.
    let by_easter_is_some = if cfg!(feature = "by-easter") {
        rrule.by_easter.is_some()
    } else {
        false
    };

    // Add some freq specific additional properties
    if !(!rrule.by_week_no.is_empty()
        || !rrule.by_year_day.is_empty()
        || !rrule.by_month_day.is_empty()
        || !rrule.by_n_month_day.is_empty()
        || !rrule.by_weekday.is_empty()
        || by_easter_is_some)
    {
        match rrule.freq {
            Frequency::Yearly => {
                if rrule.by_month.is_empty() {
                    rrule.by_month = vec![dt_start.month() as u8];
                }
                rrule.by_month_day = vec![dt_start.day() as i8];
            }
            Frequency::Monthly => {
                rrule.by_month_day = vec![dt_start.day() as i8];
            }
            Frequency::Weekly => {
                rrule.by_weekday = vec![NWeekday::Every(dt_start.weekday())];
            }
            _ => (),
        };
    }

    // by_hour
    if rrule.by_hour.is_empty() && rrule.freq < Frequency::Hourly {
        rrule.by_hour = vec![dt_start.hour() as u8];
    }

    // by_minute
    if rrule.by_minute.is_empty() && rrule.freq < Frequency::Minutely {
        rrule.by_minute = vec![dt_start.minute() as u8];
    }

    // by_second
    if rrule.by_second.is_empty() && rrule.freq < Frequency::Secondly {
        rrule.by_second = vec![dt_start.second() as u8];
    }
    rrule
}

fn parse_datestring_bit<T: FromStr>(
    bits: &regex::Captures,
    i: usize,
    dt: &str,
) -> Result<T, ParseError> {
    match bits.get(i) {
        Some(bit) => bit
            .as_str()
            .parse::<T>()
            .map_err(|_| ParseError::InvalidDateTime {
                value: dt.into(),
                field: "DTSTART".into(),
            }),
        _ => Err(ParseError::InvalidDateTime {
            value: dt.into(),
            field: "DTSTART".into(),
        }),
    }
}

fn parse_timezone(tz: &str) -> Result<Tz, ParseError> {
    Tz::from_str(tz).map_err(|_err| ParseError::InvalidTimezone(tz.into()))
}

pub(crate) fn datestring_to_date(
    dt: &str,
    tz: Option<Tz>,
    field: &str,
) -> Result<DateTime, ParseError> {
    let bits = DATESTR_RE
        .captures(dt)
        .ok_or_else(|| ParseError::InvalidDateTime {
            value: dt.into(),
            field: field.into(),
        })?;
    if bits.len() < 3 {
        return Err(ParseError::InvalidDateTime {
            value: dt.into(),
            field: field.into(),
        });
    }

    let year = parse_datestring_bit(&bits, 1, dt)?;
    let month = parse_datestring_bit(&bits, 2, dt)?;
    let day = parse_datestring_bit(&bits, 3, dt)?;
    // Combine parts to create data time.
    let date =
        NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| ParseError::InvalidDateTime {
            value: dt.into(),
            field: field.into(),
        })?;

    // Spec defines this is a date-time OR date
    // So the time can will be set to 0:0:0 if only a date is given.
    // https://icalendar.org/iCalendar-RFC-5545/3-8-2-4-date-time-start.html
    let hour = parse_datestring_bit(&bits, 5, dt).unwrap_or_default();
    let min = parse_datestring_bit(&bits, 6, dt).unwrap_or_default();
    let sec = parse_datestring_bit(&bits, 7, dt).unwrap_or_default();
    let datetime = date
        .and_hms_opt(hour, min, sec)
        .ok_or_else(|| ParseError::InvalidDateTime {
            value: dt.into(),
            field: field.into(),
        })?;

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
                    LocalResult::None => Err(ParseError::InvalidDateTimeInLocalTimezone {
                        value: dt.into(),
                        field: field.into(),
                    }),
                    LocalResult::Single(date) => Ok(date),
                    LocalResult::Ambiguous(date1, date2) => {
                        Err(ParseError::DateTimeInLocalTimezoneIsAmbiguous {
                            value: dt.into(),
                            field: field.into(),
                            date1: date1.to_rfc3339(),
                            date2: date2.to_rfc3339(),
                        })
                    }
                }?
                .with_timezone(&chrono::Utc)
            }
            None => {
                // Use current system timezone
                // TODO Add option to always use UTC when this is executed on a server.
                let local = chrono::Local;
                match local.from_local_datetime(&datetime) {
                    LocalResult::None => Err(ParseError::InvalidDateTimeInLocalTimezone {
                        value: dt.into(),
                        field: field.into(),
                    }),
                    LocalResult::Single(date) => Ok(date),
                    LocalResult::Ambiguous(date1, date2) => {
                        Err(ParseError::DateTimeInLocalTimezoneIsAmbiguous {
                            value: dt.into(),
                            field: field.into(),
                            date1: date1.to_rfc3339(),
                            date2: date2.to_rfc3339(),
                        })
                    }
                }?
                .with_timezone(&chrono::Utc)
            }
        }
    };

    // Apply timezone from `TZID=` part (if any), else set datetime as UTC
    let datetime_with_timezone = datetime.with_timezone(&tz.unwrap_or(UTC));

    Ok(datetime_with_timezone)
}

pub(crate) fn parse_dtstart(s: &str) -> Result<DateTime, ParseError> {
    let caps = DTSTART_RE.captures(s).ok_or(ParseError::MissingStartDate)?;

    let tz: Option<Tz> = match caps.get(1) {
        Some(tz) => Some(parse_timezone(tz.as_str())?),
        None => None,
    };

    let dt_start_str =
        caps.get(2)
            .map(|dt| dt.as_str())
            .ok_or_else(|| ParseError::InvalidDateTime {
                value: s.into(),
                field: "DTSTART".into(),
            })?;

    datestring_to_date(dt_start_str, tz, "DTSTART")
}

fn stringval_to_int<T: FromStr>(val: &str, err_msg: String) -> Result<T, ParseError> {
    if let Ok(val) = val.parse() {
        Ok(val)
    } else {
        Err(ParseError::Generic(err_msg))
    }
}

fn stringval_to_intvec<T: FromStr + Ord + PartialEq + Copy, F: Fn(T) -> bool>(
    val: &str,
    accept: F,
    err_msg: String,
) -> Result<Vec<T>, ParseError> {
    let mut parsed_vals = vec![];
    for val in val.split(',') {
        let val = stringval_to_int(val, err_msg.clone())?;
        if accept(val) {
            parsed_vals.push(val);
        } else {
            return Err(ParseError::Generic(err_msg));
        }
    }

    parsed_vals.sort();
    parsed_vals.dedup();

    Ok(parsed_vals)
}

// TODO too many lines
#[warn(clippy::too_many_lines)]
fn parse_rrule(line: &str) -> Result<RRule<Unvalidated>, ParseError> {
    // Store all parts independently, so we can see if things are double set or missing.
    let mut freq = None;
    let mut interval = None;
    let mut count = None;
    let mut until = None;
    // let mut tz = None;
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
            "FREQ" => {
                let new_freq = Frequency::from_str(value)?;
                if freq.is_none() {
                    freq = Some(new_freq);
                } else {
                    return Err(ParseError::DuplicatedField("FREQ".into())).map_err(From::from);
                }
            }
            "INTERVAL" => {
                let new_interval = stringval_to_int(value, "Invalid interval".to_owned())?;
                if interval.is_none() {
                    interval = Some(new_interval);
                } else {
                    return Err(ParseError::DuplicatedField("INTERVAL".into())).map_err(From::from);
                }
            }
            "COUNT" => {
                let new_count = stringval_to_int(value, "Invalid count".to_owned())?;
                if count.is_none() {
                    count = Some(new_count);
                } else {
                    return Err(ParseError::DuplicatedField("COUNT".into())).map_err(From::from);
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
                    until = Some(datestring_to_date(value, Some(UTC), "UNTIL")?);
                } else {
                    return Err(ParseError::DuplicatedField("UNTIL".into())).map_err(From::from);
                }
            }
            "DTSTART" | "TZID" => {
                // for backward compatibility
                // let dtstart_opts = parse_dtstart(line)?;
                // tz = Some(dtstart_opts.timezone());
                // dt_start = Some(dtstart_opts);
            }
            "WKST" => match value.parse::<NWeekday>() {
                Ok(new_weekday) => {
                    if week_start.is_none() {
                        let wd = match new_weekday {
                            NWeekday::Every(wd) => wd,
                            NWeekday::Nth(_n, wd) => wd,
                        };
                        week_start = Some(wd);
                    } else {
                        return Err(ParseError::DuplicatedField("WKST".into())).map_err(From::from);
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            },
            "BYSETPOS" => {
                let new_by_set_pos =
                    stringval_to_intvec(value, |_pos| true, "Invalid by_set_pos value".to_owned())?;
                if by_set_pos.is_none() {
                    by_set_pos = Some(new_by_set_pos);
                } else {
                    return Err(ParseError::DuplicatedField("BYSETPOS".into())).map_err(From::from);
                }
            }
            "BYMONTH" => {
                let new_by_month = stringval_to_intvec(
                    value,
                    |month| (1..=12).contains(&month),
                    "Invalid by_month value".to_owned(),
                )?;
                if by_month.is_none() {
                    by_month = Some(new_by_month);
                } else {
                    return Err(ParseError::DuplicatedField("BYMONTH".into())).map_err(From::from);
                }
            }
            "BYMONTHDAY" => {
                let new_by_month_day = stringval_to_intvec(
                    value,
                    |monthday| (-31..=31).contains(&monthday),
                    "Invalid by_month_day value".to_owned(),
                )?;
                if by_month_day.is_none() {
                    by_month_day = Some(new_by_month_day);
                } else {
                    return Err(ParseError::DuplicatedField("BYMONTHDAY".into()))
                        .map_err(From::from);
                }
            }
            "BYYEARDAY" => {
                let new_by_year_day = stringval_to_intvec(
                    value,
                    |yearday| (-366..=366).contains(&yearday),
                    "Invalid by_year_day value".to_owned(),
                )?;
                if by_year_day.is_none() {
                    by_year_day = Some(new_by_year_day);
                } else {
                    return Err(ParseError::DuplicatedField("BYYEARDAY".into()))
                        .map_err(From::from);
                }
            }
            "BYWEEKNO" => {
                let new_by_week_no = stringval_to_intvec(
                    value,
                    |weekno| (-53..=53).contains(&weekno),
                    "Invalid by_week_no value".to_owned(),
                )?;
                if by_week_no.is_none() {
                    by_week_no = Some(new_by_week_no);
                } else {
                    return Err(ParseError::DuplicatedField("BYWEEKNO".into())).map_err(From::from);
                }
            }
            "BYHOUR" => {
                let new_by_hour = stringval_to_intvec(
                    value,
                    |hour| hour < 24,
                    "Invalid by_hour value".to_owned(),
                )?;
                if by_hour.is_none() {
                    by_hour = Some(new_by_hour);
                } else {
                    return Err(ParseError::DuplicatedField("BYHOUR".into())).map_err(From::from);
                }
            }
            "BYMINUTE" => {
                let new_by_minute = stringval_to_intvec(
                    value,
                    |minute| minute < 60,
                    "Invalid by_minute value".to_owned(),
                )?;
                if by_minute.is_none() {
                    by_minute = Some(new_by_minute);
                } else {
                    return Err(ParseError::DuplicatedField("BYMINUTE".into())).map_err(From::from);
                }
            }
            "BYSECOND" => {
                let new_by_second = stringval_to_intvec(
                    value,
                    |sec| sec < 60,
                    "Invalid by_second value".to_owned(),
                )?;
                if by_second.is_none() {
                    by_second = Some(new_by_second);
                } else {
                    return Err(ParseError::DuplicatedField("BYSECOND".into())).map_err(From::from);
                }
            }
            "BYWEEKDAY" | "BYDAY" => {
                let new_by_weekday = parse_weekdays(value)?;

                if by_weekday.is_none() {
                    by_weekday = Some(new_by_weekday);
                } else {
                    return Err(ParseError::DuplicatedField("BYWEEKDAY /BYDAY".into()))
                        .map_err(From::from);
                }
            }
            #[cfg(feature = "by-easter")]
            "BYEASTER" => {
                let new_by_easter =
                    stringval_to_int(value, format!("Invalid by_easter val: {}", value))?;
                if by_easter.is_none() {
                    by_easter = Some(new_by_easter);
                } else {
                    return Err(ParseError::DuplicatedField("BYEASTER".into())).map_err(From::from);
                }
            }
            _ => return Err(ParseError::Generic(format!("Invalid property: {}", key))),
        };
    }

    // Check if mandatory fields are set
    Ok(RRule {
        freq: freq.ok_or_else(|| {
            ParseError::Generic(format!("Property `FREQ` was missing in `{}`", line))
        })?,
        // `1` is default value according to spec.
        interval: interval.unwrap_or(1),
        count,
        until,
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
        stage: PhantomData,
    })
}

/// Parse the "BYWEEKDAY" and "BYDAY" values
/// Example: `SU,MO,TU,WE,TH,FR` or `4MO` or `-1WE`
/// > For example, within a MONTHLY rule, +1MO (or simply 1MO) represents the first Monday
/// > within the month, whereas -1MO represents the last Monday of the month.
fn parse_weekdays(val: &str) -> Result<Vec<NWeekday>, ParseError> {
    let mut wdays = vec![];
    // Separate all days
    for day in val.split(',') {
        wdays.push(day.parse::<NWeekday>()?);
    }
    Ok(wdays)
}

fn parse_rule_line(rfc_string: &str) -> Result<Option<RRule<Unvalidated>>, ParseError> {
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
                return Err(ParseError::Generic(format!(
                    "Invalid rule line prefix: {}",
                    rfc_string
                )))
            }
        };

        match key {
            "EXRULE" | "RRULE" => Ok(Some(parse_rrule(rfc_string)?)),
            _ => Err(ParseError::Generic(format!(
                "Unsupported RFC prop {} in {}",
                key, &rfc_string
            ))),
        }
    } else {
        // If no header is set, we can parse it as `RRULE`
        Ok(Some(parse_rrule(rfc_string)?))
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

pub(crate) fn parse_rule(rfc_string: &str) -> Result<RRule<Unvalidated>, ParseError> {
    let mut option = None;
    for line in rfc_string.split('\n') {
        let parsed_line = parse_rule_line(line)?;
        if let Some(parsed_line) = parsed_line {
            if option.is_none() {
                option = Some(parsed_line);
            } else {
                return Err(ParseError::Generic(format!(
                    "Found too many RRule lines in `{}`.",
                    rfc_string
                )));
            }
        }
    }

    if let Some(option) = option {
        Ok(option)
    } else {
        Err(ParseError::Generic(format!(
            "String is not a valid RRule: `{}`.",
            rfc_string
        )))
    }
}

#[derive(Debug)]
struct ParsedInput {
    rrule_vals: Vec<RRule<Unvalidated>>,
    rdate_vals: Vec<DateTime>,
    exrule_vals: Vec<RRule<Unvalidated>>,
    exdate_vals: Vec<DateTime>,
    dt_start: DateTime,
}

fn parse_input(s: &str) -> Result<ParsedInput, ParseError> {
    let mut rrule_vals = vec![];
    let mut rdate_vals = vec![];
    let mut exrule_vals = vec![];
    let mut exdate_vals = vec![];
    let dt_start = parse_dtstart(s)?;

    let lines = s.split('\n');
    for line in lines {
        let parsed_line = break_down_line(line);
        match parsed_line.name.to_uppercase().as_str() {
            "RRULE" => {
                if !parsed_line.params.is_empty() {
                    return Err(ParseError::Generic("Unsupported RRULE value".into()));
                }
                if parsed_line.value.is_empty() {
                    continue;
                }

                rrule_vals.push(finalize_parsed_rrule(
                    parse_rule(&parsed_line.value)?,
                    &dt_start,
                ));
            }
            "EXRULE" => {
                if !parsed_line.params.is_empty() {
                    return Err(ParseError::Generic("Unsupported EXRULE value".into()));
                }
                if parsed_line.value.is_empty() {
                    continue;
                }
                exrule_vals.push(finalize_parsed_rrule(
                    parse_rule(&parsed_line.value)?,
                    &dt_start,
                ));
            }
            "RDATE" => {
                let matches = match RDATE_RE.captures(line) {
                    Some(m) => m,
                    None => return Err(ParseError::Generic("Invalid RDATE specified".into())),
                };
                let tz: Option<Tz> = match matches.get(1) {
                    Some(tz_str) => Some(parse_timezone(tz_str.as_str())?),
                    None => None,
                };

                rdate_vals.append(&mut parse_rdate(
                    &parsed_line.value,
                    &parsed_line.params,
                    tz,
                )?);
            }
            "EXDATE" => {
                let matches = match EXDATE_RE.captures(line) {
                    Some(m) => m,
                    None => return Err(ParseError::Generic("Invalid EXDATE specified".into())),
                };
                let tz: Option<Tz> = match matches.get(1) {
                    Some(tz_str) => Some(parse_timezone(tz_str.as_str())?),
                    None => None,
                };

                exdate_vals.append(&mut parse_rdate(
                    &parsed_line.value,
                    &parsed_line.params,
                    tz,
                )?);
            }
            "DTSTART" => (),
            _ => {
                return Err(ParseError::Generic(format!(
                    "Unsupported property: {}",
                    parsed_line.name
                )))
            }
        }
    }

    Ok(ParsedInput {
        rrule_vals,
        rdate_vals,
        exrule_vals,
        exdate_vals,
        dt_start,
    })
}

fn validate_date_param(params: &[&str]) -> Result<(), ParseError> {
    for param in params {
        match DATETIME_RE.captures(param) {
            Some(caps) if caps.len() > 0 => (),
            _ => {
                return Err(ParseError::Generic(format!(
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
    params: &[String],
    tz: Option<Tz>,
) -> Result<Vec<DateTime>, ParseError> {
    let params: Vec<&str> = params.iter().map(String::as_str).collect();
    validate_date_param(&params)?;

    let mut rdatevals = vec![];
    for datestr in rdateval.split(',') {
        rdatevals.push(datestring_to_date(datestr, tz, "RDATE")?);
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
    use crate::RRuleSet;

    /// Print and compare 2 lists of dates and panic it they are not the same.
    fn check_occurrences(occurrences: &[DateTime], expected: &[&str]) {
        let formater = |dt: &DateTime| -> String { format!("    \"{}\",\n", dt.to_rfc3339()) };
        println!(
            "Given: [\n{}]\nExpected: {:#?}",
            occurrences.iter().map(formater).collect::<String>(),
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
    fn sanity_tests() {
        let tests = [
"DTSTART:19970902T090000Z\nRRULE:FREQ=YEARLY;COUNT=3\n",
"DTSTART:20120201T093000Z\nRRULE:FREQ=WEEKLY;INTERVAL=5;UNTIL=20130130T230000Z;BYDAY=MO,FR",
"DTSTART;TZID=America/Denver:19990104T110000Z\nRRULE:UNTIL=19990404T110000Z;FREQ=WEEKLY;BYDAY=TU,WE",
"DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5\nEXDATE;TZID=Europe/Berlin:20120202T130000Z,20120203T130000Z"
        ];
        for test_str in tests {
            let res = build_rruleset(test_str);
            assert!(res.is_ok());
        }
    }

    #[test]
    fn rrule() {
        let res = build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5");
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.rrule.len(), 1);
        let props = &res.rrule[0];
        assert_eq!(props.interval, 1);
        assert_eq!(props.count.unwrap(), 5);
        assert_eq!(props.freq, Frequency::Daily);
    }

    #[test]
    fn exrule() {
        let res = build_rruleset(
            "DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5\nEXRULE:FREQ=WEEKLY;INTERVAL=2",
        );
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.exrule.len(), 1);
        let props = &res.exrule[0];
        assert_eq!(props.interval, 2);
        assert_eq!(props.freq, Frequency::Weekly);
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
            let res = test_case.parse::<RRule<Unvalidated>>();
            assert!(res.is_err());
            let res = parse_dtstart(test_case);
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
            res.unwrap_err(),
            ParseError::InvalidDateTime {
                value: "20120201120000Z".into(),
                field: "DTSTART".into()
            }
            .into()
        );
    }

    #[test]
    fn invalid_freq() {
        let res = build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAIL;COUNT=5");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            ParseError::InvalidFrequency("DAIL".into()).into()
        );
    }

    #[test]
    fn invalid_byhour() {
        let res = build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYHOUR=24");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            ParseError::Generic("Invalid by_hour value".into()).into()
        );

        let res =
            build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYHOUR=5,6,25");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            ParseError::Generic("Invalid by_hour value".into()).into()
        );
    }

    #[test]
    fn invalid_byminute() {
        let res = build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYMINUTE=60");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            ParseError::Generic("Invalid by_minute value".into()).into()
        );

        let res =
            build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYMINUTE=4,5,64");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            ParseError::Generic("Invalid by_minute value".into()).into()
        );
    }

    #[test]
    fn parses_dtstart_when_just_date() {
        let res = build_rruleset("DTSTART;VALUE=DATE:20200812\nRRULE:FREQ=WEEKLY;UNTIL=20210511T220000Z;INTERVAL=1;BYDAY=WE;WKST=MO");
        assert!(res.is_ok());
    }

    #[test]
    fn parses_byday_as_nweekday_when_n_is_first() {
        let res = "DTSTART;VALUE=DATE:20200701\nRRULE:FREQ=MONTHLY;UNTIL=20210303T090000Z;INTERVAL=1;BYDAY=1WE".parse::<RRuleSet>().unwrap();
        assert_eq!(
            res.rrule[0].by_weekday,
            vec![NWeekday::new(Some(1), Weekday::Wed)]
        );
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
        let cases = [
            "RRULE:FREQ=MONTHLY;UNTIL=20210504T154500Z;INTERVAL=1;BYDAY=1TU",
            "RRULE:FREQ=MONTHLY;UNTIL=20210504T220000Z;INTERVAL=1;BYDAY=1WE",
            "RRULE:FREQ=MONTHLY;UNTIL=20210505T080000Z;INTERVAL=1;BYDAY=-1WE",
            "RRULE:FREQ=MONTHLY;UNTIL=20210505T080000Z;INTERVAL=1;BYDAY=12SU",
            "RRULE:FREQ=MONTHLY;UNTIL=20210524T090000Z;INTERVAL=1;BYDAY=+4MO",
        ];
        let opts = [
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
    fn avoids_infinite_loop() {
        let rrule = "DTSTART:20200427T090000\n\
            FREQ=WEEKLY;UNTIL=20200506T035959Z;BYDAY=FR,MO,TH,TU,WE"
            .parse::<RRuleSet>()
            .unwrap();
        let instances = rrule
            .into_iter()
            .skip_while(|d| *d < chrono::Local::now())
            .take(2);
        assert_eq!(instances.count(), 0);
    }

    #[test]
    fn daytime_savings() {
        let rrule: RRuleSet =
            "DTSTART;TZID=America/Vancouver:20210301T022210\nRRULE:FREQ=DAILY;COUNT=30"
                .parse()
                .unwrap();

        let (dates, error) = rrule.all_with_error(60);
        check_occurrences(
            &dates,
            &[
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
            &dates,
            &[
                "2020-12-14T09:30:00+01:00",
                "2021-01-11T09:30:00+01:00",
                "2021-02-22T09:30:00+01:00",
                "2021-03-08T09:30:00+01:00",
            ],
        );
    }

    #[test]
    fn test_zulu() {
        let rrule_str = "DTSTART:20210405T150000Z\nRRULE:FREQ=WEEKLY;INTERVAL=1;BYDAY=MO";
        let rrule: RRuleSet = rrule_str.parse().unwrap();
        assert_eq!(rrule.rrule[0].freq, Frequency::Weekly);
        assert_eq!(
            rrule.rrule[0].by_weekday,
            vec![NWeekday::new(None, Weekday::Mon)]
        );
        assert_eq!(rrule.rrule[0].interval, 1);
        assert_eq!(rrule.dt_start, UTC.ymd(2021, 4, 5).and_hms(15, 0, 0));
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
            &dates,
            &[
                "2021-02-22T09:30:00+01:00",
                "2021-03-08T09:30:00+01:00",
                "2021-03-22T09:30:00+01:00",
                "2021-04-05T09:30:00+02:00", // Switching to daylight saving time.
                "2021-04-19T09:30:00+02:00",
                "2021-05-03T09:30:00+02:00",
            ],
        );
    }

    /// Check if datetime can be parsed correctly
    #[test]
    fn parse_datetime() {
        let rrule: RRuleSet = "DTSTART:20120201T023000Z\nFREQ=DAILY;INTERVAL=1;COUNT=2"
            .parse()
            .expect("RRule could not be parsed");

        assert_eq!(
            rrule.all(50).unwrap(),
            vec![
                UTC.ymd(2012, 2, 1).and_hms(2, 30, 0),
                UTC.ymd(2012, 2, 2).and_hms(2, 30, 0)
            ]
        );
    }

    /// Check if datetime with timezone can be parsed correctly
    #[test]
    fn parse_datetime_with_timezone() {
        let rrule: RRuleSet =
            "DTSTART;TZID=America/New_York:20120201T023000Z\nRRULE:FREQ=DAILY;INTERVAL=1;COUNT=2"
                .parse()
                .expect("RRule could not be parsed");

        assert_eq!(
            rrule.all(50).unwrap(),
            vec![
                UTC.ymd(2012, 2, 1).and_hms(2, 30, 0),
                UTC.ymd(2012, 2, 2).and_hms(2, 30, 0)
            ]
        );
    }

    /// Check if datetime errors are correctly handled
    #[test]
    fn parse_datetime_errors_invalid_hour() {
        let res = RRuleSet::from_str("DTSTART:20120201T323000Z\nFREQ=DAILY;INTERVAL=1;COUNT=2");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err,
            ParseError::InvalidDateTime {
                value: "20120201T323000Z".into(),
                field: "DTSTART".into()
            }
            .into()
        );
    }

    /// Check if datetime errors are correctly handled
    #[test]
    fn parse_datetime_errors_invalid_day() {
        let res = RRuleSet::from_str("DTSTART:20120251T023000Z\nFREQ=DAILY;INTERVAL=1;COUNT=2");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err,
            ParseError::InvalidDateTime {
                value: "20120251T023000Z".into(),
                field: "DTSTART".into()
            }
            .into()
        );
    }

    /// Check if datetime errors are correctly handled
    #[test]
    fn parse_datetime_errors_invalid_timezone() {
        let res = RRuleSet::from_str("DTSTART:20120251T023000T\nFREQ=DAILY;INTERVAL=1;COUNT=2");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err,
            ParseError::InvalidDateTime {
                value: "20120251T023000T".into(),
                field: "DTSTART".into()
            }
            .into()
        );
    }

    /// Check if datetime errors are correctly handled
    #[test]
    fn parse_datetime_errors_invalid_tzid_timezone() {
        let res = RRuleSet::from_str(
            "DTSTART;TZID=America/Everywhere:20120251T023000Z\nRRULE:FREQ=DAILY;INTERVAL=1;COUNT=2",
        );
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err,
            ParseError::InvalidTimezone("America/Everywhere".into()).into()
        );
    }
}
