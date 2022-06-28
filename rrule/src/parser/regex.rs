//! Utility functions around the regexes we use for parsing rrule strings.
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DATESTR_RE: Regex =
        Regex::new(r"(?m)^(\d{4})(\d{2})(\d{2})(T(\d{2})(\d{2})(\d{2})(Z?))?$").unwrap();
}

#[derive(Debug, PartialEq)]
pub(crate) struct ParsedDateString {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub time: Option<ParsedDateStringTime>,
    pub flags: ParsedDateStringFlags,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ParsedDateStringFlags {
    pub zulu_timezone_set: bool,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ParsedDateStringTime {
    pub hour: u32,
    pub min: u32,
    pub sec: u32,
}

/// Parses a date string with format `YYYYMMDD(THHMMSSZ)` where the part in paranthesis
/// is optional. It returns [`ParsedDateString`].
pub(crate) fn parse_datestring(val: &str) -> Result<ParsedDateString, ()> {
    let captures = DATESTR_RE.captures(val).ok_or(())?;

    let year = captures
        .get(1)
        .ok_or(())?
        .as_str()
        .parse()
        .map_err(|_| ())?;
    let month = captures
        .get(2)
        .ok_or(())?
        .as_str()
        .parse()
        .map_err(|_| ())?;
    let day = captures
        .get(3)
        .ok_or(())?
        .as_str()
        .parse()
        .map_err(|_| ())?;

    // Check if time part is captured
    let time = if captures.get(4).is_some() {
        let hour = captures
            .get(5)
            .ok_or(())?
            .as_str()
            .parse()
            .map_err(|_| ())?;
        let min = captures
            .get(6)
            .ok_or(())?
            .as_str()
            .parse()
            .map_err(|_| ())?;
        let sec = captures
            .get(7)
            .ok_or(())?
            .as_str()
            .parse()
            .map_err(|_| ())?;
        Some(ParsedDateStringTime { hour, min, sec })
    } else {
        None
    };

    let zulu_timezone_set = match captures.get(8) {
        Some(part) => part.as_str() == "Z",
        None => false,
    };
    let flags = ParsedDateStringFlags { zulu_timezone_set };

    Ok(ParsedDateString {
        year,
        month,
        day,
        time,
        flags,
    })
}

lazy_static! {
    static ref DTSTART_RE: Regex =
        Regex::new(r"(?m)DTSTART(?:;TZID=([^:=]+?))?(?::|=)([^;\s]+)").unwrap();
}

#[derive(Debug, PartialEq)]
pub(crate) struct ParsedStartDatetime {
    pub timezone: Option<String>,
    pub datetime: String,
}

/// Parses the `DTSTART:...` line and returns the datetime value along with the
/// specified timezone.
pub(crate) fn parse_start_datetime(val: &str) -> Result<ParsedStartDatetime, ()> {
    let captures = DTSTART_RE.captures(val).ok_or(())?;

    let timezone = captures.get(1).map(|tz| tz.as_str().into());
    let datetime = captures.get(2).ok_or(())?.as_str().into();

    Ok(ParsedStartDatetime { timezone, datetime })
}

lazy_static! {
    static ref RRULE_RE: Regex = Regex::new(r"(?m)^(?:RRULE|EXRULE):").unwrap();
}

/// Retrieve the attributes from an `RRULE:...` or `EXRULE:...` line.
pub(crate) fn get_rrule_attributes(val: &str) -> Option<Vec<String>> {
    let attributes = RRULE_RE.replace(val, "");
    let attributes = attributes.split(';').map(From::from).collect();
    Some(attributes)
}

lazy_static! {
    static ref PARSE_RULE_LINE_RE: Regex = Regex::new(r"(?m)^([A-Z]+?)[:;]").unwrap();
}

/// Get the line header, the `RRULE:`, `EXRULE:` etc part.
pub(crate) fn get_line_header(val: &str) -> Option<String> {
    let captures = PARSE_RULE_LINE_RE.captures(val)?;
    captures.get(1).map(|header| header.as_str().into())
}

lazy_static! {
    static ref RDATE_RE: Regex = Regex::new(r"(?m)RDATE(?:;TZID=([^:=]+))?").unwrap();
}

/// Get the timezone in the `RDATE:...` line. Returns `Err` if invalid `RDATE` line.
pub(crate) fn get_rdate_timezone(val: &str) -> Result<Option<String>, ()> {
    let captures = RDATE_RE.captures(val).ok_or(())?;
    Ok(captures.get(1).map(|tz| tz.as_str().into()))
}

lazy_static! {
    static ref EXDATE_RE: Regex = Regex::new(r"(?m)EXDATE(?:;TZID=([^:=]+))?").unwrap();
}

/// Get the timezone in the `EXDATE:...` line. Returns `Err` if invalid `EXDATE` line.
pub(crate) fn get_exdate_timezone(val: &str) -> Result<Option<String>, ()> {
    let captures = EXDATE_RE.captures(val).ok_or(())?;
    Ok(captures.get(1).map(|tz| tz.as_str().into()))
}

#[cfg(test)]
mod tests {
    use crate::parser::regex::{
        get_exdate_timezone, get_line_header, get_rdate_timezone, get_rrule_attributes,
        parse_start_datetime,
    };

    use super::{
        parse_datestring, ParsedDateString, ParsedDateStringFlags, ParsedDateStringTime,
        ParsedStartDatetime,
    };

    const GARBAGE_INPUTS: [&str; 4] = ["", "  ", "fasfa!2414", "-20101017T120000Z"];

    #[test]
    fn parses_valid_datestrings_correctly() {
        let tests = [
            (
                "20101017T120000Z",
                ParsedDateString {
                    year: 2010,
                    month: 10,
                    day: 17,
                    time: Some(ParsedDateStringTime {
                        hour: 12,
                        min: 0,
                        sec: 0,
                    }),
                    flags: ParsedDateStringFlags {
                        zulu_timezone_set: true,
                    },
                },
            ),
            (
                "20101017",
                ParsedDateString {
                    year: 2010,
                    month: 10,
                    day: 17,
                    time: None,
                    flags: ParsedDateStringFlags {
                        zulu_timezone_set: false,
                    },
                },
            ),
            (
                "20220101T121049Z",
                ParsedDateString {
                    year: 2022,
                    month: 1,
                    day: 1,
                    time: Some(ParsedDateStringTime {
                        hour: 12,
                        min: 10,
                        sec: 49,
                    }),
                    flags: ParsedDateStringFlags {
                        zulu_timezone_set: true,
                    },
                },
            ),
            (
                "20220101",
                ParsedDateString {
                    year: 2022,
                    month: 1,
                    day: 1,
                    time: None,
                    flags: ParsedDateStringFlags {
                        zulu_timezone_set: false,
                    },
                },
            ),
        ];
        for (input, expected_output) in tests {
            let output = parse_datestring(input);
            assert_eq!(output, Ok(expected_output));
        }
    }

    #[test]
    fn rejects_invalid_datestrings() {
        let tests = [
            GARBAGE_INPUTS.to_vec(),
            [
                "-20101017T120000Z",
                "20101017T",
                "201010177",
                "20101017T1200",
                "210101017T1200",
            ]
            .to_vec(),
        ]
        .concat();
        for input in tests {
            let res = parse_datestring(input);
            assert!(res.is_err());
        }
    }

    #[test]
    fn parses_valid_start_dates_correctly() {
        let tests = [
            (
                "DTSTART;TZID=America/Everywhere:20120251T023000Z",
                ParsedStartDatetime {
                    datetime: "20120251T023000Z".into(),
                    timezone: Some("America/Everywhere".into()),
                },
            ),
            (
                "DTSTART:20120251T023000Z",
                ParsedStartDatetime {
                    datetime: "20120251T023000Z".into(),
                    timezone: None,
                },
            ),
            (
                "DTSTART;TZID=America/Everywhere:20120251T023000Z\nFREQ=DAILY;INTERVAL=1;",
                ParsedStartDatetime {
                    datetime: "20120251T023000Z".into(),
                    timezone: Some("America/Everywhere".into()),
                },
            ),
        ];
        for (input, expected_output) in tests {
            let output = parse_start_datetime(input);
            assert_eq!(output, Ok(expected_output));
        }
    }

    #[test]
    fn rejects_invalid_start_dates() {
        let tests = [
            GARBAGE_INPUTS.to_vec(),
            [
                "DTSTAR;TZID=America/Everywhere:20120251T023000Z",
                "DTSTART20120251T023000Z",
            ]
            .to_vec(),
        ]
        .concat();

        for input in tests {
            let res = parse_start_datetime(input);
            assert!(
                res.is_err(),
                "unexpected result {:?} for input {:?}",
                res,
                input
            );
        }
    }

    #[test]
    fn parses_valid_attributes_from_an_rrule_line() {
        let tests = [
            (
                "RRULE:FREQ=YEARLY;INTERVAL=2;BYMONTH=1;BYDAY=SU;BYHOUR=8,9;BYMINUTE=30",
                vec![
                    "FREQ=YEARLY",
                    "INTERVAL=2",
                    "BYMONTH=1",
                    "BYDAY=SU",
                    "BYHOUR=8,9",
                    "BYMINUTE=30",
                ],
            ),
            (
                "FREQ=YEARLY;INTERVAL=2;BYMONTH=1;BYDAY=SU;BYHOUR=8,9;BYMINUTE=30",
                vec![
                    "FREQ=YEARLY",
                    "INTERVAL=2",
                    "BYMONTH=1",
                    "BYDAY=SU",
                    "BYHOUR=8,9",
                    "BYMINUTE=30",
                ],
            ),
            (
                "EXRULE:FREQ=YEARLY;INTERVAL=2;BYMONTH=1;BYDAY=SU;BYHOUR=8,9;BYMINUTE=30",
                vec![
                    "FREQ=YEARLY",
                    "INTERVAL=2",
                    "BYMONTH=1",
                    "BYDAY=SU",
                    "BYHOUR=8,9",
                    "BYMINUTE=30",
                ],
            ),
        ];
        for (input, expected_output) in tests {
            let output = get_rrule_attributes(input);
            let expected_output = expected_output.iter().map(|val| val.to_string()).collect();
            assert_eq!(output, Some(expected_output));
        }
    }

    #[test]
    fn parses_line_header() {
        let tests = [
            (
                "RRULE:FREQ=YEARLY;INTERVAL=2;BYMONTH=1;BYDAY=SU;BYHOUR=8,9;BYMINUTE=30",
                "RRULE",
            ),
            (
                "EXRULE:FREQ=YEARLY;INTERVAL=2;BYMONTH=1;BYDAY=SU;BYHOUR=8,9;BYMINUTE=30",
                "EXRULE",
            ),
            (
                "DTSTART;TZID=America/Everywhere:20120251T023000Z\nFREQ=DAILY;INTERVAL=1;",
                "DTSTART",
            ),
        ];
        for (input, expected_output) in tests {
            let output = get_line_header(input);
            assert_eq!(output, Some(expected_output.into()));
        }
    }

    #[test]
    fn parses_line_without_header() {
        let tests = [
            GARBAGE_INPUTS.to_vec(),
            vec![
                "FREQ=YEARLY;INTERVAL=2;BYMONTH=1;BYDAY=SU;BYHOUR=8,9;BYMINUTE=30",
                "TZID=America/Everywhere:20120251T023000Z\nFREQ=DAILY;INTERVAL=1;",
            ],
        ]
        .concat();
        for input in tests {
            let res = get_line_header(input);
            assert!(res.is_none());
        }
    }

    macro_rules! test_parses_timezone_from_xdate_line {
        ($test_name:ident, $fail_test_name:ident, $fn:ident, $variant:expr) => {
            #[test]
            fn $test_name() {
                let variant = $variant;
                let tests = [
                    (
                        format!("{};TZID=America/Everywhere:19970714T083000", variant),
                        Some("America/Everywhere".to_string()),
                    ),
                    (format!("{}:19970714T123000Z", variant), None),
                    (format!("{};VALUE=DATE:19970101,19970120", variant), None),
                    (
                        format!(
                            "{};TZID=America/Everywhere:VALUE=DATE:19970101,19970120",
                            variant
                        ),
                        Some("America/Everywhere".to_string()),
                    ),
                ];
                for (input, expected_output) in tests {
                    let output = $fn(&input);
                    assert_eq!(output, Ok(expected_output));
                }
            }

            #[test]
            fn $fail_test_name() {
                let tests = [
                    GARBAGE_INPUTS.to_vec(),
                    vec![
                        "FREQ=YEARLY;INTERVAL=2;BYMONTH=1;BYDAY=SU;BYHOUR=8,9;BYMINUTE=30",
                        "TZID=America/Everywhere:20120251T023000Z\nFREQ=DAILY;INTERVAL=1;",
                    ],
                ]
                .concat();
                for input in tests {
                    let res = $fn(input);
                    assert!(res.is_err());
                }
            }
        };
    }

    test_parses_timezone_from_xdate_line!(
        parses_timezone_from_rdate_line,
        rejects_timezone_from_invalid_rdate_line,
        get_rdate_timezone,
        "RDATE"
    );

    test_parses_timezone_from_xdate_line!(
        parses_timezone_from_exdate_line,
        rejects_timezone_from_invalid_exdate_line,
        get_exdate_timezone,
        "EXDATE"
    );
}
