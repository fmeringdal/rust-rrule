use crate::parser::ParseError;
use crate::tests::common;
use crate::{RRuleError, RRuleSet};

#[test]
fn issue_34() {
    let dates = "DTSTART;TZID=America/New_York:19970929T090000
RRULE:FREQ=MONTHLY;BYDAY=MO,TU,WE,TH,FR;BYSETPOS=-2"
        .parse::<RRuleSet>()
        .unwrap()
        .all(7)
        .unwrap();
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
#[ignore = "stick in an infinite loop in exrule calculation"]
fn edge_case_1() {
    let rrule_set: RRuleSet = "DTSTART;TZID=Europe/Berlin:20210101T000000;
RRULE:FREQ=MONTHLY
EXRULE:FREQ=MONTHLY"
        .parse()
        .expect("The RRule is not valid");

    let _ = rrule_set.all(10);
}

#[test]
#[ignore = "This time doesn't exist at all and some rruleset errors must be there: https://www.timeanddate.com/time/change/germany/berlin?year=1893"]
fn edge_case_2() {
    let rrule_set: RRuleSet = "DTSTART;TZID=Europe/Berlin:18930401T010000;\nRRULE:FREQ=DAILY"
        .parse()
        .expect("The RRule is not valid");

    assert!(rrule_set.all_with_error(10).1.is_some());
}
