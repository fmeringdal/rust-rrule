#![allow(clippy::wildcard_imports, clippy::module_name_repetitions)]

use crate::take_data::*;
use chrono::Month;
use num_traits::cast::FromPrimitive;
use rrule::{Frequency, RRule, RRuleSet};

/// This function uses the data to construct a deterministic input for [`RRuleSet`].
/// This can also be used to reconstruct the [`RRuleSet`] from crashes in order to debug the code.
#[must_use]
pub fn take_rrule_from_data(mut data: &[u8]) -> Option<RRuleSet> {
    // Byte uses: (always account for max used)
    // bytes => variable
    // ----------------
    // 1  => freq
    // 2  => interval
    // 5  => count
    // 21 => until
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

    let freq = match take_byte(&mut data) % 7 {
        0 => Frequency::Yearly,
        1 => Frequency::Monthly,
        2 => Frequency::Weekly,
        3 => Frequency::Daily,
        4 => Frequency::Hourly,
        5 => Frequency::Minutely,
        _ => Frequency::Secondly,
    };
    let interval = take_data_u16(&mut data);
    let count = match take_byte(&mut data) % 2 {
        // use 1 + 4 bytes
        0 => Some(take_data_u32(&mut data)),
        _ => None,
    };
    let until = match take_byte(&mut data) % 2 {
        // use 1 + 20 bytes
        0 => Some(take_datetime(&mut data)),
        _ => None,
    };
    let week_start = take_weekday(&mut data);
    let by_set_pos = take_vec_i32(&mut data);
    let by_month = take_vec_u8(&mut data)
        .iter()
        .map(|x| Month::from_u8(*x).unwrap())
        .collect::<Vec<_>>();
    let by_month_day = take_vec_i8(&mut data);
    let _by_n_month_day = take_vec_i8(&mut data);
    let by_year_day = take_vec_i16(&mut data);
    let by_week_no = take_vec_i8(&mut data);
    let by_weekday = take_vec_of_nweekday(&mut data);
    let by_hour = take_vec_u8(&mut data);
    let by_minute = take_vec_u8(&mut data);
    let by_second = take_vec_u8(&mut data);
    #[cfg(feature = "by-easter")]
    let by_easter = match take_byte(&mut data) % 2 {
        // use 1 + 2 bytes
        0 => Some(take_data_i16(&mut data)),
        _ => None,
    };
    let dt_start = take_datetime(&mut data);

    let mut rrule = RRule::new(freq)
        .interval(interval)
        .week_start(week_start)
        .by_set_pos(by_set_pos)
        .by_month(&by_month)
        .by_month_day(by_month_day)
        .by_year_day(by_year_day)
        .by_week_no(by_week_no)
        .by_weekday(by_weekday)
        .by_hour(by_hour)
        .by_minute(by_minute)
        .by_second(by_second);

    if let Some(c) = count {
        rrule = rrule.count(c);
    }

    if let Some(u) = until {
        rrule = rrule.until(u);
    }

    #[cfg(feature = "by-easter")]
    if let Some(be) = by_easter {
        rrule = rrule.by_easter(be);
    }

    match rrule.build(dt_start) {
        Ok(rrule) => Some(rrule),
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}
