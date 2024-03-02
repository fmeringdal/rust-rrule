use wasm_bindgen::prelude::*;

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub enum Weekday {
    /// Monday.
    Mon = 0,
    /// Tuesday.
    Tue = 1,
    /// Wednesday.
    Wed = 2,
    /// Thursday.
    Thu = 3,
    /// Friday.
    Fri = 4,
    /// Saturday.
    Sat = 5,
    /// Sunday.
    Sun = 6,
}

// Methods can be attached to an enum
impl Weekday {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Sun,
            1 => Self::Mon,
            2 => Self::Tue,
            3 => Self::Wed,
            4 => Self::Thu,
            5 => Self::Fri,
            6 => Self::Sat,
            _ => Self::Sun,
        }
    }

    pub fn convert(&self) -> chrono::Weekday {
        match *self {
            Self::Sun => chrono::Weekday::Sun,
            Self::Mon => chrono::Weekday::Mon,
            Self::Tue => chrono::Weekday::Tue,
            Self::Wed => chrono::Weekday::Wed,
            Self::Thu => chrono::Weekday::Thu,
            Self::Fri => chrono::Weekday::Fri,
            Self::Sat => chrono::Weekday::Sat,
        }
    }
}