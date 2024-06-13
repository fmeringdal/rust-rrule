use crate::core::Tz;
use crate::tests::common::{test_recurring_rrule, ymd_hms};
use crate::{Frequency, NWeekday, RRule, RRuleSet, Weekday};
use chrono::{Datelike, TimeZone};

#[test]
fn yearly() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1998, 9, 2, 9, 0, 0),
            ymd_hms(1999, 9, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_interval() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1999, 9, 2, 9, 0, 0),
            ymd_hms(2001, 9, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_interval_large() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        interval: 40,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(2037, 9, 2, 9, 0, 0),
            ymd_hms(2077, 9, 2, 9, 0, 0),
        ],
    );
}

#[test]
#[cfg(feature = "by-easter")]
fn yearly_by_easter() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_easter: Some(0),
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 4, 12, 9, 0, 0),
            ymd_hms(1999, 4, 4, 9, 0, 0),
            ymd_hms(2000, 4, 23, 9, 0, 0),
        ],
    );
}

#[test]
#[cfg(feature = "by-easter")]
fn yearly_by_easterpos() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_easter: Some(1),
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 4, 13, 9, 0, 0),
            ymd_hms(1999, 4, 5, 9, 0, 0),
            ymd_hms(2000, 4, 24, 9, 0, 0),
        ],
    );
}

#[test]
#[cfg(feature = "by-easter")]
fn yearly_by_easterpos_neg() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_easter: Some(-2),
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 4, 10, 9, 0, 0),
            ymd_hms(1999, 4, 2, 9, 0, 0),
            ymd_hms(2000, 4, 21, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_month() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![1, 3],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 2, 9, 0, 0),
            ymd_hms(1998, 3, 2, 9, 0, 0),
            ymd_hms(1999, 1, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_monthday() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 10, 1, 9, 0, 0),
            ymd_hms(1997, 10, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_month_and_monthday() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![1, 3],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![5, 7],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 5, 9, 0, 0),
            ymd_hms(1998, 1, 7, 9, 0, 0),
            ymd_hms(1998, 3, 5, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_weekday() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_nweekday() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_weekday: vec![
            NWeekday::Nth(1, Weekday::Tue),
            NWeekday::Nth(-1, Weekday::Thu),
        ],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 12, 25, 9, 0, 0),
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 12, 31, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_nweekday_large() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_weekday: vec![
            NWeekday::Nth(13, Weekday::Tue),
            NWeekday::Nth(-13, Weekday::Thu),
        ],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 10, 2, 9, 0, 0),
            ymd_hms(1998, 3, 31, 9, 0, 0),
            ymd_hms(1998, 10, 8, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_month_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 8, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_month_and_nweekday() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![
            NWeekday::Nth(1, Weekday::Tue),
            NWeekday::Nth(-1, Weekday::Thu),
        ],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 29, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_month_and_nweekday_large() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![
            NWeekday::Nth(3, Weekday::Tue),
            NWeekday::Nth(-3, Weekday::Thu),
        ],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 15, 9, 0, 0),
            ymd_hms(1998, 1, 20, 9, 0, 0),
            ymd_hms(1998, 3, 12, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_monthday_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 2, 3, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_month_and_monthday_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
            ymd_hms(2001, 3, 1, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_yearday() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(4),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 12, 31, 9, 0, 0),
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 4, 10, 9, 0, 0),
            ymd_hms(1998, 7, 19, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_yeardayneg() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(4),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![-365, -266, -166, -1],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 12, 31, 9, 0, 0),
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 4, 10, 9, 0, 0),
            ymd_hms(1998, 7, 19, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_month_and_yearday() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(4),
        by_month: vec![4, 7],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 4, 10, 9, 0, 0),
            ymd_hms(1998, 7, 19, 9, 0, 0),
            ymd_hms(1999, 4, 10, 9, 0, 0),
            ymd_hms(1999, 7, 19, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_weekno() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_hour: vec![9],
        by_week_no: vec![20],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 5, 11, 9, 0, 0),
            ymd_hms(1998, 5, 12, 9, 0, 0),
            ymd_hms(1998, 5, 13, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_weekno_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        by_hour: vec![9],
        by_week_no: vec![1],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 12, 29, 9, 0, 0),
            ymd_hms(1999, 1, 4, 9, 0, 0),
            ymd_hms(2000, 1, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_weekno_and_weekday_large() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_hour: vec![9],
        by_week_no: vec![52],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 12, 28, 9, 0, 0),
            ymd_hms(1998, 12, 27, 9, 0, 0),
            ymd_hms(2000, 1, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_weekno_and_weekday_last() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_hour: vec![9],
        by_week_no: vec![-1],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 12, 28, 9, 0, 0),
            ymd_hms(1999, 1, 3, 9, 0, 0),
            ymd_hms(2000, 1, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_weekno_and_weekday53_last() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        by_hour: vec![9],
        by_week_no: vec![53],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 12, 28, 9, 0, 0),
            ymd_hms(2004, 12, 27, 9, 0, 0),
            ymd_hms(2009, 12, 28, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_hour() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 0, 0),
            ymd_hms(1998, 9, 2, 6, 0, 0),
            ymd_hms(1998, 9, 2, 18, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_minute() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        by_hour: vec![9],
        by_minute: vec![6, 18],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 6, 0),
            ymd_hms(1997, 9, 2, 9, 18, 0),
            ymd_hms(1998, 9, 2, 9, 6, 0),
        ],
    );
}

