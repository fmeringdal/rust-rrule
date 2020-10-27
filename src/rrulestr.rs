use crate::options::*;
use crate::parse_options::parse_options;
use crate::rrule::RRule;
use crate::datetime::DTime;
use crate::rruleset::RRuleSet;
use chrono::prelude::*;
use chrono_tz::{UTC, Tz};
use once_cell::sync::Lazy;
use regex::Regex;

static DATESTR_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^(\d{4})(\d{2})(\d{2})(T(\d{2})(\d{2})(\d{2})Z?)?$").unwrap());
static DTSTART_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)DTSTART(?:;TZID=([^:=]+?))?(?::|=)([^;\s]+)").unwrap());

static RRULE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^(?:RRULE|EXRULE):").unwrap());



fn datestring_to_date(dt: &str, tz: &Tz) -> DTime {
    let bits = DATESTR_RE.captures(dt).unwrap();
    return tz
        .ymd(
            bits.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            bits.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            bits.get(3).unwrap().as_str().parse::<u32>().unwrap(),
        )
        .and_hms(
            bits.get(5).unwrap().as_str().parse::<u32>().unwrap(),
            bits.get(6).unwrap().as_str().parse::<u32>().unwrap(),
            bits.get(7).unwrap().as_str().parse::<u32>().unwrap(),
        );
}

