use crate::options::*;
use crate::parse_options::parse_options;
use crate::rruleset::RRuleSet;
use chrono::prelude::*;
use chrono::DateTime;
use chrono_tz::Tz;
use once_cell::sync::Lazy;
use regex::Regex;

static DATESTR_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^(\d{4})(\d{2})(\d{2})(T(\d{2})(\d{2})(\d{2})Z?)?$").unwrap());
static DTSTART_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)DTSTART(?:;TZID=([^:=]+?))?(?::|=)([^;\s]+)").unwrap());

static RRUle_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^(?:RRULE|EXRULE):").unwrap());

#[derive(Debug, Clone)]
pub struct PartialOptions {
    pub freq: Option<Frequenzy>,
    pub interval: Option<usize>,
    pub count: Option<u32>,
    pub until: Option<DateTime<Utc>>,
    pub tzid: Option<String>,
    pub dtstart: Option<DateTime<Utc>>,
    pub wkst: Option<usize>,
    pub bysetpos: Option<Vec<isize>>,
    pub bymonth: Option<Vec<usize>>,
    pub bymonthday: Option<Vec<isize>>,
    pub bynmonthday: Option<Vec<isize>>,
    pub byyearday: Option<Vec<isize>>,
    pub byweekno: Option<Vec<isize>>,
    pub byweekday: Option<Vec<usize>>,
    pub byhour: Option<Vec<usize>>,
    pub byminute: Option<Vec<usize>>,
    pub bysecond: Option<Vec<usize>>,
    pub bynweekday: Option<Vec<Vec<isize>>>,
    pub byeaster: Option<isize>,
}

impl PartialOptions {
    pub fn new() -> Self {
        Self {
            freq: None,
            interval: None,
            count: None,
            until: None,
            tzid: None,
            dtstart: None,
            wkst: None,
            bysetpos: None,
            bymonth: None,
            bymonthday: None,
            bynmonthday: None,
            byyearday: None,
            byweekno: None,
            byweekday: None,
            byhour: None,
            byminute: None,
            bysecond: None,
            bynweekday: None,
            byeaster: None,
        }
    }

    fn is_some_or_none<'a, T>(prop1: &'a Option<T>, prop2: &'a Option<T>) -> &'a Option<T> {
        if prop2.is_some() {
            return prop2;
        } 
        prop1
    }

    pub fn concat(opt1: &Self, opt2: &Self) -> Self {
        Self {
            freq: Self::is_some_or_none(&opt1.freq, &opt2.freq).clone(),
            interval: Self::is_some_or_none(&opt1.interval, &opt2.interval).clone(),
            count: Self::is_some_or_none(&opt1.count, &opt2.count).clone(),
            until: Self::is_some_or_none(&opt1.until, &opt2.until).clone(),
            tzid: Self::is_some_or_none(&opt1.tzid, &opt2.tzid).clone(),
            dtstart: Self::is_some_or_none(&opt1.dtstart, &opt2.dtstart).clone(),
            wkst: Self::is_some_or_none(&opt1.wkst, &opt2.wkst).clone(),
            bysetpos: Self::is_some_or_none(&opt1.bysetpos, &opt2.bysetpos).clone(),
            bymonth: Self::is_some_or_none(&opt1.bymonth, &opt2.bymonth).clone(),
            bymonthday: Self::is_some_or_none(&opt1.bymonthday, &opt2.bymonthday).clone(),
            bynmonthday: Self::is_some_or_none(&opt1.bynmonthday, &opt2.bynmonthday).clone(),
            byyearday: Self::is_some_or_none(&opt1.byyearday, &opt2.byyearday).clone(),
            byweekno: Self::is_some_or_none(&opt1.byweekno, &opt2.byweekno).clone(),
            byweekday: Self::is_some_or_none(&opt1.byweekday, &opt2.byweekday).clone(),
            byhour: Self::is_some_or_none(&opt1.byhour, &opt2.byhour).clone(),
            byminute: Self::is_some_or_none(&opt1.byminute, &opt2.byminute).clone(),
            bysecond: Self::is_some_or_none(&opt1.bysecond, &opt2.bysecond).clone(),
            bynweekday: Self::is_some_or_none(&opt1.bynweekday, &opt2.bynweekday).clone(),
            byeaster: Self::is_some_or_none(&opt1.byeaster, &opt2.byeaster).clone(),
        }
    }
}

