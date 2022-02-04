mod common;

use chrono::{Datelike, TimeZone};
use chrono_tz::UTC;
use common::{test_recurring_rrule, ymd_hms};
use rrule::{DateFilter, Frequency, NWeekday, RRule, RRuleProperties, Weekday};

#[test]
fn yearly() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1998, 9, 2, 9, 0, 0),
            ymd_hms(1999, 9, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_interval() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1999, 9, 2, 9, 0, 0),
            ymd_hms(2001, 9, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_interval_large() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        interval: 40,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_easter: Some(0),
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_easter: Some(1),
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_easter: Some(-2),
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 4, 10, 9, 0, 0),
            ymd_hms(1999, 4, 2, 9, 0, 0),
            ymd_hms(2000, 4, 21, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_month() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 2, 9, 0, 0),
            ymd_hms(1998, 3, 2, 9, 0, 0),
            ymd_hms(1999, 1, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_monthday() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 10, 1, 9, 0, 0),
            ymd_hms(1997, 10, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_month_and_monthday() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![5, 7],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 5, 9, 0, 0),
            ymd_hms(1998, 1, 7, 9, 0, 0),
            ymd_hms(1998, 3, 5, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_nweekday() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        properties,
        &[
            ymd_hms(1997, 12, 25, 9, 0, 0),
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 12, 31, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_nweekday_large() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        properties,
        &[
            ymd_hms(1997, 10, 2, 9, 0, 0),
            ymd_hms(1998, 3, 31, 9, 0, 0),
            ymd_hms(1998, 10, 8, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_month_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 8, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_month_and_nweekday() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        properties,
        &[
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 29, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_month_and_nweekday_large() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        properties,
        &[
            ymd_hms(1998, 1, 15, 9, 0, 0),
            ymd_hms(1998, 1, 20, 9, 0, 0),
            ymd_hms(1998, 3, 12, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_monthday_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 2, 3, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_month_and_monthday_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
            ymd_hms(2001, 3, 1, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_yearday() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(4),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(4),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![-365, -266, -166, -1],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(4),
        by_month: vec![4, 7],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_week_no: vec![20],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 5, 11, 9, 0, 0),
            ymd_hms(1998, 5, 12, 9, 0, 0),
            ymd_hms(1998, 5, 13, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_weekno_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        by_hour: vec![9],
        by_week_no: vec![1],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 29, 9, 0, 0),
            ymd_hms(1999, 1, 4, 9, 0, 0),
            ymd_hms(2000, 1, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_weekno_and_weekday_large() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_hour: vec![9],
        by_week_no: vec![52],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 28, 9, 0, 0),
            ymd_hms(1998, 12, 27, 9, 0, 0),
            ymd_hms(2000, 1, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_weekno_and_weekday_last() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_hour: vec![9],
        by_week_no: vec![-1],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 28, 9, 0, 0),
            ymd_hms(1999, 1, 3, 9, 0, 0),
            ymd_hms(2000, 1, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_weekno_and_weekday53_last() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        by_hour: vec![9],
        by_week_no: vec![53],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 12, 28, 9, 0, 0),
            ymd_hms(2004, 12, 27, 9, 0, 0),
            ymd_hms(2009, 12, 28, 9, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_hour() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 0, 0),
            ymd_hms(1998, 9, 2, 6, 0, 0),
            ymd_hms(1998, 9, 2, 18, 0, 0),
        ],
    );
}

#[test]
fn yearly_by_minute() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![6, 18],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 6, 0),
            ymd_hms(1997, 9, 2, 9, 18, 0),
            ymd_hms(1998, 9, 2, 9, 6, 0),
        ],
    );
}

#[test]
fn yearly_by_second() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![6, 18],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 6),
            ymd_hms(1997, 9, 2, 9, 0, 18),
            ymd_hms(1998, 9, 2, 9, 0, 6),
        ],
    );
}

#[test]
fn yearly_by_hour_and_minute() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 6, 0),
            ymd_hms(1997, 9, 2, 18, 18, 0),
            ymd_hms(1998, 9, 2, 6, 6, 0),
        ],
    );
}

#[test]
fn yearly_by_hour_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![6, 18],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 0, 6),
            ymd_hms(1997, 9, 2, 18, 0, 18),
            ymd_hms(1998, 9, 2, 6, 0, 6),
        ],
    );
}

#[test]
fn yearly_by_minute_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 6, 6),
            ymd_hms(1997, 9, 2, 9, 6, 18),
            ymd_hms(1997, 9, 2, 9, 18, 6),
        ],
    );
}

#[test]
fn yearly_by_hour_and_minute_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![9],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 6, 6),
            ymd_hms(1997, 9, 2, 18, 6, 18),
            ymd_hms(1997, 9, 2, 18, 18, 6),
        ],
    );
}

#[test]
fn yearly_by_setpos() {
    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_set_pos: vec![3, -3],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![15],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 11, 15, 18, 0, 0),
            ymd_hms(1998, 2, 15, 6, 0, 0),
            ymd_hms(1998, 11, 15, 18, 0, 0),
        ],
    );
}

