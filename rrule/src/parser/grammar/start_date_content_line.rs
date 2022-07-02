use std::collections::HashMap;

use super::{
    content_line::ContentLineCaptures, date_content_line::DateParameter,
    parameters::parse_parametes,
};
use crate::parser::ParseError;

#[derive(Debug, PartialEq)]
pub(crate) struct StartDateContentLine {
    pub parameters: HashMap<DateParameter, String>,
    pub date: String,
}

impl TryFrom<ContentLineCaptures> for StartDateContentLine {
    type Error = ParseError;

    fn try_from(value: ContentLineCaptures) -> Result<Self, Self::Error> {
        let parameters = value
            .parameters
            .map(|p| parse_parametes(&p))
            .transpose()?
            .unwrap_or_default();

        Ok(Self {
            parameters,
            date: value.properties,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::grammar::PropertyName;

    use super::*;

    #[test]
    fn parses_dtstart_content_line() {
        let tests = [
            (
                ContentLineCaptures {
                    property_name: PropertyName::DtStart,
                    parameters: None,
                    properties: "".into(),
                },
                StartDateContentLine {
                    date: "".into(),
                    parameters: Default::default(),
                },
            ),
            (
                ContentLineCaptures {
                    property_name: PropertyName::DtStart,
                    parameters: None,
                    properties: "19970714T123000Z".into(),
                },
                StartDateContentLine {
                    date: "19970714T123000Z".into(),
                    parameters: Default::default(),
                },
            ),
            (
                ContentLineCaptures {
                    property_name: PropertyName::DtStart,
                    parameters: Some("VALUE=DATE".into()),
                    properties: "19970101".into(),
                },
                StartDateContentLine {
                    date: "19970101".into(),
                    parameters: [(DateParameter::Value, "DATE".into())]
                        .into_iter()
                        .collect(),
                },
            ),
        ];

        for (input, expected_output) in tests {
            let output = StartDateContentLine::try_from(input);
            assert_eq!(output, Ok(expected_output));
        }
    }
}
