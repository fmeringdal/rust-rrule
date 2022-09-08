use crate::tests::common;
use crate::RRuleSet;

#[test]
fn issue_34() {
    let dates = "DTSTART;TZID=America/New_York:19970929T090000
RRULE:FREQ=MONTHLY;BYDAY=MO,TU,WE,TH,FR;BYSETPOS=-2"
        .parse::<RRuleSet<crate::Tz>>()
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
        .parse::<RRuleSet<_>>()
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
        .parse::<RRuleSet<_>>()
        .expect("The RRule is not valid");

    let res = rrule_set.all(10).dates;
    assert_eq!(res.len(), 10);
}
