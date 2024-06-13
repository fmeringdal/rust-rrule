use std::collections::HashMap;

use super::{
    content_line_parts::ContentLineCaptures, date_content_line::DateParameter,
    parameters::parse_parameters,
};
use crate::{
    core::Tz,
    parser::{
        datetime::{datestring_to_date, parse_timezone},
        ParseError,
    },
};

const UTC: Tz = Tz::Tz(chrono_tz::Tz::UTC);

#[derive(Debug, PartialEq)]
pub(crate) struct StartDateContentLine {
    pub datetime: chrono::DateTime<Tz>,
    pub timezone: Option<Tz>,
    pub value: &'static str,
}

impl<'a> TryFrom<&ContentLineCaptures<'a>> for StartDateContentLine {
    type Error = ParseError;

    fn try_from(content_line: &ContentLineCaptures) -> Result<Self, Self::Error> {
        let parameters: HashMap<DateParameter, String> = content_line
            .parameters
            .as_ref()
            .map(|p| parse_parameters(p))
            .transpose()?
            .unwrap_or_default();

        let mut timezone = parameters
            .get(&DateParameter::Timezone)
            .map(|tz| parse_timezone(tz))
            .transpose()?;
        if timezone.is_none() && content_line.value.to_uppercase().ends_with('Z') {
            timezone = Some(UTC);
        }

        let value_in_parameter = parameters.get(&DateParameter::Value);
        let value = if content_line.value.len() > 8 {
            "DATE-TIME"
        } else {
            "DATE"
        };
        if let Some(value_in_parameter) = value_in_parameter {
            if value_in_parameter != value {
                return Err(ParseError::ParameterValueMismatch {
                    parameter: "VALUE".into(),
                    parameter_value: value_in_parameter.into(),
                    found_value: value.into(),
                });
            }
        }

        let datetime = datestring_to_date(content_line.value, timezone, "DTSTART")?;

        Ok(Self {
            datetime,
            timezone,
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use crate::parser::content_line::PropertyName;

    use super::*;

    #[test]
    fn parses_dtstart_content_line() {
        let tests = [
            (
                ContentLineCaptures {
                    property_name: PropertyName::DtStart,
                    parameters: None,
                    value: "19970714T123000Z",
                },
                StartDateContentLine {
                    datetime: UTC.with_ymd_and_hms(1997, 7, 14, 12, 30, 0).unwrap(),
                    timezone: Some(UTC),
                    value: "DATE-TIME",
                },
            ),
            (
                ContentLineCaptures {
                    property_name: PropertyName::DtStart,
                    parameters: Some("VALUE=DATE;TZID=UTC"),
                    value: "19970101",
                },
                StartDateContentLine {
                    datetime: UTC.with_ymd_and_hms(1997, 1, 1, 0, 0, 0).unwrap(),
                    timezone: Some(UTC),
                    value: "DATE",
                },
            ),
            (
                ContentLineCaptures {
                    property_name: PropertyName::DtStart,
                    parameters: Some("TZID=UTC"),
                    value: "19970101",
                },
                StartDateContentLine {
                    datetime: UTC.with_ymd_and_hms(1997, 1, 1, 0, 0, 0).unwrap(),
                    timezone: Some(UTC),
                    value: "DATE",
                },
            ),
        ];

        for (input, expected_output) in tests {
            let output = TryFrom::try_from(&input);
            assert_eq!(output, Ok(expected_output));
        }
    }

    #[test]
    fn rejects_invalid_dtstart() {
        let tests = [
            ContentLineCaptures {
                property_name: PropertyName::DtStart,
                parameters: None,
                value: "20120201120000Z",
            },
            ContentLineCaptures {
                property_name: PropertyName::DtStart,
                parameters: None,
                value: "2012",
            },
            ContentLineCaptures {
                property_name: PropertyName::DtStart,
                parameters: None,
                value: "",
            },
        ];

        for input in tests {
            let output = StartDateContentLine::try_from(&input);
            assert_eq!(
                output,
                Err(ParseError::InvalidDateTime {
                    value: input.value.into(),
                    property: "DTSTART".into()
                })
            );
        }
    }

    #[test]
    fn reject_invalid_timezone_in_start_date() {
        let content = ContentLineCaptures {
            property_name: PropertyName::DtStart,
            parameters: Some("TZID=America/Everywhere"),
            value: "20120251T023000Z",
        };
        let res = StartDateContentLine::try_from(&content);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err,
            ParseError::InvalidTimezone("America/Everywhere".into())
        );
    }

    #[test]
    fn reject_value_mismatch_with_parameter() {
        let content = ContentLineCaptures {
            property_name: PropertyName::DtStart,
            parameters: Some("VALUE=DATE"),
            value: "20120251T023000Z",
        };
        let res = StartDateContentLine::try_from(&content);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err,
            ParseError::ParameterValueMismatch {
                parameter: "VALUE".into(),
                parameter_value: "DATE".into(),
                found_value: "DATE-TIME".into()
            }
        );
    }
}