fn datestring_to_date(dt: &str) -> DateTime<Utc> {
    let bits = DATESTR_RE.captures(dt).unwrap();
    return Utc
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

fn parse_dtstart(s: &str) -> Option<PartialOptions> {
    let caps = DTSTART_RE.captures(s);
    println!("captures -----------------------");
    println!("{:?}", caps);

    match caps {
        Some(caps) => {
            let mut options = PartialOptions::new();
            options.dtstart = Some(datestring_to_date(caps.get(2).unwrap().as_str()));
            options.tzid = if let Some(tzid) = caps.get(1) {
                Some(String::from(tzid.as_str()))
            } else {
                Some(String::from("UTC"))
            };
            Some(options)
        }
        None => None,
    }
}

fn from_str_to_freq(s: &str) -> Option<Frequenzy> {
    match s.to_uppercase().as_str() {
        "YEARLY" => Some(Frequenzy::YEARLY),
        "MONTHLY" => Some(Frequenzy::MONTHLY),
        "WEEKLY" => Some(Frequenzy::WEEKLY),
        "DAILY" => Some(Frequenzy::DAILY),
        "HOURLY" => Some(Frequenzy::HOURLY),
        "MINUTELY" => Some(Frequenzy::MINUTELY),
        "SECONDLY" => Some(Frequenzy::SECONDLY),
        _ => None,
    }
}

fn parse_rrule(line: &str) -> PartialOptions {
    let stripped_line = if line.starts_with("RRULE:") {
        &line[6..]
    } else {
        line
    };


    let mut options = parse_dtstart(stripped_line).unwrap_or(PartialOptions::new());

    let attrs = RRUle_RE.replace(line, "");
    let attrs = attrs.split(";");

    for attr in attrs {
        println!("Attr: {}", attr);
        let l: Vec<&str> = attr.split("=").collect();

        let key = l[0];
        let mut value = "";
        if l.len() > 1 {
            value = l[1];
        }
        println!("Ket: {}", key);
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
                options.byweekday = Some(parse_weekday(value));
            }
            "DTSTART" | "TZID" => {
                // for backwards compatibility
                let dtstart_opts = parse_dtstart(line).unwrap();
                println!("After parsing tzid");
                println!("{:?}", dtstart_opts);
                options.tzid = Some(dtstart_opts.tzid.unwrap());
                options.dtstart = Some(dtstart_opts.dtstart.unwrap());
            }
            "UNTIL" => {
                options.until = Some(datestring_to_date(value));
            }
            "BYEASTER" => {
                options.byeaster = Some(value.parse::<isize>().unwrap());
            }
            _ => panic!("Invalid property: {}", key),
        };
    }

    options
}

fn str_to_weekday(d: &str) -> usize {
    match d.to_uppercase().as_str() {
        "MO" => 0,
        "TU" => 1,
        "WE" => 2,
        "TH" => 3,
        "FR" => 4,
        "SA" => 5,
        "SU" => 6,
        _ => panic!("Invalid weekday: {}", d),
    }
}

