use std::{collections::HashMap, str::FromStr};

use crate::{
    core::DateTime,
    parser::{
        datetime::{datestring_to_date, parse_timezone},
        ParseError,
    },
};

use super::{content_line_parts::ContentLineCaptures, parameters::parse_parameters};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum DateParameter {
    Timezone,
    Value,
}

impl FromStr for DateParameter {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let param = match &s.to_uppercase()[..] {
            "TZID" => Self::Timezone,
            "VALUE" => Self::Value,
            _ => return Err(ParseError::UnrecognizedParameter(s.into())),
        };
        Ok(param)
    }
}

impl TryFrom<ContentLineCaptures> for Vec<DateTime> {
    type Error = ParseError;

    fn try_from(value: ContentLineCaptures) -> Result<Self, Self::Error> {
        let parameters: HashMap<DateParameter, String> = value
            .parameters
            .map(|p| parse_parameters(&p))
            .transpose()?
            .unwrap_or_default();

        let timezone = parameters
            .get(&DateParameter::Timezone)
            .map(|tz| parse_timezone(tz))
            .transpose()?;
        let property = format!("{}", value.property_name);

        let mut dates = vec![];
        for val in value.value.split(',') {
            if val.is_empty() {
                continue;
            }
            let datetime = datestring_to_date(val, timezone, &property)?;
            dates.push(datetime);
        }

        Ok(dates)
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use chrono_tz::UTC;

    use crate::parser::content_line::PropertyName;

    use super::*;

    #[test]
    fn parses_date_content_line() {
        let tests = [
            (
                ContentLineCaptures {
                    property_name: PropertyName::RDate,
                    parameters: None,
                    value: "19970714T123000Z".into(),
                },
                vec![UTC.ymd(1997, 7, 14).and_hms(12, 30, 0)],
            ),
            (
                ContentLineCaptures {
                    property_name: PropertyName::RDate,
                    parameters: Some("VALUE=DATE;TZID=UTC".into()),
                    value: "19970101,19970120,19970217,19970421".into(),
                },
                vec![
                    UTC.ymd(1997, 1, 1).and_hms(0, 0, 0),
                    UTC.ymd(1997, 1, 20).and_hms(0, 0, 0),
                    UTC.ymd(1997, 2, 17).and_hms(0, 0, 0),
                    UTC.ymd(1997, 4, 21).and_hms(0, 0, 0),
                ],
            ),
        ];

        for (input, expected_output) in tests {
            let output = TryFrom::try_from(input);
            assert_eq!(output, Ok(expected_output));
        }
    }
}