#[test]
fn monthly() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 10, 2, 9, 0, 0),
            ymd_hms(1997, 11, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_interval() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 11, 2, 9, 0, 0),
            ymd_hms(1998, 1, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_interval_large() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        interval: 18,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_easter: Some(0),
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_easter: Some(1),
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_easter: Some(-2),
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 4, 10, 9, 0, 0),
            ymd_hms(1999, 4, 2, 9, 0, 0),
            ymd_hms(2000, 4, 21, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_neg_by_monthday_janfeb_for_nonleapyear() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(4),
        dt_start: ymd_hms(2013, 12, 1, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_n_month_day: vec![-1],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(4),
        dt_start: ymd_hms(2015, 12, 1, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_n_month_day: vec![-1],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(6),
        dt_start: ymd_hms(2015, 12, 1, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_n_month_day: vec![-1, -3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 2, 9, 0, 0),
            ymd_hms(1998, 3, 2, 9, 0, 0),
            ymd_hms(1999, 1, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_monthday() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 10, 1, 9, 0, 0),
            ymd_hms(1997, 10, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_month_and_monthday() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![5, 7],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 5, 9, 0, 0),
            ymd_hms(1998, 1, 7, 9, 0, 0),
            ymd_hms(1998, 3, 5, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_nweekday() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 25, 9, 0, 0),
            ymd_hms(1997, 10, 7, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_nweekday_large() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        properties,
        &[
            ymd_hms(1997, 9, 11, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 10, 16, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_month_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 8, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_month_and_nweekday() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        properties,
        &[
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 29, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_month_and_nweekday_large() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
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
        properties,
        &[
            ymd_hms(1998, 1, 15, 9, 0, 0),
            ymd_hms(1998, 1, 20, 9, 0, 0),
            ymd_hms(1998, 3, 12, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_monthday_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 2, 3, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_month_and_monthday_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
            ymd_hms(2001, 3, 1, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYYEARDAY` can not be used with the current frequency (monthly)."]
fn monthly_by_yearday() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(4),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 31, 9, 0, 0),
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 4, 10, 9, 0, 0),
            ymd_hms(1998, 7, 19, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYYEARDAY` can not be used with the current frequency (monthly)."]
fn monthly_by_yeardayneg() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(4),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![-365, -266, -166, -1],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 31, 9, 0, 0),
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 4, 10, 9, 0, 0),
            ymd_hms(1998, 7, 19, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYYEARDAY` can not be used with the current frequency (monthly)."]
fn monthly_by_month_and_yearday() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(4),
        by_month: vec![4, 7],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 4, 10, 9, 0, 0),
            ymd_hms(1998, 7, 19, 9, 0, 0),
            ymd_hms(1999, 4, 10, 9, 0, 0),
            ymd_hms(1999, 7, 19, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (monthly)."]
fn monthly_by_weekno() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_week_no: vec![20],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 5, 11, 9, 0, 0),
            ymd_hms(1998, 5, 12, 9, 0, 0),
            ymd_hms(1998, 5, 13, 9, 0, 0),
        ],
    );
}

// That's a nice one. The first days of week number one
// may be in the last year.
#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (monthly)."]
fn monthly_by_weekno_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        by_hour: vec![9],
        by_week_no: vec![1],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 29, 9, 0, 0),
            ymd_hms(1999, 1, 4, 9, 0, 0),
            ymd_hms(2000, 1, 3, 9, 0, 0),
        ],
    );
}

// Another nice test. The last days of week number 52/53
// may be in the next year.
#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (monthly)."]
fn monthly_by_weekno_and_weekday_large() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_hour: vec![9],
        by_week_no: vec![52],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 28, 9, 0, 0),
            ymd_hms(1998, 12, 27, 9, 0, 0),
            ymd_hms(2000, 1, 2, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (monthly)."]
fn monthly_by_weekno_and_weekday_last() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_hour: vec![9],
        by_week_no: vec![-1],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 28, 9, 0, 0),
            ymd_hms(1999, 1, 3, 9, 0, 0),
            ymd_hms(2000, 1, 2, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (monthly)."]
fn monthly_by_weekno_and_weekday53() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        by_hour: vec![9],
        by_week_no: vec![53],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 12, 28, 9, 0, 0),
            ymd_hms(2004, 12, 27, 9, 0, 0),
            ymd_hms(2009, 12, 28, 9, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_hour() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 0, 0),
            ymd_hms(1997, 10, 2, 6, 0, 0),
            ymd_hms(1997, 10, 2, 18, 0, 0),
        ],
    );
}

#[test]
fn monthly_by_minute() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![6, 18],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 6, 0),
            ymd_hms(1997, 9, 2, 9, 18, 0),
            ymd_hms(1997, 10, 2, 9, 6, 0),
        ],
    );
}

#[test]
fn monthly_by_second() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![6, 18],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 6),
            ymd_hms(1997, 9, 2, 9, 0, 18),
            ymd_hms(1997, 10, 2, 9, 0, 6),
        ],
    );
}

#[test]
fn monthly_by_hour_and_minute() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 6, 0),
            ymd_hms(1997, 9, 2, 18, 18, 0),
            ymd_hms(1997, 10, 2, 6, 6, 0),
        ],
    );
}

#[test]
fn monthly_by_hour_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![6, 18],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 0, 6),
            ymd_hms(1997, 9, 2, 18, 0, 18),
            ymd_hms(1997, 10, 2, 6, 0, 6),
        ],
    );
}

#[test]
fn monthly_by_minute_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 6, 6),
            ymd_hms(1997, 9, 2, 9, 6, 18),
            ymd_hms(1997, 9, 2, 9, 18, 6),
        ],
    );
}

#[test]
fn monthly_by_hour_and_minute_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        by_month_day: vec![2],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 6, 6),
            ymd_hms(1997, 9, 2, 18, 6, 18),
            ymd_hms(1997, 9, 2, 18, 18, 6),
        ],
    );
}

#[test]
fn monthly_by_setpos() {
    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_set_pos: vec![3, -3],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![13, 17],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 13, 18, 0, 0),
            ymd_hms(1997, 9, 17, 6, 0, 0),
            ymd_hms(1997, 10, 13, 18, 0, 0),
        ],
    );
}

#[test]
fn weekly() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
        ],
    );
}

#[test]
fn weekly_interval() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 9, 30, 9, 0, 0),
        ],
    );
}

#[test]
fn weekly_interval_large() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 20,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1998, 1, 20, 9, 0, 0),
            ymd_hms(1998, 6, 9, 9, 0, 0),
        ],
    );
}

#[test]
fn weekly_by_month() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(6),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
#[ignore = "`BYMONTHDAY` can not be used with the current frequency (weekly)."]
fn weekly_by_monthday() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 10, 1, 9, 0, 0),
            ymd_hms(1997, 10, 3, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYMONTHDAY` can not be used with the current frequency (weekly)."]
fn weekly_by_month_and_monthday() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![5, 7],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 5, 9, 0, 0),
            ymd_hms(1998, 1, 7, 9, 0, 0),
            ymd_hms(1998, 3, 5, 9, 0, 0),
        ],
    );
}

#[test]
fn weekly_by_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
        ],
    );
}

// ! why isn't this using nweekday ???
#[test]
fn weekly_by_nweekday() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
        ],
    );
}

