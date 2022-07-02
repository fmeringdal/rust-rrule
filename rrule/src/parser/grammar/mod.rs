mod content_line;
mod date_content_line;
mod parameters;
mod rule_content_line;
mod start_date_content_line;

use std::fmt::Display;
use std::str::FromStr;

use self::content_line::get_content_line_parts;

use super::ParseError;
pub(crate) use date_content_line::DateContentLine;
pub(crate) use date_content_line::DateParameter;
pub(crate) use rule_content_line::RRuleContentLine;
pub(crate) use rule_content_line::RRuleProperty;
pub(crate) use start_date_content_line::StartDateContentLine;

#[derive(Debug)]
pub(crate) struct Grammar {
    pub start_datetime: StartDateContentLine,
    pub content_lines: Vec<ContentLine>,
}

#[derive(Debug)]
pub(crate) enum ContentLine {
    RRule(RRuleContentLine),
    ExRule(RRuleContentLine),
    ExDate(DateContentLine),
    RDate(DateContentLine),
}

#[derive(Debug, PartialEq)]
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

impl FromStr for Grammar {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut content_lines = vec![];

        let mut start_datetime = None;

        for content_line in s.lines() {
            let parts = get_content_line_parts(&content_line)?;
            let line = match parts.property_name {
                PropertyName::RRule => ContentLine::RRule(RRuleContentLine::try_from(parts)?),
                PropertyName::ExRule => ContentLine::ExRule(RRuleContentLine::try_from(parts)?),
                PropertyName::RDate => ContentLine::RDate(DateContentLine::try_from(parts)?),
                PropertyName::ExDate => ContentLine::ExDate(DateContentLine::try_from(parts)?),
                PropertyName::DtStart => {
                    if start_datetime
                        .replace(StartDateContentLine::try_from(parts)?)
                        .is_some()
                    {
                        // TODO: return error
                    }
                    continue;
                }
            };
            content_lines.push(line);
        }
        Ok(Self {
            // TODO: better error
            start_datetime: start_datetime.ok_or(ParseError::MissingProperty("DTSTART".into()))?,
            content_lines,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tmp() {
        let grammar = Grammar::from_str(
            "DTSTART;TZID=America/New_York:19970902T090000
RRULE:FREQ=DAILY;COUNT=10",
        );
        eprintln!("{:#?}", grammar);
        let grammar = Grammar::from_str("DTSTART:20120201T093000Z\nRRULE:FREQ=YEARLY;UNTIL=20000131T140000Z;BYMONTH=1;BYDAY=SU,MO,TU,WE,TH,FR,SA\nRDATE;TZID=America/New_York:19970714T083000,19980714T083000\nRDATE;VALUE=DATE:19970101,19970120,19970217,19970421,19970526,19970704,19970901,19971014,19971128,19971129,19971225");
        eprintln!("{:#?}", grammar);

        let rrule = crate::parser::parse("DTSTART:20120201T093000Z\nRRULE:FREQ=YEARLY;UNTIL=20000131T140000Z;BYMONTH=1;BYDAY=SU,MO,TU,WE,TH,FR,SA\nRDATE;TZID=America/New_York:19970714T083000,19980714T083000\nRDATE;VALUE=DATE:19970101,19970120,19970217,19970421,19970526,19970704,19970901,19971014,19971128,19971129,19971225");
        eprintln!("{:#?}", rrule);
    }
}