#[test]
fn yearly_by_second() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![6, 18],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 6),
            ymd_hms(1997, 9, 2, 9, 0, 18),
            ymd_hms(1998, 9, 2, 9, 0, 6),
        ],
    );
}

#[test]
fn yearly_by_hour_and_minute() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 6, 0),
            ymd_hms(1997, 9, 2, 18, 18, 0),
            ymd_hms(1998, 9, 2, 6, 6, 0),
        ],
    );
}

#[test]
fn yearly_by_hour_and_second() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![6, 18],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 0, 6),
            ymd_hms(1997, 9, 2, 18, 0, 18),
            ymd_hms(1998, 9, 2, 6, 0, 6),
        ],
    );
}

#[test]
fn yearly_by_minute_and_second() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        by_hour: vec![9],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 6, 6),
            ymd_hms(1997, 9, 2, 9, 6, 18),
            ymd_hms(1997, 9, 2, 9, 18, 6),
        ],
    );
}

#[test]
fn yearly_by_hour_and_minute_and_second() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 6, 6),
            ymd_hms(1997, 9, 2, 18, 6, 18),
            ymd_hms(1997, 9, 2, 18, 18, 6),
        ],
    );
}

#[test]
fn yearly_by_setpos() {
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_hour: vec![6, 18],
        by_set_pos: vec![3, -3],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![15],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 11, 15, 18, 0, 0),
            ymd_hms(1998, 2, 15, 6, 0, 0),
            ymd_hms(1998, 11, 15, 18, 0, 0),
        ],
    );
}

#[test]
fn monthly() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 10, 2, 9, 0, 0),
            ymd_hms(1997, 11, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_interval() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 11, 2, 9, 0, 0),
            ymd_hms(1998, 1, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_interval_large() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        interval: 18,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1999, 3, 2, 9, 0, 0),
            ymd_hms(2000, 9, 2, 9, 0, 0),
        ],
    );
}

#[test]
#[cfg(feature = "by-easter")]
fn monthly_by_easter() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_easter: Some(0),
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 4, 12, 9, 0, 0),
            ymd_hms(1999, 4, 4, 9, 0, 0),
            ymd_hms(2000, 4, 23, 9, 0, 0),
        ],
    );
}

#[test]
#[cfg(feature = "by-easter")]
fn monthly_by_easterpos() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_easter: Some(1),
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 4, 13, 9, 0, 0),
            ymd_hms(1999, 4, 5, 9, 0, 0),
            ymd_hms(2000, 4, 24, 9, 0, 0),
        ],
    );
}

#[test]
#[cfg(feature = "by-easter")]
fn monthly_by_easterpos_neg() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_easter: Some(-2),
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 4, 10, 9, 0, 0),
            ymd_hms(1999, 4, 2, 9, 0, 0),
            ymd_hms(2000, 4, 21, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_neg_by_monthday_janfeb_for_nonleapyear() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(4),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_n_month_day: vec![-1],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(2013, 12, 1, 9, 0, 0),
        &[
            ymd_hms(2013, 12, 31, 9, 0, 0),
            ymd_hms(2014, 1, 31, 9, 0, 0),
            ymd_hms(2014, 2, 28, 9, 0, 0),
            ymd_hms(2014, 3, 31, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_neg_by_monthday_janfeb_for_leapyear() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(4),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_n_month_day: vec![-1],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(2015, 12, 1, 9, 0, 0),
        &[
            ymd_hms(2015, 12, 31, 9, 0, 0),
            ymd_hms(2016, 1, 31, 9, 0, 0),
            ymd_hms(2016, 2, 29, 9, 0, 0),
            ymd_hms(2016, 3, 31, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_neg_monthday() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(6),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_n_month_day: vec![-1, -3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(2015, 12, 1, 9, 0, 0),
        &[
            ymd_hms(2015, 12, 29, 9, 0, 0),
            ymd_hms(2015, 12, 31, 9, 0, 0),
            ymd_hms(2016, 1, 29, 9, 0, 0),
            ymd_hms(2016, 1, 31, 9, 0, 0),
            ymd_hms(2016, 2, 27, 9, 0, 0),
            ymd_hms(2016, 2, 29, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_month() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_month: vec![1, 3],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 2, 9, 0, 0),
            ymd_hms(1998, 3, 2, 9, 0, 0),
            ymd_hms(1999, 1, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_monthday() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 10, 1, 9, 0, 0),
            ymd_hms(1997, 10, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_month_and_monthday() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_month: vec![1, 3],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![5, 7],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 5, 9, 0, 0),
            ymd_hms(1998, 1, 7, 9, 0, 0),
            ymd_hms(1998, 3, 5, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_weekday() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_nweekday() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_weekday: vec![
            NWeekday::Nth(1, Weekday::Tue),
            NWeekday::Nth(-1, Weekday::Thu),
        ],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 25, 9, 0, 0),
            ymd_hms(1997, 10, 7, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_nweekday_large() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_weekday: vec![
            NWeekday::Nth(3, Weekday::Tue),
            NWeekday::Nth(-3, Weekday::Thu),
        ],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 11, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 10, 16, 9, 0, 0),
        ],
    );
}

#[test]
fn issue_104() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        interval: 2,
        count: Some(3),
        by_weekday: vec![NWeekday::Nth(-1, Weekday::Mon)],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(2023, 10, 30, 9, 0, 0),
        &[
            ymd_hms(2023, 10, 30, 9, 0, 0),
            ymd_hms(2023, 12, 25, 9, 0, 0),
            ymd_hms(2024, 2, 26, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_month_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 8, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_month_and_nweekday() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![
            NWeekday::Nth(1, Weekday::Tue),
            NWeekday::Nth(-1, Weekday::Thu),
        ],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 29, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_month_and_nweekday_large() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![
            NWeekday::Nth(3, Weekday::Tue),
            NWeekday::Nth(-3, Weekday::Thu),
        ],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 15, 9, 0, 0),
            ymd_hms(1998, 1, 20, 9, 0, 0),
            ymd_hms(1998, 3, 12, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_monthday_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 2, 3, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_month_and_monthday_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
            ymd_hms(2001, 3, 1, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_hour() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 0, 0),
            ymd_hms(1997, 10, 2, 6, 0, 0),
            ymd_hms(1997, 10, 2, 18, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_minute() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![6, 18],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 6, 0),
            ymd_hms(1997, 9, 2, 9, 18, 0),
            ymd_hms(1997, 10, 2, 9, 6, 0),
        ],
    );
}

#[test]
fn monthly_by_second() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![6, 18],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 6),
            ymd_hms(1997, 9, 2, 9, 0, 18),
            ymd_hms(1997, 10, 2, 9, 0, 6),
        ],
    );
}

