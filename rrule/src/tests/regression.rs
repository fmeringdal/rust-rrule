use chrono::Month;

use crate::tests::common;
use crate::{Frequency, RRule, RRuleSet, Unvalidated};

#[test]
fn issue_34() {
    let dates = "DTSTART;TZID=America/New_York:19970929T090000
RRULE:FREQ=MONTHLY;BYDAY=MO,TU,WE,TH,FR;BYSETPOS=-2"
        .parse::<RRuleSet>()
        .unwrap()
        .all(7)
        .dates;
    common::check_occurrences(
        &dates,
        &[
            "1997-09-29T09:00:00-04:00",
            "1997-10-30T09:00:00-05:00",
            "1997-11-27T09:00:00-05:00",
            "1997-12-30T09:00:00-05:00",
            "1998-01-29T09:00:00-05:00",
            "1998-02-26T09:00:00-05:00",
            "1998-03-30T09:00:00-05:00",
        ],
    );
}

#[test]
fn issue_49() {
    let rrule_set = "DTSTART:20211214T091500\nEXDATE:20211228T091500,20220104T091500\nRRULE:FREQ=WEEKLY;UNTIL=20220906T091500;INTERVAL=1;BYDAY=TU;WKST=MO"
        .parse::<RRuleSet>()
        .expect("The RRule is not valid");

    let res = rrule_set.all(1).dates;
    assert!(!res.is_empty());
    let res_str = format!("{}", res[0]);
    // Check that result datetime is not in UTC
    assert!(!res_str.contains("UTC"));
}

#[test]
fn issue_61() {
    let rrule_set = "DTSTART;TZID=Europe/Berlin:18930401T010000\nRRULE:FREQ=DAILY"
        .parse::<RRuleSet>()
        .expect("The RRule is not valid");

    let res = rrule_set.all(10).dates;
    assert_eq!(res.len(), 10);
}

// Frequency should be capitalized
#[test]
fn issue_97() {
    let rrule = RRule::new(Frequency::Yearly)
        .by_month_day((24..=26).collect())
        .by_month(&[Month::December]);

    assert_eq!(
        rrule.to_string(),
        "FREQ=YEARLY;BYMONTH=12;BYMONTHDAY=24,25,26"
    );
}

#[test]
fn issue_111() {
    let rrule = "RRULE:FREQ=WEEKLY;INTERVAL=1;BYDAY=TU;WKST=SU".parse::<RRule<Unvalidated>>();

    // Convert to string...
    let rrule_str = format!("{}", rrule.unwrap());
    assert!(rrule_str.contains("WKST=SU"));
}
