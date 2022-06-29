use super::datetime::{datestring_to_date, parse_dtstart, parse_timezone, parse_weekdays};
use super::regex::{self, get_rrule_attributes};
use super::utils::{check_str_validity, parse_str_to_vec};
use super::{str_to_weekday, ParseError};
use crate::core::Unvalidated;
use crate::{core::DateTime, Frequency, RRule, RRuleError, RRuleSet};
use chrono::Weekday;
use chrono_tz::{Tz, UTC};
use std::marker::PhantomData;
use std::str::FromStr;

/// Creates [`RRuleSet`] from parsing the String.
pub(crate) fn build_rruleset(s: &str) -> Result<RRuleSet, RRuleError> {
    check_str_validity(s)?;

    let s = preprocess_rrule_string(s);
    let ParsedInput {
        rrule_vals,
        rdate_vals,
        exrule_vals,
        exdate_vals,
        dt_start,
    } = parse_input(&s)?;

    #[allow(deprecated)]
    let rrule_set = RRuleSet::new(dt_start)
        .set_rrules(
            rrule_vals
                .into_iter()
                .map(|r| r.validate(dt_start))
                .collect::<Result<Vec<_>, _>>()?,
        )
        .set_rdates(rdate_vals)
        .set_exrules(
            exrule_vals
                .into_iter()
                .map(|r| r.validate(dt_start))
                .collect::<Result<Vec<_>, _>>()?,
        )
        .set_exdates(exdate_vals);

    Ok(rrule_set)
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

    let attributes = get_rrule_attributes(line).expect(
        "the line should be an rrule as that should have been verified before reaching this point",
    );

    for attr in attributes {
        let l: Vec<&str> = attr.split('=').collect();

        let key = l[0];
        let mut value = "";
        if l.len() > 1 {
            value = l[1];
        }

        match key.to_uppercase().as_str() {
            "FREQ" => {
                let new_freq = Frequency::from_str(value)?;
                if freq.replace(new_freq).is_some() {
                    return Err(ParseError::DuplicatedField("FREQ".into())).map_err(From::from);
                }
            }
            "INTERVAL" => {
                let new_interval = value
                    .parse()
                    .map_err(|_| ParseError::InvalidInterval(value.into()))?;
                if interval.replace(new_interval).is_some() {
                    return Err(ParseError::DuplicatedField("INTERVAL".into())).map_err(From::from);
                }
            }
            "COUNT" => {
                let new_count = value
                    .parse()
                    .map_err(|_| ParseError::InvalidCount(value.into()))?;

                if count.replace(new_count).is_some() {
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
                let new_until = datestring_to_date(value, Some(UTC), "UNTIL")?;
                if until.replace(new_until).is_some() {
                    return Err(ParseError::DuplicatedField("UNTIL".into())).map_err(From::from);
                }
            }
            "WKST" => match str_to_weekday(value) {
                Ok(new_weekday) => {
                    if week_start.replace(new_weekday).is_some() {
                        return Err(ParseError::DuplicatedField("WKST".into())).map_err(From::from);
                    }
                }
                Err(_) => {
                    return Err(ParseError::InvalidWeekdayStart(value.into()));
                }
            },
            "BYSETPOS" => {
                let new_by_set_pos = parse_str_to_vec(value, |_pos| true)
                    .map_err(|_| ParseError::InvalidBySetPos(value.into()))?;
                if by_set_pos.replace(new_by_set_pos).is_some() {
                    return Err(ParseError::DuplicatedField("BYSETPOS".into())).map_err(From::from);
                }
            }
            "BYMONTH" => {
                let new_by_month = parse_str_to_vec(value, |month| (1..=12).contains(&month))
                    .map_err(|_| ParseError::InvalidByMonth(value.into()))?;
                if by_month.replace(new_by_month).is_some() {
                    return Err(ParseError::DuplicatedField("BYMONTH".into())).map_err(From::from);
                }
            }
            "BYMONTHDAY" => {
                let new_by_month_day =
                    parse_str_to_vec(value, |monthday| (-31..=31).contains(&monthday))
                        .map_err(|_| ParseError::InvalidByMonthDay(value.into()))?;
                if by_month_day.replace(new_by_month_day).is_some() {
                    return Err(ParseError::DuplicatedField("BYMONTHDAY".into()))
                        .map_err(From::from);
                }
            }
            "BYYEARDAY" => {
                let new_by_year_day =
                    parse_str_to_vec(value, |yearday| (-366..=366).contains(&yearday))
                        .map_err(|_| ParseError::InvalidByYearDay(value.into()))?;
                if by_year_day.replace(new_by_year_day).is_some() {
                    return Err(ParseError::DuplicatedField("BYYEARDAY".into()))
                        .map_err(From::from);
                }
            }
            "BYWEEKNO" => {
                let new_by_week_no = parse_str_to_vec(value, |weekno| (-53..=53).contains(&weekno))
                    .map_err(|_| ParseError::InvalidByWeekNo(value.into()))?;
                if by_week_no.replace(new_by_week_no).is_some() {
                    return Err(ParseError::DuplicatedField("BYWEEKNO".into())).map_err(From::from);
                }
            }
            "BYHOUR" => {
                let new_by_hour = parse_str_to_vec(value, |hour| hour < 24)
                    .map_err(|_| ParseError::InvalidByHour(value.into()))?;
                if by_hour.replace(new_by_hour).is_some() {
                    return Err(ParseError::DuplicatedField("BYHOUR".into())).map_err(From::from);
                }
            }
            "BYMINUTE" => {
                let new_by_minute = parse_str_to_vec(value, |minute| minute < 60)
                    .map_err(|_| ParseError::InvalidByMinute(value.into()))?;
                if by_minute.replace(new_by_minute).is_some() {
                    return Err(ParseError::DuplicatedField("BYMINUTE".into())).map_err(From::from);
                }
            }
            "BYSECOND" => {
                let new_by_second = parse_str_to_vec(value, |sec| sec < 60)
                    .map_err(|_| ParseError::InvalidBySecond(value.into()))?;
                if by_second.replace(new_by_second).is_some() {
                    return Err(ParseError::DuplicatedField("BYSECOND".into())).map_err(From::from);
                }
            }
            "BYWEEKDAY" | "BYDAY" => {
                let new_by_weekday = parse_weekdays(value)?;

                if by_weekday.replace(new_by_weekday).is_some() {
                    return Err(ParseError::DuplicatedField("BYWEEKDAY / BYDAY".into()))
                        .map_err(From::from);
                }
            }
            #[cfg(feature = "by-easter")]
            "BYEASTER" => {
                let new_by_easter = value
                    .parse()
                    .map_err(|_| ParseError::InvalidByEaster(value.into()))?;
                if by_easter.replace(new_by_easter).is_some() {
                    return Err(ParseError::DuplicatedField("BYEASTER".into())).map_err(From::from);
                }
            }
            // for backward compatibility
            "DTSTART" | "TZID" => {}
            _ => return Err(ParseError::UnexpectedField(key.into())),
        };
    }

    // Check if mandatory fields are set
    Ok(RRule {
        freq: freq.ok_or_else(|| ParseError::MissingProperty("FREQ".into()))?,
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

fn parse_rule_line(rfc_string: &str) -> Result<Option<RRule<Unvalidated>>, ParseError> {
    let rfc_string = rfc_string.trim();
    // If this part is empty return
    if rfc_string.is_empty() {
        return Ok(None);
    }

    let rfc_string_upper = rfc_string.to_uppercase();
    // Get header, `RRULE:` or `EXRULE;` part.
    let header = regex::get_line_header(&rfc_string_upper);

    match header {
        Some(header) => match &header[..] {
            "EXRULE" | "RRULE" => Ok(Some(parse_rrule(rfc_string)?)),
            _ => Err(ParseError::UnsupportedRFCProperty(header)),
        },
        None => {
            // If no header is set, we can parse it as `RRULE`
            Ok(Some(parse_rrule(rfc_string)?))
        }
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
    check_str_validity(rfc_string)?;

    let mut option = None;
    for line in rfc_string.split('\n') {
        let parsed_line = parse_rule_line(line)?;
        if let Some(parsed_line) = parsed_line {
            if option.is_none() {
                option = Some(parsed_line);
            } else {
                return Err(ParseError::TooManyRulesInLine(rfc_string.into()));
            }
        }
    }

    if let Some(option) = option {
        Ok(option)
    } else {
        Err(ParseError::InvalidRule(rfc_string.into()))
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
        let key = &parsed_line.name;
        match &key[..] {
            "RRULE" | "EXRULE" => {
                if !parsed_line.params.is_empty() {
                    return Err(ParseError::UnsupportedValueInRuleLine(
                        parsed_line.params.join(";"),
                        line.to_string(),
                    ));
                }
                if parsed_line.value.is_empty() {
                    continue;
                }

                let properties = parse_rule(&parsed_line.value)?.finalize_parsed_rrule(&dt_start);

                match &key[..] {
                    "RRULE" => {
                        rrule_vals.push(properties);
                    }
                    "EXRULE" => {
                        exrule_vals.push(properties);
                    }
                    _ => unreachable!(),
                }
            }
            "RDATE" | "EXDATE" => {
                let timezone = match &key[..] {
                    "RDATE" => regex::get_rdate_timezone(line),
                    "EXDATE" => regex::get_exdate_timezone(line),
                    _ => unreachable!(),
                };
                let timezone = timezone.map_err(|_| ParseError::InvalidDateTime {
                    field: key.to_string(),
                    value: line.to_string(),
                })?;
                let tz = timezone.map(|tz: String| parse_timezone(&tz)).transpose()?;

                let mut dates = parse_rdate(&parsed_line.value, tz)?;

                match &key[..] {
                    "RDATE" => {
                        rdate_vals.append(&mut dates);
                    }
                    "EXDATE" => {
                        exdate_vals.append(&mut dates);
                    }
                    _ => unreachable!(),
                }
            }
            "DTSTART" => (),
            _ => return Err(ParseError::UnsupportedProperty(parsed_line.name)),
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

fn parse_rdate(rdateval: &str, tz: Option<Tz>) -> Result<Vec<DateTime>, ParseError> {
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
    use chrono::TimeZone;

    use super::*;
    use crate::{NWeekday, RRuleSet};

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
        let res = parse_input("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYHOUR=24");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ParseError::InvalidByHour("24".into()));

        let res = parse_input("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYHOUR=5,6,25");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ParseError::InvalidByHour("5,6,25".into()));
    }

    #[test]
    fn invalid_byminute() {
        let res = parse_input("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYMINUTE=60");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ParseError::InvalidByMinute("60".into()));

        let res = parse_input("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYMINUTE=4,5,64");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            ParseError::InvalidByMinute("4,5,64".into())
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
