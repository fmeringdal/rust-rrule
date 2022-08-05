//! Module for parsing text inputs to a [`Grammar`] which can further be used
//! to construct an [`crate::RRuleSet`].
mod content_line;
mod datetime;
mod error;
mod regex;
mod utils;

use std::str::FromStr;

pub(crate) use content_line::{ContentLine, ContentLineCaptures};
pub(crate) use datetime::str_to_weekday;
pub use error::ParseError;

use crate::RRule;

use self::content_line::{PropertyName, StartDateContentLine};

/// Grammar represents a well formatted rrule input.
#[derive(Debug, PartialEq)]
pub(crate) struct Grammar {
    pub start: StartDateContentLine,
    pub content_lines: Vec<ContentLine>,
}

impl FromStr for Grammar {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let content_lines_parts = s
            .lines()
            .into_iter()
            .map(ContentLineCaptures::new)
            .collect::<Result<Vec<_>, _>>()?;

        let start = content_lines_parts
            .iter()
            .find(|parts| matches!(parts.property_name, PropertyName::DtStart))
            .map(StartDateContentLine::try_from)
            .ok_or(ParseError::MissingStartDate)??;

        let mut content_lines = vec![];

        for parts in content_lines_parts {
            let line = match parts.property_name {
                PropertyName::RRule => {
                    let rrule = RRule::try_from(parts)?;
                    ContentLine::RRule(rrule)
                }
                PropertyName::ExRule => {
                    let rrule = RRule::try_from(parts)?;
                    ContentLine::ExRule(rrule)
                }
                PropertyName::RDate => ContentLine::RDate(TryFrom::try_from(parts)?),
                PropertyName::ExDate => ContentLine::ExDate(TryFrom::try_from(parts)?),
                PropertyName::DtStart => {
                    // Nothing to do
                    continue;
                }
            };
            content_lines.push(line);
        }

        // Need to be at least one `RDATE` or `RRULE`
        if !content_lines
            .iter()
            .any(|line| matches!(line, ContentLine::RRule(_) | ContentLine::RDate(_)))
        {
            return Err(ParseError::MissingDateGenerationRules);
        }

        Ok(Self {
            start,
            content_lines,
        })
    }
}

#[cfg(test)]
mod test {
    use chrono::{TimeZone, Weekday};

    use super::*;
    use crate::{core::Tz, parser::content_line::ContentLine, Frequency, NWeekday, RRule};

    const UTC: Tz = Tz::Tz(chrono_tz::Tz::UTC);
    const BERLIN: Tz = Tz::Tz(chrono_tz::Europe::Berlin);

    #[test]
    fn parses_valid_input_to_grammar() {
        let tests = [
(
    "DTSTART:19970902T090000Z\nRRULE:FREQ=YEARLY;COUNT=3\n", Grammar {
    start: StartDateContentLine { datetime: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0), timezone: Some(UTC), value: "DATE-TIME" },
    content_lines: vec![
        ContentLine::RRule(RRule {
            freq: Frequency::Yearly,
            count: Some(3),
            ..Default::default()
        })
    ]
}
),
("DTSTART:20120201T093000Z\nRRULE:FREQ=WEEKLY;INTERVAL=5;UNTIL=20130130T230000Z;BYDAY=MO,FR", Grammar {
    start: StartDateContentLine { datetime: UTC.ymd(2012, 2, 1).and_hms(9, 30, 0), timezone: Some(UTC), value: "DATE-TIME" },
    content_lines: vec![
        ContentLine::RRule(RRule {
            freq: Frequency::Weekly,
            interval: 5,
            until: Some(UTC.ymd(2013, 1, 30).and_hms(23, 0, 0)),
            by_weekday: vec![NWeekday::Every(Weekday::Mon), NWeekday::Every(Weekday::Fri)],
            ..Default::default()
        })
    ]
}),
("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5\nEXDATE;TZID=Europe/Berlin:20120202T130000,20120203T130000", Grammar {
    start: StartDateContentLine { datetime: UTC.ymd(2012, 2, 1).and_hms(12, 0, 0), timezone: Some(UTC), value: "DATE-TIME" },
    content_lines: vec![
        ContentLine::RRule(RRule {
            freq: Frequency::Daily,
            count: Some(5),
            ..Default::default()
        }),
        ContentLine::ExDate(vec![
            BERLIN.ymd(2012, 2, 2).and_hms(13, 0, 0),
            BERLIN.ymd(2012, 2, 3).and_hms(13, 0, 0),
        ])
    ]
}),
("DTSTART:20120201T120000Z\nRRULE:FREQ=DAILY;COUNT=5\nEXDATE;TZID=Europe/Berlin:20120202T130000,20120203T130000\nEXRULE:FREQ=WEEKLY;COUNT=10", Grammar {
    start: StartDateContentLine { datetime: UTC.ymd(2012, 2, 1).and_hms(12, 0, 0), timezone: Some(UTC), value: "DATE-TIME" },
    content_lines: vec![
        ContentLine::RRule(RRule {
            freq: Frequency::Daily,
            count: Some(5),
            ..Default::default()
        }),
        ContentLine::ExDate(vec![
            BERLIN.ymd(2012, 2, 2).and_hms(13, 0, 0),
            BERLIN.ymd(2012, 2, 3).and_hms(13, 0, 0),
        ]),
        ContentLine::ExRule(RRule {
            freq: Frequency::Weekly,
            count: Some(10),
            ..Default::default()
        }),
    ]
})
        ];
        for (input, expected_grammar) in tests {
            let grammar = Grammar::from_str(input);
            assert_eq!(grammar, Ok(expected_grammar));
        }
    }

    #[test]
    fn rejects_input_without_date_generation() {
        let tests = [
"DTSTART:19970902T090000Z",
"DTSTART:20120201T093000Z\nEXRULE:FREQ=WEEKLY;INTERVAL=5;UNTIL=20130130T230000Z;BYDAY=MO,FR",
"DTSTART:20120201T120000Z\nEXDATE;TZID=Europe/Berlin:20120202T130000,20120203T130000",
"DTSTART:20120201T120000Z\nEXRULE:FREQ=DAILY;COUNT=5\nEXDATE;TZID=Europe/Berlin:20120202T130000,20120203T130000"
        ];
        for input in tests {
            let res = Grammar::from_str(input);
            assert_eq!(res, Err(ParseError::MissingDateGenerationRules));
        }
    }

    #[test]
    fn rejects_input_without_start_date() {
        let tests = [
            "RRULE:FREQ=WEEKLY;INTERVAL=5;UNTIL=20130130T230000Z;BYDAY=MO,FR",
            "RDATE;TZID=Europe/Berlin:20120202T130000,20120203T130000",
            "RRULE:FREQ=DAILY;COUNT=5\nEXDATE;TZID=Europe/Berlin:20120202T130000,20120203T130000",
        ];
        for input in tests {
            let res = Grammar::from_str(input);
            assert_eq!(res, Err(ParseError::MissingStartDate));
        }
    }
}
