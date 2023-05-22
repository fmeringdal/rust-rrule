mod wasm_rrule;
mod datetime_utils;

use wasm_bindgen::prelude::*;
use crate::{RRuleSet, RRuleError};

/// Get all recurrences of the rrule
///
/// # Arguments
///
/// * `rule_set` - List of rrules
///
/// * `limit` - Limit must be set in order to prevent infinite loops
///
/// * `after` - Returns occurrences of the rrule between after and before
///
/// * `before` - Returns occurrences of the rrule between after and before
///
#[wasm_bindgen]
pub fn get_all_date_recurrences_between(rule_set: &str, limit: Option<u16>, after: js_sys::Date, before: js_sys::Date) -> Result<Vec<JsValue>, JsError> {
    let after = datetime_utils::convert_js_date_to_datetime(&after).map_err(JsError::from);
    let before = datetime_utils::convert_js_date_to_datetime(&before).map_err(JsError::from);
    match (parser_rule_set(rule_set), after, before) {
        (Ok(rrule), Ok(after), Ok(before)) =>  {
            let rrule = rrule.after(after).before(before);
            let result = get_all_date_recurrences_for(rrule, limit);
            Ok(result)      
        },
        (Err(e), _, _) => Err(e),
        (_, Err(e), _) => Err(e),
        (_, _, Err(e)) => Err(e),        
    }
}

fn parser_rule_set(rule_set: &str) -> Result<RRuleSet, JsError> {
    let rrule_result: Result<RRuleSet, RRuleError> = rule_set.parse();
    match rrule_result {
        Ok(rrule) =>  {
            Ok(rrule)
        },
        Err(e) => Err(JsError::from(e))
    }
}

fn get_all_date_recurrences_for(rule_set: RRuleSet, limit: Option<u16>) -> Vec<JsValue> {
    //  Set hard limit in case of infinitely recurring rules
    let rule_set_collection = rule_set.all(limit.unwrap_or(100));
    let result: Vec<JsValue> = rule_set_collection.dates
        .into_iter()
        .map(|s| {
            JsValue::from_str(&s.to_string())
        })
        .collect();
    result
}