#[test]
fn weekly_by_month_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 8, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYMONTHDAY` can not be used with the current frequency (weekly)."]
fn weekly_by_monthday_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 2, 3, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYMONTHDAY` can not be used with the current frequency (weekly)."]
fn weekly_by_month_monthday_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
            ymd_hms(2001, 3, 1, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYYEARDAY` can not be used with the current frequency (weekly)."]
fn weekly_by_yearday() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(4),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 31, 9, 0, 0),
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 4, 10, 9, 0, 0),
            ymd_hms(1998, 7, 19, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYYEARDAY` can not be used with the current frequency (weekly)."]
fn weekly_by_yeardayneg() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(4),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![-365, -266, -166, -1],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 31, 9, 0, 0),
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 4, 10, 9, 0, 0),
            ymd_hms(1998, 7, 19, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYYEARDAY` can not be used with the current frequency (weekly)."]
fn weekly_by_month_yearday() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(4),
        by_month: vec![1, 7],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 7, 19, 9, 0, 0),
            ymd_hms(1999, 1, 1, 9, 0, 0),
            ymd_hms(1999, 7, 19, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (weekly)."]
fn weekly_by_weekno() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_week_no: vec![20],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 5, 11, 9, 0, 0),
            ymd_hms(1998, 5, 12, 9, 0, 0),
            ymd_hms(1998, 5, 13, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (weekly)."]
fn weekly_by_weekno_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        by_hour: vec![9],
        by_week_no: vec![1],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 29, 9, 0, 0),
            ymd_hms(1999, 1, 4, 9, 0, 0),
            ymd_hms(2000, 1, 3, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (weekly)."]
fn weekly_by_weekno_and_weekday_large() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_hour: vec![9],
        by_week_no: vec![52],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 28, 9, 0, 0),
            ymd_hms(1998, 12, 27, 9, 0, 0),
            ymd_hms(2000, 1, 2, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (weekly)."]
fn weekly_by_weekno_and_weekday_last() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_hour: vec![9],
        by_week_no: vec![-1],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 28, 9, 0, 0),
            ymd_hms(1999, 1, 3, 9, 0, 0),
            ymd_hms(2000, 1, 2, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (weekly)."]
fn weekly_by_weekno_and_weekday53() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        by_hour: vec![9],
        by_week_no: vec![53],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 12, 28, 9, 0, 0),
            ymd_hms(2004, 12, 27, 9, 0, 0),
            ymd_hms(2009, 12, 28, 9, 0, 0),
        ],
    );
}

#[test]
fn weekly_by_hour() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 0, 0),
            ymd_hms(1997, 9, 9, 6, 0, 0),
            ymd_hms(1997, 9, 9, 18, 0, 0),
        ],
    );
}

#[test]
fn weekly_by_minute() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![9],
        by_minute: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 6, 0),
            ymd_hms(1997, 9, 2, 9, 18, 0),
            ymd_hms(1997, 9, 9, 9, 6, 0),
        ],
    );
}

#[test]
fn weekly_by_second() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 6),
            ymd_hms(1997, 9, 2, 9, 0, 18),
            ymd_hms(1997, 9, 9, 9, 0, 6),
        ],
    );
}

#[test]
fn weekly_by_hour_and_minute() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 6, 0),
            ymd_hms(1997, 9, 2, 18, 18, 0),
            ymd_hms(1997, 9, 9, 6, 6, 0),
        ],
    );
}

#[test]
fn weekly_by_hour_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 0, 6),
            ymd_hms(1997, 9, 2, 18, 0, 18),
            ymd_hms(1997, 9, 9, 6, 0, 6),
        ],
    );
}

#[test]
fn weekly_by_minute_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![9],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 6, 6),
            ymd_hms(1997, 9, 2, 9, 6, 18),
            ymd_hms(1997, 9, 2, 9, 18, 6),
        ],
    );
}

#[test]
fn weekly_by_hour_and_minute_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(5),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue)],
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![6, 18],
        by_set_pos: vec![3, -3],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 0, 0),
            ymd_hms(1997, 9, 4, 6, 0, 0),
            ymd_hms(1997, 9, 9, 18, 0, 0),
        ],
    );
}

#[test]
fn daily() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_interval() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 6, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_interval_large() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 92,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 12, 3, 9, 0, 0),
            ymd_hms(1998, 3, 5, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_by_month() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 1, 2, 9, 0, 0),
            ymd_hms(1998, 1, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_by_monthday() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 10, 1, 9, 0, 0),
            ymd_hms(1997, 10, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_by_month_and_monthday() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![5, 7],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 5, 9, 0, 0),
            ymd_hms(1998, 1, 7, 9, 0, 0),
            ymd_hms(1998, 3, 5, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_by_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_by_month_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 8, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_by_monthday_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 2, 3, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_by_month_and_monthday_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
            ymd_hms(2001, 3, 1, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYYEARDAY` can not be used with the current frequency (daily)."]
fn daily_by_yearday() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(4),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 31, 9, 0, 0),
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 4, 10, 9, 0, 0),
            ymd_hms(1998, 7, 19, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYYEARDAY` can not be used with the current frequency (daily)."]
fn daily_by_yeardayneg() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(4),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![-365, -266, -166, -1],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 31, 9, 0, 0),
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 4, 10, 9, 0, 0),
            ymd_hms(1998, 7, 19, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYYEARDAY` can not be used with the current frequency (daily)."]
fn daily_by_month_and_yearday() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(4),
        by_month: vec![1, 7],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 7, 19, 9, 0, 0),
            ymd_hms(1999, 1, 1, 9, 0, 0),
            ymd_hms(1999, 7, 19, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (daily)."]
fn daily_by_weekno() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_week_no: vec![20],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 5, 11, 9, 0, 0),
            ymd_hms(1998, 5, 12, 9, 0, 0),
            ymd_hms(1998, 5, 13, 9, 0, 0),
        ],
    );
}