#[test]
fn monthly_by_hour_and_minute() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 6, 0),
            ymd_hms(1997, 9, 2, 18, 18, 0),
            ymd_hms(1997, 10, 2, 6, 6, 0),
        ],
    );
}

#[test]
fn monthly_by_hour_and_second() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![6, 18],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 0, 6),
            ymd_hms(1997, 9, 2, 18, 0, 18),
            ymd_hms(1997, 10, 2, 6, 0, 6),
        ],
    );
}

#[test]
fn monthly_by_minute_and_second() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 6, 6),
            ymd_hms(1997, 9, 2, 9, 6, 18),
            ymd_hms(1997, 9, 2, 9, 18, 6),
        ],
    );
}

#[test]
fn monthly_by_hour_and_minute_and_second() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 6, 6),
            ymd_hms(1997, 9, 2, 18, 6, 18),
            ymd_hms(1997, 9, 2, 18, 18, 6),
        ],
    );
}

#[test]
fn monthly_by_setpos() {
    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(3),
        by_hour: vec![6, 18],
        by_set_pos: vec![3, -3],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![13, 17],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 13, 18, 0, 0),
            ymd_hms(1997, 9, 17, 6, 0, 0),
            ymd_hms(1997, 10, 13, 18, 0, 0),
        ],
    );
}

#[test]
fn weekly() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
        ],
    );
}

#[test]
fn weekly_interval() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 9, 30, 9, 0, 0),
        ],
    );
}

#[test]
fn weekly_interval_large() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 20,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1998, 1, 20, 9, 0, 0),
            ymd_hms(1998, 6, 9, 9, 0, 0),
        ],
    );
}

#[test]
fn weekly_by_month() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(6),
        by_month: vec![1, 3],
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 13, 9, 0, 0),
            ymd_hms(1998, 1, 20, 9, 0, 0),
            ymd_hms(1998, 1, 27, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
            ymd_hms(1998, 3, 10, 9, 0, 0),
        ],
    );
}

#[test]
fn weekly_by_weekday() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
        ],
    );
}

// TODO: why isn't this using nweekday?
#[test]
fn weekly_by_nweekday() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
        ],
    );
}

#[test]
fn weekly_by_month_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 8, 9, 0, 0),
        ],
    );
}

#[test]
fn weekly_by_hour() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 0, 0),
            ymd_hms(1997, 9, 9, 6, 0, 0),
            ymd_hms(1997, 9, 9, 18, 0, 0),
        ],
    );
}

#[test]
fn weekly_by_minute() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![9],
        by_minute: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 6, 0),
            ymd_hms(1997, 9, 2, 9, 18, 0),
            ymd_hms(1997, 9, 9, 9, 6, 0),
        ],
    );
}

#[test]
fn weekly_by_second() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 6),
            ymd_hms(1997, 9, 2, 9, 0, 18),
            ymd_hms(1997, 9, 9, 9, 0, 6),
        ],
    );
}