fn parse_dtstart(s: &str) -> Option<Options> {
    let caps = DTSTART_RE.captures(s);
    

    match caps {
        Some(caps) => {
            let tzid: Tz = if let Some(tzid) = caps.get(1) {
                String::from(tzid.as_str()).parse().unwrap_or(UTC)
            } else {
                UTC
            };

            let mut options = Options::new();
            options.dtstart = Some(datestring_to_date(caps.get(2).unwrap().as_str(), &tzid));
            options.tzid = Some(tzid);
            Some(options)
        }
        None => None,
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
            "FREQ" => {
                options.freq = Some(from_str_to_freq(value).unwrap());
            }
            "WKST" => {
                options.wkst = Some(value.parse::<usize>().unwrap());
            }
            "COUNT" => {
              options.count = Some(value.parse::<u32>().unwrap());
            }
            "INTERVAL" => {
              options.interval = Some(value.parse::<usize>().unwrap());
            }
            "BYSETPOS" => {
              let bysetpos = value.split(",").map(|d| d.parse::<isize>().unwrap()).collect();
              options.bysetpos = Some(bysetpos);
            }
            "BYMONTH" => {
              let bymonth = value.split(",").map(|d| d.parse::<usize>().unwrap()).collect();
              options.bymonth = Some(bymonth);
            }
            "BYMONTHDAY"  => {
              let bymonthday = value.split(",").map(|d| d.parse::<isize>().unwrap()).collect();
              options.bymonthday = Some(bymonthday);
            }
            "BYYEARDAY" => {
              let byyearday = value.split(",").map(|d| d.parse::<isize>().unwrap()).collect();
              options.byyearday = Some(byyearday);
            }
            "BYWEEKNO" => {
              let byweekno = value.split(",").map(|d| d.parse::<isize>().unwrap()).collect();
              options.byweekno = Some(byweekno);
            }
            "BYHOUR"  => {
              let byhour = value.split(",").map(|d| d.parse::<usize>().unwrap()).collect();
              options.byhour = Some(byhour);
            }
            "BYMINUTE"  => {
              let byminute = value.split(",").map(|d| d.parse::<usize>().unwrap()).collect();
              options.byminute = Some(byminute);
            }
            "BYSECOND" => {
              let bysecond = value.split(",").map(|d| d.parse::<usize>().unwrap()).collect();
              options.bysecond = Some(bysecond);
            }
            "BYWEEKDAY" | "BYDAY" => {
                options.byweekday = Some(parse_weekday(value)?);
            }
            "DTSTART" | "TZID" => {
                // for backwards compatibility
                let dtstart_opts = parse_dtstart(line).unwrap();
                options.tzid = Some(dtstart_opts.tzid.unwrap());
                options.dtstart = Some(dtstart_opts.dtstart.unwrap());
            }
            "UNTIL" => {
                // Until is always in UTC
                options.until = Some(datestring_to_date(value, &UTC));
            }
            "BYEASTER" => {
                options.byeaster = Some(value.parse::<isize>().unwrap());
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

fn parse_weekday(val: &str) -> Result<Vec<usize>, RRuleParseError> {
    val.split(",").map(|day| {
        if day.len() == 2 {
            // MO, TU, ...
            return str_to_weekday(day);
        }

        // ! NOT SUPPORTED YET
        // -1MO, +3FR, 1SO, 13TU ...
        // let regex = Regex::new(r"(?m)^([+-]?\d{1,2})([A-Z]{2})$").unwrap();
        // let parts = regex.captures(day).unwrap();
        // let n = parts.get(1).unwrap();
        // let wdaypart = parts.get(2).unwrap();
        // let wday = str_to_weekday(d)
        
        return str_to_weekday(day);
    }).collect()
}

fn parse_line(rfc_string: &str) -> Result<Option<Options>, RRuleParseError> {
    let re = Regex::new(r"(?m)^\s+|\s+$").unwrap();
    let rfc_string = re.replace(rfc_string, "");
    if rfc_string.is_empty() {
        return Ok(None);
    }

    let re = Regex::new(r"(?m)^([A-Z]+?)[:;]").unwrap();
    let rfc_string_upper = rfc_string.to_uppercase();
    let header = re.captures(&rfc_string_upper);
    
    

    let rfc_string = rfc_string.to_string();
    if header.is_none() {
        return Ok(Some(parse_rrule(&rfc_string)?));
    }
    let header = header.unwrap();
    let key = header.get(1).unwrap().as_str();

    match key {
        "EXRULE" | "RRULE" => Ok(Some(parse_rrule(&rfc_string)?)),
        "DTSTART" => Ok(Some(parse_dtstart(&rfc_string).unwrap())),
        _ => Err(RRuleParseError(format!("Unsupported RFC prop {} in {}", key, &rfc_string)))
    }
}

#[derive(Debug)]
struct ParsedLine {
    name: String,
    params: Vec<String>,
    value: String
}

fn break_down_line(line: &str) -> ParsedLine {
    let parsed_line_name = extract_name(String::from(line));
    let params: Vec<&str> = parsed_line_name.name.split(";").collect();

    ParsedLine {
        name: params[0].to_uppercase(),
        params: params[1..].iter().map(|s| String::from(*s)).collect(),
        value: String::from(parsed_line_name.value)
    }
}

struct LineName {
    name: String,
    value: String
}

fn extract_name(line: String) -> LineName {
    if !line.contains(":") {
        return LineName {
            name: String::from("RRULE"),
            value: line
        };
    }

    let parts: Vec<&str> = line.split(":").collect();
    let name = parts[0];
    let value = parts[1..].join("");

    LineName {
        name: String::from(name),
        value
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

    let Options {
        dtstart,
        tzid,
        ..
    } = parse_dtstart(s).unwrap();


    let lines: Vec<&str> = s.split("\n").collect();
    println!("Lines: {:?}", lines);
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
                let re = Regex::new(r"(?m)RDATE(?:;TZID=([^:=]+))?").unwrap();
                let matches = re.captures(line).unwrap();
                let mut tz = UTC;
                if let Some(tzid) = matches.get(1) {
                    tz = String::from(tzid.as_str()).parse().unwrap_or(UTC);
                }
                
                rdate_vals.append(&mut parse_rdate(&parsed_line.value, parsed_line.params, &tz));
            }
            "EXDATE" => {
                let re = Regex::new(r"(?m)EXDATE(?:;TZID=([^:=]+))?").unwrap();
                let matches = re.captures(line).unwrap();
                let tz: Tz = if let Some(tzid) = matches.get(1) {
                    String::from(tzid.as_str()).parse().unwrap_or(UTC)
                } else {
                    UTC
                };
                exdate_vals.append(&mut parse_rdate(&parsed_line.value, parsed_line.params, &tz));
            }
            "DTSTART" => (),
            _ => return Err(RRuleParseError(format!("Unsupported property: {}", parsed_line.name)))
        }
    }

    return Ok(ParsedInput {
        dtstart,
        tzid,
        rrule_vals,
        rdate_vals,
        exrule_vals,
        exdate_vals
    })
}

fn validate_date_param(params: Vec<&str>) -> Result<(), RRuleParseError>{
    let re = Regex::new(r"(?m)(VALUE=DATE(-TIME)?)|(TZID=)").unwrap();

    for param in &params {
        if re.captures(param).unwrap().len() == 0 {
            return Err(RRuleParseError(format!("Unsupported RDATE/EXDATE parm: {}", param)));
        }
    }
    Ok(())
}

// ! works needs to be done here
fn parse_rdate(rdateval: &str, params: Vec<String>, tz: &Tz) -> Vec<DTime> {
    let params: Vec<&str> = params.iter().map(|p| p.as_str()).collect();
    validate_date_param(params);
    // let re_timezone = Regex::new(r"(?m)TZID=(.+):").unwrap();
    // let caps = re_timezone.captures(text)
    // let tzid = re_timezone


    rdateval.split(",").map(|datestr| datestring_to_date(datestr, tz)).collect()
}


pub fn build_rruleset(s: &str) -> Result<RRuleSet, RRuleParseError> {
    let ParsedInput {
        mut rrule_vals,
        rdate_vals,
        mut exrule_vals,
        exdate_vals,
        dtstart,
        tzid,
        ..
    } = parse_input(s)?;


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
        rset.rrule(exrule);        
    }

    for exdate in exdate_vals {
        rset.exdate(exdate);
    }


    Ok(rset)
}