// That's a nice one. The first days of week number one
// may be in the last year.
#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (daily)."]
fn daily_by_weekno_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        by_hour: vec![9],
        by_week_no: vec![1],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 29, 9, 0, 0),
            ymd_hms(1999, 1, 4, 9, 0, 0),
            ymd_hms(2000, 1, 3, 9, 0, 0),
        ],
    );
}

// Another nice test. The last days of week number 52/53
// may be in the next year.
#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (daily)."]
fn daily_by_weekno_and_weekday_large() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_hour: vec![9],
        by_week_no: vec![52],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 28, 9, 0, 0),
            ymd_hms(1998, 12, 27, 9, 0, 0),
            ymd_hms(2000, 1, 2, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (daily)."]
fn daily_by_weekno_and_weekday_last() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_hour: vec![9],
        by_week_no: vec![-1],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 28, 9, 0, 0),
            ymd_hms(1999, 1, 3, 9, 0, 0),
            ymd_hms(2000, 1, 2, 9, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (daily)."]
fn daily_by_weekno_and_weekday53() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_hour: vec![9],
        by_week_no: vec![53],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 12, 28, 9, 0, 0),
            ymd_hms(2004, 12, 27, 9, 0, 0),
            ymd_hms(2009, 12, 28, 9, 0, 0),
        ],
    );
}

#[test]
fn daily_by_hour() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 0, 0),
            ymd_hms(1997, 9, 3, 6, 0, 0),
            ymd_hms(1997, 9, 3, 18, 0, 0),
        ],
    );
}

#[test]
fn daily_by_minute() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 6, 0),
            ymd_hms(1997, 9, 2, 9, 18, 0),
            ymd_hms(1997, 9, 3, 9, 6, 0),
        ],
    );
}

#[test]
fn daily_by_second() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 6),
            ymd_hms(1997, 9, 2, 9, 0, 18),
            ymd_hms(1997, 9, 3, 9, 0, 6),
        ],
    );
}

#[test]
fn daily_by_hour_and_minute() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 6, 0),
            ymd_hms(1997, 9, 2, 18, 18, 0),
            ymd_hms(1997, 9, 3, 6, 6, 0),
        ],
    );
}

#[test]
fn daily_by_hour_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 0, 6),
            ymd_hms(1997, 9, 2, 18, 0, 18),
            ymd_hms(1997, 9, 3, 6, 0, 6),
        ],
    );
}

#[test]
fn daily_by_minute_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 6, 6),
            ymd_hms(1997, 9, 2, 9, 6, 18),
            ymd_hms(1997, 9, 2, 9, 18, 6),
        ],
    );
}

#[test]
fn daily_by_hour_and_minute_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 6, 6),
            ymd_hms(1997, 9, 2, 18, 6, 18),
            ymd_hms(1997, 9, 2, 18, 18, 6),
        ],
    );
}

#[test]
fn daily_by_setpos() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_set_pos: vec![3, -3],
        by_minute: vec![15, 45],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 15, 0),
            ymd_hms(1997, 9, 3, 6, 45, 0),
            ymd_hms(1997, 9, 3, 18, 15, 0),
        ],
    );
}

#[test]
fn hourly() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 10, 0, 0),
            ymd_hms(1997, 9, 2, 11, 0, 0),
        ],
    );
}

#[test]
fn hourly_interval() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_minute: vec![0],
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 11, 0, 0),
            ymd_hms(1997, 9, 2, 13, 0, 0),
        ],
    );
}

