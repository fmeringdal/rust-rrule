use crate::datetime::get_weekday_val;
use crate::datetime::DTime;
use crate::options::*;
use crate::parse_options::parse_options;
use crate::rrule::RRule;
use crate::rruleset::RRuleSet;
use chrono::prelude::*;
use chrono_tz::{Tz, UTC};
use regex::Regex;
use std::str::FromStr;

// Some regex used for parsing the rrule string.
lazy_static! {
    static ref DATESTR_RE: Regex =
        Regex::new(r"(?m)^(\d{4})(\d{2})(\d{2})(T(\d{2})(\d{2})(\d{2})Z?)?$").unwrap();
    static ref DTSTART_RE: Regex =
        Regex::new(r"(?m)DTSTART(?:;TZID=([^:=]+?))?(?::|=)([^;\s]+)").unwrap();
    static ref RRULE_RE: Regex = Regex::new(r"(?m)^(?:RRULE|EXRULE):").unwrap();
    static ref PARSE_LINE_RE_1: Regex = Regex::new(r"(?m)^\s+|\s+$").unwrap();
    static ref PARSE_LINE_RE_2: Regex = Regex::new(r"(?m)^([A-Z]+?)[:;]").unwrap();
    static ref RDATE_RE: Regex = Regex::new(r"(?m)RDATE(?:;TZID=([^:=]+))?").unwrap();
    static ref EXDATE_RE: Regex = Regex::new(r"(?m)EXDATE(?:;TZID=([^:=]+))?").unwrap();
    static ref DATETIME_RE: Regex = Regex::new(r"(?m)(VALUE=DATE(-TIME)?)|(TZID=)").unwrap();
    static ref NWEEKDAY_REGEX: Regex = Regex::new(r"(?m)^([+-]?\d{1,2})([A-Z]{2})$").unwrap();
}

fn parse_datestring_bit<T: FromStr>(
    bits: &regex::Captures,
    i: usize,
    dt: &str,
) -> Result<T, RRuleParseError> {
    match bits.get(i) {
        Some(bit) => match bit.as_str().parse::<T>() {
            Err(_) => Err(RRuleParseError(format!("Invalid datetime: {}", dt))),
            Ok(val) => Ok(val),
        },
        _ => Err(RRuleParseError(format!("Invalid datetime: {}", dt))),
    }
}

fn datestring_to_date(dt: &str, tz: &Tz) -> Result<DTime, RRuleParseError> {
    let bits = DATESTR_RE.captures(dt);
    if bits.is_none() {
        return Err(RRuleParseError(format!("Invalid datetime: {}", dt)));
    }
    let bits = bits.unwrap();
    if bits.len() < 3 {
        return Err(RRuleParseError(format!("Invalid datetime: {}", dt)));
    }

    return Ok(tz
        .ymd(
            parse_datestring_bit(&bits, 1, dt)?,
            parse_datestring_bit(&bits, 2, dt)?,
            parse_datestring_bit(&bits, 3, dt)?,
        )
        .and_hms(
            parse_datestring_bit(&bits, 5, dt).unwrap_or(0),
            parse_datestring_bit(&bits, 6, dt).unwrap_or(0),
            parse_datestring_bit(&bits, 7, dt).unwrap_or(0),
        ));
}

fn parse_dtstart(s: &str) -> Result<Options, RRuleParseError> {
    let caps = DTSTART_RE.captures(s);

    match caps {
        Some(caps) => {
            let tzid: Tz = if let Some(tzid) = caps.get(1) {
                String::from(tzid.as_str()).parse().unwrap_or(UTC)
            } else {
                UTC
            };

            let dtstart_str = match caps.get(2) {
                Some(dt) => dt.as_str(),
                None => return Err(RRuleParseError(format!("Invalid datetime: {}", s))),
            };

            let mut options = Options::new();
            options.dtstart = Some(datestring_to_date(dtstart_str, &tzid)?);
            options.tzid = Some(tzid);
            Ok(options)
        }
        // None => Err(RRuleParseError(format!("Invalid datetime: {}", s))),
        // Allow no dtstart which defaults to now in utc timezone
        None => {
            let mut options = Options::new();
            options.dtstart = Some(UTC.timestamp(Utc::now().timestamp(), 0));
            options.tzid = Some(UTC);
            Ok(options)
        }
    }
}

