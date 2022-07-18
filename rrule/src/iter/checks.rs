use crate::validator::{ValidationError, MONTH_RANGE, YEAR_RANGE};

pub(crate) fn check_month_range(month: u8) -> Result<(), ValidationError> {
    if MONTH_RANGE.contains(&month) {
        Ok(())
    } else {
        Err(ValidationError::InvalidFieldValueRange {
            field: "MONTH".into(),
            value: month.to_string(),
            start_idx: MONTH_RANGE.start().to_string(),
            end_idx: MONTH_RANGE.end().to_string(),
        })
    }
}

pub(crate) fn check_year_range(year: i32) -> Result<(), ValidationError> {
    if YEAR_RANGE.contains(&year) {
        Ok(())
    } else {
        Err(ValidationError::InvalidFieldValueRange {
            field: "YEAR".into(),
            value: year.to_string(),
            start_idx: YEAR_RANGE.start().to_string(),
            end_idx: YEAR_RANGE.end().to_string(),
        })
    }
}
