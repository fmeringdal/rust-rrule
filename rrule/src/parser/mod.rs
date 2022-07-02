//! Module for parsing text inputs to `RRule` and `RRuleSet` types.
//!
//! # Implementation
//! Text input -> Grammar (AST) -> RRuleSet
mod datetime;
mod error;
pub(crate) mod grammar;
mod regex;
mod utils;

use std::{marker::PhantomData, str::FromStr};

use chrono::Weekday;
use chrono_tz::UTC;
pub(crate) use datetime::str_to_weekday;
pub use error::ParseError;

use crate::core::DateTime;
use crate::{Frequency, RRule, Unvalidated};

use datetime::{datestring_to_date, parse_timezone, parse_weekdays};
use utils::parse_str_to_vec;

use self::grammar::{ContentLine, DateParameter, Grammar, RRuleContentLine, RRuleProperty};

/// Takes a recurrence line (i.e. `RRULE` or `EXRULE` line) and parses it
/// into an [`RRule`].
#[allow(clippy::too_many_lines)]
pub(crate) fn parse_rrule(line: &RRuleContentLine) -> Result<RRule<Unvalidated>, ParseError> {
    let freq = line
        .0
        .get(&RRuleProperty::Freq)
        .map(|freq| Frequency::from_str(freq))
        .transpose()?
        .ok_or_else(|| ParseError::MissingProperty("FREQ".into()))?;
    let interval = line
        .0
        .get(&RRuleProperty::Interval)
        .map(|interval| {
            interval
                .parse()
                .map_err(|_| ParseError::InvalidInterval(interval.into()))
        })
        .transpose()?
        .unwrap_or(1);
    let count = line
        .0
        .get(&RRuleProperty::Count)
        .map(|count| {
            count
                .parse()
                .map_err(|_| ParseError::InvalidCount(count.into()))
        })
        .transpose()?;
    let until = line
        .0
        .get(&RRuleProperty::Until)
        .map(|until| {
            // Until is always in UTC
            // TODO: Comment above is not true because of:
            // > [...]
            // > Furthermore, if the "DTSTART" property is specified as a date with local time,
            // > then the UNTIL rule part MUST also be specified as a date with local time.
            //
            // Thus This can be in local time
            datestring_to_date(until, Some(UTC), "UNTIL")
        })
        .transpose()?;
    let week_start = line
        .0
        .get(&RRuleProperty::Wkst)
        .map(|week_start| {
            str_to_weekday(week_start)
                .map_err(|_| ParseError::InvalidWeekdayStart(week_start.into()))
        })
        .transpose()?
        .unwrap_or(Weekday::Mon);
    let by_set_pos = line
        .0
        .get(&RRuleProperty::BySetPos)
        .map(|by_set_pos| {
            parse_str_to_vec(by_set_pos, |_| true)
                .map_err(|_| ParseError::InvalidBySetPos(by_set_pos.into()))
        })
        .transpose()?
        .unwrap_or_default();
    let by_month = line
        .0
        .get(&RRuleProperty::ByMonth)
        .map(|by_month| {
            parse_str_to_vec(by_month, |month| (1..=12).contains(&month))
                .map_err(|_| ParseError::InvalidByMonth(by_month.into()))
        })
        .transpose()?
        .unwrap_or_default();
    let by_month_day = line
        .0
        .get(&RRuleProperty::ByMonthDay)
        .map(|by_month_day| {
            parse_str_to_vec(by_month_day, |monthday| (-31..=31).contains(&monthday))
                .map_err(|_| ParseError::InvalidByMonthDay(by_month_day.into()))
        })
        .transpose()?
        .unwrap_or_default();
    let by_year_day = line
        .0
        .get(&RRuleProperty::ByYearDay)
        .map(|by_year_day| {
            parse_str_to_vec(by_year_day, |yearday| (-366..=366).contains(&yearday))
                .map_err(|_| ParseError::InvalidByYearDay(by_year_day.into()))
        })
        .transpose()?
        .unwrap_or_default();
    let by_week_no = line
        .0
        .get(&RRuleProperty::ByWeekNo)
        .map(|by_week_no| {
            parse_str_to_vec(by_week_no, |weekno| (-53..=53).contains(&weekno))
                .map_err(|_| ParseError::InvalidByWeekNo(by_week_no.into()))
        })
        .transpose()?
        .unwrap_or_default();
    let by_weekday = line
        .0
        .get(&RRuleProperty::ByDay)
        .map(|by_weekday| parse_weekdays(by_weekday))
        .transpose()?
        .unwrap_or_default();
    let by_hour = line
        .0
        .get(&RRuleProperty::ByHour)
        .map(|by_hour| {
            parse_str_to_vec(by_hour, |hour| hour < 24)
                .map_err(|_| ParseError::InvalidByHour(by_hour.into()))
        })
        .transpose()?
        .unwrap_or_default();
    let by_minute = line
        .0
        .get(&RRuleProperty::ByMinute)
        .map(|by_minute| {
            parse_str_to_vec(by_minute, |minute| minute < 60)
                .map_err(|_| ParseError::InvalidByMinute(by_minute.into()))
        })
        .transpose()?
        .unwrap_or_default();
    let by_second = line
        .0
        .get(&RRuleProperty::BySecond)
        .map(|by_second| {
            parse_str_to_vec(by_second, |second| second < 60)
                .map_err(|_| ParseError::InvalidBySecond(by_second.into()))
        })
        .transpose()?
        .unwrap_or_default();

    #[allow(unused_mut)]
    // TODO: fix by easter
    let mut by_easter = None;

    // Check if mandatory fields are set
    Ok(RRule {
        freq,
        interval,
        count,
        until,
        week_start,
        by_set_pos,
        by_month,
        by_month_day,
        by_n_month_day: vec![],
        by_year_day,
        by_week_no,
        by_weekday,
        by_hour,
        by_minute,
        by_second,
        by_easter,
        stage: PhantomData,
    })
}

