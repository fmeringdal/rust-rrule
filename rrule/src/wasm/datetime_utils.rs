use crate::{core::Tz};
use chrono::{DateTime, TimeZone};

pub fn convert_js_date_to_datetime(date: &js_sys::Date) -> Result<DateTime<Tz>, DateTimeError> {
    if !is_valid_date(date) {
        return Err(DateTimeError::new("invalid datetime"));
    }
    let timestamp_ms = date.get_time();
    let timestamp_secs = (timestamp_ms / 1000.0) as i64;
    let nanosecs = ((timestamp_ms % 1000.0) * 1_000_000.0) as u32;
    {
        let datetime = chrono::NaiveDateTime::from_timestamp_opt(timestamp_secs, nanosecs);
        match datetime {
            Some(datetime) => {
                match convert_to_timezone(datetime, Tz::UTC) {
                    Ok(datetime) => Ok(datetime),
                    Err(e) => Err(e)                    
                }
            },
            None => Err(DateTimeError::new("invalid or out-of-range datetime"))
        }
    }
}

fn is_valid_date(date: &js_sys::Date) -> bool {
    let milliseconds = date.get_time();
    let is_nan = milliseconds.is_nan();
    !is_nan
}

fn convert_to_timezone(datetime: chrono::NaiveDateTime, timezone: Tz) -> Result<DateTime<Tz>, DateTimeError> {
    let result = timezone.from_local_datetime(&datetime);
    match result {
        chrono::LocalResult::Single(datetime) => Ok(datetime),
        chrono::LocalResult::Ambiguous(_, _) => Err(DateTimeError::new("ambiguous or out-of-range datetime")),
        chrono::LocalResult::None => Err(DateTimeError::new("d invalid or out-of-range datetime d")) 
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateTimeError {
    message: String,
}

impl DateTimeError {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}

impl std::error::Error for DateTimeError {}

impl std::fmt::Display for DateTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DateTimeError Error: {}", self.message)
    }
}