#[test]
fn weekly_by_hour_and_minute() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 6, 0),
            ymd_hms(1997, 9, 2, 18, 18, 0),
            ymd_hms(1997, 9, 9, 6, 6, 0),
        ],
    );
}

#[test]
fn weekly_by_hour_and_second() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 0, 6),
            ymd_hms(1997, 9, 2, 18, 0, 18),
            ymd_hms(1997, 9, 9, 6, 0, 6),
        ],
    );
}

#[test]
fn weekly_by_minute_and_second() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![9],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 6, 6),
            ymd_hms(1997, 9, 2, 9, 6, 18),
            ymd_hms(1997, 9, 2, 9, 18, 6),
        ],
    );
}

#[test]
fn weekly_by_hour_and_minute_and_second() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(5),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 6, 6),
            ymd_hms(1997, 9, 2, 18, 6, 18),
            ymd_hms(1997, 9, 2, 18, 18, 6),
            ymd_hms(1997, 9, 2, 18, 18, 18),
            ymd_hms(1997, 9, 9, 6, 6, 6),
        ],
    );
}

#[test]
fn weekly_by_setpos() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![6, 18],
        by_set_pos: vec![3, -3],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 0, 0),
            ymd_hms(1997, 9, 4, 6, 0, 0),
            ymd_hms(1997, 9, 9, 18, 0, 0),
        ],
    );
}

#[test]
fn daily() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_interval() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 6, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_interval_large() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 92,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 12, 3, 9, 0, 0),
            ymd_hms(1998, 3, 5, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_by_month() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_month: vec![1, 3],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 1, 2, 9, 0, 0),
            ymd_hms(1998, 1, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_by_monthday() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 10, 1, 9, 0, 0),
            ymd_hms(1997, 10, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_by_month_and_monthday() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_month: vec![1, 3],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![5, 7],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 5, 9, 0, 0),
            ymd_hms(1998, 1, 7, 9, 0, 0),
            ymd_hms(1998, 3, 5, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_by_weekday() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_by_month_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 8, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_by_monthday_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 2, 3, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_by_month_and_monthday_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
            ymd_hms(2001, 3, 1, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_by_hour() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 0, 0),
            ymd_hms(1997, 9, 3, 6, 0, 0),
            ymd_hms(1997, 9, 3, 18, 0, 0),
        ],
    );
}

#[test]
fn daily_by_minute() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 6, 0),
            ymd_hms(1997, 9, 2, 9, 18, 0),
            ymd_hms(1997, 9, 3, 9, 6, 0),
        ],
    );
}

#[test]
fn daily_by_second() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 6),
            ymd_hms(1997, 9, 2, 9, 0, 18),
            ymd_hms(1997, 9, 3, 9, 0, 6),
        ],
    );
}

#[test]
fn daily_by_hour_and_minute() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 6, 0),
            ymd_hms(1997, 9, 2, 18, 18, 0),
            ymd_hms(1997, 9, 3, 6, 6, 0),
        ],
    );
}

#[test]
fn daily_by_hour_and_second() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 0, 6),
            ymd_hms(1997, 9, 2, 18, 0, 18),
            ymd_hms(1997, 9, 3, 6, 0, 6),
        ],
    );
}

#[test]
fn daily_by_minute_and_second() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_hour: vec![9],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 6, 6),
            ymd_hms(1997, 9, 2, 9, 6, 18),
            ymd_hms(1997, 9, 2, 9, 18, 6),
        ],
    );
}

#[test]
fn daily_by_hour_and_minute_and_second() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 6, 6),
            ymd_hms(1997, 9, 2, 18, 6, 18),
            ymd_hms(1997, 9, 2, 18, 18, 6),
        ],
    );
}

#[test]
fn daily_by_setpos() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        by_hour: vec![6, 18],
        by_set_pos: vec![3, -3],
        by_minute: vec![15, 45],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 15, 0),
            ymd_hms(1997, 9, 3, 6, 45, 0),
            ymd_hms(1997, 9, 3, 18, 15, 0),
        ],
    );
}

#[test]
fn hourly() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(3),
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 10, 0, 0),
            ymd_hms(1997, 9, 2, 11, 0, 0),
        ],
    );
}

#[test]
fn hourly_interval() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(3),
        by_minute: vec![0],
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 11, 0, 0),
            ymd_hms(1997, 9, 2, 13, 0, 0),
        ],
    );
}

