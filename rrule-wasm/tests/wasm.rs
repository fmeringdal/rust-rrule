use wasm_bindgen_test::*;
use rrule_wasm::get_all_date_recurrences;

#[wasm_bindgen_test]
fn test_date_recurrences() {
   let dates: Vec<wasm_bindgen::JsValue> = get_all_date_recurrences("DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3", Some(100));
   assert_eq!(dates.len(), 3);
   assert_eq!(dates.get(0).unwrap().as_string().unwrap(), "2012-02-01 09:30:00 UTC");
   assert_eq!(dates.get(1).unwrap().as_string().unwrap(), "2012-02-02 09:30:00 UTC");
   assert_eq!(dates.get(2).unwrap().as_string().unwrap(), "2012-02-03 09:30:00 UTC");
}
