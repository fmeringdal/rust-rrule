mod common;

use common::ymd_hms;
use rrule::{Frequency, NWeekday, Unvalidated, Weekday};
use rrule::{RRule, RRuleSet};
use std::str::FromStr;

#[test]
fn rrule_properties_from_str() {
    let test_str_cases = vec![
        "RRULE:UNTIL=19990404T110000Z;FREQ=WEEKLY;BYDAY=TU,WE",
        "UNTIL=20190101T230000Z;FREQ=WEEKLY;BYDAY=TU,WE",
    ];

    for test_str in test_str_cases {
        let res = RRule::from_str(test_str);
        assert!(res.is_ok());
    }
}

#[test]
fn rrule_properties_to_and_from_str() {
    for test_obj in get_obj_cases() {
        let test_str = test_obj.to_string();
        let res = RRule::from_str(&test_str).unwrap();
        assert_eq!(res, test_obj);
    }
}

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_json_to_and_from_rrule_properties() {
    #[derive(orig_serde::Deserialize, orig_serde::Serialize)]
    #[serde(crate = "orig_serde")]
    struct RruleTest {
        rrule: RRule<Unvalidated>,
    }

    for test_obj in get_obj_cases() {
        let test_str = serde_json::to_string(&RruleTest {
            rrule: test_obj.clone(),
        })
        .unwrap();

        let res = serde_json::from_str::<RruleTest>(&test_str).unwrap();

        assert_eq!(res.rrule, test_obj);
    }
}

#[test]
fn rrule_to_and_from_str() {
    let test_cases = [
        "DTSTART:20120201T093000Z\nRRULE:FREQ=YEARLY;COUNT=3",
        "DTSTART:20120201T093000Z\nRRULE:FREQ=WEEKLY;INTERVAL=5;BYDAY=-2MO,FR",
        "DTSTART;TZID=America/New_York:19000201T093000Z\nRRULE:UNTIL=19990404T110000Z;FREQ=WEEKLY;BYDAY=TU,WE",
    ];

    for test_str in test_cases {
        let test_obj = RRuleSet::from_str(test_str).unwrap();
        let test_str = test_obj.to_string();
        let test_obj2 = RRuleSet::from_str(&test_str).unwrap();

        assert_eq!(test_obj, test_obj2);
    }
}

#[cfg(feature = "serde")]
#[test]
fn serialize_deserialize_json_to_and_from_rrule() {
    #[derive(orig_serde::Deserialize, orig_serde::Serialize, PartialEq, Debug)]
    #[serde(crate = "orig_serde")]
    struct RruleTest {
        rrule: RRuleSet,
    }

    let test_cases = [
        "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=5",
        "DTSTART:20120201T093000Z\nRRULE:FREQ=WEEKLY;INTERVAL=5;BYDAY=-2MO,FR",
        "DTSTART;TZID=America/New_York:19000201T093000Z\nRRULE:UNTIL=19990404T110000Z;FREQ=WEEKLY;BYDAY=TU,WE",
    ];

    for test_str in test_cases {
        let rrule = RRuleSet::from_str(test_str).unwrap();
        let src_obj = RruleTest { rrule };

        let test_str = serde_json::to_string(&src_obj).unwrap();
        let final_obj = serde_json::from_str::<RruleTest>(&test_str).unwrap();

        assert_eq!(src_obj, final_obj);
    }
}

fn get_obj_cases() -> Vec<RRule<Unvalidated>> {
    vec![
        RRule {
            freq: Frequency::Yearly,
            count: Some(3),
            ..Default::default()
        },
        RRule {
            freq: Frequency::Weekly,
            interval: 5,
            by_weekday: vec![
                NWeekday::Nth(-2, Weekday::Mon),
                NWeekday::Every(Weekday::Fri),
            ],
            ..Default::default()
        },
        RRule {
            freq: Frequency::Weekly,
            until: Some(ymd_hms(1999, 4, 4, 11, 0, 0)),
            by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Wed)],
            ..Default::default()
        },
        RRule {
            freq: Frequency::Weekly,
            until: Some(ymd_hms(2019, 1, 1, 23, 0, 0)),
            by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Wed)],
            ..Default::default()
        },
    ]
}
