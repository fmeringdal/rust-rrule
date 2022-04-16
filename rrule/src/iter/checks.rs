use crate::validator::{ValidationError, DAY_RANGE, MONTH_RANGE, YEAR_RANGE};

#[allow(dead_code)]
pub(crate) fn check_day_range(day: u8) -> Result<(), ValidationError> {
    if !DAY_RANGE.contains(&day) {
        Err(ValidationError::InvalidFieldValueRange {
            field: "DAY".into(),
            value: day.to_string(),
            start_idx: DAY_RANGE.start().to_string(),
            end_idx: DAY_RANGE.end().to_string(),
        })
    } else {
        Ok(())
    }
}

pub(crate) fn check_month_range(month: u8) -> Result<(), ValidationError> {
    if !MONTH_RANGE.contains(&month) {
        Err(ValidationError::InvalidFieldValueRange {
            field: "MONTH".into(),
            value: month.to_string(),
            start_idx: MONTH_RANGE.start().to_string(),
            end_idx: MONTH_RANGE.end().to_string(),
        })
    } else {
        Ok(())
    }
}

pub(crate) fn check_year_range(year: i32) -> Result<(), ValidationError> {
    if !YEAR_RANGE.contains(&year) {
        Err(ValidationError::InvalidFieldValueRange {
            field: "YEAR".into(),
            value: year.to_string(),
            start_idx: YEAR_RANGE.start().to_string(),
            end_idx: YEAR_RANGE.end().to_string(),
        })
    } else {
        Ok(())
    }
}
