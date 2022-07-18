pub mod take_data;
pub mod take_rrule;

use take_rrule::take_rrule_from_data;

use afl::fuzz;
use core::str::FromStr;
use rrule::RRuleSet;
use std::{fs, str};

// Used for reading the fuzz output
#[allow(dead_code)]
fn main2() {
    let crash_dir = fs::read_dir("out/default/crashes").unwrap();
    println!("Crashes:");
    for path in crash_dir {
        let bytes = fs::read(path.unwrap().path()).unwrap();
        let rrule = take_rrule_from_data(&bytes);
        eprintln!("RRule: {:?}", rrule);
        if let Some(rrule) = rrule {
            rrule.all(50).unwrap();
        }
    }
    println!("Hangs:");
    let crash_dir = fs::read_dir("out/default/hangs").unwrap();
    for path in crash_dir {
        let bytes = fs::read(path.unwrap().path()).unwrap();
        let rrule = take_rrule_from_data(&bytes);
        if let Some(rrule) = rrule {
            eprintln!("RRule {:?}", rrule);
            let dates = rrule.all(50);
            eprintln!("Dates {:?}", dates);
        }
    }
}

#[allow(clippy::single_match)]
fn main() {
    let fuzz_selector: u8 = 2;
    // let fuzz_target
    match fuzz_selector {
        // Fuzz the RRuleSet from string
        0 => fuzz!(|data: &[u8]| {
            if let Ok(s) = str::from_utf8(data) {
                // Everything here is acceptable, but panic is not what we want.
                match RRuleSet::from_str(s) {
                    Ok(_rule) => {}
                    Err(_) => {}
                }
            }
        }),
        // Fuzz the RRuleSet from string
        1 => fuzz!(|data: &[u8]| {
            if let Ok(s) = str::from_utf8(data) {
                // Everything here is acceptable, but panic is not what we want.
                match RRuleSet::from_str(s) {
                    Ok(rule) => {
                        let _ = rule.all(50);
                    }
                    Err(_) => {}
                }
            }
        }),
        // Fuzz directly to RRule, skip parser
        2 => fuzz!(|data: &[u8]| {
            let rule = match take_rrule_from_data(data) {
                Some(rule) => rule,
                None => return, // Not enough data to create `RRule`
            };
            let _ = rule.all(50);
        }),
        _ => {}
    }
}
