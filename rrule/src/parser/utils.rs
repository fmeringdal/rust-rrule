//! Utility functions for parsing rrule input.
use std::str::FromStr;

use super::ParseError;

/// Attempts to convert a comma separated `&str` to a `Vec<T>` of unique and sorted values.
/// The function accepts a closure which can be used to validate the values which are parsed.
pub(crate) fn parse_str_to_vec<T: FromStr + Ord + PartialEq + Copy, F: Fn(T) -> bool>(
    val: &str,
    accept: F,
) -> Result<Vec<T>, String> {
    if val.is_empty() {
        return Ok(vec![]);
    }

    let mut parsed_vals = vec![];
    for val in val.split(',') {
        let parsed_val = val.parse().map_err(|_| val.to_string())?;
        if accept(parsed_val) {
            parsed_vals.push(parsed_val);
        } else {
            return Err(val.into());
        }
    }

    parsed_vals.sort();
    parsed_vals.dedup();

    Ok(parsed_vals)
}

/// Helper function to validate the input string.
pub(crate) fn check_str_validity(s: &str) -> Result<(), ParseError> {
    if let Some(unsupported_car) = s.chars().find(|c| {
        let valid = char::is_ascii_alphanumeric(c)
            || char::is_ascii_punctuation(c)
            || char::is_ascii_whitespace(c);
        // Want to find an invalid char
        !valid
    }) {
        Err(ParseError::UnsupportedCharacter(
            unsupported_car.to_string(),
        ))
    } else {
        Ok(())
    }
}

/// A parsed line of the recurrence rule input.
#[derive(Debug, PartialEq)]
pub(crate) struct ParsedLine {
    /// Name of the line. Either `RRULE`, `EXRULE`, `EXDATE`, `RDATE` or `DTSTART`.
    pub name: String,
    /// The parameters for the line. This is empty for `RRULE` and `EXRULE`.
    pub params: Vec<String>,
    /// The remaining part of the line after the `name` and `params`.
    pub value: String,
}

/// Breaks down a line in the RFC input to its grammar.
pub(crate) fn break_down_line(line: &str) -> ParsedLine {
    let parsed_line_name = extract_name(line.into());
    let params: Vec<&str> = parsed_line_name.name.split(';').collect();

    ParsedLine {
        name: params[0].to_string(),
        params: params[1..].iter().map(|s| String::from(*s)).collect(),
        value: parsed_line_name.value,
    }
}

#[derive(Debug, PartialEq)]
struct LineName {
    name: String,
    value: String,
}

fn extract_name(line: String) -> LineName {
    if !line.contains(':') {
        // Defaults to RRULE
        return LineName {
            name: String::from("RRULE"),
            value: line,
        };
    }

    let parts: Vec<&str> = line.split(':').collect();
    let name = parts[0];
    let value = parts[1..].join("");

    LineName {
        name: String::from(name),
        value,
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        utils::{break_down_line, check_str_validity, extract_name},
        ParseError,
    };

    use super::{parse_str_to_vec, LineName, ParsedLine};

    #[test]
    fn test_parse_str_to_vec() {
        const INVALID_VALUE: usize = 17;
        let accept_fn = |val: usize| val != 17;
        let tests = [
            ("", Ok(vec![])),
            ("1,2,3", Ok(vec![1, 2, 3])),
            ("3,2,1", Ok(vec![1, 2, 3])),
            ("3,2,1,4", Ok(vec![1, 2, 3, 4])),
            ("3,2,1,4,3,4,4,3,1", Ok(vec![1, 2, 3, 4])),
            (
                &format!("14,15,16,{},18", INVALID_VALUE),
                Err(format!("{}", INVALID_VALUE)),
            ),
        ];
        for (input, expected_output) in tests {
            let output = parse_str_to_vec(input, accept_fn);
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn test_string_validity() {
        let tests = [
            ("fasofjao", Ok(())),
            ("  .  124124jopjfpas", Ok(())),
            ("Θ", Err(ParseError::UnsupportedCharacter("Θ".into()))),
        ];
        for (input, expected_output) in tests {
            let output = check_str_validity(input);
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn parses_rrule_line_grammar() {
        let tests = [
            (
                "DTSTART;VALUE=DATE:20200812",
                ParsedLine {
                    name: "DTSTART".into(),
                    params: vec!["VALUE=DATE".into()],
                    value: "20200812".into(),
                },
            ),
            (
                "RRULE:FREQ=YEARLY;COUNT=3",
                ParsedLine {
                    name: "RRULE".into(),
                    params: vec![],
                    value: "FREQ=YEARLY;COUNT=3".into(),
                },
            ),
            (
                "RRULE:",
                ParsedLine {
                    name: "RRULE".into(),
                    params: vec![],
                    value: "".into(),
                },
            ),
        ];
        for (input, expected_output) in tests {
            let output = break_down_line(input);
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn parses_name_and_value_from_line() {
        let tests = [
            (
                "DTSTART;VALUE=DATE:20200812",
                LineName {
                    name: "DTSTART;VALUE=DATE".into(),
                    value: "20200812".into(),
                },
            ),
            (
                "RRULE:FREQ=YEARLY;COUNT=3",
                LineName {
                    name: "RRULE".into(),
                    value: "FREQ=YEARLY;COUNT=3".into(),
                },
            ),
            (
                "RRULE:",
                LineName {
                    name: "RRULE".into(),
                    value: "".into(),
                },
            ),
            (
                "",
                LineName {
                    name: "RRULE".into(),
                    value: "".into(),
                },
            ),
            (
                ":",
                LineName {
                    name: "".into(),
                    value: "".into(),
                },
            ),
        ];
        for (input, expected_output) in tests {
            let output = extract_name(input.into());
            assert_eq!(output, expected_output);
        }
    }
}