#[test]
fn hourly_interval_large() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_minute: vec![0],
        by_second: vec![0],
        interval: 769,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 10, 4, 10, 0, 0),
            ymd_hms(1997, 11, 5, 11, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_month() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 1, 0, 0),
            ymd_hms(1998, 1, 1, 2, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_monthday() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 3, 0, 0, 0),
            ymd_hms(1997, 9, 3, 1, 0, 0),
            ymd_hms(1997, 9, 3, 2, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_month_and_monthday() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![5, 7],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 5, 0, 0, 0),
            ymd_hms(1998, 1, 5, 1, 0, 0),
            ymd_hms(1998, 1, 5, 2, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(5),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 5,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 1, 0, 0),
            ymd_hms(1998, 1, 1, 2, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_monthday_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 1, 0, 0),
            ymd_hms(1998, 1, 1, 2, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_month_and_monthday_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 1, 0, 0),
            ymd_hms(1998, 1, 1, 2, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_yearday() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(8),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        interval: 12,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(8),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![-365, -266, -166, -1],
        interval: 12,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(4),
        by_month: vec![4, 7],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 4, 10, 0, 0, 0),
            ymd_hms(1998, 4, 10, 1, 0, 0),
            ymd_hms(1998, 4, 10, 2, 0, 0),
            ymd_hms(1998, 4, 10, 3, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (hourly)."]
fn hourly_by_weekno() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_week_no: vec![20],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 5, 11, 0, 0, 0),
            ymd_hms(1998, 5, 11, 1, 0, 0),
            ymd_hms(1998, 5, 11, 2, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (hourly)."]
fn hourly_by_weekno_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        by_week_no: vec![1],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 29, 0, 0, 0),
            ymd_hms(1997, 12, 29, 1, 0, 0),
            ymd_hms(1997, 12, 29, 2, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (hourly)."]
fn hourly_by_weekno_and_weekday_large() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_week_no: vec![52],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 28, 0, 0, 0),
            ymd_hms(1997, 12, 28, 1, 0, 0),
            ymd_hms(1997, 12, 28, 2, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (hourly)."]
fn hourly_by_weekno_and_weekday_last() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_week_no: vec![-1],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 28, 0, 0, 0),
            ymd_hms(1997, 12, 28, 1, 0, 0),
            ymd_hms(1997, 12, 28, 2, 0, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (hourly)."]
fn hourly_by_weekno_and_weekday53() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        by_week_no: vec![53],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 12, 28, 0, 0, 0),
            ymd_hms(1998, 12, 28, 1, 0, 0),
            ymd_hms(1998, 12, 28, 2, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_hour() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 0, 0),
            ymd_hms(1997, 9, 3, 6, 0, 0),
            ymd_hms(1997, 9, 3, 18, 0, 0),
        ],
    );
}

#[test]
fn hourly_by_minute() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_minute: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 6, 0),
            ymd_hms(1997, 9, 2, 9, 18, 0),
            ymd_hms(1997, 9, 2, 10, 6, 0),
        ],
    );
}

#[test]
fn hourly_by_second() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_minute: vec![0],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 6),
            ymd_hms(1997, 9, 2, 9, 0, 18),
            ymd_hms(1997, 9, 2, 10, 0, 6),
        ],
    );
}

#[test]
fn hourly_by_hour_and_minute() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 6, 0),
            ymd_hms(1997, 9, 2, 18, 18, 0),
            ymd_hms(1997, 9, 3, 6, 6, 0),
        ],
    );
}

#[test]
fn hourly_by_hour_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![0],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 0, 6),
            ymd_hms(1997, 9, 2, 18, 0, 18),
            ymd_hms(1997, 9, 3, 6, 0, 6),
        ],
    );
}

#[test]
fn hourly_by_minute_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 6, 6),
            ymd_hms(1997, 9, 2, 9, 6, 18),
            ymd_hms(1997, 9, 2, 9, 18, 6),
        ],
    );
}

#[test]
fn hourly_by_hour_and_minute_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(5),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_set_pos: vec![3, -3],
        by_minute: vec![15, 45],
        by_second: vec![15, 45],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 15, 45),
            ymd_hms(1997, 9, 2, 9, 45, 15),
            ymd_hms(1997, 9, 2, 10, 15, 45),
        ],
    );
}

#[test]
fn minutely() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 9, 1, 0),
            ymd_hms(1997, 9, 2, 9, 2, 0),
        ],
    );
}

#[test]
fn minutely_interval() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 9, 2, 0),
            ymd_hms(1997, 9, 2, 9, 4, 0),
        ],
    );
}

