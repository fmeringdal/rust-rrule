use crate::tests::common;
use crate::RRuleSet;

/// Monthly on the 31st of the month
#[test]
fn monthly_on_31th() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=MONTHLY;COUNT=10;BYMONTHDAY=31"
        .parse::<RRuleSet>()
        .unwrap()
        .all(u16::MAX)
        .dates;
    // TODO: Is this the correct behavior?
    common::check_occurrences(
        &dates,
        &[
            "1997-10-31T09:00:00-05:00",
            "1997-12-31T09:00:00-05:00",
            "1998-01-31T09:00:00-05:00",
            "1998-03-31T09:00:00-05:00",
            "1998-05-31T09:00:00-04:00",
            "1998-07-31T09:00:00-04:00",
            "1998-08-31T09:00:00-04:00",
            "1998-10-31T09:00:00-05:00",
            "1998-12-31T09:00:00-05:00",
            "1999-01-31T09:00:00-05:00",
        ],
    );
}

/// Monthly on the 31th-to-last of the month
#[test]
fn monthly_on_31th_to_last() {
    let dates = "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=MONTHLY;COUNT=10;BYMONTHDAY=-31"
        .parse::<RRuleSet>()
        .unwrap()
        .all(u16::MAX)
        .dates;
    // TODO: Is this the correct behavior?
    common::check_occurrences(
        &dates,
        &[
            "1997-10-01T09:00:00-04:00",
            "1997-12-01T09:00:00-05:00",
            "1998-01-01T09:00:00-05:00",
            "1998-03-01T09:00:00-05:00",
            "1998-05-01T09:00:00-04:00",
            "1998-07-01T09:00:00-04:00",
            "1998-08-01T09:00:00-04:00",
            "1998-10-01T09:00:00-04:00",
            "1998-12-01T09:00:00-05:00",
            "1999-01-01T09:00:00-05:00",
        ],
    );
}
