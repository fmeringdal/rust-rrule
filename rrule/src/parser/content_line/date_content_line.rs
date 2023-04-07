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

impl<'a> TryFrom<ContentLineCaptures<'a>> for Vec<DateTime> {
    type Error = ParseError;

    fn try_from(value: ContentLineCaptures) -> Result<Self, Self::Error> {
        let parameters: HashMap<DateParameter, String> = value
            .parameters
            .map(parse_parameters)
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

    use crate::{core::Tz, parser::content_line::PropertyName};

    use super::*;

    const UTC: Tz = Tz::UTC;

    #[test]
    fn parses_date_content_line() {
        let tests = [
            (
                ContentLineCaptures {
                    property_name: PropertyName::RDate,
                    parameters: None,
                    value: "19970714T123000Z",
                },
                vec![UTC.with_ymd_and_hms(1997, 7, 14, 12, 30, 0).unwrap()],
            ),
            (
                ContentLineCaptures {
                    property_name: PropertyName::RDate,
                    parameters: None,
                    value: "19970714T123000",
                },
                vec![Tz::LOCAL.with_ymd_and_hms(1997, 7, 14, 12, 30, 0).unwrap()],
            ),
            (
                ContentLineCaptures {
                    property_name: PropertyName::RDate,
                    parameters: Some("VALUE=DATE;TZID=UTC"),
                    value: "19970101,19970120,19970217,19970421",
                },
                vec![
                    UTC.with_ymd_and_hms(1997, 1, 1, 0, 0, 0).unwrap(),
                    UTC.with_ymd_and_hms(1997, 1, 20, 0, 0, 0).unwrap(),
                    UTC.with_ymd_and_hms(1997, 2, 17, 0, 0, 0).unwrap(),
                    UTC.with_ymd_and_hms(1997, 4, 21, 0, 0, 0).unwrap(),
                ],
            ),
        ];

        for (input, expected_output) in tests {
            let output = TryFrom::try_from(input);
            assert_eq!(output, Ok(expected_output));
        }
    }
}
