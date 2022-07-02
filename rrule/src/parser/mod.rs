//! Module for parsing text inputs to a [`Grammar`] which can be used
//! to construct an [`RRuleSet`].
mod content_line;
mod datetime;
mod error;
mod regex;
mod utils;

use std::str::FromStr;

pub(crate) use content_line::ContentLine;
pub(crate) use datetime::str_to_weekday;
pub use error::ParseError;

use crate::{core::DateTime, RRule};

use self::content_line::{get_content_line_parts, PropertyName, StartDateContentLine};

#[derive(Debug, PartialEq)]
pub(crate) struct Grammar {
    pub start_datetime: DateTime,
    pub content_lines: Vec<ContentLine>,
}

impl FromStr for Grammar {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut content_lines = vec![];

        let parsed_lines = s
            .lines()
            .into_iter()
            .map(get_content_line_parts)
            .collect::<Result<Vec<_>, _>>()?;

        let start_datetime = parsed_lines
            .iter()
            .find(|parts| matches!(parts.property_name, PropertyName::DtStart))
            .map(|parts| StartDateContentLine::try_from(parts.clone()))
            .ok_or(ParseError::MissingStartDate)??;

        for parts in parsed_lines {
            let line = match parts.property_name {
                PropertyName::RRule => {
                    let rrule = RRule::try_from((parts, &start_datetime))?;
                    ContentLine::RRule(rrule)
                }
                PropertyName::ExRule => {
                    let rrule = RRule::try_from((parts, &start_datetime))?;
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
            return Err(ParseError::MissingRecurrenceRules);
        }

        Ok(Self {
            start_datetime: start_datetime.datetime,
            content_lines,
        })
    }
}

#[cfg(test)]
mod test {
    use chrono::{TimeZone, Weekday};
    use chrono_tz::{Europe, UTC};

    use super::*;
    use crate::{parser::content_line::ContentLine, Frequency, NWeekday, RRule};

    #[test]
    fn parses_valid_input_to_grammar() {
        let tests = [
("DTSTART:19970902T090000Z\nRRULE:FREQ=YEARLY;COUNT=3\n", Grammar {
    start_datetime: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
    content_lines: vec![
        ContentLine::RRule(RRule {
            freq: Frequency::Yearly,
            count: Some(3),
            ..Default::default()
        })
    ]
}),
("DTSTART:20120201T093000Z\nRRULE:FREQ=WEEKLY;INTERVAL=5;UNTIL=20130130T230000Z;BYDAY=MO,FR", Grammar {
    start_datetime: UTC.ymd(2012, 2, 1).and_hms(9, 30, 0),
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
    start_datetime: UTC.ymd(2012, 2, 1).and_hms(12, 0, 0),
    content_lines: vec![
        ContentLine::RRule(RRule {
            freq: Frequency::Daily,
            count: Some(5),
            ..Default::default()
        }),
        ContentLine::ExDate(vec![
            Europe::Berlin.ymd(2012, 2, 2).and_hms(13, 0, 0),
            Europe::Berlin.ymd(2012, 2, 3).and_hms(13, 0, 0),
        ])
    ]
})
        ];
        for (input, expected_grammar) in tests {
            let grammar = Grammar::from_str(input);
            assert_eq!(grammar, Ok(expected_grammar));
        }
    }
}
