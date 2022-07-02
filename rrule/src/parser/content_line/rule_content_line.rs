use std::{collections::HashMap, marker::PhantomData, str::FromStr};

use chrono::Weekday;
use chrono_tz::UTC;

use crate::{
    core::DateTime,
    parser::{
        content_line::parameters::parse_parameters,
        datetime::{datestring_to_date, parse_weekdays},
        str_to_weekday,
        utils::parse_str_to_vec,
        ParseError,
    },
    Frequency, RRule, Unvalidated,
};

use super::{
    content_line_parts::{get_content_line_parts, ContentLineCaptures},
    start_date_content_line::StartDateContentLine,
    PropertyName,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
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
    #[cfg(feature = "by-easter")]
    ByEaster,
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
            #[cfg(feature = "by-easter")]
            "BYEASTER" => Self::ByEaster,
            _ => return Err(ParseError::UnrecognizedParameter(s.into())),
        };
        Ok(prop)
    }
}

impl TryFrom<(ContentLineCaptures, &StartDateContentLine)> for RRule<Unvalidated> {
    type Error = ParseError;

    fn try_from(
        (value, dtstart): (ContentLineCaptures, &StartDateContentLine),
    ) -> Result<Self, Self::Error> {
        if let Some(parameters) = value.parameters {
            if !parameters.is_empty() {
                return Err(ParseError::PropertyParametersNotSupported(
                    parameters.into(),
                ));
            }
        }

        let properties: HashMap<RRuleProperty, String> = parse_parameters(&value.properties)?;

        props_to_rrule(properties, dtstart)
    }
}

