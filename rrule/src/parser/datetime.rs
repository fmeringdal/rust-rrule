//! Parser functions related to date, time and timezones.

use chrono::{NaiveDate, TimeZone, Weekday};
use chrono_tz::{Tz, UTC};
use regex::Regex;
use std::str::FromStr;

/// A convenient type alias.
pub type DateTime = chrono::DateTime<Tz>;
