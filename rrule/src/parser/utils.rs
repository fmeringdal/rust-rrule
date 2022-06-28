//! Utility functions for parsing rrule input.
use std::str::FromStr;

use chrono::Weekday;

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

pub(crate) fn str_to_weekday(d: &str) -> Result<Weekday, ParseError> {
    let day = match &d.to_uppercase()[..] {
        "MO" => Weekday::Mon,
        "TU" => Weekday::Tue,
        "WE" => Weekday::Wed,
        "TH" => Weekday::Thu,
        "FR" => Weekday::Fri,
        "SA" => Weekday::Sat,
        "SU" => Weekday::Sun,
        _ => return Err(ParseError::InvalidWeekday(d.to_string())),
    };
    Ok(day)
}

#[cfg(test)]
mod tests {
    use crate::parser::{utils::check_str_validity, ParseError};

    use super::parse_str_to_vec;

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
}
