use std::{collections::HashMap, str::FromStr};

use crate::parser::ParseError;

use super::{content_line::ContentLineCaptures, parameters::parse_parametes};

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

#[derive(Debug, PartialEq)]
pub(crate) struct DateContentLine {
    pub parameters: HashMap<DateParameter, String>,
    pub dates: Vec<String>,
}

impl TryFrom<ContentLineCaptures> for DateContentLine {
    type Error = ParseError;

    fn try_from(value: ContentLineCaptures) -> Result<Self, Self::Error> {
        let parameters = value
            .parameters
            .map(|p| parse_parametes(&p))
            .transpose()?
            .unwrap_or_default();

        Ok(Self {
            parameters,
            dates: value
                .properties
                .split(",")
                .map(From::from)
                .filter(|value: &String| !value.is_empty())
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::grammar::PropertyName;

    use super::*;

    #[test]
    fn parses_date_content_line() {
        let tests = [
            (
                ContentLineCaptures {
                    property_name: PropertyName::RDate,
                    parameters: None,
                    properties: "".into(),
                },
                DateContentLine {
                    dates: vec![],
                    parameters: Default::default(),
                },
            ),
            (
                ContentLineCaptures {
                    property_name: PropertyName::RDate,
                    parameters: None,
                    properties: "19970714T123000Z".into(),
                },
                DateContentLine {
                    dates: vec!["19970714T123000Z".into()],
                    parameters: Default::default(),
                },
            ),
            (
                ContentLineCaptures {
                    property_name: PropertyName::RDate,
                    parameters: Some("VALUE=DATE".into()),
                    properties: "19970101,19970120,19970217,19970421".into(),
                },
                DateContentLine {
                    dates: vec![
                        "19970101".into(),
                        "19970120".into(),
                        "19970217".into(),
                        "19970421".into(),
                    ],
                    parameters: [(DateParameter::Value, "DATE".into())]
                        .into_iter()
                        .collect(),
                },
            ),
        ];

        for (input, expected_output) in tests {
            let output = DateContentLine::try_from(input);
            assert_eq!(output, Ok(expected_output));
        }
    }
}
