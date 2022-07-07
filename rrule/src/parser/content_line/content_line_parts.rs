use crate::parser::{regex::get_property_name, ParseError};

use super::PropertyName;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct ContentLineCaptures<'a> {
    pub property_name: PropertyName,
    pub parameters: Option<&'a str>,
    pub value: &'a str,
}

impl<'a> ContentLineCaptures<'a> {
    pub(crate) fn new(line: &'a str) -> Result<Self, ParseError> {
        // Default property name to RRULE.
        let property_name = get_property_name(line)?.unwrap_or(PropertyName::RRule);
        match property_name {
            // If the line did not contain a property name (i.e. no ':'), then the
            // entire line is interpreted as the value
            PropertyName::RRule if !line.contains(':') => Ok(ContentLineCaptures {
                property_name: PropertyName::RRule,
                parameters: None,
                value: line,
            }),
            property_name => {
                let mut parameters = None;
                if line.starts_with(&format!("{};", property_name)) {
                    let only_colon_idx = line.find(':');
                    if let Some(only_colon_idx) = only_colon_idx {
                        parameters =
                            Some(&line[property_name.to_string().len() + 1..only_colon_idx]);
                    }
                }

                Ok(Self {
                    property_name,
                    parameters,
                    value: line
                        .split_once(':')
                        .map(|(_name, val)| val)
                        .unwrap_or_default(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_content_line_parts() {
        let tests = [
            (
                "DTSTART;TZID=America/Everywhere:20120251T023000Z",
                ContentLineCaptures {
                    property_name: PropertyName::DtStart,
                    parameters: Some("TZID=America/Everywhere"),
                    value: "20120251T023000Z",
                },
            ),
            (
                "DTSTART:20120251T023000Z",
                ContentLineCaptures {
                    property_name: PropertyName::DtStart,
                    parameters: None,
                    value: "20120251T023000Z",
                },
            ),
            (
                "DTSTART;TZID=America/Everywhere:20120251T023000Z",
                ContentLineCaptures {
                    property_name: PropertyName::DtStart,
                    parameters: Some("TZID=America/Everywhere"),
                    value: "20120251T023000Z",
                },
            ),
            (
                "RDATE:19970714T123000Z",
                ContentLineCaptures {
                    property_name: PropertyName::RDate,
                    parameters: None,
                    value: "19970714T123000Z",
                },
            ),
            (
                "RDATE;VALUE=DATE:19970101,19970120,19970217,19970421",
                ContentLineCaptures {
                    property_name: PropertyName::RDate,
                    parameters: Some("VALUE=DATE"),
                    value: "19970101,19970120,19970217,19970421",
                },
            ),
            (
                "RRULE:FREQ=DAILY;COUNT=10",
                ContentLineCaptures {
                    property_name: PropertyName::RRule,
                    parameters: None,
                    value: "FREQ=DAILY;COUNT=10",
                },
            ),
            (
                "FREQ=DAILY;COUNT=10",
                ContentLineCaptures {
                    // Defaults to RRULE
                    property_name: PropertyName::RRule,
                    parameters: None,
                    value: "FREQ=DAILY;COUNT=10",
                },
            ),
        ];
        for (input, expected_output) in tests {
            let output = ContentLineCaptures::new(input);
            assert_eq!(output, Ok(expected_output));
        }
    }
}
