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
    content_line_parts::ContentLineCaptures, start_date_content_line::StartDateContentLine,
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

impl<'a> TryFrom<(ContentLineCaptures<'a>, &StartDateContentLine)> for RRule<Unvalidated> {
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

        let properties: HashMap<RRuleProperty, String> = parse_parameters(value.value)?;

        props_to_rrule(&properties, dtstart)
    }
}

/// Takes a map of [`RRuleProperty`] and returns an [`RRule`].
#[allow(clippy::too_many_lines)]
fn props_to_rrule(
    props: &HashMap<RRuleProperty, String>,
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
        .map(|until| parse_until(until, dtstart))
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

/// Parses UNTIL string to a `DateTime` based on values parsed from the start date.
fn parse_until(until: &str, dtstart: &StartDateContentLine) -> Result<DateTime, ParseError> {
    let until_value = if until.len() > 8 { "DATE-TIME" } else { "DATE" };
    if until_value != dtstart.value {
        return Err(ParseError::DtStartUntilMismatchValue);
    }

    let timezone = if dtstart.timezone.is_none() {
        if until_value == "DATE-TIME" && until.to_uppercase().ends_with('Z') {
            return Err(ParseError::DtStartUntilMismatchTimezone);
        }

        None
    } else {
        Some(UTC)
    };

    datestring_to_date(until, timezone, "UNTIL")
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDate, TimeZone};
    use chrono_tz::Tz;

    use crate::parser::content_line::{ContentLineCaptures, PropertyName};

    use super::*;

    #[test]
    fn parses_rrule_content_line() {
        let tests = [
            (
                ContentLineCaptures {
                    property_name: PropertyName::RRule,
                    parameters: None,
                    value: "FREQ=DAILY",
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
                    value: "BYHOUR=4;FREQ=DAILY",
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
                    value: "byhour=4;freQ=DAILY",
                },
                RRule {
                    by_hour: vec![4],
                    freq: Frequency::Daily,
                    ..Default::default()
                },
            ),
        ];

        let start_date = StartDateContentLine {
            datetime: UTC.ymd(2000, 1, 1).and_hms(0, 0, 0).into(),
            timezone: None,
            value: "DATE-TIME",
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
                parameters: Some("TZID=Europe/London"),
                value: "BYHOUR=4",
            },
            ParseError::PropertyParametersNotSupported("TZID=Europe/London".into()),
        )];
        let start_date = StartDateContentLine {
            datetime: UTC.ymd(2000, 1, 1).and_hms(0, 0, 0).into(),
            timezone: None,
            value: "DATE-TIME",
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
            datetime: UTC.ymd(2000, 1, 1).and_hms(0, 0, 0).into(),
            timezone: None,
            value: "DATE-TIME",
        };
        let res = props_to_rrule(&props, &start_date);
        assert_eq!(
            res.unwrap_err(),
            ParseError::InvalidFrequency("DAIL".into())
        );
    }

    #[test]
    fn rejects_invalid_byhour() {
        let mut props = HashMap::new();
        props.insert(RRuleProperty::Freq, "DAILY".into());
        props.insert(RRuleProperty::ByHour, "24".into());
        let start_date = StartDateContentLine {
            datetime: UTC.ymd(2000, 1, 1).and_hms(0, 0, 0).into(),
            timezone: None,
            value: "DATE-TIME",
        };
        let res = props_to_rrule(&props, &start_date);
        assert_eq!(res.unwrap_err(), ParseError::InvalidByHour("24".into()));

        props.insert(RRuleProperty::ByHour, "5,6,25".into());
        let res = props_to_rrule(&props, &start_date);
        assert_eq!(res.unwrap_err(), ParseError::InvalidByHour("5,6,25".into()));
    }

    #[test]
    fn rejects_invalid_byminute() {
        let mut props = HashMap::new();
        props.insert(RRuleProperty::Freq, "DAILY".into());
        props.insert(RRuleProperty::ByMinute, "60".into());
        let start_date = StartDateContentLine {
            datetime: UTC.ymd(2000, 1, 1).and_hms(0, 0, 0).into(),
            timezone: None,
            value: "DATE-TIME",
        };
        let res = props_to_rrule(&props, &start_date);
        assert_eq!(res.unwrap_err(), ParseError::InvalidByMinute("60".into()));

        props.insert(RRuleProperty::ByMinute, "4,5,64".into());
        let res = props_to_rrule(&props, &start_date);
        assert_eq!(
            res.unwrap_err(),
            ParseError::InvalidByMinute("4,5,64".into())
        );
    }

    #[test]
    fn until_is_local_time_when_start_date_is_local() {
        let mut props = HashMap::new();
        props.insert(RRuleProperty::Freq, "DAILY".into());
        let until_str = "19970904";
        let until_local = NaiveDate::from_ymd(1997, 9, 4).and_hms(0, 0, 0);
        let until_local = chrono::Local.from_local_datetime(&until_local).unwrap();
        props.insert(RRuleProperty::Until, until_str.into());

        let start_date = ContentLineCaptures::new("DTSTART:19970902").unwrap();
        let start_date = StartDateContentLine::try_from(&start_date).unwrap();

        let rrule = props_to_rrule(&props, &start_date).unwrap();
        assert_eq!(rrule.until, Some(until_local.into()));
    }

    #[test]
    fn until_is_local_time_when_start_datetime_is_local() {
        let mut props = HashMap::new();
        props.insert(RRuleProperty::Freq, "DAILY".into());
        let until_str = "19970904T090000";
        let until_local = NaiveDate::from_ymd(1997, 9, 4).and_hms(9, 0, 0);
        let until_local = chrono::Local.from_local_datetime(&until_local).unwrap();
        props.insert(RRuleProperty::Until, until_str.into());

        let start_date = ContentLineCaptures::new("DTSTART:19970902T090000").unwrap();
        let start_date = StartDateContentLine::try_from(&start_date).unwrap();

        let rrule = props_to_rrule(&props, &start_date).unwrap();
        assert_eq!(rrule.until, Some(until_local.into()));
    }

    #[test]
    fn until_is_utc_when_start_date_has_timezone() {
        let mut props = HashMap::new();
        props.insert(RRuleProperty::Freq, "DAILY".into());
        let until_str = "20000902";
        let until_local: DateTime<Tz> = UTC.ymd(2000, 9, 2).and_hms(0, 0, 0);
        props.insert(RRuleProperty::Until, until_str.into());

        let start_dates = [
            "DTSTART;TZID=UTC:19970902",
            "DTSTART;TZID=Europe/London:19970902",
        ];

        for start_date in start_dates {
            let start_date = ContentLineCaptures::new(start_date).unwrap();
            let start_date = StartDateContentLine::try_from(&start_date).unwrap();

            let rrule = props_to_rrule(&props, &start_date).unwrap();
            assert_eq!(rrule.until, Some(until_local.into()));
        }
    }

    #[test]
    fn until_is_utc_when_start_datetime_has_timezone() {
        let mut props = HashMap::new();
        props.insert(RRuleProperty::Freq, "DAILY".into());
        let until_str = "19970904T090000";
        let until_local: DateTime<Tz> = UTC.ymd(1997, 9, 4).and_hms(9, 0, 0);
        props.insert(RRuleProperty::Until, until_str.into());

        let start_dates = [
            "DTSTART:19970902T090000Z",
            "DTSTART;TZID=UTC:19970902T090000",
            "DTSTART;TZID=UTC:19970902T090000Z",
            "DTSTART;TZID=Europe/London:19970902T090000",
        ];

        for start_date in start_dates {
            let start_date = ContentLineCaptures::new(start_date).unwrap();
            let start_date = StartDateContentLine::try_from(&start_date).unwrap();

            let rrule = props_to_rrule(&props, &start_date).unwrap();
            assert_eq!(rrule.until, Some(until_local.into()));
        }
    }

    #[test]
    fn reject_until_with_zulu_if_start_date_is_local() {
        let mut props = HashMap::new();
        props.insert(RRuleProperty::Freq, "DAILY".into());
        let until_str = "19970902T090000Z";
        props.insert(RRuleProperty::Until, until_str.into());

        let start_date = ContentLineCaptures::new("DTSTART:19970902T090000").unwrap();
        let start_date = StartDateContentLine::try_from(&start_date).unwrap();

        let res = props_to_rrule(&props, &start_date);
        assert_eq!(res, Err(ParseError::DtStartUntilMismatchTimezone));
    }

    #[test]
    fn reject_until_with_value_paramter_different_from_start_date() {
        let mut props = HashMap::new();
        props.insert(RRuleProperty::Freq, "DAILY".into());
        let until_str = "19970902T090000";
        props.insert(RRuleProperty::Until, until_str.into());

        let start_date = ContentLineCaptures::new("DTSTART:19970902").unwrap();
        let start_date = StartDateContentLine::try_from(&start_date).unwrap();

        let res = props_to_rrule(&props, &start_date);
        assert_eq!(res, Err(ParseError::DtStartUntilMismatchValue));
    }
}
