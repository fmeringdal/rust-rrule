use chrono::Duration;

use crate::RRuleError;

pub(crate) fn checked_mul_u32(v1: u32, v2: u32, hint: Option<&str>) -> Result<u32, RRuleError> {
    v1.checked_mul(v2).ok_or_else(|| match hint {
        Some(hint) => RRuleError::new_iter_err(format!(
            "Could not multiply number, would overflow (`{} * {}`), {}.",
            v1, v2, hint
        )),
        None => RRuleError::new_iter_err(format!(
            "Could not multiply number, would overflow (`{} * {}`).",
            v1, v2,
        )),
    })
}

pub(crate) fn checked_add_u32(v1: u32, v2: u32, hint: Option<&str>) -> Result<u32, RRuleError> {
    v1.checked_add(v2).ok_or_else(|| match hint {
        Some(hint) => RRuleError::new_iter_err(format!(
            "Could not add numbers, would overflow (`{} + {}`), {}.",
            v1, v2, hint
        )),
        None => RRuleError::new_iter_err(format!(
            "Could not add numbers, would overflow (`{} + {}`).",
            v1, v2,
        )),
    })
}

pub(crate) fn checked_add_datetime_duration<TZ: chrono::TimeZone>(
    v1: chrono::DateTime<TZ>,
    v2: Duration,
    hint: Option<&str>,
) -> Result<chrono::DateTime<TZ>, RRuleError> {
    v1.clone().checked_add_signed(v2).ok_or_else(|| match hint {
        Some(hint) => RRuleError::new_iter_err(format!(
            "Could not add Duration to DateTime, would overflow (`{:?} + {}`), {}.",
            v1, v2, hint
        )),
        None => RRuleError::new_iter_err(format!(
            "Could not add Duration to DateTime, would overflow (`{:?} + {}`).",
            v1, v2,
        )),
    })
}
