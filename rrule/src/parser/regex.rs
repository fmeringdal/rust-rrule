//! Utility functions around the regexes we use for parsing rrule strings.
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use super::{content_line::PropertyName, ParseError};

lazy_static! {
    static ref DATESTR_RE: Regex =
        Regex::new(r"(?m)^([0-9]{4})([0-9]{2})([0-9]{2})(T([0-9]{2})([0-9]{2})([0-9]{2})(Z?))?$")
            .expect("DATESTR_RE regex failed");
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

fn get_datetime_captures<T: FromStr>(
    captures: &Captures,
    idx: usize,
    val: &str,
) -> Result<T, ParseError> {
    captures
        .get(idx)
        .ok_or_else(|| ParseError::InvalidDateTimeFormat(val.into()))?
        .as_str()
        .parse()
        .map_err(|_| ParseError::InvalidDateTimeFormat(val.into()))
}

impl ParsedDateString {
    /// Parses a date string with format `YYYYMMDD(THHMMSSZ)` where the part in parentheses
    /// is optional. It returns [`ParsedDateString`].
    pub(crate) fn from_ical_datetime(val: &str) -> Result<Self, ParseError> {
        let captures = DATESTR_RE
            .captures(val)
            .ok_or_else(|| ParseError::InvalidDateTimeFormat(val.into()))?;

        let year = get_datetime_captures(&captures, 1, val)?;
        let month = get_datetime_captures(&captures, 2, val)?;
        let day = get_datetime_captures(&captures, 3, val)?;

        // Check if time part is captured
        let time = if captures.get(4).is_some() {
            let hour = get_datetime_captures(&captures, 5, val)?;
            let min = get_datetime_captures(&captures, 6, val)?;
            let sec = get_datetime_captures(&captures, 7, val)?;
            Some(ParsedDateStringTime { hour, min, sec })
        } else {
            None
        };

        let zulu_timezone_set = match captures.get(8) {
            Some(part) => part.as_str() == "Z",
            None => false,
        };
        let flags = ParsedDateStringFlags { zulu_timezone_set };

        Ok(Self {
            year,
            month,
            day,
            time,
            flags,
        })
    }
}

lazy_static! {
    static ref PARSE_PROPERTY_NAME_RE: Regex =
        Regex::new(r"(?m)^([A-Z]+?)[:;]").expect("PARSE_PROPERTY_NAME_RE regex failed");
}

/// Get the line property name, the `RRULE:`, `EXRULE:` etc part.
pub(crate) fn get_property_name(val: &str) -> Result<Option<PropertyName>, ParseError> {
    PARSE_PROPERTY_NAME_RE
        .captures(val)
        .and_then(|captures| captures.get(1))
        .map(|name| PropertyName::from_str(name.as_str()))
        .transpose()
}

#[cfg(test)]
mod tests {
    use crate::parser::{content_line::PropertyName, regex::get_property_name, ParseError};

    use super::{ParsedDateString, ParsedDateStringFlags, ParsedDateStringTime};

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
            let output = ParsedDateString::from_ical_datetime(input);
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
            let res = ParsedDateString::from_ical_datetime(input);
            assert!(res.is_err());
        }
    }

    #[test]
    fn parses_property_name_from_line() {
        let tests = [
            (
                "RRULE:FREQ=YEARLY;INTERVAL=2;BYMONTH=1;BYDAY=SU;BYHOUR=8,9;BYMINUTE=30",
                PropertyName::RRule,
            ),
            (
                "EXRULE:FREQ=YEARLY;INTERVAL=2;BYMONTH=1;BYDAY=SU;BYHOUR=8,9;BYMINUTE=30",
                PropertyName::ExRule,
            ),
            (
                "DTSTART;TZID=America/Everywhere:20120251T023000Z\nFREQ=DAILY;INTERVAL=1;",
                PropertyName::DtStart,
            ),
        ];
        for (input, expected_output) in tests {
            let output = get_property_name(input);
            assert_eq!(output, Ok(Some(expected_output)));
        }
    }

    #[test]
    fn parses_line_without_property_name() {
        let tests = [
            GARBAGE_INPUTS.to_vec(),
            vec![
                "FREQ=YEARLY;INTERVAL=2;BYMONTH=1;BYDAY=SU;BYHOUR=8,9;BYMINUTE=30",
                "TZID=America/Everywhere:20120251T023000Z\nFREQ=DAILY;INTERVAL=1;",
            ],
        ]
        .concat();
        for input in tests {
            let res = get_property_name(input);
            assert_eq!(res, Ok(None));
        }
    }

    #[test]
    fn rejects_line_with_invalid_property_name() {
        let tests = [
            (
                "RRULES:FREQ=YEARLY;INTERVAL=2;BYMONTH=1;BYDAY=SU;BYHOUR=8,9;BYMINUTE=30",
                ParseError::UnrecognizedPropertyName("RRULES".into()),
            ),
            (
                "START;TZID=America/Everywhere:20120251T023000Z\nFREQ=DAILY;INTERVAL=1;",
                ParseError::UnrecognizedPropertyName("START".into()),
            ),
        ];
        for (input, expected_output) in tests {
            let output = get_property_name(input);
            assert_eq!(output, Err(expected_output));
        }
    }
}
