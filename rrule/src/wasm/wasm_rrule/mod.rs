mod frequency;
mod week_day;
mod nweek_day;
use frequency::Frequency;
use week_day::Weekday;
use crate::core::{rrule, DateTime};
use js_sys::Date;
use wasm_bindgen::prelude::*;
use self::nweek_day::NWeekdayCollection;

#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmRRule {
    /// The frequency of the rrule.
    /// For example, yearly, weekly, hourly
    pub freq: Frequency,
    /// The interval between each frequency iteration.
    /// For example,
    /// - A yearly frequency with an interval of `2` creates 1 event every two years.
    /// - An hourly frequency with an interval of `2` created 1 event every two hours.
    pub interval: u16,
    /// How many occurrences will be generated.
    pub count: Option<u32>,
    /// The end date after which new events will no longer be generated.
    /// If the `DateTime` is equal to an instance of the event, it will be the last event.
    until: Option<Date>,
    /// The start day of the week.
    /// This will affect recurrences based on weekly periods.
    pub week_start: Weekday,
    /// Occurrence number corresponding to the frequency period.
    /// For example:
    /// - A monthly frequency with an `by_set_pos` of `-1` meaning the last day of the month.
    /// - An hourly frequency with an `by_set_pos` of `2` meaning the 2nd hour. (TODO Check)
    by_set_pos: Vec<i32>,
    /// The months to apply the recurrence to.
    /// Can be a value from 1 to 12.
    by_month: Vec<u8>,
    /// The month days to apply the recurrence to.
    /// Can be a value from -31 to -1 and 1 to 31.
    by_month_day: Vec<i8>,
    by_n_month_day: Vec<i8>,
    /// The year days to apply the recurrence to.
    /// Can be a value from -366 to -1 and 1 to 366.
    by_year_day: Vec<i16>,
    /// The week numbers to apply the recurrence to.
    /// Week numbers have the meaning described in ISO8601, that is,
    /// the first week of the year is that it contains at least four days of the new year.
    /// Week day starts counting on from `week_start` value.
    /// Can be a value from -53 to -1 and 1 to 53.
    by_week_no: Vec<i8>,
    /// The weekday to apply the recurrence to.
    /// by_weekday be a value from 0 to 6.
    by_weekday: NWeekdayCollection,
    /// The hours to apply the recurrence to.
    /// Can be a value from 0 to 23.
    by_hour: Vec<u8>,
    /// The minutes to apply the recurrence to.
    /// Can be a value from 0 to 59.
    by_minute: Vec<u8>,
    /// The seconds to apply the recurrence to.
    /// Can be a value from 0 to 59.
    by_second: Vec<u8>,
    /// Extension, not part of RFC spec.
    /// Amount of days/months from Easter Sunday itself.
    /// Can be a value from -366 to 366.
    /// Note: Only used when `by-easter` feature flag is set. Otherwise, it is ignored.
    pub by_easter: Option<i16>,
    // A phantom data to have the stage (unvalidated or validated).
    //#[cfg_attr(feature = "serde", serde_as(as = "ignore"))]
    // TODO: to be implemented
    //stage: PhantomData<Stage>,
}