#[test]
fn hourly_interval_large() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(3),
        by_minute: vec![0],
        by_second: vec![0],
        interval: 769,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 10, 4, 10, 0, 0),
            ymd_hms(1997, 11, 5, 11, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_month() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(3),
        by_month: vec![1, 3],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 1, 0, 0),
            ymd_hms(1998, 1, 1, 2, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_monthday() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(3),
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 3, 0, 0, 0),
            ymd_hms(1997, 9, 3, 1, 0, 0),
            ymd_hms(1997, 9, 3, 2, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_month_and_monthday() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(3),
        by_month: vec![1, 3],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![5, 7],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 5, 0, 0, 0),
            ymd_hms(1998, 1, 5, 1, 0, 0),
            ymd_hms(1998, 1, 5, 2, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_weekday() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(5),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 5,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 14, 0, 0),
            ymd_hms(1997, 9, 2, 19, 0, 0),
            ymd_hms(1997, 9, 4, 1, 0, 0),
            ymd_hms(1997, 9, 4, 6, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_month_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 1, 0, 0),
            ymd_hms(1998, 1, 1, 2, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_monthday_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 1, 0, 0),
            ymd_hms(1998, 1, 1, 2, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_month_and_monthday_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 1, 0, 0),
            ymd_hms(1998, 1, 1, 2, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_yearday() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(8),
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        interval: 12,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 12, 31, 9, 0, 0),
            ymd_hms(1997, 12, 31, 21, 0, 0),
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 1, 1, 21, 0, 0),
            ymd_hms(1998, 4, 10, 9, 0, 0),
            ymd_hms(1998, 4, 10, 21, 0, 0),
            ymd_hms(1998, 7, 19, 9, 0, 0),
            ymd_hms(1998, 7, 19, 21, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_yeardayneg() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(8),
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![-365, -266, -166, -1],
        interval: 12,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 12, 31, 9, 0, 0),
            ymd_hms(1997, 12, 31, 21, 0, 0),
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 1, 1, 21, 0, 0),
            ymd_hms(1998, 4, 10, 9, 0, 0),
            ymd_hms(1998, 4, 10, 21, 0, 0),
            ymd_hms(1998, 7, 19, 9, 0, 0),
            ymd_hms(1998, 7, 19, 21, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_month_and_yearday() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(4),
        by_month: vec![4, 7],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 4, 10, 0, 0, 0),
            ymd_hms(1998, 4, 10, 1, 0, 0),
            ymd_hms(1998, 4, 10, 2, 0, 0),
            ymd_hms(1998, 4, 10, 3, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_hour() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(3),
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 0, 0),
            ymd_hms(1997, 9, 3, 6, 0, 0),
            ymd_hms(1997, 9, 3, 18, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_minute() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(3),
        by_minute: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 6, 0),
            ymd_hms(1997, 9, 2, 9, 18, 0),
            ymd_hms(1997, 9, 2, 10, 6, 0),
        ],
    );
}

#[test]
fn hourly_by_second() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(3),
        by_minute: vec![0],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 6),
            ymd_hms(1997, 9, 2, 9, 0, 18),
            ymd_hms(1997, 9, 2, 10, 0, 6),
        ],
    );
}

#[test]
fn hourly_by_hour_and_minute() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(3),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 6, 0),
            ymd_hms(1997, 9, 2, 18, 18, 0),
            ymd_hms(1997, 9, 3, 6, 6, 0),
        ],
    );
}

#[test]
fn hourly_by_hour_and_second() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(3),
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 0, 6),
            ymd_hms(1997, 9, 2, 18, 0, 18),
            ymd_hms(1997, 9, 3, 6, 0, 6),
        ],
    );
}

#[test]
fn hourly_by_minute_and_second() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(3),
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 6, 6),
            ymd_hms(1997, 9, 2, 9, 6, 18),
            ymd_hms(1997, 9, 2, 9, 18, 6),
        ],
    );
}

#[test]
fn hourly_by_hour_and_minute_and_second() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(5),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 6, 6),
            ymd_hms(1997, 9, 2, 18, 6, 18),
            ymd_hms(1997, 9, 2, 18, 18, 6),
            ymd_hms(1997, 9, 2, 18, 18, 18),
            ymd_hms(1997, 9, 3, 6, 6, 6),
        ],
    );
}

#[test]
fn hourly_by_setpos() {
    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(3),
        by_set_pos: vec![3, -3],
        by_minute: vec![15, 45],
        by_second: vec![15, 45],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 15, 45),
            ymd_hms(1997, 9, 2, 9, 45, 15),
            ymd_hms(1997, 9, 2, 10, 15, 45),
        ],
    );
}

#[test]
fn minutely() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 9, 1, 0),
            ymd_hms(1997, 9, 2, 9, 2, 0),
        ],
    );
}

#[test]
fn minutely_interval() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 9, 2, 0),
            ymd_hms(1997, 9, 2, 9, 4, 0),
        ],
    );
}

#[test]
fn minutely_interval_large() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_second: vec![0],
        interval: 1501,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 3, 10, 1, 0),
            ymd_hms(1997, 9, 4, 11, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_month() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_month: vec![1, 3],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 0, 1, 0),
            ymd_hms(1998, 1, 1, 0, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_monthday() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 3, 0, 0, 0),
            ymd_hms(1997, 9, 3, 0, 1, 0),
            ymd_hms(1997, 9, 3, 0, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_month_and_monthday() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_month: vec![1, 3],
        by_second: vec![0],
        by_month_day: vec![5, 7],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 5, 0, 0, 0),
            ymd_hms(1998, 1, 5, 0, 1, 0),
            ymd_hms(1998, 1, 5, 0, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_weekday() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Thu), NWeekday::Every(Weekday::Sat)],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 4, 0, 0, 0),
            ymd_hms(1997, 9, 4, 0, 1, 0),
            ymd_hms(1997, 9, 4, 0, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_month_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 0, 1, 0),
            ymd_hms(1998, 1, 1, 0, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_monthday_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 0, 1, 0),
            ymd_hms(1998, 1, 1, 0, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_month_and_monthday_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 0, 1, 0),
            ymd_hms(1998, 1, 1, 0, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_yearday() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(4),
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 12, 31, 0, 0, 0),
            ymd_hms(1997, 12, 31, 0, 1, 0),
            ymd_hms(1997, 12, 31, 0, 2, 0),
            ymd_hms(1997, 12, 31, 0, 3, 0),
        ],
    );
}

