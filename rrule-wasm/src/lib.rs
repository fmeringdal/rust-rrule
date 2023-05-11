use rrule::RRuleSet;
use wasm_bindgen::prelude::*;

/**
  Get all recurrences of the rrule!
     rule_set_string: List of rrules
     limit: Limit must be set in order to prevent infinite loops
*/
#[wasm_bindgen]
pub fn get_all_date_recurrences(rule_set_string: &str, limit: Option<u16>) -> Vec<JsValue> {
    let rrule: RRuleSet = rule_set_string.parse().unwrap();
    //  Set hard limit in case of infinitely recurring rules
    let rule_set = rrule.all(limit.unwrap_or(100));
    let result: Vec<JsValue> = rule_set.dates
        .into_iter()
        .map(|s| JsValue::from_str(&Some(s).unwrap().to_string()))
        .collect();
    return result;
}
