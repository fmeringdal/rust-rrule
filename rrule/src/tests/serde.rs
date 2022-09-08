use crate::RRuleSet;
use std::str::FromStr;

#[test]
fn rrule_set_to_and_from_str() {
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
fn serialize_deserialize_json_to_and_from_rrule_set() {
    #[derive(orig_serde::Deserialize, orig_serde::Serialize, PartialEq, Eq, Debug)]
    #[serde(crate = "orig_serde")]
    struct RruleTest {
        rrule: RRuleSet<crate::Tz>,
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