#[test]
fn minutely_by_yeardayneg() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(4),
        by_second: vec![0],
        by_year_day: vec![-365, -266, -166, -1],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 12, 31, 0, 0, 0),
            ymd_hms(1997, 12, 31, 0, 1, 0),
            ymd_hms(1997, 12, 31, 0, 2, 0),
            ymd_hms(1997, 12, 31, 0, 3, 0),
        ],
    );
}

#[test]
fn minutely_by_month_and_yearday() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(4),
        by_month: vec![4, 7],
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 4, 10, 0, 0, 0),
            ymd_hms(1998, 4, 10, 0, 1, 0),
            ymd_hms(1998, 4, 10, 0, 2, 0),
            ymd_hms(1998, 4, 10, 0, 3, 0),
        ],
    );
}

#[test]
fn minutely_by_hour() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_hour: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 0, 0),
            ymd_hms(1997, 9, 2, 18, 1, 0),
            ymd_hms(1997, 9, 2, 18, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_minute() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_minute: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 6, 0),
            ymd_hms(1997, 9, 2, 9, 18, 0),
            ymd_hms(1997, 9, 2, 10, 6, 0),
        ],
    );
}

#[test]
fn minutely_by_second() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 6),
            ymd_hms(1997, 9, 2, 9, 0, 18),
            ymd_hms(1997, 9, 2, 9, 1, 6),
        ],
    );
}

#[test]
fn minutely_by_hour_and_minute() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 6, 0),
            ymd_hms(1997, 9, 2, 18, 18, 0),
            ymd_hms(1997, 9, 3, 6, 6, 0),
        ],
    );
}

#[test]
fn minutely_by_hour_and_second() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_hour: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 0, 6),
            ymd_hms(1997, 9, 2, 18, 0, 18),
            ymd_hms(1997, 9, 2, 18, 1, 6),
        ],
    );
}

#[test]
fn minutely_by_minute_and_second() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 6, 6),
            ymd_hms(1997, 9, 2, 9, 6, 18),
            ymd_hms(1997, 9, 2, 9, 18, 6),
        ],
    );
}

#[test]
fn minutely_by_hour_and_minute_and_second() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(5),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 6, 6),
            ymd_hms(1997, 9, 2, 18, 6, 18),
            ymd_hms(1997, 9, 2, 18, 18, 6),
            ymd_hms(1997, 9, 2, 18, 18, 18),
            ymd_hms(1997, 9, 3, 6, 6, 6),
        ],
    );
}

#[test]
fn minutely_by_setpos() {
    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(3),
        by_set_pos: vec![3, -3],
        by_second: vec![15, 30, 45],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 15),
            ymd_hms(1997, 9, 2, 9, 0, 45),
            ymd_hms(1997, 9, 2, 9, 1, 15),
        ],
    );
}

#[test]
fn secondly() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 9, 0, 1),
            ymd_hms(1997, 9, 2, 9, 0, 2),
        ],
    );
}

#[test]
fn secondly_interval() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 9, 0, 2),
            ymd_hms(1997, 9, 2, 9, 0, 4),
        ],
    );
}

#[test]
fn secondly_interval_large_under_limit() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        interval: 50000,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 22, 53, 20),
            ymd_hms(1997, 9, 3, 12, 46, 40),
        ],
    );
}

#[test]
fn secondly_interval_large() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        interval: 60061,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        false,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 3, 1, 41, 1),
            ymd_hms(1997, 9, 3, 18, 22, 2),
        ],
    );
}

#[test]
fn secondly_by_month() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        by_month: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 0, 0, 1),
            ymd_hms(1998, 1, 1, 0, 0, 2),
        ],
    );
}

#[test]
fn secondly_by_monthday() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 3, 0, 0, 0),
            ymd_hms(1997, 9, 3, 0, 0, 1),
            ymd_hms(1997, 9, 3, 0, 0, 2),
        ],
    );
}

#[test]
fn secondly_by_month_and_monthday() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        by_month: vec![1, 3],
        by_month_day: vec![5, 7],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 5, 0, 0, 0),
            ymd_hms(1998, 1, 5, 0, 0, 1),
            ymd_hms(1998, 1, 5, 0, 0, 2),
        ],
    );
}