/// Takes a map of [`RRuleProperty`] and returns an [`RRule`].
#[allow(clippy::too_many_lines)]
fn props_to_rrule(
    props: HashMap<RRuleProperty, String>,
    dtstart: &StartDateContentLine,
) -> Result<RRule<Unvalidated>, ParseError> {
    let freq = props
        .get(&RRuleProperty::Freq)
        .map(|freq| Frequency::from_str(freq))
        .transpose()?
        .ok_or_else(|| ParseError::MissingProperty("FREQ".into()))?;
    let interval = props
        .get(&RRuleProperty::Interval)
        .map(|interval| {
            interval
                .parse()
                .map_err(|_| ParseError::InvalidInterval(interval.into()))
        })
        .transpose()?
        .unwrap_or(1);
    let count = props
        .get(&RRuleProperty::Count)
        .map(|count| {
            count
                .parse()
                .map_err(|_| ParseError::InvalidCount(count.into()))
        })
        .transpose()?;
    let until = props
        .get(&RRuleProperty::Until)
        .map(|until| {
            // Make sure until is not using zulu which would override timezone we set
            let mut until = until.clone();
            if until.to_uppercase().ends_with("Z") {
                until.remove(until.len() - 1);
            }

            // > Furthermore, if the "DTSTART" property is specified as a date with local time,
            // > then the UNTIL rule part MUST also be specified as a date with local time.
            //
            // Otherwise until will be in UTC
            let timezone = if dtstart.is_local_tz { None } else { Some(UTC) };
            datestring_to_date(&until, timezone, "UNTIL")
        })
        .transpose()?;
    let week_start = props
        .get(&RRuleProperty::Wkst)
        .map(|week_start| {
            str_to_weekday(week_start)
                .map_err(|_| ParseError::InvalidWeekdayStart(week_start.into()))
        })
        .transpose()?
        .unwrap_or(Weekday::Mon);
    let by_set_pos = props
        .get(&RRuleProperty::BySetPos)
        .map(|by_set_pos| {
            parse_str_to_vec(by_set_pos, |_| true)
                .map_err(|_| ParseError::InvalidBySetPos(by_set_pos.into()))
        })
        .transpose()?
        .unwrap_or_default();
    let by_month = props
        .get(&RRuleProperty::ByMonth)
        .map(|by_month| {
            parse_str_to_vec(by_month, |month| (1..=12).contains(&month))
                .map_err(|_| ParseError::InvalidByMonth(by_month.into()))
        })
        .transpose()?
        .unwrap_or_default();
    let by_month_day = props
        .get(&RRuleProperty::ByMonthDay)
        .map(|by_month_day| {
            parse_str_to_vec(by_month_day, |monthday| (-31..=31).contains(&monthday))
                .map_err(|_| ParseError::InvalidByMonthDay(by_month_day.into()))
        })
        .transpose()?
        .unwrap_or_default();
    let by_year_day = props
        .get(&RRuleProperty::ByYearDay)
        .map(|by_year_day| {
            parse_str_to_vec(by_year_day, |yearday| (-366..=366).contains(&yearday))
                .map_err(|_| ParseError::InvalidByYearDay(by_year_day.into()))
        })
        .transpose()?
        .unwrap_or_default();
    let by_week_no = props
        .get(&RRuleProperty::ByWeekNo)
        .map(|by_week_no| {
            parse_str_to_vec(by_week_no, |weekno| (-53..=53).contains(&weekno))
                .map_err(|_| ParseError::InvalidByWeekNo(by_week_no.into()))
        })
        .transpose()?
        .unwrap_or_default();
    let by_weekday = props
        .get(&RRuleProperty::ByDay)
        .map(|by_weekday| parse_weekdays(by_weekday))
        .transpose()?
        .unwrap_or_default();
    let by_hour = props
        .get(&RRuleProperty::ByHour)
        .map(|by_hour| {
            parse_str_to_vec(by_hour, |hour| hour < 24)
                .map_err(|_| ParseError::InvalidByHour(by_hour.into()))
        })
        .transpose()?
        .unwrap_or_default();
    let by_minute = props
        .get(&RRuleProperty::ByMinute)
        .map(|by_minute| {
            parse_str_to_vec(by_minute, |minute| minute < 60)
                .map_err(|_| ParseError::InvalidByMinute(by_minute.into()))
        })
        .transpose()?
        .unwrap_or_default();
    let by_second = props
        .get(&RRuleProperty::BySecond)
        .map(|by_second| {
            parse_str_to_vec(by_second, |second| second < 60)
                .map_err(|_| ParseError::InvalidBySecond(by_second.into()))
        })
        .transpose()?
        .unwrap_or_default();

    #[cfg(not(feature = "by-easter"))]
    let by_easter = None;
    #[cfg(feature = "by-easter")]
    let by_easter = props
        .get(&RRuleProperty::ByEaster)
        .map(|new_by_easter: &String| {
            i16::from_str(new_by_easter)
                .map_err(|_| ParseError::InvalidByEaster(new_by_easter.into()))
        })
        .transpose()?;

    // Check if mandatory fields are set
    Ok(RRule {
        freq,
        interval,
        count,
        until,
        week_start,
        by_set_pos,
        by_month,
        by_month_day,
        by_n_month_day: vec![],
        by_year_day,
        by_week_no,
        by_weekday,
        by_hour,
        by_minute,
        by_second,
        by_easter,
        stage: PhantomData,
    })
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
    use chrono_tz::Tz;

    use crate::{parser::content_line::PropertyName, RRuleSet};

    use super::*;

    #[test]
    fn parses_rrule_content_line() {
        let tests = [
            (
                ContentLineCaptures {
                    property_name: PropertyName::RRule,
                    parameters: None,
                    properties: "FREQ=DAILY".into(),
                },
                RRule {
                    freq: Frequency::Daily,
                    ..Default::default()
                },
            ),
            (
                ContentLineCaptures {
                    property_name: PropertyName::RRule,
                    parameters: None,
                    properties: "BYHOUR=4;FREQ=DAILY".into(),
                },
                RRule {
                    by_hour: vec![4],
                    freq: Frequency::Daily,
                    ..Default::default()
                },
            ),
            (
                ContentLineCaptures {
                    property_name: PropertyName::RRule,
                    parameters: None,
                    // Testing case insensitivity
                    properties: "byhour=4;freQ=DAILY".into(),
                },
                RRule {
                    by_hour: vec![4],
                    freq: Frequency::Daily,
                    ..Default::default()
                },
            ),
        ];

        let start_date = StartDateContentLine {
            datetime: UTC.ymd(2000, 1, 1).and_hms(0, 0, 0),
            is_local_tz: false,
        };

        for (input, expected_output) in tests {
            let output = RRule::try_from((input, &start_date));
            assert_eq!(output, Ok(expected_output));
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
        let start_date = StartDateContentLine {
            datetime: UTC.ymd(2000, 1, 1).and_hms(0, 0, 0),
            is_local_tz: false,
        };

        for (input, expected_output) in tests {
            let output = RRule::try_from((input, &start_date));
            assert_eq!(output, Err(expected_output));
        }
    }

    #[test]
    fn rejects_invalid_freq() {
        let mut props = HashMap::new();
        props.insert(RRuleProperty::Freq, "DAIL".into());
        let start_date = StartDateContentLine {
            datetime: UTC.ymd(2000, 1, 1).and_hms(0, 0, 0),
            is_local_tz: false,
        };
        let res = props_to_rrule(props, &start_date);
        assert_eq!(
            res.unwrap_err(),
            ParseError::InvalidFrequency("DAIL".into()).into()
        );
    }

    #[test]
    fn rejects_invalid_byhour() {
        let mut props = HashMap::new();
        props.insert(RRuleProperty::Freq, "DAILY".into());
        props.insert(RRuleProperty::ByHour, "24".into());
        let start_date = StartDateContentLine {
            datetime: UTC.ymd(2000, 1, 1).and_hms(0, 0, 0),
            is_local_tz: false,
        };
        let res = props_to_rrule(props.clone(), &start_date);
        assert_eq!(res.unwrap_err(), ParseError::InvalidByHour("24".into()));

        props.insert(RRuleProperty::ByHour, "5,6,25".into());
        let res = props_to_rrule(props, &start_date);
        assert_eq!(res.unwrap_err(), ParseError::InvalidByHour("5,6,25".into()));
    }

    #[test]
    fn rejects_invalid_byminute() {
        let mut props = HashMap::new();
        props.insert(RRuleProperty::Freq, "DAILY".into());
        props.insert(RRuleProperty::ByMinute, "60".into());
        let start_date = StartDateContentLine {
            datetime: UTC.ymd(2000, 1, 1).and_hms(0, 0, 0),
            is_local_tz: false,
        };
        let res = props_to_rrule(props.clone(), &start_date);
        assert_eq!(res.unwrap_err(), ParseError::InvalidByMinute("60".into()));

        props.insert(RRuleProperty::ByMinute, "4,5,64".into());
        let res = props_to_rrule(props, &start_date);
        assert_eq!(
            res.unwrap_err(),
            ParseError::InvalidByMinute("4,5,64".into())
        );
    }

    #[test]
    fn until_is_local_time_when_start_date_is_local() {
        let mut props = HashMap::new();
        props.insert(RRuleProperty::Freq, "DAILY".into());
        let until_str = "19970902T120000";
        let until_local = NaiveDate::from_ymd(1997, 9, 2).and_hms(12, 0, 0);
        let until_local = chrono::Local
            .from_local_datetime(&until_local)
            .unwrap()
            .with_timezone(&chrono_tz::UTC);
        props.insert(RRuleProperty::Until, until_str.into());

        let start_date = get_content_line_parts("DTSTART:19970902T090000").unwrap();
        let start_date = StartDateContentLine::try_from(start_date).unwrap();

        let rrule = props_to_rrule(props.clone(), &start_date).unwrap();
        assert_eq!(rrule.until, Some(until_local));
    }

    #[test]
    fn until_zulu_is_ignored_when_start_date_is_local() {
        let mut props = HashMap::new();
        props.insert(RRuleProperty::Freq, "DAILY".into());
        // Zulu set
        let until_str = "19970902T120000Z";
        let until_local = NaiveDate::from_ymd(1997, 9, 2).and_hms(12, 0, 0);
        let until_local = chrono::Local
            .from_local_datetime(&until_local)
            .unwrap()
            .with_timezone(&chrono_tz::UTC);
        props.insert(RRuleProperty::Until, until_str.into());

        let start_date = get_content_line_parts("DTSTART:19970902T090000").unwrap();
        let start_date = StartDateContentLine::try_from(start_date).unwrap();

        let rrule = props_to_rrule(props.clone(), &start_date).unwrap();
        assert_eq!(rrule.until, Some(until_local));
    }

    #[test]
    fn until_is_utc_when_start_date_has_timezone() {
        let mut props = HashMap::new();
        props.insert(RRuleProperty::Freq, "DAILY".into());
        let until_str = "20000902T120000";
        let until_local: DateTime<Tz> = UTC.ymd(2000, 9, 2).and_hms(12, 0, 0);
        props.insert(RRuleProperty::Until, until_str.into());

        let start_dates = [
            "DTSTART:19970902T090000Z",
            "DTSTART;TZID=UTC:19970902T090000",
            "DTSTART;TZID=UTC:19970902T090000Z",
            "DTSTART;TZID=Europe/London:19970902T090000",
        ];

        for start_date in start_dates {
            let start_date = get_content_line_parts(start_date).unwrap();
            let start_date = StartDateContentLine::try_from(start_date).unwrap();

            let rrule = props_to_rrule(props.clone(), &start_date).unwrap();
            assert_eq!(rrule.until, Some(until_local));
        }
    }
}
