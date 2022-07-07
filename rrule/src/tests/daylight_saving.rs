use crate::{tests::common::check_occurrences, RRuleSet};

#[test]
fn daylight_savings_1() {
    let rrule: RRuleSet =
        "DTSTART;TZID=America/Vancouver:20210301T022210\nRRULE:FREQ=DAILY;COUNT=30"
            .parse()
            .unwrap();

    let (dates, error) = rrule.all_with_error(60);
    check_occurrences(
        &dates,
        &[
            "2021-03-01T02:22:10-08:00",
            "2021-03-02T02:22:10-08:00",
            "2021-03-03T02:22:10-08:00",
            "2021-03-04T02:22:10-08:00",
            "2021-03-05T02:22:10-08:00",
            "2021-03-06T02:22:10-08:00",
            "2021-03-07T02:22:10-08:00",
            "2021-03-08T02:22:10-08:00",
            "2021-03-09T02:22:10-08:00",
            "2021-03-10T02:22:10-08:00",
            "2021-03-11T02:22:10-08:00",
            "2021-03-12T02:22:10-08:00",
            "2021-03-13T02:22:10-08:00",
            "2021-03-14T03:22:10-07:00",
            "2021-03-15T02:22:10-07:00",
            "2021-03-16T02:22:10-07:00",
            "2021-03-17T02:22:10-07:00",
            "2021-03-18T02:22:10-07:00",
            "2021-03-19T02:22:10-07:00",
            "2021-03-20T02:22:10-07:00",
            "2021-03-21T02:22:10-07:00",
            "2021-03-22T02:22:10-07:00",
            "2021-03-23T02:22:10-07:00",
            "2021-03-24T02:22:10-07:00",
            "2021-03-25T02:22:10-07:00",
            "2021-03-26T02:22:10-07:00",
            "2021-03-27T02:22:10-07:00",
            "2021-03-28T02:22:10-07:00",
            "2021-03-29T02:22:10-07:00",
            "2021-03-30T02:22:10-07:00",
        ],
    );
    assert!(error.is_none());
}

#[test]
fn daylight_savings_2() {
    let dates = "DTSTART;TZID=Europe/Paris:20210214T093000\n\
        RRULE:FREQ=WEEKLY;UNTIL=20210508T083000Z;INTERVAL=2;BYDAY=MO;WKST=MO"
        .parse::<RRuleSet>()
        .unwrap()
        .all(50)
        .unwrap();
    check_occurrences(
        &dates,
        &[
            "2021-02-22T09:30:00+01:00",
            "2021-03-08T09:30:00+01:00",
            "2021-03-22T09:30:00+01:00",
            "2021-04-05T09:30:00+02:00", // Switching to daylight saving time.
            "2021-04-19T09:30:00+02:00",
            "2021-05-03T09:30:00+02:00",
        ],
    );
}