use std::str::FromStr;

use super::{regex::ParsedDateString, ParseError};
use crate::{
    core::{DateTime, Tz},
    NWeekday,
};
use chrono::{NaiveDate, TimeZone, Weekday};

/// Attempts to convert a `str` to a `chrono_tz::Tz`.
pub(crate) fn parse_timezone(tz: &str) -> Result<Tz, ParseError> {
    chrono_tz::Tz::from_str(tz)
        .map_err(|_| ParseError::InvalidTimezone(tz.into()))
        .map(Tz::Tz)
}

/// Convert a datetime string and a timezone to a `chrono::DateTime<Tz>`.
/// If the string specifies a zulu timezone with `Z`, then the timezone
/// argument will be ignored.
pub(crate) fn datestring_to_date(
    dt: &str,
    tz: Option<Tz>,
    property: &str,
) -> Result<DateTime, ParseError> {
    let ParsedDateString {
        year,
        month,
        day,
        time,
        flags,
    } = ParsedDateString::from_ical_datetime(dt).map_err(|_| ParseError::InvalidDateTime {
        value: dt.into(),
        property: property.into(),
    })?;

    // Combine parts to create data time.
    let date =
        NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| ParseError::InvalidDateTime {
            value: dt.into(),
            property: property.into(),
        })?;

    // Spec defines this is a date-time OR date
    // So the time can will be set to 0:0:0 if only a date is given.
    // https://icalendar.org/iCalendar-RFC-5545/3-8-2-4-date-time-start.html
    let (hour, min, sec) = if let Some(time) = time {
        (time.hour, time.min, time.sec)
    } else {
        (0, 0, 0)
    };
    let datetime = date
        .and_hms_opt(hour, min, sec)
        .ok_or_else(|| ParseError::InvalidDateTime {
            value: dt.into(),
            property: property.into(),
        })?;

    // Apply timezone appended to the datetime before converting to UTC.
    // For more info https://icalendar.org/iCalendar-RFC-5545/3-3-5-date-time.html
    let datetime: chrono::DateTime<Tz> = if flags.zulu_timezone_set {
        // If a `Z` is present, UTC should be used.
        chrono::DateTime::<chrono::Utc>::from_utc(datetime, chrono::Utc).with_timezone(&Tz::UTC)
    } else {
        // If no `Z` is present, local time should be used.
        use chrono::offset::LocalResult;
        // Get datetime in local time or machine local time.
        // So this also takes into account daylight or standard time (summer/winter).
        match tz {
            Some(tz) => {
                // Use the timezone specified in the `tz`
                match tz.from_local_datetime(&datetime) {
                    LocalResult::None => Err(ParseError::InvalidDateTimeInLocalTimezone {
                        value: dt.into(),
                        property: property.into(),
                    }),
                    LocalResult::Single(date) => Ok(date),
                    LocalResult::Ambiguous(date1, date2) => {
                        Err(ParseError::DateTimeInLocalTimezoneIsAmbiguous {
                            value: dt.into(),
                            property: property.into(),
                            date1: date1.to_rfc3339(),
                            date2: date2.to_rfc3339(),
                        })
                    }
                }?
            }
            None => {
                // Use current system timezone
                // TODO Add option to always use UTC when this is executed on a server.
                let local = Tz::LOCAL;
                match local.from_local_datetime(&datetime) {
                    LocalResult::None => {
                        return Err(ParseError::InvalidDateTimeInLocalTimezone {
                            value: dt.into(),
                            property: property.into(),
                        })
                    }
                    LocalResult::Single(date) => date,
                    LocalResult::Ambiguous(date1, date2) => {
                        return Err(ParseError::DateTimeInLocalTimezoneIsAmbiguous {
                            value: dt.into(),
                            property: property.into(),
                            date1: date1.to_rfc3339(),
                            date2: date2.to_rfc3339(),
                        })
                    }
                }
            }
        }
    };

    Ok(datetime)
}

/// Attempts to convert a `str` to a `Weekday`.
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