pub fn build_rrule(s: &str) -> Result<RRule, RRuleParseError> {
    let ParsedInput {
        mut rrule_vals,
        tzid,
        dtstart,
        ..
    } = parse_input(s)?;

    rrule_vals[0].tzid = tzid;
    rrule_vals[0].dtstart = dtstart;

    let parsed_opts = parse_options(&rrule_vals[0])?;

    Ok(RRule::new(parsed_opts))
}


#[cfg(test)]
mod test {
    use super::*;
    use chrono_tz::{Tz, UTC};

    #[test]
    fn it_works_1() {
        let options = build_rruleset("DTSTART:19970902T090000Z\nRRULE:FREQ=YEARLY;COUNT=3\n").unwrap();
        println!("?????????????=================?????????????");
        println!("{:?}", options);
    }

    #[test]
    fn it_works_2() {
        let mut options = build_rrule("DTSTART:20120201T093000Z\nRRULE:FREQ=WEEKLY;INTERVAL=5;UNTIL=20130130T230000Z;BYDAY=MO,FR").unwrap();
        println!("?????????????=================?????????????");
        println!("{:?}", options);
        println!("?????????????=== ALLL    ==============?????????????");
        println!("{:?}", options.all());
    }

    #[test]
    fn it_works_3() {
        let mut options = build_rruleset("RRULE:UNTIL=19990404T110000Z;DTSTART;TZID=America/Denver:19990104T110000Z;FREQ=WEEKLY;BYDAY=TU,WE").unwrap();
        println!("?????????????=================?????????????");
        println!("{:?}", options);
        let tzid: Tz = "America/Denver".parse().unwrap();
        println!("?????????????=== ALLL    ==============?????????????");
        println!("{:?}", options.all().into_iter().take(2).collect::<Vec<DateTime<Tz>>>());
        println!("{:?}", options.all().iter().take(2).map(|d| d.with_timezone(&UTC)).collect::<Vec<DateTime<Tz>>>());
        println!("Diff : {:?}", options.all()[0].timestamp() - options.all()[0].with_timezone(&UTC).timestamp());
    }

    #[test]
    fn it_works_4() {
        let mut set = build_rruleset("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5\nEXDATE;TZID=Europe/Berlin:20120202T130000Z,20120203T130000Z").unwrap();
        println!("?????????????=================??======?????????????");
        println!("{:?}", set.exdate.iter().map(|d| d.timestamp()).collect::<Vec<i64>>());
        let all = set.all();
        println!("{:?}", all.iter().map(|d| d.timestamp()).collect::<Vec<i64>>());
        println!("------------------ alll ----------------");
        println!("{:?}", all);
    }
}
