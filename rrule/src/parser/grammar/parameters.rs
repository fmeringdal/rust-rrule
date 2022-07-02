use std::{collections::HashMap, hash::Hash, str::FromStr};

use crate::parser::ParseError;

/// Parses a string of semi colon seperated key value pairs into a HashMap with
/// predefined keys. It will return an error if duplicate keys are found.
pub(super) fn parse_parametes<K: FromStr<Err = ParseError> + Hash + Eq>(
    raw_parameters: &str,
) -> Result<HashMap<K, String>, ParseError> {
    let mut parameters = HashMap::new();
    for raw_parameter in raw_parameters.split(";") {
        if raw_parameter.is_empty() {
            continue;
        }
        let (raw_parameter, value) = raw_parameter
            .split_once("=")
            .ok_or_else(|| ParseError::InvalidParameterFormat(raw_parameter.into()))?;
        let parameter = K::from_str(raw_parameter)?;

        if parameters.insert(parameter, value.into()).is_some() {
            return Err(ParseError::DuplicateProperty(raw_parameter.into()));
        }
    }
    Ok(parameters)
}

#[cfg(test)]
mod tests {
    use crate::parser::grammar::DateParameter;

    use super::*;

    #[test]
    fn parses_valid_property_parameters() {
        let tests = [
            (
                "VALUE=DATE",
                [(DateParameter::Value, "DATE".to_string())]
                    .into_iter()
                    .collect::<HashMap<_, _>>(),
            ),
            (
                "TZID=Europe/London",
                [(DateParameter::Timezone, "Europe/London".to_string())]
                    .into_iter()
                    .collect::<HashMap<_, _>>(),
            ),
        ];

        for (input, expected_output) in tests {
            let output = parse_parametes(input);
            assert_eq!(output, Ok(expected_output));
        }
    }

    #[test]
    fn rejecets_unrecognized_property_parameters() {
        let tests = [(
            "VALUES=DATE",
            ParseError::UnrecognizedParameter("VALUES".into()),
        )];

        for (input, expected_output) in tests {
            let output: Result<HashMap<DateParameter, String>, _> = parse_parametes(input);
            assert_eq!(output, Err(expected_output));
        }
    }

    #[test]
    fn rejecets_malformed_property_parameters() {
        let tests = [(
            "VALUE:DATE",
            ParseError::InvalidParameterFormat("VALUE:DATE".into()),
        )];

        for (input, expected_output) in tests {
            let output: Result<HashMap<DateParameter, String>, _> = parse_parametes(input);
            assert_eq!(output, Err(expected_output));
        }
    }

    #[test]
    fn rejecets_duplicate_parameters() {
        let tests = [(
            "VALUE=DATE;TZID=Europe/London;TZID=Europe/Frankfurt",
            ParseError::DuplicateProperty("TZID".into()),
        )];

        for (input, expected_output) in tests {
            let output: Result<HashMap<DateParameter, String>, _> = parse_parametes(input);
            assert_eq!(output, Err(expected_output));
        }
    }

    #[test]
    fn does_not_attempt_to_parse_empty_parameters() {
        let tests = [
            ("", [].into_iter().collect::<HashMap<_, _>>()),
            (";", [].into_iter().collect::<HashMap<_, _>>()),
        ];

        for (input, expected_output) in tests {
            let output: Result<HashMap<DateParameter, String>, _> = parse_parametes(input);
            assert_eq!(output, Ok(expected_output));
        }
    }
}
