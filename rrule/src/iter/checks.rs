use crate::{
    validator::{DAY_RANGE, MONTH_RANGE, YEAR_RANGE},
    RRuleError,
};

#[allow(dead_code)]
pub fn check_day_range(day: u8) -> Result<(), RRuleError> {
    if !DAY_RANGE.contains(&day) {
        Err(RRuleError::new_validation_err(format!(
            "Day is `{}`, but is not allowed outside of the range: `{}..={}`.",
            day,
            DAY_RANGE.start(),
            DAY_RANGE.end()
        )))
    } else {
        Ok(())
    }
}

pub fn check_month_range(month: u8) -> Result<(), RRuleError> {
    if !MONTH_RANGE.contains(&month) {
        Err(RRuleError::new_validation_err(format!(
            "Month is `{}`, but is not allowed outside of the range: `{}..={}`.",
            month,
            MONTH_RANGE.start(),
            MONTH_RANGE.end()
        )))
    } else {
        Ok(())
    }
}

pub fn check_year_range(year: i32) -> Result<(), RRuleError> {
    if !YEAR_RANGE.contains(&year) {
        Err(RRuleError::new_validation_err(format!(
            "Year is `{}`, but is not allowed outside of the range: `{}..={}`.",
            year,
            YEAR_RANGE.start(),
            YEAR_RANGE.end()
        )))
    } else {
        Ok(())
    }
}