#[test]
fn minutely_interval_large() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_second: vec![0],
        interval: 1501,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 3, 10, 1, 0),
            ymd_hms(1997, 9, 4, 11, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_month() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 0, 1, 0),
            ymd_hms(1998, 1, 1, 0, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_monthday() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 3, 0, 0, 0),
            ymd_hms(1997, 9, 3, 0, 1, 0),
            ymd_hms(1997, 9, 3, 0, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_month_and_monthday() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_second: vec![0],
        by_month_day: vec![5, 7],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 5, 0, 0, 0),
            ymd_hms(1998, 1, 5, 0, 1, 0),
            ymd_hms(1998, 1, 5, 0, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Thu), NWeekday::Every(Weekday::Sat)],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 4, 0, 0, 0),
            ymd_hms(1997, 9, 4, 0, 1, 0),
            ymd_hms(1997, 9, 4, 0, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_month_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 0, 1, 0),
            ymd_hms(1998, 1, 1, 0, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_monthday_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 0, 1, 0),
            ymd_hms(1998, 1, 1, 0, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_month_and_monthday_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_second: vec![0],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 0, 1, 0),
            ymd_hms(1998, 1, 1, 0, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_yearday() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(4),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(4),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_second: vec![0],
        by_year_day: vec![-365, -266, -166, -1],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(4),
        by_month: vec![4, 7],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_second: vec![0],
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 4, 10, 0, 0, 0),
            ymd_hms(1998, 4, 10, 0, 1, 0),
            ymd_hms(1998, 4, 10, 0, 2, 0),
            ymd_hms(1998, 4, 10, 0, 3, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (minutely)."]
fn minutely_by_weekno() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_week_no: vec![20],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 5, 11, 0, 0, 0),
            ymd_hms(1998, 5, 11, 0, 1, 0),
            ymd_hms(1998, 5, 11, 0, 2, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (minutely)."]
fn minutely_by_weekno_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        by_week_no: vec![1],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 29, 0, 0, 0),
            ymd_hms(1997, 12, 29, 0, 1, 0),
            ymd_hms(1997, 12, 29, 0, 2, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (minutely)."]
fn minutely_by_weekno_and_weekday_large() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_week_no: vec![52],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 28, 0, 0, 0),
            ymd_hms(1997, 12, 28, 0, 1, 0),
            ymd_hms(1997, 12, 28, 0, 2, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (minutely)."]
fn minutely_by_weekno_and_weekday_last() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_week_no: vec![-1],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 28, 0, 0, 0),
            ymd_hms(1997, 12, 28, 0, 1, 0),
            ymd_hms(1997, 12, 28, 0, 2, 0),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (minutely)."]
fn minutely_by_weekno_and_weekday53() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        by_week_no: vec![53],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 12, 28, 0, 0, 0),
            ymd_hms(1998, 12, 28, 0, 1, 0),
            ymd_hms(1998, 12, 28, 0, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_hour() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 0, 0),
            ymd_hms(1997, 9, 2, 18, 1, 0),
            ymd_hms(1997, 9, 2, 18, 2, 0),
        ],
    );
}

#[test]
fn minutely_by_minute() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_minute: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 6, 0),
            ymd_hms(1997, 9, 2, 9, 18, 0),
            ymd_hms(1997, 9, 2, 10, 6, 0),
        ],
    );
}

#[test]
fn minutely_by_second() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 6),
            ymd_hms(1997, 9, 2, 9, 0, 18),
            ymd_hms(1997, 9, 2, 9, 1, 6),
        ],
    );
}

#[test]
fn minutely_by_hour_and_minute() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 6, 0),
            ymd_hms(1997, 9, 2, 18, 18, 0),
            ymd_hms(1997, 9, 3, 6, 6, 0),
        ],
    );
}

#[test]
fn minutely_by_hour_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 0, 6),
            ymd_hms(1997, 9, 2, 18, 0, 18),
            ymd_hms(1997, 9, 2, 18, 1, 6),
        ],
    );
}

#[test]
fn minutely_by_minute_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 6, 6),
            ymd_hms(1997, 9, 2, 9, 6, 18),
            ymd_hms(1997, 9, 2, 9, 18, 6),
        ],
    );
}

#[test]
fn minutely_by_hour_and_minute_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(5),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_set_pos: vec![3, -3],
        by_second: vec![15, 30, 45],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 15),
            ymd_hms(1997, 9, 2, 9, 0, 45),
            ymd_hms(1997, 9, 2, 9, 1, 15),
        ],
    );
}

#[test]
fn secondly() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 9, 0, 1),
            ymd_hms(1997, 9, 2, 9, 0, 2),
        ],
    );
}

#[test]
fn secondly_interval() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 9, 0, 2),
            ymd_hms(1997, 9, 2, 9, 0, 4),
        ],
    );
}

#[test]
fn secondly_interval_large_under_limit() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        interval: 50000,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 22, 53, 20),
            ymd_hms(1997, 9, 3, 12, 46, 40),
        ],
    );
}

#[test]
#[cfg(feature = "no-validation-limits")]
fn secondly_interval_large() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        interval: 60061,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 3, 1, 41, 1),
            ymd_hms(1997, 9, 3, 18, 22, 2),
        ],
    );
}

#[test]
fn secondly_by_month() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 0, 0, 1),
            ymd_hms(1998, 1, 1, 0, 0, 2),
        ],
    );
}

#[test]
fn secondly_by_monthday() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 3, 0, 0, 0),
            ymd_hms(1997, 9, 3, 0, 0, 1),
            ymd_hms(1997, 9, 3, 0, 0, 2),
        ],
    );
}

#[test]
fn secondly_by_month_and_monthday() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_month_day: vec![5, 7],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 5, 0, 0, 0),
            ymd_hms(1998, 1, 5, 0, 0, 1),
            ymd_hms(1998, 1, 5, 0, 0, 2),
        ],
    );
}

#[test]
fn secondly_by_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Thu), NWeekday::Every(Weekday::Sat)],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 4, 0, 0, 0),
            ymd_hms(1997, 9, 4, 0, 0, 1),
            ymd_hms(1997, 9, 4, 0, 0, 2),
        ],
    );
}

#[test]
fn secondly_by_month_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 0, 0, 1),
            ymd_hms(1998, 1, 1, 0, 0, 2),
        ],
    );
}

#[test]
fn secondly_by_monthday_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 0, 0, 1),
            ymd_hms(1998, 1, 1, 0, 0, 2),
        ],
    );
}

