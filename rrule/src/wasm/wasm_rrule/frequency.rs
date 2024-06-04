use wasm_bindgen::prelude::*;
use crate::core::rrule;

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub enum Frequency {
    Yearly = 0,
    Monthly = 1,
    Weekly = 2,
    Daily = 3,
    Hourly = 4,
    Minutely = 5,
    Secondly = 6,
}
// Methods can be attached to an enum
impl Frequency {
    pub fn convert(&self) -> rrule::Frequency {
        match *self {
            Self::Yearly => rrule::Frequency::Yearly,
            Self::Monthly => rrule::Frequency::Monthly,
            Self::Weekly => rrule::Frequency::Weekly,
            Self::Daily => rrule::Frequency::Daily,
            Self::Hourly => rrule::Frequency::Hourly,
            Self::Minutely => rrule::Frequency::Minutely,
            Self::Secondly => rrule::Frequency::Secondly,
        }
    }
}
