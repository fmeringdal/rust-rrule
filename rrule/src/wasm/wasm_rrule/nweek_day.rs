use wasm_bindgen::{prelude::*};
use super::week_day::Weekday;

#[derive(Clone)]
#[wasm_bindgen]
pub struct NWeekdayCollection {
    collection: Vec<NWeekday>
}

impl Default for NWeekdayCollection {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl NWeekdayCollection {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            collection: Vec::new()
        }
    }

    /// The weekday to apply the recurrence to.
    /// Can be a value from 0 to 6.
    /// by_weekday: Vec<u8>
    /// 
    #[wasm_bindgen(js_name = "newEvery")]
    pub fn new_every(by_weekday: Vec<u8>) -> Self {
        let mut collection = Self::new();
        collection.add_by_weekday(by_weekday);
        collection
    }

    #[wasm_bindgen(js_name = "newNth")]
    pub fn new_nth(nth: i16, by_weekday: Vec<u8>) -> Self {
        let mut collection = Self::new();
        collection.add_by_nth(nth, by_weekday);
        collection
    }

    /// The weekday to apply the recurrence to.
    /// Can be a value from 0 to 6.
    /// by_weekday: Vec<u8>
    /// 
    #[wasm_bindgen]
    pub fn add_by_weekday(&mut self, by_weekday: Vec<u8>) {
        for week_day in Self::convert_by_weekday(by_weekday) {
            self.collection.push(week_day);
        }
    }
    
    fn convert_by_weekday(by_weekday: Vec<u8>) -> Vec<NWeekday> {
        let mut result = vec![];
        for weekday in by_weekday {
            result.push(NWeekday::new_every(Weekday::from_u8(weekday)));
        }
        result
    }

    /// The weekday to apply the recurrence to.
    /// Can be a value from 0 to 6.
    /// by_weekday: Vec<u8>
    /// 
    #[wasm_bindgen]
    pub fn add_by_nth(&mut self, nth: i16, by_weekday: Vec<u8>) {
        for week_day in Self::convert_by_nth(nth, by_weekday) {
            self.collection.push(week_day);
        }
    }

    fn convert_by_nth(nth: i16, by_weekday: Vec<u8>) -> Vec<NWeekday> {
        let mut result = vec![];
        for weekday in by_weekday {
            result.push(NWeekday::new_nth(nth, Weekday::from_u8(weekday)));
        }
        result
    }
    #[wasm_bindgen]
    pub fn add(&mut self, custom_struct: NWeekday) {
        self.collection.push(custom_struct);
    }

    #[wasm_bindgen]
    pub fn length(&self) -> usize {
        self.collection.len()
    }

    #[wasm_bindgen]
    pub fn get(&self, index: usize) -> NWeekday {
        self.collection[index].clone()
    }
}

pub fn convert_nweek_day_collection(collection: &NWeekdayCollection) -> Vec<crate::NWeekday> {
    let mut result = vec![];
    let length = collection.length();
    for i in 0..length {
        let wasm_weekday = collection.get(i);

        if let Some(n_nth) = wasm_weekday.nth {
            if wasm_weekday.every {
                result.push(crate::NWeekday::Every(wasm_weekday.weekday.convert()));
            } else {
                result.push(crate::NWeekday::Nth(n_nth, wasm_weekday.weekday.convert()));
            }                
        } else {
            result.push(crate::NWeekday::Every(wasm_weekday.weekday.convert()));
        }
    }
    result
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct NWeekday {
    pub every: bool,
    pub weekday: Weekday,
    pub nth: Option<i16>,
}

#[wasm_bindgen]
impl NWeekday {
   #[wasm_bindgen(js_name = "newEvery")]
    pub fn new_every(weekday: Weekday) -> Self {
        Self {
            every: true,
            weekday,
            nth: None,
        }
    }

    #[wasm_bindgen(js_name = "newNth")]
    pub fn new_nth(n: i16, weekday: Weekday) -> Self {
        Self {
            every: false,
            weekday,
            nth: Some(n),
        }
    }
}