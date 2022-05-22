use rrule::RRuleSet;

#[test]
fn non_ascii_string() {
    let result = "Input string contains some invalid characters";
    let strs = [
        "DTSTART;19970902T090000Z\nRRULE:FREQ=DAILY;COeeeEDDUNT=10",
        "DTSTART;19970902T090000Z\nRRULE:FREQ=DAILY;CÖUNT=10",
        "DTSTART;19970902T090000Z\nRRULE:FREQ=DAILY;CO␀NT=10",
    ];

    for s in strs {
        let error = s.parse::<RRuleSet>();
        assert!(error.unwrap_err().to_string().contains(result));
    }
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