fn from_str_to_freq(s: &str) -> Option<Frequenzy> {
    match s.to_uppercase().as_str() {
        "YEARLY" => Some(Frequenzy::Yearly),
        "MONTHLY" => Some(Frequenzy::Monthly),
        "WEEKLY" => Some(Frequenzy::Weekly),
        "DAILY" => Some(Frequenzy::Daily),
        "HOURLY" => Some(Frequenzy::Hourly),
        "MINUTELY" => Some(Frequenzy::Minutely),
        "SECONDLY" => Some(Frequenzy::Secondly),
        _ => None,
    }
}

fn stringval_to_int<T: FromStr>(val: &str, err_msg: String) -> Result<T, RRuleParseError> {
    if let Ok(val) = val.parse() {
        Ok(val)
    } else {
        return Err(RRuleParseError(err_msg));
    }
}

fn stringval_to_intvec<T: FromStr + Ord + PartialEq + Copy, F: Fn(T) -> bool>(
    val: &str,
    accept: F,
    err_msg: String,
) -> Result<Vec<T>, RRuleParseError> {
    let mut parsed_vals = vec![];
    for val in val.split(",") {
        let val = stringval_to_int(val, err_msg.clone())?;
        if accept(val) {
            parsed_vals.push(val);
        } else {
            return Err(RRuleParseError(err_msg));
        }
    }

    parsed_vals.sort();
    parsed_vals.dedup();

    Ok(parsed_vals)
}

fn parse_rrule(line: &str) -> Result<Options, RRuleParseError> {
    let stripped_line = if line.starts_with("RRULE:") {
        &line[6..]
    } else {
        line
    };

    let mut options = parse_dtstart(stripped_line).unwrap_or(Options::new());

    let attrs = RRULE_RE.replace(line, "");
    let attrs = attrs.split(";");

    for attr in attrs {
        let l: Vec<&str> = attr.split("=").collect();

        let key = l[0];
        let mut value = "";
        if l.len() > 1 {
            value = l[1];
        }

        match key.to_uppercase().as_str() {
            "FREQ" => match from_str_to_freq(value) {
                Some(freq) => options.freq = Some(freq),
                None => return Err(RRuleParseError(format!("Invalid frequenzy: {}", value))),
            },
            "WKST" => match weekday_from_str(value) {
                Ok(weekday) => {
                    options.wkst = Some(get_weekday_val(&weekday));
                }
                Err(e) => {
                    return Err(RRuleParseError(e));
                }
            },
            "COUNT" => {
                let count = stringval_to_int(value, format!("Invalid count"))?;
                options.count = Some(count);
            }
            "INTERVAL" => {
                let interval = stringval_to_int(value, format!("Invalid interval"))?;
                options.interval = Some(interval);
            }
            "BYSETPOS" => {
                let bysetpos =
                    stringval_to_intvec(value, |_pos| true, format!("Invalid bysetpos value"))?;
                options.bysetpos = Some(bysetpos);
            }
            "BYMONTH" => {
                let bymonth = stringval_to_intvec(
                    value,
                    |month| month <= 11,
                    format!("Invalid bymonth value"),
                )?;
                options.bymonth = Some(bymonth);
            }
            "BYMONTHDAY" => {
                let bymonthday = stringval_to_intvec(
                    value,
                    |monthday| monthday >= 0 && monthday <= 31,
                    format!("Invalid bymonthday value"),
                )?;
                options.bymonthday = Some(bymonthday);
            }
            "BYYEARDAY" => {
                let byyearday = stringval_to_intvec(
                    value,
                    |yearday| yearday >= -366 && yearday <= 366,
                    format!("Invalid byyearday value"),
                )?;
                options.byyearday = Some(byyearday);
            }
            "BYWEEKNO" => {
                let byweekno = stringval_to_intvec(
                    value,
                    |weekno| weekno >= 0 && weekno <= 53,
                    format!("Invalid byweekno value"),
                )?;
                options.byweekno = Some(byweekno);
            }
            "BYHOUR" => {
                let byhour =
                    stringval_to_intvec(value, |hour| hour < 24, format!("Invalid byhour value"))?;
                options.byhour = Some(byhour);
            }
            "BYMINUTE" => {
                let byminute = stringval_to_intvec(
                    value,
                    |minute| minute < 60,
                    format!("Invalid byminute value"),
                )?;
                options.byminute = Some(byminute);
            }
            "BYSECOND" => {
                let bysecond =
                    stringval_to_intvec(value, |sec| sec < 60, format!("Invalid bysecond value"))?;
                options.bysecond = Some(bysecond);
            }
            "BYWEEKDAY" | "BYDAY" => {
                options.byweekday = Some(parse_weekday(value)?);
            }
            "DTSTART" | "TZID" => {
                // for backwards compatibility
                let dtstart_opts = parse_dtstart(line)?;
                options.tzid = Some(dtstart_opts.tzid.unwrap());
                options.dtstart = Some(dtstart_opts.dtstart.unwrap());
            }
            "UNTIL" => {
                // Until is always in UTC
                options.until = Some(datestring_to_date(value, &UTC)?);
            }
            "BYEASTER" => {
                options.byeaster = Some(stringval_to_int(
                    value,
                    format!("Invalid byeaster val: {}", value),
                )?);
            }
            _ => return Err(RRuleParseError(format!("Invalid property: {}", key))),
        };
    }

    Ok(options)
}

