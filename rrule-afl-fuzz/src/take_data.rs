#![allow(clippy::module_name_repetitions)]

use chrono::{DateTime, TimeZone, Utc, Weekday};
use rrule::{NWeekday, Tz};
use std::convert::TryInto;

// https://doc.rust-lang.org/std/mem/fn.size_of.html

/// Uses max 1 + 5*4 = 21 bytes
pub fn take_vec_of_nweekday(input: &mut &[u8]) -> Vec<NWeekday> {
    let max_amount_of_items = 5;
    let amount_of_items = take_byte(input) % (max_amount_of_items + 1);
    (0..amount_of_items).map(|_| take_nweekday(input)).collect()
}

/// Uses max 1 + 5*8 = 41 bytes
pub fn take_vec_usize(input: &mut &[u8]) -> Vec<usize> {
    let max_amount_of_items = 5;
    let amount_of_items = take_byte(input) % (max_amount_of_items + 1);
    (0..amount_of_items)
        .map(|_| take_data_usize(input))
        .collect()
}

/// Uses max 1 + 5*8 = 41 bytes
pub fn take_vec_isize(input: &mut &[u8]) -> Vec<isize> {
    let max_amount_of_items = 5;
    let amount_of_items = take_byte(input) % (max_amount_of_items + 1);
    (0..amount_of_items)
        .map(|_| take_data_isize(input))
        .collect()
}

/// Uses max 1 + 5*1 = 6 bytes
pub fn take_vec_u8(input: &mut &[u8]) -> Vec<u8> {
    let max_amount_of_items = 5;
    let amount_of_items = take_byte(input) % (max_amount_of_items + 1);
    (0..amount_of_items).map(|_| take_data_u8(input)).collect()
}

/// Uses max 1 + 5*1 = 6 bytes
pub fn take_vec_i8(input: &mut &[u8]) -> Vec<i8> {
    let max_amount_of_items = 5;
    let amount_of_items = take_byte(input) % (max_amount_of_items + 1);
    (0..amount_of_items).map(|_| take_data_i8(input)).collect()
}

/// Uses max 1 + 5*2 = 11 bytes
pub fn take_vec_i16(input: &mut &[u8]) -> Vec<i16> {
    let max_amount_of_items = 5;
    let amount_of_items = take_byte(input) % (max_amount_of_items + 1);
    (0..amount_of_items).map(|_| take_data_i16(input)).collect()
}

/// Uses max 1 + 5*4 = 21 bytes
pub fn take_vec_i32(input: &mut &[u8]) -> Vec<i32> {
    let max_amount_of_items = 5;
    let amount_of_items = take_byte(input) % (max_amount_of_items + 1);
    (0..amount_of_items).map(|_| take_data_i32(input)).collect()
}

/// Uses 8+4+8 = 20 bytes
pub fn take_datetime(input: &mut &[u8]) -> DateTime<Tz> {
    use chrono::offset::LocalResult;
    // M1: Larger year range
    match Utc.timestamp_opt(take_data_i64(input), take_data_u32(input)) {
        LocalResult::None => {
            // Will always succeed
            let nanos: i64 = take_data_i64(input);
            Utc.timestamp_nanos(nanos).with_timezone(&Tz::UTC)
        }
        LocalResult::Single(datetime) | LocalResult::Ambiguous(datetime, _) => {
            datetime.with_timezone(&Tz::UTC)
        }
    }
}

/// Uses 1 byte
/// If no bytes left it will always return default (`Mon`)
pub fn take_weekday(input: &mut &[u8]) -> Weekday {
    match take_byte(input) % 7 {
        0 => Weekday::Mon,
        1 => Weekday::Tue,
        2 => Weekday::Wed,
        3 => Weekday::Thu,
        4 => Weekday::Fri,
        5 => Weekday::Sat,
        _ => Weekday::Sun,
    }
}

/// Uses 1 byte
/// If no bytes left it will always return default (`0`)
pub fn take_byte(input: &mut &[u8]) -> u8 {
    let byte_len = std::mem::size_of::<u8>();
    if input.len() < byte_len {
        return u8::default();
    }
    let (int_bytes, rest) = input.split_at(byte_len);
    *input = rest;
    u8::from_be_bytes(int_bytes.try_into().expect("Failed to convert to u8"))
}