#[test]
fn secondly_by_month_and_monthday_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        by_month: vec![1, 3],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_month_day: vec![1, 3],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 1, 1, 0, 0, 0),
            ymd_hms(1998, 1, 1, 0, 0, 1),
            ymd_hms(1998, 1, 1, 0, 0, 2),
        ],
    );
}

#[test]
fn secondly_by_yearday() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(4),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(4),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_year_day: vec![-365, -266, -166, -1],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(4),
        by_month: vec![4, 7],
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_year_day: vec![1, 100, 200, 365],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 4, 10, 0, 0, 0),
            ymd_hms(1998, 4, 10, 0, 0, 1),
            ymd_hms(1998, 4, 10, 0, 0, 2),
            ymd_hms(1998, 4, 10, 0, 0, 3),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (secondly)."]
fn secondly_by_weekno() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_week_no: vec![20],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 5, 11, 0, 0, 0),
            ymd_hms(1998, 5, 11, 0, 0, 1),
            ymd_hms(1998, 5, 11, 0, 0, 2),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (secondly)."]
fn secondly_by_weekno_and_weekday() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        by_week_no: vec![1],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 29, 0, 0, 0),
            ymd_hms(1997, 12, 29, 0, 0, 1),
            ymd_hms(1997, 12, 29, 0, 0, 2),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (secondly)."]
fn secondly_by_weekno_and_weekday_large() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_week_no: vec![52],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 28, 0, 0, 0),
            ymd_hms(1997, 12, 28, 0, 0, 1),
            ymd_hms(1997, 12, 28, 0, 0, 2),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (secondly)."]
fn secondly_by_weekno_and_weekday_last() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Sun)],
        by_week_no: vec![-1],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 12, 28, 0, 0, 0),
            ymd_hms(1997, 12, 28, 0, 0, 1),
            ymd_hms(1997, 12, 28, 0, 0, 2),
        ],
    );
}

#[test]
#[ignore = "`BYWEEKNO` can not be used with the current frequency (secondly)."]
fn secondly_by_weekno_and_weekday53() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        by_week_no: vec![53],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1998, 12, 28, 0, 0, 0),
            ymd_hms(1998, 12, 28, 0, 0, 1),
            ymd_hms(1998, 12, 28, 0, 0, 2),
        ],
    );
}

#[test]
#[ignore = "This assumes it will just loop over 9*60*60=32400 seconds. \
This will thus trigger the loop_limit."]
fn secondly_by_hour() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 0, 0),
            ymd_hms(1997, 9, 2, 18, 0, 1),
            ymd_hms(1997, 9, 2, 18, 0, 2),
        ],
    );
}

#[test]
fn secondly_by_minute() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_minute: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 6, 0),
            ymd_hms(1997, 9, 2, 9, 6, 1),
            ymd_hms(1997, 9, 2, 9, 6, 2),
        ],
    );
}

#[test]
fn secondly_by_second() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 6),
            ymd_hms(1997, 9, 2, 9, 0, 18),
            ymd_hms(1997, 9, 2, 9, 1, 6),
        ],
    );
}

#[test]
#[ignore = "This assumes it will just loop over 9*60*60=32400 seconds. \
This will thus trigger the loop_limit."]
fn secondly_by_hour_and_minute() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 6, 0),
            ymd_hms(1997, 9, 2, 18, 6, 1),
            ymd_hms(1997, 9, 2, 18, 6, 2),
        ],
    );
}

#[test]
#[ignore = "This assumes it will just loop over 9*60*60=32400 seconds. \
This will thus trigger the loop_limit."]
fn secondly_by_hour_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 18, 0, 6),
            ymd_hms(1997, 9, 2, 18, 0, 18),
            ymd_hms(1997, 9, 2, 18, 1, 6),
        ],
    );
}

#[test]
fn secondly_by_minute_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 6, 6),
            ymd_hms(1997, 9, 2, 9, 6, 18),
            ymd_hms(1997, 9, 2, 9, 18, 6),
        ],
    );
}

#[test]
#[ignore = "This assumes it will just loop over 9*60*60=32400 seconds. \
This will thus trigger the loop_limit."]
fn secondly_by_hour_and_minute_and_second() {
    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(5),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_hour: vec![6, 18],
        by_minute: vec![6, 18],
        by_second: vec![6, 18],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        until: Some(ymd_hms(1997, 9, 5, 8, 0, 0)),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
        ],
    );
}

#[test]
fn until_matching() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(4),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        until: Some(ymd_hms(1997, 9, 4, 9, 0, 0)),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
        ],
    );
}

#[test]
fn until_single() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(1),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        until: Some(ymd_hms(1997, 9, 2, 9, 0, 0)),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(properties, &[ymd_hms(1997, 9, 2, 9, 0, 0)]);
}

#[test]
#[ignore = "`UNTIL` is `1997-09-01T09:00:00+00:00`, but `DTSTART` (`1997-09-02T09:00:00+00:00`) \
is later. That should not be happening."]
fn until_empty() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(1),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        until: Some(ymd_hms(1997, 9, 1, 9, 0, 0)),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(properties, &[]);
}

#[test]
fn until_with_date() {
    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(4),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        until: Some(ymd_hms(1997, 9, 5, 0, 0, 0)),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
        ],
    );
}

#[test]
fn week_start_interval_mo() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Sun)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 7, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
        ],
    );
}

