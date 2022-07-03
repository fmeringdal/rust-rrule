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

#[derive(Debug, PartialEq)]
pub(crate) struct StartDateContentLine {
    pub datetime: DateTime,
    pub is_local_tz: bool,
}

impl TryFrom<&ContentLineCaptures> for StartDateContentLine {
    type Error = ParseError;

    fn try_from(value: &ContentLineCaptures) -> Result<Self, Self::Error> {
        let parameters: HashMap<DateParameter, String> = value
            .parameters
            .as_ref()
            .map(|p| parse_parameters(p))
            .transpose()?
            .unwrap_or_default();

        let timezone = parameters
            .get(&DateParameter::Timezone)
            .map(|tz| parse_timezone(tz))
            .transpose()?;

        let is_local_tz = timezone.is_none() && !value.value.to_uppercase().ends_with('Z');

        let datetime = datestring_to_date(&value.value, timezone, "DTSTART")?;

        Ok(StartDateContentLine {
            datetime,
            is_local_tz,
        })
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
                    value: "19970714T123000Z".into(),
                },
                StartDateContentLine {
                    datetime: UTC.ymd(1997, 7, 14).and_hms(12, 30, 0),
                    is_local_tz: false,
                },
            ),
            (
                ContentLineCaptures {
                    property_name: PropertyName::DtStart,
                    parameters: Some("VALUE=DATE;TZID=UTC".into()),
                    value: "19970101".into(),
                },
                StartDateContentLine {
                    datetime: UTC.ymd(1997, 1, 1).and_hms(0, 0, 0),
                    is_local_tz: false,
                },
            ),
        ];

        for (input, expected_output) in tests {
            let output = TryFrom::try_from(&input);
            assert_eq!(output, Ok(expected_output));
        }
    }

    #[test]
    fn rejecets_invalid_dtstart() {
        let tests = [
            ContentLineCaptures {
                property_name: PropertyName::DtStart,
                parameters: None,
                value: "20120201120000Z".into(),
            },
            ContentLineCaptures {
                property_name: PropertyName::DtStart,
                parameters: None,
                value: "2012".into(),
            },
            ContentLineCaptures {
                property_name: PropertyName::DtStart,
                parameters: None,
                value: "".into(),
            },
        ];

        for input in tests {
            let output = StartDateContentLine::try_from(&input);
            assert_eq!(
                output,
                Err(ParseError::InvalidDateTime {
                    value: input.value,
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
            value: "20120251T023000Z".into(),
        };
        let res = StartDateContentLine::try_from(&content);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err,
            ParseError::InvalidTimezone("America/Everywhere".into())
        );
    }
}