#[test]
fn secondly_by_weekday() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Thu), NWeekday::Every(Weekday::Sat)],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 4, 0, 0, 0),
            ymd_hms(1997, 9, 4, 0, 0, 1),
            ymd_hms(1997, 9, 4, 0, 0, 2),
        ],
    );
}

#[test]
fn secondly_by_month_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 0, 0, 1),
            ymd_hms(1998, 1, 1, 0, 0, 2),
        ],
    );
}

#[test]
fn secondly_by_monthday_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 0, 0, 1),
            ymd_hms(1998, 1, 1, 0, 0, 2),
        ],
    );
}

#[test]
fn secondly_by_month_and_monthday_and_weekday() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        by_month: vec![1, 3],
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 0, 0, 1),
            ymd_hms(1998, 1, 1, 0, 0, 2),
        ],
    );
}

#[test]
fn secondly_by_yearday() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(4),
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 12, 31, 0, 0, 0),
            ymd_hms(1997, 12, 31, 0, 0, 1),
            ymd_hms(1997, 12, 31, 0, 0, 2),
            ymd_hms(1997, 12, 31, 0, 0, 3),
        ],
    );
}

#[test]
fn secondly_by_yeardayneg() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(4),
        by_year_day: vec![-365, -266, -166, -1],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 12, 31, 0, 0, 0),
            ymd_hms(1997, 12, 31, 0, 0, 1),
            ymd_hms(1997, 12, 31, 0, 0, 2),
            ymd_hms(1997, 12, 31, 0, 0, 3),
        ],
    );
}

#[test]
fn secondly_by_month_and_yearday() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(4),
        by_month: vec![4, 7],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1998, 4, 10, 0, 0, 0),
            ymd_hms(1998, 4, 10, 0, 0, 1),
            ymd_hms(1998, 4, 10, 0, 0, 2),
            ymd_hms(1998, 4, 10, 0, 0, 3),
        ],
    );
}

#[test]
fn secondly_by_hour() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        by_hour: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        false,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 0, 0),
            ymd_hms(1997, 9, 2, 18, 0, 1),
            ymd_hms(1997, 9, 2, 18, 0, 2),
        ],
    );
}

#[test]
fn secondly_by_minute() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        by_minute: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 6, 0),
            ymd_hms(1997, 9, 2, 9, 6, 1),
            ymd_hms(1997, 9, 2, 9, 6, 2),
        ],
    );
}

#[test]
fn secondly_by_second() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 6),
            ymd_hms(1997, 9, 2, 9, 0, 18),
            ymd_hms(1997, 9, 2, 9, 1, 6),
        ],
    );
}

#[test]
fn secondly_by_hour_and_minute() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        false,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 6, 0),
            ymd_hms(1997, 9, 2, 18, 6, 1),
            ymd_hms(1997, 9, 2, 18, 6, 2),
        ],
    );
}

#[test]
fn secondly_by_hour_and_second() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        by_hour: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        false,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 0, 6),
            ymd_hms(1997, 9, 2, 18, 0, 18),
            ymd_hms(1997, 9, 2, 18, 1, 6),
        ],
    );
}

#[test]
fn secondly_by_minute_and_second() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(3),
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 6, 6),
            ymd_hms(1997, 9, 2, 9, 6, 18),
            ymd_hms(1997, 9, 2, 9, 18, 6),
        ],
    );
}

#[test]
fn secondly_by_hour_and_minute_and_second() {
    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(5),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        false,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 18, 6, 6),
            ymd_hms(1997, 9, 2, 18, 6, 18),
            ymd_hms(1997, 9, 2, 18, 18, 6),
            ymd_hms(1997, 9, 2, 18, 18, 18),
            ymd_hms(1997, 9, 3, 6, 6, 6),
        ],
    );
}

#[test]
fn until_not_matching() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(3),
        until: Some(ymd_hms(1997, 9, 5, 8, 0, 0)),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
        ],
    );
}

#[test]
fn until_matching() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(4),
        until: Some(ymd_hms(1997, 9, 4, 9, 0, 0)),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
        ],
    );
}

#[test]
fn until_single() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(1),
        until: Some(ymd_hms(1997, 9, 2, 9, 0, 0)),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[ymd_hms(1997, 9, 2, 9, 0, 0)],
    );
}

#[test]
fn until_with_date() {
    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(4),
        until: Some(ymd_hms(1997, 9, 5, 0, 0, 0)),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
        ],
    );
}

#[test]
fn week_start_interval_mo() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Sun)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 7, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
        ],
    );
}

#[test]
fn week_start_interval_su() {
    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(3),
        week_start: Weekday::Sun,
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Sun)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        rrule,
        true,
        ymd_hms(1997, 9, 2, 9, 0, 0),
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 14, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
        ],
    );
}

