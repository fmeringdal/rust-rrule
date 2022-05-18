pub mod take_data;
pub mod take_rrule;

use take_rrule::take_rrule_from_data;

use afl::fuzz;
use core::str::FromStr;
use rrule::{DateFilter, RRuleSet, RRuleSet};

#[allow(clippy::single_match)]
fn main() {
    let fuzz_selector: u8 = 4;
    // let fuzz_target
    match fuzz_selector {
        // Fuzz the RRule from string
        0 => fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                // Everything here is acceptable, but panic is not what we want.
                match RRuleSet::from_str(s) {
                    Ok(_rule) => {}
                    Err(_) => {}
                }
            }
        }),
        // Fuzz the RRuleSet from string
        1 => fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                // Everything here is acceptable, but panic is not what we want.
                match RRuleSet::from_str(s) {
                    Ok(_rule) => {}
                    Err(_) => {}
                }
            }
        }),
        // Fuzz the RRule from string and list all
        2 => fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                // Everything here is acceptable, but panic is not what we want.
                match RRuleSet::from_str(s) {
                    Ok(rule) => {
                        let _ = rule.into_iter().all(50);
                    }
                    Err(_) => {}
                }
            }
        }),
        // Fuzz the RRuleSet from string
        3 => fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                // Everything here is acceptable, but panic is not what we want.
                match RRuleSet::from_str(s) {
                    Ok(rule) => {
                        let _ = rule.into_iter().all(50);
                    }
                    Err(_) => {}
                }
            }
        }),
        // Fuzz directly to RRule, skip parser
        4 => fuzz!(|data: &[u8]| {
            let rule = match take_rrule_from_data(data) {
                Some(rule) => rule,
                None => return, // Not enough data to create `RRule`
            };
            let _ = rule.into_iter().all(50);
        }),
        _ => {}
    }
}
