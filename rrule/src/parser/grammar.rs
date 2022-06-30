use std::{collections::HashMap, str::FromStr};

use super::{
    regex::{get_content_line_parts, ContentLineCaptures},
    ParseError,
};

#[derive(Debug)]
pub(crate) struct Grammar {
    start_datetime: StartDateContentLine,
    content_lines: Vec<ContentLine>,
}

#[derive(Debug)]
pub(crate) enum ContentLine {
    RRule(RRuleContentLine),
    ExRule(RRuleContentLine),
    ExDate(DateContentLine),
    RDate(DateContentLine),
}

#[derive(Debug)]
pub(crate) struct DateContentLine {
    parameters: HashMap<DateParameter, String>,
    dates: Vec<String>,
}

#[derive(Debug)]
pub(crate) struct StartDateContentLine {
    parameters: HashMap<DateParameter, String>,
    date: String,
}

#[derive(Debug)]
pub(crate) struct RRuleContentLine(HashMap<RRuleProperty, String>);

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
        let prop = match s {
            "FREQ" => Self::Freq,
            "UNTIL" => Self::Until,
            "COUNT" => Self::Count,
            "INTERVAL" => Self::Interval,
            "BYSECOND" => Self::BySecond,
            "BYMINUTE" => Self::ByMinute,
            "BYHOUR" => Self::ByHour,
            "BYDAY" => Self::ByDay,
            "BYMONTHDAY" => Self::ByMonthDay,
            "BYYEARDAY" => Self::ByYearDay,
            "BYWEEKNO" => Self::ByWeekNo,
            "BYMONTH" => Self::ByMonth,
            "BYSETPOS" => Self::BySetPos,
            "WKST" => Self::Wkst,
            _ => todo!(),
        };
        Ok(prop)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum DateParameter {
    Timezone,
    Value,
}

impl FromStr for DateParameter {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let param = match s {
            "TZID" => Self::Timezone,
            "VALUE" => Self::Value,
            _ => todo!(),
        };
        Ok(param)
    }
}

impl FromStr for Grammar {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut content_lines = vec![];

        let mut start_datetime = None;

        for content_line in s.lines() {
            let content_line = content_line.to_uppercase();

            let parts = get_content_line_parts(&content_line).unwrap();
            let property_name = parts.property_name.clone();
            let line = match &property_name[..] {
                "RRULE" => ContentLine::RRule(RRuleContentLine::try_from(parts)?),
                "EXRULE" => ContentLine::ExRule(RRuleContentLine::try_from(parts)?),
                "RDATE" => ContentLine::RDate(DateContentLine::try_from(parts)?),
                "EXDATE" => ContentLine::ExDate(DateContentLine::try_from(parts)?),
                "DTSTART" => {
                    start_datetime = Some(StartDateContentLine::try_from(parts)?);
                    continue;
                }
                _ => todo!(),
            };
            content_lines.push(line);
        }
        Ok(Self {
            start_datetime: start_datetime.unwrap(),
            content_lines,
        })
    }
}

impl TryFrom<ContentLineCaptures> for RRuleContentLine {
    type Error = ParseError;

    fn try_from(value: ContentLineCaptures) -> Result<Self, Self::Error> {
        assert!(value.parameters.is_none());

        let mut properties = HashMap::new();
        for property in value.properties.split(";") {
            let (property, value) = property.split_once("=").unwrap();
            let property = RRuleProperty::from_str(property)?;

            properties.insert(property, value.into());
        }

        Ok(Self(properties))
    }
}

impl TryFrom<ContentLineCaptures> for DateContentLine {
    type Error = ParseError;

    fn try_from(value: ContentLineCaptures) -> Result<Self, Self::Error> {
        let mut parameters = HashMap::new();
        if let Some(raw_parameters) = &value.parameters {
            for raw_parameter in raw_parameters.split(";") {
                let (raw_parameter, value) = raw_parameter.split_once("=").unwrap();
                let parameter = DateParameter::from_str(raw_parameter)?;

                parameters.insert(parameter, value.into());
            }
        }

        Ok(Self {
            parameters,
            dates: value.properties.split(",").map(From::from).collect(),
        })
    }
}

impl TryFrom<ContentLineCaptures> for StartDateContentLine {
    type Error = ParseError;

    fn try_from(value: ContentLineCaptures) -> Result<Self, Self::Error> {
        let mut parameters = HashMap::new();
        if let Some(raw_parameters) = &value.parameters {
            for raw_parameter in raw_parameters.split(";") {
                let (raw_parameter, value) = raw_parameter.split_once("=").unwrap();
                let parameter = DateParameter::from_str(raw_parameter)?;

                parameters.insert(parameter, value.into());
            }
        }

        Ok(Self {
            parameters,
            date: value.properties,
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
        let grammar = Grammar::from_str("DTSTART:20120201T093000Z\nRRULE:FREQ=YEARLY;UNTIL=20000131T140000Z;BYMONTH=1;BYDAY=SU,MO,TU,WE,TH,FR,SA\nRDATE;TZID=America/New_York:19970714T083000,19980714T083000");
        eprintln!("{:#?}", grammar);
    }
}
