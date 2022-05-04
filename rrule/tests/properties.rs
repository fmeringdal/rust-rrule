mod common;

use chrono::TimeZone;
use chrono_tz::UTC;
use rrule::{Frequency, NWeekday, RRuleProperties, Weekday};
use std::str::FromStr;

fn get_obj_cases() -> Vec<RRuleProperties> {
    vec![
        RRuleProperties {
            freq: Frequency::Yearly,
            count: Some(3),
            ..Default::default()
        },
        RRuleProperties {
            freq: Frequency::Weekly,
            interval: 5,
            by_weekday: vec![
                NWeekday::Nth(-2, Weekday::Mon),
                NWeekday::Every(Weekday::Fri),
            ],
            ..Default::default()
        },
        RRuleProperties {
            freq: Frequency::Weekly,
            until: Some(UTC.ymd(1999, 4, 4).and_hms(11, 0, 0)),
            by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Wed)],
            ..Default::default()
        },
        RRuleProperties {
            freq: Frequency::Weekly,
            until: Some(UTC.ymd(2019, 1, 1).and_hms(23, 0, 0)),
            by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Wed)],
            ..Default::default()
        },
    ]
}

#[test]
fn rrule_properties_from_str() {
    let test_str_cases = vec![
        "RRULE:UNTIL=19990404T110000Z;FREQ=WEEKLY;BYDAY=TU,WE",
        "UNTIL=20190101T230000Z;FREQ=WEEKLY;BYDAY=TU,WE",
    ];

    for test_str in test_str_cases {
        let res = RRuleProperties::from_str(test_str);
        assert!(res.is_ok());
    }
}

#[test]
fn rrule_properties_to_and_from_str() {
    for test_obj in get_obj_cases() {
        let test_str = test_obj.to_string();
        let res = RRuleProperties::from_str(&test_str).unwrap();
        assert_eq!(res, test_obj);
    }
}

#[cfg(feature = "with-serde")]
#[test]
fn serialize_deserialize_json_to_and_from_rrule_properties() {
    #[allow(dead_code)]
    #[derive(serde::Deserialize, serde::Serialize)]
    struct RruleTest {
        rrule: RRuleProperties,
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