impl Default for WasmRRule {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl WasmRRule {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            freq: Frequency::Yearly,
            interval: 1,
            count: Some(3),
            until: None,
            week_start: Weekday::Mon,
            by_set_pos: vec![],
            by_month: vec![],
            by_month_day: vec![],
            by_n_month_day: vec![],
            by_year_day: vec![],
            by_week_no: vec![],
            by_weekday: NWeekdayCollection::new(),
            by_hour: vec![],
            by_minute: vec![],
            by_second: vec![],
            by_easter: None,
            // stage: PhantomData,
        }
    }

    /// The end date after which new events will no longer be generated.
    /// If the `DateTime` is equal to an instance of the event, it will be the last event.
    #[wasm_bindgen(getter)]
    pub fn until(&self) -> Option<Date> {
        self.until.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_until(&mut self, until: Option<Date>) {
        self.until = until;
    }

    /// Occurrence number corresponding to the frequency period.
    /// For example:
    /// - A monthly frequency with an `by_set_pos` of `-1` meaning the last day of the month.
    /// - An hourly frequency with an `by_set_pos` of `2` meaning the 2nd hour. (TODO Check)
    #[wasm_bindgen(getter)]
    pub fn by_set_pos(&self) -> Box<[i32]> {
        self.by_set_pos.clone().into_boxed_slice()
    }

    #[wasm_bindgen(setter)]
    pub fn set_by_set_pos(&mut self, by_set_pos: Box<[i32]>) {
        self.by_set_pos = by_set_pos.into_vec();
    }

    /// The months to apply the recurrence to.
    /// Can be a value from 1 to 12.
    #[wasm_bindgen(getter)]
    pub fn by_month(&self) -> Box<[u8]> {
        self.by_month.clone().into_boxed_slice()
    }

    #[wasm_bindgen(setter)]
    pub fn set_by_month(&mut self, by_month: Box<[u8]>) {
        self.by_month = by_month.into_vec();
    }

    /// The month days to apply the recurrence to.
    /// Can be a value from -31 to -1 and 1 to 31.
    #[wasm_bindgen(getter)]
    pub fn by_month_day(&self) -> Box<[i8]> {
        self.by_month_day.clone().into_boxed_slice()
    }

    #[wasm_bindgen(setter)]
    pub fn set_by_month_day(&mut self, by_month_day: Box<[i8]>) {
        self.by_month_day = by_month_day.into_vec();
    }

    /// The month days to apply the recurrence to.
    /// Can be a value from -31 to -1 and 1 to 31.
    #[wasm_bindgen(getter)]
    pub fn by_n_month_day(&self) -> Box<[i8]> {
        self.by_n_month_day.clone().into_boxed_slice()
    }

    #[wasm_bindgen(setter)]
    pub fn set_by_n_month_day(&mut self, by_n_month_day: Box<[i8]>) {
        self.by_n_month_day = by_n_month_day.into_vec();
    }

    /// The year days to apply the recurrence to.
    /// Can be a value from -366 to -1 and 1 to 366.
    #[wasm_bindgen(getter)]
    pub fn by_year_day(&self) -> Box<[i16]> {
        self.by_year_day.clone().into_boxed_slice()
    }

    #[wasm_bindgen(setter)]
    pub fn set_by_year_day(&mut self, by_year_day: Box<[i16]>) {
        self.by_year_day = by_year_day.into_vec();
    }

    /// The week numbers to apply the recurrence to.
    /// Week numbers have the meaning described in ISO8601, that is,
    /// the first week of the year is that it contains at least four days of the new year.
    /// Week day starts counting on from `week_start` value.
    /// Can be a value from -53 to -1 and 1 to 53.
    #[wasm_bindgen(getter)]
    pub fn by_week_no(&self) -> Box<[i8]> {
        self.by_week_no.clone().into_boxed_slice()
    }

    #[wasm_bindgen(setter)]
    pub fn set_by_week_no(&mut self, by_week_no: Box<[i8]>) {
        self.by_week_no = by_week_no.into_vec();
    }

    /// The weekday to apply the recurrence to.
    /// Can be a value from 1 to 7.
    #[wasm_bindgen(getter)]
    pub fn by_weekday(&self) -> NWeekdayCollection {
        self.by_weekday.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_by_weekday(&mut self, by_weekday: NWeekdayCollection) {
        self.by_weekday = by_weekday;
    }

    #[wasm_bindgen(setter)]
    pub fn set_by_weekday_every(&mut self, by_weekday: Vec<u8>) {
        self.by_weekday = NWeekdayCollection::new_every(by_weekday);
    }

    pub fn set_by_weekday_nth(&mut self, nth:i16, by_weekday: Vec<u8>) {
        self.by_weekday = NWeekdayCollection::new_nth(nth, by_weekday)
    }

    /// The hours to apply the recurrence to.
    /// Can be a value from 0 to 23.
    #[wasm_bindgen(getter)]
    pub fn by_hour(&self) -> Box<[u8]> {
        self.by_hour.clone().into_boxed_slice()
    }

    #[wasm_bindgen(setter)]
    pub fn set_by_hour(&mut self, by_hour: Box<[u8]>) {
        self.by_hour = by_hour.into_vec();
    }

    /// The minutes to apply the recurrence to.
    /// Can be a value from 0 to 59.
    #[wasm_bindgen(getter)]
    pub fn by_minute(&self) -> Box<[u8]> {
        self.by_minute.clone().into_boxed_slice()
    }

    #[wasm_bindgen(setter)]
    pub fn set_by_minute(&mut self, by_minute: Box<[u8]>) {
        self.by_minute = by_minute.into_vec();
    }

    /// The seconds to apply the recurrence to.
    /// Can be a value from 0 to 59.
    #[wasm_bindgen(getter)]
    pub fn by_second(&self) -> Box<[u8]> {
        self.by_second.clone().into_boxed_slice()
    }

    #[wasm_bindgen(setter)]
    pub fn set_by_second(&mut self, by_second: Box<[u8]>) {
        self.by_second = by_second.into_vec();
    }
}

/**
  Get all recurrences of the rrule!
     rule_set_string: List of rrules
     limit: Limit must be set in order to prevent infinite loops
*/
#[wasm_bindgen]
pub fn convert_rule_options_to_string(wasm_rrule: &WasmRRule) -> String {
    let rrule = rrule::RRule {
        freq: wasm_rrule.freq.convert(),
        interval: wasm_rrule.interval,
        count: wasm_rrule.count,
        until: convert_js_date_to_chrono_date_time(&wasm_rrule.until),
        week_start: wasm_rrule.week_start.convert(),
        by_set_pos: wasm_rrule.by_set_pos.clone(),
        by_month: wasm_rrule.by_month.clone(),
        by_month_day: wasm_rrule.by_month_day.clone(),
        by_n_month_day: wasm_rrule.by_n_month_day.clone(),
        by_year_day: wasm_rrule.by_year_day.clone(),
        by_week_no: wasm_rrule.by_week_no.clone(),
        by_weekday: nweek_day::convert_nweek_day_collection(&wasm_rrule.by_weekday),
        by_hour: wasm_rrule.by_hour.clone(),
        by_minute: wasm_rrule.by_minute.clone(),
        by_second: wasm_rrule.by_second.clone(),
        by_easter: wasm_rrule.by_easter,
        ..Default::default()
    };

    rrule.to_string()
}

fn convert_js_date_to_chrono_date_time(date: &Option<Date>) -> Option<DateTime> {
    match date {
        Some(date) => {
            let date =
                super::datetime_utils::convert_js_date_to_datetime(date).map_err(JsError::from);
            match date {
                Ok(date) => Some(date),
                Err(_) => None,
            }
        }
        None => None,
    }
}
