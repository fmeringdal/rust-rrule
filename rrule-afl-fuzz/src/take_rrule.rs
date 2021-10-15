use crate::take_data::*;

use rrule::{Frequency, RRule, RRuleProperties};

/// This function uses the data to construct a deterministic input for RRule.
/// This can also be used to reconstruct the RRule from crashes in order to debug the code.
pub fn take_rrule_from_data(mut data: &[u8]) -> Option<RRule> {
    // Bytes uses: (always account for max used)
    // bytes => variable
    // ----------------
    // 1  => freq
    // 2  => interval
    // 5  => count
    // 21 => until
    // 1  => tz
    // 20 => dt_start
    // 1  => week_start
    // Subtotal: 51 bytes
    // 21 => by_set_pos
    // 6  => by_month
    // 6  => by_month_day
    // 6  => by_n_month_day
    // 11 => by_year_day
    // 6  => by_week_no
    // 21 => by_weekday
    // 6  => by_hour
    // 6  => by_minute
    // 6  => by_second
    // 3  => by_easter
    // ---------------
    // Total: 166 bytes
    //
    // We use at least x bytes of data.
    // If we don't have enough data it will just use default data (`0` or `vec![]`).
    // if data.len() < 166 {
    //     return None;
    // }
    let properties = RRuleProperties {
        freq: match take_byte(&mut data) % 7 {
            0 => Frequency::Yearly,
            1 => Frequency::Monthly,
            2 => Frequency::Weekly,
            3 => Frequency::Daily,
            4 => Frequency::Hourly,
            5 => Frequency::Minutely,
            _ => Frequency::Secondly,
        },
        interval: take_data_u16(&mut data),
        count: match take_byte(&mut data) % 2 {
            // use 1 + 4 bytes
            0 => Some(take_data_u32(&mut data)),
            _ => None,
        },
        until: match take_byte(&mut data) % 2 {
            // use 1 + 20 bytes
            0 => Some(take_datetime(&mut data)),
            _ => None,
        },
        // Just selected a few timezones
        tz: match take_byte(&mut data) % 2 {
            0 => chrono_tz::Europe::London,
            _ => chrono_tz::America::New_York,
        },
        dt_start: take_datetime(&mut data),
        week_start: take_weekday(&mut data),
        by_set_pos: take_vec_i32(&mut data),
        by_month: take_vec_u8(&mut data),
        by_month_day: take_vec_i8(&mut data),
        by_n_month_day: take_vec_i8(&mut data),
        by_year_day: take_vec_i16(&mut data),
        by_week_no: take_vec_i8(&mut data),
        by_weekday: take_vec_of_nweekday(&mut data),
        by_hour: take_vec_u8(&mut data),
        by_minute: take_vec_u8(&mut data),
        by_second: take_vec_u8(&mut data),
        #[cfg(feature = "by-easter")]
        by_easter: match take_byte(&mut data) % 2 {
            // use 1 + 2 bytes
            0 => Some(take_data_i16(&mut data)),
            _ => None,
        },
        #[cfg(not(feature = "by-easter"))]
        by_easter: None,
    };
    match RRule::new(properties) {
        Ok(rrule) => Some(rrule),
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}