/// Parse the "BYWEEKDAY" and "BYDAY" values
/// Example: `SU,MO,TU,WE,TH,FR` or `4MO` or `-1WE`
/// > For example, within a MONTHLY rule, +1MO (or simply 1MO) represents the first Monday
/// > within the month, whereas -1MO represents the last Monday of the month.
pub(crate) fn parse_weekdays(val: &str) -> Result<Vec<NWeekday>, ParseError> {
    let mut wdays = vec![];
    // Separate all days
    for day in val.split(',') {
        let wday = day.parse::<NWeekday>()?;
        wdays.push(wday);
    }
    Ok(wdays)
}

#[cfg(test)]
mod tests {
    use super::*;

    const US_PACIFIC: Tz = Tz::US__Pacific;

    #[test]
    fn parses_valid_nweekdays() {
        let tests = [
            ("SU", vec![NWeekday::Every(Weekday::Sun)]),
            ("-12TU", vec![NWeekday::Nth(-12, Weekday::Tue)]),
            (
                "MO,WE",
                vec![NWeekday::Every(Weekday::Mon), NWeekday::Every(Weekday::Wed)],
            ),
            (
                "MO,WE,3TU,-4SA",
                vec![
                    NWeekday::Every(Weekday::Mon),
                    NWeekday::Every(Weekday::Wed),
                    NWeekday::Nth(3, Weekday::Tue),
                    NWeekday::Nth(-4, Weekday::Sat),
                ],
            ),
        ];

        for (input, expected_output) in tests {
            let output = parse_weekdays(input);
            assert_eq!(output, Ok(expected_output));
        }
    }

    #[test]
    fn rejects_invalid_nweekdays() {
        let tests = ["", "    ", "fjoasfjapsjop", "MONDAY", "MONDAY, TUESDAY"];

        for input in tests {
            let res = parse_weekdays(input);
            assert!(res.is_err());
        }
    }

    #[test]
    fn parses_valid_weekdays() {
        let tests = [
            ("MO", Weekday::Mon),
            ("TU", Weekday::Tue),
            ("WE", Weekday::Wed),
            ("TH", Weekday::Thu),
            ("FR", Weekday::Fri),
            ("SA", Weekday::Sat),
            ("SU", Weekday::Sun),
        ];

        for (input, expected_output) in tests {
            let output = str_to_weekday(input);
            assert_eq!(output, Ok(expected_output));
        }
    }

    #[test]
    fn rejects_invalid_weekdays() {
        let tests = ["", "    ", "fjoasfjapsjop", "MONDAY", "MONDAY, TUESDAY"];

        for input in tests {
            let res = str_to_weekday(input);
            assert!(res.is_err());
        }
    }

    #[test]
    fn parses_valid_datestime_str() {
        let tests = [
            (
                "19970902T090000Z",
                None,
                Tz::UTC.with_ymd_and_hms(1997, 9, 2, 9, 0, 0).unwrap(),
            ),
            (
                "19970902T090000",
                Some(Tz::UTC),
                Tz::UTC.with_ymd_and_hms(1997, 9, 2, 9, 0, 0).unwrap(),
            ),
            (
                "19970902T090000",
                Some(US_PACIFIC),
                US_PACIFIC.with_ymd_and_hms(1997, 9, 2, 9, 0, 0).unwrap(),
            ),
            (
                "19970902T090000Z",
                Some(US_PACIFIC),
                // Timezone is overwritten by the zulu specified in the datetime string
                Tz::UTC.with_ymd_and_hms(1997, 9, 2, 9, 0, 0).unwrap(),
            ),
        ];

        for (datetime_str, timezone, expected_output) in tests {
            let output = datestring_to_date(datetime_str, timezone, "DTSTART");
            assert_eq!(output, Ok(expected_output));
        }
    }

    #[test]
    fn rejects_invalid_datetime_str() {
        let tests = [
            ("", None),
            ("TZID=America/New_York:19970902T090000", None),
            ("19970902T09", None),
            ("19970902T09", Some(US_PACIFIC)),
        ];

        for (datetime_str, timezone) in tests {
            let res = datestring_to_date(datetime_str, timezone, "DTSTART");
            assert!(res.is_err());
        }
    }
}
