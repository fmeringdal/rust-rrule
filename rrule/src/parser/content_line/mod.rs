mod content_line_parts;
mod date_content_line;
mod parameters;
mod rule_content_line;
mod start_date_content_line;

use std::fmt::Display;
use std::str::FromStr;

use crate::RRule;
use crate::Unvalidated;

pub(crate) use content_line_parts::ContentLineCaptures;
pub(crate) use start_date_content_line::StartDateContentLine;

use super::ParseError;

#[derive(Debug, PartialEq)]
pub(crate) enum ContentLine<TZ: chrono::TimeZone> {
    RRule(RRule<TZ, Unvalidated>),
    ExRule(RRule<TZ, Unvalidated>),
    ExDate(Vec<chrono::DateTime<TZ>>),
    RDate(Vec<chrono::DateTime<TZ>>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum PropertyName {
    RRule,
    ExRule,
    ExDate,
    RDate,
    DtStart,
}

impl Display for PropertyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::RRule => write!(f, "RRULE"),
            Self::ExRule => write!(f, "EXRULE"),
            Self::ExDate => write!(f, "EXDATE"),
            Self::RDate => write!(f, "RDATE"),
            Self::DtStart => write!(f, "DTSTART"),
        }
    }
}

impl FromStr for PropertyName {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = match &s[..].to_uppercase()[..] {
            "RRULE" => Self::RRule,
            "EXRULE" => Self::ExRule,
            "RDATE" => Self::RDate,
            "EXDATE" => Self::ExDate,
            "DTSTART" => Self::DtStart,
            _ => return Err(ParseError::UnrecognizedPropertyName(s.into())),
        };
        Ok(name)
    }
}
