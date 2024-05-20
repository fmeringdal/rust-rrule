//! Utility functions around the parsing rrule strings.
use std::str::FromStr;

use crate::{parser::content_line::PropertyName, ParseError};

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

impl ParsedDateString {
    /// Parses a date string with format `YYYYMMDD(THHMMSSZ)` where the part in parentheses
    /// is optional. It returns [`ParsedDateString`].
    pub(crate) fn from_ical_datetime(val: &str) -> Result<Self, ParseError> {
        if !val.is_ascii() {
            // String should only contain valid ascii characters (0-9TZ), eg. no
            // multi-byte characters.
            return Err(ParseError::InvalidDateTimeFormat(val.into()));
        }
        let len = val.find(|c| c == '\n' || c == '\r').unwrap_or(val.len());
        if len < 8 {
            // Not a valid YYYYMMDD date.
            return Err(ParseError::InvalidDateTimeFormat(val.into()));
        }

        // Parse date (YYYYMMDD).
        let year = val[0..4]
            .parse::<u32>()
            .map_err(|_err| ParseError::InvalidDateTimeFormat(val.into()))?
            .try_into()
            .map_err(|_err| ParseError::InvalidDateTimeFormat(val.into()))?;
        let month = val[4..6]
            .parse()
            .map_err(|_err| ParseError::InvalidDateTimeFormat(val.into()))?;
        let day = val[6..8]
            .parse()
            .map_err(|_err| ParseError::InvalidDateTimeFormat(val.into()))?;

        // Parse optional time (THHMMSS(Z)).
        let (time, zulu_timezone_set) = if (15..=16).contains(&len) && &val[8..9] == "T" {
            let hour = val[9..11]
                .parse()
                .map_err(|_err| ParseError::InvalidDateTimeFormat(val.into()))?;
            let min = val[11..13]
                .parse()
                .map_err(|_err| ParseError::InvalidDateTimeFormat(val.into()))?;
            let sec = val[13..15]
                .parse()
                .map_err(|_err| ParseError::InvalidDateTimeFormat(val.into()))?;

            let time = ParsedDateStringTime { hour, min, sec };
            let zulu_timezone_set = val.get(15..16) == Some("Z");

            (Some(time), zulu_timezone_set)
        } else if len > 8 {
            // Value is longer than date but either not long enough to fit time or missing 'T'.
            return Err(ParseError::InvalidDateTimeFormat(val.into()));
        } else {
            // No time provided.
            (None, false)
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

/// Get the line property name, the `RRULE:`, `EXRULE:` etc part.
pub(crate) fn get_property_name(val: &str) -> Result<Option<PropertyName>, ParseError> {
    let Some(end) = val.find(|c| c == ':' || c == ';') else {
        return Ok(None);
    };
    if val[..end].chars().all(|c| c.is_ascii_uppercase()) {
        PropertyName::from_str(&val[..end]).map(Some)
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                "202💥0123T023000Z",
                "20210💥23T023000Z",
                "202101💥3T023000Z",
                "20210123T02💥000Z",
                "20210123T023💥00Z",
            ]
            .to_vec(),
        ]
        .concat();
        for input in tests {
            let res = ParsedDateString::from_ical_datetime(input);
            assert!(res.is_err(), "{}", input);
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