fn str_to_weekday(d: &str) -> Result<usize, RRuleParseError> {
    match d.to_uppercase().as_str() {
        "MO" => Ok(0),
        "TU" => Ok(1),
        "WE" => Ok(2),
        "TH" => Ok(3),
        "FR" => Ok(4),
        "SA" => Ok(5),
        "SU" => Ok(6),
        _ => Err(RRuleParseError(format!("Invalid weekday: {}", d))),
    }
}

fn parse_weekday(val: &str) -> Result<Vec<NWeekday>, RRuleParseError> {
    let mut wdays = vec![];
    for day in val.split(",") {
        if day.len() == 2 {
            // MO, TU, ...
            let wday = str_to_weekday(day)?;
            wdays.push(NWeekday::new(wday, NWeekdayIdentifier::Every));
            continue;
        }

        let parts = NWEEKDAY_REGEX.captures(day).unwrap();
        let n = parts.get(1).unwrap().as_str().parse().unwrap();
        let wdaypart = parts.get(2).unwrap();
        let wday = str_to_weekday(wdaypart.as_str())?;
        wdays.push(NWeekday::new(wday, NWeekdayIdentifier::Identifier(n)));
    }
    Ok(wdays)
}

fn parse_line(rfc_string: &str) -> Result<Option<Options>, RRuleParseError> {
    let rfc_string = PARSE_LINE_RE_1.replace(rfc_string, "");
    if rfc_string.is_empty() {
        return Ok(None);
    }

    let rfc_string_upper = rfc_string.to_uppercase();
    let header = PARSE_LINE_RE_2.captures(&rfc_string_upper);

    let rfc_string = rfc_string.to_string();
    if header.is_none() {
        return Ok(Some(parse_rrule(&rfc_string)?));
    }
    let header = header.unwrap();
    let key = match header.get(1) {
        Some(k) => k.as_str(),
        None => {
            return Err(RRuleParseError(format!(
                "Invalid rfc_string: {}",
                rfc_string
            )))
        }
    };

    match key {
        "EXRULE" | "RRULE" => Ok(Some(parse_rrule(&rfc_string)?)),
        "DTSTART" => Ok(Some(parse_dtstart(&rfc_string)?)),
        _ => Err(RRuleParseError(format!(
            "Unsupported RFC prop {} in {}",
            key, &rfc_string
        ))),
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
    let params: Vec<&str> = parsed_line_name.name.split(";").collect();

    ParsedLine {
        name: params[0].to_uppercase(),
        params: params[1..].iter().map(|s| String::from(*s)).collect(),
        value: String::from(parsed_line_name.value),
    }
}

struct LineName {
    name: String,
    value: String,
}

fn extract_name(line: String) -> LineName {
    if !line.contains(":") {
        return LineName {
            name: String::from("RRULE"),
            value: line,
        };
    }

    let parts: Vec<&str> = line.split(":").collect();
    let name = parts[0];
    let value = parts[1..].join("");

    LineName {
        name: String::from(name),
        value,
    }
}

fn parse_string(rfc_string: &str) -> Result<Options, RRuleParseError> {
    let mut options = vec![];
    for line in rfc_string.split("\n") {
        let parsed_line = parse_line(line)?;
        if let Some(parsed_line) = parsed_line {
            options.push(parsed_line);
        }
    }

    if options.len() == 1 {
        return Ok(options[0].clone());
    }

    Ok(Options::concat(&options[0], &options[1]))
}

#[derive(Debug)]
struct ParsedInput {
    rrule_vals: Vec<Options>,
    rdate_vals: Vec<DTime>,
    exrule_vals: Vec<Options>,
    exdate_vals: Vec<DTime>,
    dtstart: Option<DTime>,
    tzid: Option<Tz>,
}

fn parse_input(s: &str) -> Result<ParsedInput, RRuleParseError> {
    let mut rrule_vals = vec![];
    let mut rdate_vals = vec![];
    let mut exrule_vals = vec![];
    let mut exdate_vals = vec![];
    let Options { dtstart, tzid, .. } = parse_dtstart(s)?;

    let lines: Vec<&str> = s.split("\n").collect();
    for line in &lines {
        let parsed_line = break_down_line(line);
        match parsed_line.name.to_uppercase().as_str() {
            "RRULE" => {
                if !parsed_line.params.is_empty() {
                    return Err(RRuleParseError(String::from("Unsupported RRULE value")));
                }
                if parsed_line.value.is_empty() {
                    continue;
                }

                rrule_vals.push(parse_string(line)?);
            }
            "EXRULE" => {
                if !parsed_line.params.is_empty() {
                    return Err(RRuleParseError(String::from("Unsupported EXRULE value")));
                }
                if parsed_line.value.is_empty() {
                    continue;
                }
                // TODO: why is it parsed_line.value here and line for RRULE ?? Do some testing
                exrule_vals.push(parse_string(&parsed_line.value)?);
            }
            "RDATE" => {
                let matches = match RDATE_RE.captures(line) {
                    Some(m) => m,
                    None => return Err(RRuleParseError(format!("Invalid RDATE specified"))),
                };
                let mut tz = UTC;
                if let Some(tzid) = matches.get(1) {
                    tz = String::from(tzid.as_str()).parse().unwrap_or(UTC);
                }

                rdate_vals.append(&mut parse_rdate(
                    &parsed_line.value,
                    parsed_line.params,
                    &tz,
                )?);
            }
            "EXDATE" => {
                let matches = match EXDATE_RE.captures(line) {
                    Some(m) => m,
                    None => return Err(RRuleParseError(format!("Invalid EXDATE specified"))),
                };
                let tz: Tz = if let Some(tzid) = matches.get(1) {
                    String::from(tzid.as_str()).parse().unwrap_or(UTC)
                } else {
                    UTC
                };
                exdate_vals.append(&mut parse_rdate(
                    &parsed_line.value,
                    parsed_line.params,
                    &tz,
                )?);
            }
            "DTSTART" => (),
            _ => {
                return Err(RRuleParseError(format!(
                    "Unsupported property: {}",
                    parsed_line.name
                )))
            }
        }
    }

    return Ok(ParsedInput {
        dtstart,
        tzid,
        rrule_vals,
        rdate_vals,
        exrule_vals,
        exdate_vals,
    });
}

fn validate_date_param(params: Vec<&str>) -> Result<(), RRuleParseError> {
    for param in &params {
        match DATETIME_RE.captures(param) {
            Some(caps) if caps.len() > 0 => (),
            _ => {
                return Err(RRuleParseError(format!(
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
    tz: &Tz,
) -> Result<Vec<DTime>, RRuleParseError> {
    let params: Vec<&str> = params.iter().map(|p| p.as_str()).collect();
    validate_date_param(params)?;

    let mut rdatevals = vec![];
    for datestr in rdateval.split(",") {
        rdatevals.push(datestring_to_date(datestr, tz)?);
    }

    Ok(rdatevals)
}

fn preprocess_rrule_string(s: &str) -> String {
    s.replace("DTSTART;VALUE=DATETIME", "DTSTART")
        .replace("DTSTART;VALUE=DATE", "DTSTART")
}

pub fn build_rruleset(s: &str) -> Result<RRuleSet, RRuleParseError> {
    let s = preprocess_rrule_string(s);
    let ParsedInput {
        mut rrule_vals,
        rdate_vals,
        mut exrule_vals,
        exdate_vals,
        dtstart,
        tzid,
        ..
    } = parse_input(&s)?;

    let mut rset = RRuleSet::new();
    rset.dtstart = dtstart;

    for rruleval in rrule_vals.iter_mut() {
        rruleval.tzid = tzid.clone();
        rruleval.dtstart = dtstart;
        let parsed_opts = parse_options(&rruleval)?;
        let rrule = RRule::new(parsed_opts);
        rset.rrule(rrule);
    }

    for rdate in rdate_vals {
        rset.rdate(rdate);
    }

    for exrule in exrule_vals.iter_mut() {
        exrule.tzid = tzid.clone();
        exrule.dtstart = dtstart;

        let parsed_opts = parse_options(&exrule)?;
        let exrule = RRule::new(parsed_opts);
        rset.exrule(exrule);
    }

    for exdate in exdate_vals {
        rset.exdate(exdate);
    }

    Ok(rset)
}

pub fn build_rrule(s: &str) -> Result<RRule, RRuleParseError> {
    let s = preprocess_rrule_string(s);

    let ParsedInput {
        mut rrule_vals,
        tzid,
        dtstart,
        ..
    } = parse_input(&s)?;

    rrule_vals[0].tzid = tzid;
    rrule_vals[0].dtstart = dtstart;

    let parsed_opts = parse_options(&rrule_vals[0])?;

    Ok(RRule::new(parsed_opts))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works_1() {
        let res = build_rruleset("DTSTART:19970902T090000Z\nRRULE:FREQ=YEARLY;COUNT=3\n");
        assert!(res.is_ok());
    }

    #[test]
    fn it_works_2() {
        let res = build_rrule("DTSTART:20120201T093000Z\nRRULE:FREQ=WEEKLY;INTERVAL=5;UNTIL=20130130T230000Z;BYDAY=MO,FR");
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
        assert_eq!(res.rrule[0].options.interval, 1);
        assert_eq!(res.rrule[0].options.count.unwrap(), 5);
        assert_eq!(res.rrule[0].options.freq, Frequenzy::Daily);
    }

    #[test]
    fn exrule() {
        let res = build_rruleset(
            "DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5\nEXRULE:FREQ=WEEKLY;INTERVAL=2",
        );
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.exrule.len(), 1);
        assert_eq!(res.exrule[0].options.interval, 2);
        assert_eq!(res.exrule[0].options.freq, Frequenzy::Weekly);
    }

    ////////////////////////////////////////////////////
    // Invalid stuff
    ////////////////////////////////////////////////////
    #[test]
    fn garbage_strings() {
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
        assert_eq!(res.err().unwrap().0, "Invalid datetime: 20120201120000Z");
    }

    #[test]
    fn invalid_freq() {
        let res = build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAIL;COUNT=5");
        assert!(res.is_err());
        assert_eq!(res.err().unwrap().0, "Invalid frequenzy: DAIL");
    }

    #[test]
    fn invalid_byhour() {
        let res = build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYHOUR=24");
        assert!(res.is_err());
        assert_eq!(res.err().unwrap().0, "Invalid byhour value");

        let res =
            build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYHOUR=5,6,25");
        assert!(res.is_err());
        assert_eq!(res.err().unwrap().0, "Invalid byhour value");
    }

    #[test]
    fn invalid_byminute() {
        let res = build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYMINUTE=60");
        assert!(res.is_err());
        assert_eq!(res.err().unwrap().0, "Invalid byminute value");

        let res =
            build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYMINUTE=4,5,64");
        assert!(res.is_err());
        assert_eq!(res.err().unwrap().0, "Invalid byminute value");
    }

    #[test]
    fn parses_dtstart_when_just_date() {
        let res = build_rruleset("DTSTART;VALUE=DATE:20200812\nRRULE:FREQ=WEEKLY;UNTIL=20210511T220000Z;INTERVAL=1;BYDAY=WE;WKST=MO");
        assert!(res.is_ok());
    }

    #[test]
    fn parses_byday_as_nweekday_when_n_is_first() {
        let res = build_rrule("DTSTART;VALUE=DATE:20200701\nRRULE:FREQ=MONTHLY;UNTIL=20210303T090000Z;INTERVAL=1;BYDAY=1WE").unwrap();
        assert_eq!(res.options.bynweekday, vec![vec![2, 1]]);
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
            "RRULE:FREQ=MONTHLY;UNTIL=20210524T090000Z;INTERVAL=1;BYDAY=4MO",
        ];
        let opts = vec![
            vec![NWeekday::new(1, NWeekdayIdentifier::Identifier(1))],
            vec![NWeekday::new(2, NWeekdayIdentifier::Identifier(1))],
            vec![NWeekday::new(2, NWeekdayIdentifier::Identifier(-1))],
            vec![NWeekday::new(6, NWeekdayIdentifier::Identifier(12))],
            vec![NWeekday::new(0, NWeekdayIdentifier::Identifier(4))],
        ];
        for i in 0..cases.len() {
            let opts_or_err = parse_string(cases[i]);
            assert!(opts_or_err.is_ok());
            assert_eq!(opts_or_err.unwrap().byweekday.unwrap(), opts[i]);
        }
    }

    #[test]
    #[ignore = "Only for benching"]
    fn bench() {
        let now = std::time::SystemTime::now();
        for _ in 0..10000 {
            let res = build_rruleset("RRULE:UNTIL=19990404T110000Z;DTSTART;TZID=America/New_York:19990104T110000Z;FREQ=WEEKLY;BYDAY=TU,WE").unwrap();

            // println!("Parsing took: {:?}", now.elapsed().unwrap().as_millis());
            let tmp_now = std::time::SystemTime::now();

            res.all();
            println!("All took: {:?}", tmp_now.elapsed().unwrap().as_nanos());
        }
        println!("Time took: {:?}", now.elapsed().unwrap().as_millis());
    }

    #[test]
    fn parses_rrule_without_dtstart() {
        let res = build_rrule("FREQ=DAILY;COUNT=7");
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.options.count, Some(7));
        assert_eq!(res.options.freq, Frequenzy::Daily);
        assert!(Utc::now().timestamp() - res.options.dtstart.timestamp() < 2);

        let res = build_rruleset("FREQ=DAILY;COUNT=7");
        assert!(res.is_ok());
        let occurences = res.unwrap().all();
        assert_eq!(occurences.len(), 7);
        assert!(Utc::now().timestamp() - occurences[0].timestamp() < 2);
    }
}
