use std::{collections::HashMap, str::FromStr};

use crate::parser::{grammar::parameters::parse_parametes, ParseError};

use super::{
    content_line::{get_content_line_parts, ContentLineCaptures},
    PropertyName,
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum RRuleProperty {
    Freq,
    Until,
    Count,
    Interval,
    BySecond,
    ByMinute,
    ByHour,
    ByDay,
    ByMonthDay,
    ByYearDay,
    ByWeekNo,
    ByMonth,
    BySetPos,
    Wkst,
}

impl FromStr for RRuleProperty {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prop = match &s.to_uppercase()[..] {
            "FREQ" => Self::Freq,
            "UNTIL" => Self::Until,
            "COUNT" => Self::Count,
            "INTERVAL" => Self::Interval,
            "BYSECOND" => Self::BySecond,
            "BYMINUTE" => Self::ByMinute,
            "BYHOUR" => Self::ByHour,
            "BYWEEKDAY" | "BYDAY" => Self::ByDay,
            "BYMONTHDAY" => Self::ByMonthDay,
            "BYYEARDAY" => Self::ByYearDay,
            "BYWEEKNO" => Self::ByWeekNo,
            "BYMONTH" => Self::ByMonth,
            "BYSETPOS" => Self::BySetPos,
            "WKST" => Self::Wkst,
            _ => return Err(ParseError::UnrecognizedParameter(s.into())),
        };
        Ok(prop)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct RRuleContentLine(pub HashMap<RRuleProperty, String>);

impl TryFrom<ContentLineCaptures> for RRuleContentLine {
    type Error = ParseError;

    fn try_from(value: ContentLineCaptures) -> Result<Self, Self::Error> {
        if let Some(parameters) = value.parameters {
            if !parameters.is_empty() {
                return Err(ParseError::PropertyParametersNotSupported(
                    parameters.into(),
                ));
            }
        }

        let properties = parse_parametes(&value.properties)?;

        Ok(Self(properties))
    }
}

impl FromStr for RRuleContentLine {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = get_content_line_parts(&s)?;
        match parts.property_name {
            PropertyName::RRule | PropertyName::ExRule => RRuleContentLine::try_from(parts),
            // TODO: better error
            _ => Err(ParseError::MissingProperty("DTSTART".into())),
        }
    }
}

mod tests {
    use crate::parser::grammar::PropertyName;

    use super::*;

    #[test]
    fn parses_rrule_content_line() {
        let tests = [
            (
                ContentLineCaptures {
                    property_name: PropertyName::RRule,
                    parameters: None,
                    properties: "BYHOUR=4".into(),
                },
                [(RRuleProperty::ByHour, "4".to_string())]
                    .into_iter()
                    .collect::<HashMap<RRuleProperty, String>>(),
            ),
            (
                ContentLineCaptures {
                    property_name: PropertyName::RRule,
                    parameters: None,
                    properties: "BYHOUR=4;FREQ=DAILY".into(),
                },
                [
                    (RRuleProperty::ByHour, "4".to_string()),
                    (RRuleProperty::Freq, "DAILY".to_string()),
                ]
                .into_iter()
                .collect::<HashMap<RRuleProperty, String>>(),
            ),
            (
                ContentLineCaptures {
                    property_name: PropertyName::RRule,
                    parameters: None,
                    // Testing case insensitivity
                    properties: "byhour=4;freQ=DAILY".into(),
                },
                [
                    (RRuleProperty::ByHour, "4".to_string()),
                    (RRuleProperty::Freq, "DAILY".to_string()),
                ]
                .into_iter()
                .collect::<HashMap<RRuleProperty, String>>(),
            ),
        ];

        for (input, expected_output) in tests {
            let output = RRuleContentLine::try_from(input);
            assert_eq!(output, Ok(RRuleContentLine(expected_output)));
        }
    }

    #[test]
    fn rejects_property_parameters_in_rrule_line() {
        let tests = [(
            ContentLineCaptures {
                property_name: PropertyName::RRule,
                parameters: Some("TZID=Europe/London".into()),
                properties: "BYHOUR=4".into(),
            },
            ParseError::PropertyParametersNotSupported("TZID=Europe/London".into()),
        )];

        for (input, expected_output) in tests {
            let output = RRuleContentLine::try_from(input);
            assert_eq!(output, Err(expected_output));
        }
    }
}