/// Uses max 1+2+1 = 4 byte
/// If no bytes left it will always return default (`Every(Mon)`)
pub fn take_nweekday(input: &mut &[u8]) -> NWeekday {
    match take_byte(input) % 2 {
        0 => NWeekday::Every(take_weekday(input)),
        _ => NWeekday::Nth(take_data_i16(input), take_weekday(input)),
    }
}

/// Uses 1 byte
/// If no bytes left it will always return default (`0`)
pub fn take_data_u8(input: &mut &[u8]) -> u8 {
    take_byte(input)
}

/// Uses 1 byte
/// If no bytes left it will always return default (`0`)
pub fn take_data_i8(input: &mut &[u8]) -> i8 {
    let byte_len = std::mem::size_of::<i8>();
    if input.len() < byte_len {
        return i8::default();
    }
    let (int_bytes, rest) = input.split_at(byte_len);
    *input = rest;
    i8::from_be_bytes(int_bytes.try_into().expect("Failed to convert to u8"))
}

/// Uses 2 bytes
/// If no bytes left it will always return default (`0`)
pub fn take_data_i16(input: &mut &[u8]) -> i16 {
    let byte_len = std::mem::size_of::<i16>();
    if input.len() < byte_len {
        return i16::default();
    }
    let (int_bytes, rest) = input.split_at(byte_len);
    *input = rest;
    i16::from_be_bytes(int_bytes.try_into().expect("Failed to convert to u8"))
}

/// Uses 4 bytes
/// If no bytes left it will always return default (`0`)
pub fn take_data_i32(input: &mut &[u8]) -> i32 {
    let byte_len = std::mem::size_of::<i32>();
    if input.len() < byte_len {
        return i32::default();
    }
    let (int_bytes, rest) = input.split_at(byte_len);
    *input = rest;
    i32::from_be_bytes(int_bytes.try_into().expect("Failed to convert to u8"))
}

/// Uses 8 bytes
/// If no bytes left it will always return default (`0`)
pub fn take_data_i64(input: &mut &[u8]) -> i64 {
    let byte_len = std::mem::size_of::<i64>();
    if input.len() < byte_len {
        return i64::default();
    }
    let (int_bytes, rest) = input.split_at(byte_len);
    *input = rest;
    i64::from_be_bytes(int_bytes.try_into().expect("Failed to convert to u8"))
}

/// Uses 2 bytes
/// If no bytes left it will always return default (`0`)
pub fn take_data_u16(input: &mut &[u8]) -> u16 {
    let byte_len = std::mem::size_of::<u16>();
    if input.len() < byte_len {
        return u16::default();
    }
    let (int_bytes, rest) = input.split_at(byte_len);
    *input = rest;
    u16::from_be_bytes(int_bytes.try_into().expect("Failed to convert to u8"))
}

/// Uses 4 bytes
/// If no bytes left it will always return default (`0`)
pub fn take_data_u32(input: &mut &[u8]) -> u32 {
    let byte_len = std::mem::size_of::<u32>();
    if input.len() < byte_len {
        return u32::default();
    }
    let (int_bytes, rest) = input.split_at(byte_len);
    *input = rest;
    u32::from_be_bytes(int_bytes.try_into().expect("Failed to convert to u8"))
}

/// Uses 8 bytes
/// If no bytes left it will always return default (`0`)
pub fn take_data_u64(input: &mut &[u8]) -> u64 {
    let byte_len = std::mem::size_of::<u64>();
    if input.len() < byte_len {
        return u64::default();
    }
    let (int_bytes, rest) = input.split_at(byte_len);
    *input = rest;
    u64::from_be_bytes(int_bytes.try_into().expect("Failed to convert to u8"))
}

/// Uses max 8 bytes
/// If no bytes left it will always return default (`0`)
pub fn take_data_usize(input: &mut &[u8]) -> usize {
    let byte_len = std::mem::size_of::<usize>();
    if input.len() < byte_len {
        return usize::default();
    }
    let (int_bytes, rest) = input.split_at(byte_len);
    *input = rest;
    usize::from_be_bytes(int_bytes.try_into().expect("Failed to convert to u8"))
}

/// Uses max 8 bytes
/// If no bytes left it will always return default (`0`)
pub fn take_data_isize(input: &mut &[u8]) -> isize {
    let byte_len = std::mem::size_of::<isize>();
    if input.len() < byte_len {
        return isize::default();
    }
    let (int_bytes, rest) = input.split_at(byte_len);
    *input = rest;
    isize::from_be_bytes(int_bytes.try_into().expect("Failed to convert to u8"))
}