#[test]
fn test_timezones_weekly() {
    use chrono::Weekday::Sat;

    const LOS_ANGELES: Tz = Tz::Tz(chrono_tz::Tz::America__Los_Angeles);
    const NEW_YORK: Tz = Tz::Tz(chrono_tz::Tz::America__New_York);
    const BERLIN: Tz = Tz::Tz(chrono_tz::Tz::Europe__Berlin);

    let rrule = RRule::default()
        .count(2)
        .freq(Frequency::Weekly)
        .by_weekday(vec![NWeekday::Every(Sat)]);
    let rrule_set = rrule.build(ymd_hms(2021, 1, 1, 9, 0, 0)).unwrap();
    for o in &rrule_set {
        assert_eq!(o.weekday(), Sat);
    }

    // NYC (-5)
    let rrule = RRule::default()
        .count(1)
        .freq(Frequency::Weekly)
        .by_weekday(vec![NWeekday::Every(Sat)]);
    let rrule_set = rrule
        .build(NEW_YORK.with_ymd_and_hms(2021, 1, 1, 9, 0, 0).unwrap())
        .unwrap();
    for o in &rrule_set {
        assert_eq!(o.weekday(), Sat);
    }

    // How about Berlin (+1)
    let rrule = RRule::default()
        .count(1)
        .freq(Frequency::Weekly)
        .by_weekday(vec![NWeekday::Every(Sat)]);
    let rrule_set = rrule
        .build(BERLIN.with_ymd_and_hms(2021, 1, 1, 9, 0, 0).unwrap())
        .unwrap();
    for o in &rrule_set {
        assert_eq!(o.weekday(), Sat);
    }

    // Los Angeles (-7)
    let rrule = RRule::default()
        .count(1)
        .freq(Frequency::Weekly)
        .by_weekday(vec![NWeekday::Every(Sat)]);
    let rrule_set = rrule
        .build(LOS_ANGELES.with_ymd_and_hms(2021, 1, 1, 9, 0, 0).unwrap())
        .unwrap();
    for o in &rrule_set {
        assert_eq!(o.weekday(), Sat);
    }
}

#[test]
fn test_before_inclusive_hit() {
    let rrule: RRuleSet = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3"
        .parse()
        .unwrap();

    let before = ymd_hms(2012, 2, 2, 9, 30, 0);
    let rrule = rrule.before(before);

    assert_eq!(Some(&before), rrule.all_unchecked().last());
}

#[test]
fn test_before_inclusive_miss() {
    let rrule: RRuleSet = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3"
        .parse()
        .unwrap();

    let before = ymd_hms(2012, 2, 3, 9, 0, 0);
    let rrule = rrule.before(before);
    let oracle = ymd_hms(2012, 2, 2, 9, 30, 0);

    assert_eq!(Some(&oracle), rrule.all_unchecked().last());
}

#[test]
fn test_after_inclusive_hit() {
    let rrule: RRuleSet = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3"
        .parse()
        .unwrap();

    let after = ymd_hms(2012, 2, 2, 9, 30, 0);
    let rrule = rrule.after(after);

    assert_eq!(after, rrule.all(1).dates[0]);
}

#[test]
fn test_after_inclusive_miss() {
    let rrule: RRuleSet = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3"
        .parse()
        .unwrap();

    let after = ymd_hms(2012, 2, 2, 10, 0, 0);
    let rrule = rrule.after(after);
    let oracle = ymd_hms(2012, 2, 3, 9, 30, 0);

    assert_eq!(oracle, rrule.all(1).dates[0]);
}

#[test]
fn test_between_inclusive_both_miss() {
    let rrule: RRuleSet = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=5"
        .parse()
        .unwrap();

    let after = ymd_hms(2012, 2, 2, 10, 0, 0);
    let middle = ymd_hms(2012, 2, 3, 9, 30, 0);
    let before = ymd_hms(2012, 2, 4, 9, 0, 0);

    let rrule = rrule.before(before).after(after);

    assert_eq!(vec![middle], rrule.all_unchecked());
}

#[test]
fn test_between_inclusive_lower_miss() {
    let rrule: RRuleSet = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=5"
        .parse()
        .unwrap();

    let after = ymd_hms(2012, 2, 2, 10, 0, 0);
    let middle = ymd_hms(2012, 2, 3, 9, 30, 0);
    let before = ymd_hms(2012, 2, 4, 9, 30, 0);

    let rrule = rrule.before(before).after(after);

    assert_eq!(vec![middle, before], rrule.all_unchecked());
}

#[test]
fn test_between_inclusive_upper_miss() {
    let rrule: RRuleSet = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=5"
        .parse()
        .unwrap();

    let after = ymd_hms(2012, 2, 2, 9, 30, 0);
    let middle = ymd_hms(2012, 2, 3, 9, 30, 0);
    let before = ymd_hms(2012, 2, 4, 9, 0, 0);

    let rrule = rrule.before(before).after(after);

    assert_eq!(vec![after, middle], rrule.all_unchecked());
}

#[test]
fn test_between_inclusive_both_hit() {
    let rrule: RRuleSet = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=5"
        .parse()
        .unwrap();

    let after = ymd_hms(2012, 2, 2, 9, 30, 0);
    let middle = ymd_hms(2012, 2, 3, 9, 30, 0);
    let before = ymd_hms(2012, 2, 4, 9, 30, 0);

    let rrule = rrule.before(before).after(after);

    assert_eq!(vec![after, middle, before], rrule.all_unchecked());
}