#[test]
fn week_start_interval_su() {
    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(3),
        dt_start: ymd_hms(1997, 9, 2, 9, 0, 0),
        week_start: Weekday::Sun,
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Sun)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    test_recurring_rrule(
        properties,
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
    use chrono_tz::{
        America::{Los_Angeles, New_York},
        Europe::Berlin,
    };

    let rrule_properties = RRuleProperties::default()
        .dt_start(UTC.ymd(2021, 1, 1).and_hms(9, 0, 0))
        .count(2)
        .freq(Frequency::Weekly)
        .by_weekday(vec![NWeekday::Every(Sat)]);
    let rrule = RRule::new(rrule_properties).unwrap();
    for o in rrule.into_iter() {
        assert_eq!(o.weekday(), Sat);
    }

    // NYC (-5)
    let rrule_properties = RRuleProperties::default()
        .dt_start(New_York.ymd(2021, 1, 1).and_hms(9, 0, 0))
        .count(1)
        .freq(Frequency::Weekly)
        .by_weekday(vec![NWeekday::Every(Sat)]);
    let rrule = RRule::new(rrule_properties).unwrap();
    for o in rrule.into_iter() {
        assert_eq!(o.weekday(), Sat);
    }

    // How about Berlin (+1)
    let rrule_properties = RRuleProperties::default()
        .dt_start(Berlin.ymd(2021, 1, 1).and_hms(9, 0, 0))
        .count(1)
        .freq(Frequency::Weekly)
        .by_weekday(vec![NWeekday::Every(Sat)]);
    let rrule = RRule::new(rrule_properties).unwrap();
    for o in rrule.into_iter() {
        assert_eq!(o.weekday(), Sat);
    }

    // Los Angeles (-7)
    let rrule_properties = RRuleProperties::default()
        .dt_start(Los_Angeles.ymd(2021, 1, 1).and_hms(9, 0, 0))
        .count(1)
        .freq(Frequency::Weekly)
        .by_weekday(vec![NWeekday::Every(Sat)]);
    let rrule = RRule::new(rrule_properties).unwrap();
    for o in rrule.into_iter() {
        assert_eq!(o.weekday(), Sat);
    }
}

#[test]
fn test_before_inclusive_hit() {
    let rrule: RRule = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3"
        .parse()
        .unwrap();

    let before = UTC.ymd(2012, 2, 2).and_hms(9, 30, 0);
    let inc = true;

    assert_eq!(Some(before), rrule.just_before(before, inc).unwrap());
}

#[test]
fn test_before_inclusive_miss() {
    let rrule: RRule = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3"
        .parse()
        .unwrap();

    let before = UTC.ymd(2012, 2, 3).and_hms(9, 0, 0);
    let oracle = UTC.ymd(2012, 2, 2).and_hms(9, 30, 0);
    let inc = true;

    assert_eq!(Some(oracle), rrule.just_before(before, inc).unwrap());
}

#[test]
fn test_after_inclusive_hit() {
    let rrule: RRule = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3"
        .parse()
        .unwrap();

    let after = UTC.ymd(2012, 2, 2).and_hms(9, 30, 0);
    let inc = true;

    assert_eq!(Some(after), rrule.just_after(after, inc).unwrap());
}

#[test]
fn test_after_inclusive_miss() {
    let rrule: RRule = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3"
        .parse()
        .unwrap();

    let after = UTC.ymd(2012, 2, 2).and_hms(10, 0, 0);
    let oracle = UTC.ymd(2012, 2, 3).and_hms(9, 30, 0);
    let inc = true;

    assert_eq!(Some(oracle), rrule.just_after(after, inc).unwrap());
}

#[test]
fn test_between_inclusive_both_miss() {
    let rrule: RRule = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=5"
        .parse()
        .unwrap();

    let before = UTC.ymd(2012, 2, 2).and_hms(10, 0, 0);
    let middle = UTC.ymd(2012, 2, 3).and_hms(9, 30, 0);
    let after = UTC.ymd(2012, 2, 4).and_hms(9, 0, 0);
    let inc = true;

    assert_eq!(vec![middle], rrule.all_between(before, after, inc).unwrap());
}

#[test]
fn test_between_inclusive_lower_miss() {
    let rrule: RRule = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=5"
        .parse()
        .unwrap();

    let before = UTC.ymd(2012, 2, 2).and_hms(10, 0, 0);
    let middle = UTC.ymd(2012, 2, 3).and_hms(9, 30, 0);
    let after = UTC.ymd(2012, 2, 4).and_hms(9, 30, 0);
    let inc = true;

    assert_eq!(
        vec![middle, after],
        rrule.all_between(before, after, inc).unwrap()
    );
}

#[test]
fn test_between_inclusive_upper_miss() {
    let rrule: RRule = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=5"
        .parse()
        .unwrap();

    let before = UTC.ymd(2012, 2, 2).and_hms(9, 30, 0);
    let middle = UTC.ymd(2012, 2, 3).and_hms(9, 30, 0);
    let after = UTC.ymd(2012, 2, 4).and_hms(9, 0, 0);
    let inc = true;

    assert_eq!(
        vec![before, middle],
        rrule.all_between(before, after, inc).unwrap()
    );
}

#[test]
fn test_between_inclusive_both_hit() {
    let rrule: RRule = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=5"
        .parse()
        .unwrap();

    let before = UTC.ymd(2012, 2, 2).and_hms(9, 30, 0);
    let middle = UTC.ymd(2012, 2, 3).and_hms(9, 30, 0);
    let after = UTC.ymd(2012, 2, 4).and_hms(9, 30, 0);
    let inc = true;

    assert_eq!(
        vec![before, middle, after],
        rrule.all_between(before, after, inc).unwrap()
    );
}