#[derive(Debug)]
pub(crate) struct ParsedInput {
    pub rrule_vals: Vec<RRule<Unvalidated>>,
    pub rdate_vals: Vec<DateTime>,
    pub exrule_vals: Vec<RRule<Unvalidated>>,
    pub exdate_vals: Vec<DateTime>,
    pub dt_start: DateTime,
}

// TODO: document this entrypoint
pub(crate) fn parse(s: &str) -> Result<ParsedInput, ParseError> {
    let mut rrule_vals = vec![];
    let mut rdate_vals = vec![];
    let mut exrule_vals = vec![];
    let mut exdate_vals = vec![];

    let grammar = Grammar::from_str(s)?;

    let start_timezone = grammar
        .start_datetime
        .parameters
        .get(&DateParameter::Timezone)
        .map(|tz| parse_timezone(tz))
        .transpose()?;
    let dt_start = datestring_to_date(&grammar.start_datetime.date, start_timezone, "DTSTART")?;

    for content_line in &grammar.content_lines {
        match content_line {
            ContentLine::RRule(rrule_line) => {
                rrule_vals.push(parse_rrule(rrule_line)?);
            }
            ContentLine::ExRule(rrule_line) => {
                exrule_vals.push(parse_rrule(rrule_line)?);
            }
            ContentLine::RDate(date_line) => {
                let timezone = date_line
                    .parameters
                    .get(&DateParameter::Timezone)
                    .map(|tz| parse_timezone(tz))
                    .transpose()?;

                for datestr in &date_line.dates {
                    rdate_vals.push(datestring_to_date(datestr, timezone, "RDATE")?);
                }
            }
            ContentLine::ExDate(exdate_line) => {
                let timezone = exdate_line
                    .parameters
                    .get(&DateParameter::Timezone)
                    .map(|tz| parse_timezone(tz))
                    .transpose()?;

                for datestr in &exdate_line.dates {
                    exdate_vals.push(datestring_to_date(datestr, timezone, "EXDATE")?);
                }
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
            let res = parse(test_str);
            assert!(res.is_ok());
        }
    }

    #[test]
    fn rrule() {
        let res = parse("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5");
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.rrule_vals.len(), 1);
        let props = &res.rrule_vals[0];
        assert_eq!(props.interval, 1);
        assert_eq!(props.count.unwrap(), 5);
        assert_eq!(props.freq, Frequency::Daily);
    }

    #[test]
    fn exrule() {
        let res = parse(
            "DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5\nEXRULE:FREQ=WEEKLY;INTERVAL=2",
        );
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.exrule_vals.len(), 1);
        let props = &res.exrule_vals[0];
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
        }
    }

    #[test]
    fn garbage_strings_rrule_set() {
        let test_cases = vec!["helloworld", "foo bar", "hello\nworld", "RRUle:test"];
        for test_case in &test_cases {
            let res = parse(test_case);
            assert!(res.is_err());
        }
    }

    #[test]
    fn invalid_dtstart() {
        let res = parse("DTSTART:20120201120000Z\nRRULE:FREQ=DAILY;COUNT=5");
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
        let res = parse("DTSTART:20120201T120000Z\nRRULE:FREQ=DAIL;COUNT=5");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            ParseError::InvalidFrequency("DAIL".into()).into()
        );
    }

    #[test]
    fn invalid_byhour() {
        let res = parse("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYHOUR=24");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ParseError::InvalidByHour("24".into()));

        let res = parse("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYHOUR=5,6,25");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ParseError::InvalidByHour("5,6,25".into()));
    }

    #[test]
    fn invalid_byminute() {
        let res = parse("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYMINUTE=60");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ParseError::InvalidByMinute("60".into()));

        let res = parse("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5;BYMINUTE=4,5,64");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            ParseError::InvalidByMinute("4,5,64".into())
        );
    }

    #[test]
    fn parses_dtstart_when_just_date() {
        let res = parse("DTSTART;VALUE=DATE:20200812\nRRULE:FREQ=WEEKLY;UNTIL=20210511T220000Z;INTERVAL=1;BYDAY=WE;WKST=MO");
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
            let res = parse(case);
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
            let opts_or_err = RRule::from_str(cases[i]);
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
