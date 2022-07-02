use std::collections::HashMap;

use super::{
    content_line_parts::ContentLineCaptures, date_content_line::DateParameter,
    parameters::parse_parameters,
};
use crate::{
    core::DateTime,
    parser::{
        datetime::{datestring_to_date, parse_timezone},
        ParseError,
    },
};

impl TryFrom<ContentLineCaptures> for DateTime {
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

        let datetime = datestring_to_date(&value.properties, timezone, "DTSTART")?;

        Ok(datetime)
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use chrono_tz::UTC;

    use crate::parser::content_line::PropertyName;

    use super::*;

    #[test]
    fn parses_dtstart_content_line() {
        let tests = [
            (
                ContentLineCaptures {
                    property_name: PropertyName::DtStart,
                    parameters: None,
                    properties: "19970714T123000Z".into(),
                },
                UTC.ymd(1997, 7, 14).and_hms(12, 30, 0),
            ),
            (
                ContentLineCaptures {
                    property_name: PropertyName::DtStart,
                    parameters: Some("VALUE=DATE;TZID=UTC".into()),
                    properties: "19970101".into(),
                },
                UTC.ymd(1997, 1, 1).and_hms(0, 0, 0),
            ),
        ];

        for (input, expected_output) in tests {
            let output = TryFrom::try_from(input);
            assert_eq!(output, Ok(expected_output));
        }
    }

    #[test]
    fn rejecets_invalid_dtstart() {
        let tests = [
            ContentLineCaptures {
                property_name: PropertyName::DtStart,
                parameters: None,
                properties: "20120201120000Z".into(),
            },
            ContentLineCaptures {
                property_name: PropertyName::DtStart,
                parameters: None,
                properties: "2012".into(),
            },
            ContentLineCaptures {
                property_name: PropertyName::DtStart,
                parameters: None,
                properties: "".into(),
            },
        ];

        for input in tests {
            let output = DateTime::try_from(input.clone());
            assert_eq!(
                output,
                Err(ParseError::InvalidDateTime {
                    value: input.properties,
                    property: "DTSTART".into()
                })
            );
        }
    }

    #[test]
    fn reject_invalid_timezone_in_start_date() {
        let content = ContentLineCaptures {
            property_name: PropertyName::DtStart,
            parameters: Some("TZID=America/Everywhere".into()),
            properties: "20120251T023000Z".into(),
        };
        let res = DateTime::try_from(content);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err,
            ParseError::InvalidTimezone("America/Everywhere".into()).into()
        );
    }
}
