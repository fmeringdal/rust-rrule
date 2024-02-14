use wasm_bindgen::prelude::*;

use crate::{RRuleSet, RRuleError};

/// Get all recurrences of the rrule
///
/// # Arguments
///
/// * `rule_set` - List of rrules
///
/// * `limit` - Limit must be set in order to prevent infinite loops
#[wasm_bindgen]
pub fn get_all_date_recurrences(rule_set: &str, limit: Option<u16>) -> Result<Vec<JsValue>, JsError> {
    let rrule_result: Result<RRuleSet, RRuleError> = rule_set.parse();
    match rrule_result {
        Ok(rrule) =>  {
            ///  Set hard limit in case of infinitely recurring rules
            let rule_set_collection = rrule.all(limit.unwrap_or(100));
            let result: Vec<JsValue> = rule_set_collection.dates
                .into_iter()
                .map(|s| {
                    JsValue::from_str(&s.to_string())
                })
                .collect();
            Ok(result)      
        },
        Err(e) => Err(JsError::from(e))
    }
}