fn parse_weekday(val: &str) -> Vec<usize> {
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

fn parse_line(rfc_string: &str) -> Option<PartialOptions> {
    let re = Regex::new(r"(?m)^\s+|\s+$").unwrap();
    let rfc_string = re.replace(rfc_string, "");
    if rfc_string.is_empty() {
        return None;
    }

    let re = Regex::new(r"(?m)^([A-Z]+?)[:;]").unwrap();
    let rfc_string_upper = rfc_string.to_uppercase();
    let header = re.captures(&rfc_string_upper);
    
    

    let rfc_string = rfc_string.to_string();
    if header.is_none() {
        return Some(parse_rrule(&rfc_string));
    }
    let header = header.unwrap();
    let key = header.get(1).unwrap().as_str();

    match key {
        "EXRULE" | "RRULE" => Some(parse_rrule(&rfc_string)),
        "DTSTART" => Some(parse_dtstart(&rfc_string).unwrap()),
        _ => panic!("Unsupported RFC prop {} in {}", key, &rfc_string)
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

fn parse_string(rfc_string: &str) -> PartialOptions {
    let options: Vec<PartialOptions> = rfc_string.split("\n").map(|l| parse_line(l)).filter(|x| x.is_some())
    .map(|o| o.unwrap())
    .collect();

    if options.len() == 1 {
        println!("Options i got from str: {}", rfc_string);
        println!("{:?}", options[0]);
        return options[0].clone();
    }

    PartialOptions::concat(&options[0], &options[1])
}

#[derive(Debug)]
struct ParsedInput {
    rrule_vals: Vec<PartialOptions>,
    rdate_vals: Vec<DateTime<Utc>>,
    exrule_vals: Vec<PartialOptions>,
    exdate_vals: Vec<DateTime<Utc>>,
    dtstart: Option<DateTime<Utc>>,
    tzid: Option<String>,
}

fn parse_input(s: &str) -> ParsedInput {
    let mut rrule_vals = vec![];
    let mut rdate_vals = vec![];
    let mut exrule_vals = vec![];
    let mut exdate_vals = vec![];

    let PartialOptions {
        dtstart,
        mut tzid,
        ..
    } = parse_dtstart(s).unwrap();


    let lines: Vec<&str> = s.split("\n").collect();
    for line in &lines {
        let parsed_line = break_down_line(line);
        match parsed_line.name.to_uppercase().as_str() {
            "RRULE" => {
                if !parsed_line.params.is_empty() {
                    panic!("Unsupported RRULE value");
                }
                if parsed_line.value.is_empty() {
                    continue;
                }
                
                rrule_vals.push(parse_string(line));
            }
            "RDATE" => {
                let re = Regex::new(r"(?m)RDATE(?:;TZID=([^:=]+))?").unwrap();
                let matches = re.captures(line).unwrap();
                if tzid.is_none() && matches.get(1).is_some() {
                    tzid = Some(String::from(matches.get(1).unwrap().as_str()));
                }
                
                rdate_vals.append(&mut parse_rdate(&parsed_line.value, parsed_line.params));
            }
            "EXRULE" => {
                exrule_vals.push(parse_string(&parsed_line.value));
            }
            "EXDATE" => {
                exdate_vals.append(&mut parse_rdate(&parsed_line.value, parsed_line.params));
            }
            "DTSTART" => (),
            _ => panic!("Unsupported property: {}", parsed_line.name)
        }
    }

    return ParsedInput {
        dtstart,
        tzid,
        rrule_vals,
        rdate_vals,
        exrule_vals,
        exdate_vals
    }
}

fn validate_date_param(params: Vec<&str>){
    let re = Regex::new(r"(?m)(VALUE=DATE(-TIME)?)|(TZID=)").unwrap();

    for param in &params {
        if re.captures(param).unwrap().len() == 0 {
            panic!("Unsupported RDATE/EXDATE parm: {}", param);
        }
    }
}

fn parse_rdate(rdateval: &str, params: Vec<String>) -> Vec<DateTime<Utc>> {
    let params: Vec<&str> = params.iter().map(|p| p.as_str()).collect();
    validate_date_param(params);

    rdateval.split(",").map(|datestr| datestring_to_date(datestr)).collect()
}

fn build_rule(s: &str) -> RRuleSet {
    let ParsedInput {
        mut rrule_vals,
        rdate_vals,
        mut exrule_vals,
        exdate_vals,
        dtstart,
        tzid,
        ..
    } = parse_input(s);

    let mut rset = RRuleSet::new();
    if rrule_vals.len() > 1 ||
        !rdate_vals.is_empty() ||
        !exrule_vals.is_empty() ||
        !exdate_vals.is_empty() {

        rset.dtstart = dtstart;
        // rset.tzid(tzid || undefined);
        
        for rruleval in rrule_vals.iter_mut() {
            rruleval.tzid = tzid.clone();
            rruleval.dtstart = dtstart;

            // let rrule = RRule::new(rruleval.clone());
            // rset.rrule(rrule);
        }
    
        for rdate in rdate_vals {
            rset.rdate(rdate);
        }

        for exrule in exrule_vals.iter_mut() {
            exrule.tzid = tzid.clone();
            exrule.dtstart = dtstart;

            // let exrule = RRule::new(exrule.clone());
            // rset.rrule(exrule);        
        }
    
        for exdate in exdate_vals {
            rset.exdate(exdate);
        }
    
        // if (options.compatible && options.dtstart) rset.rdate(dtstart!);
        return rset;
    }


    rset
}


#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn it_works_2() {
    //     let options = build_rule("DTSTART:19970902T090000Z\nRRULE:FREQ=YEARLY;COUNT=3\n");
    //     println!("?????????????=================?????????????");
    //     println!("{:?}", options);
    // }

    // #[test]
    // fn it_works() {
    //     let options = build_rule("RRULE:UNTIL=19990404T110000Z;DTSTART=19990104T110000Z;FREQ=WEEKLY;BYDAY=TU,WE");
    //     println!("?????????????=================?????????????");
    //     println!("{:?}", options);
    // }

    // #[test]
    // fn it_works() {
    //     let options = build_rule("RRULE:UNTIL=19990404T110000Z;DTSTART;TZID=America/New_York:19990104T110000Z;FREQ=WEEKLY;BYDAY=TU,WE");
    //     let parsed_opts = parse_options(&options);
    //     println!("?????????????=================?????????????");
    //     println!("{:?}", options);
    // }

    #[test]
    fn it_works() {
        let options = parse_string("RRULE:UNTIL=19990404T110000Z;DTSTART;TZID=America/New_York:19990104T110000Z;FREQ=WEEKLY;BYDAY=TU,WE");
        let parsed_opts = parse_options(&options);
        println!("?????????????=================?????????????");
        println!("{:?}", options);
        println!("?????????????=== PARSED ==============?????????????");
        println!("{:?}", parsed_opts);
        let all = crate::rrule::RRule::new(parsed_opts).all();
        println!("------------------ alll ----------------");
        println!("{:?}", all);
    }
}
