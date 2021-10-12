mod common;

use chrono::TimeZone;
use chrono_tz::UTC;
use common::{test_recurring_rrule_set, ymd_hms};
use rrule::{Frequency, NWeekday, RRule, RRuleProperties, RRuleSet, Weekday};

#[test]
fn rrule_and_exrule() {
    let mut set = RRuleSet::default();

    let properties1 = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(6),
        by_month: vec![],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_set_pos: vec![],
        by_week_no: vec![],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![],
        by_month_day: vec![],
        by_n_month_day: vec![],
        until: None,
        week_start: Weekday::Mon,
        tz: UTC,
        interval: 1,
        by_easter: None,
    };
    let rrule = RRule::new(properties1).unwrap();
    set.rrule(rrule);
    let properties2 = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_set_pos: vec![],
        by_week_no: vec![],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![],
        by_month_day: vec![],
        by_n_month_day: vec![],
        until: None,
        week_start: Weekday::Mon,
        tz: UTC,
        interval: 1,
        by_easter: None,
    };
    let exrule = RRule::new(properties2).unwrap();
    set.exrule(exrule);

    test_recurring_rrule_set(
        set,
        &vec![
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
        ],
    );
}

#[test]
fn setdate_and_exdate() {
    let mut set = RRuleSet::default();

    set.rdate(ymd_hms(1997, 9, 2, 9, 0, 0));
    set.rdate(ymd_hms(1997, 9, 4, 9, 0, 0));
    set.rdate(ymd_hms(1997, 9, 9, 9, 0, 0));
    set.rdate(ymd_hms(1997, 9, 11, 9, 0, 0));
    set.rdate(ymd_hms(1997, 9, 16, 9, 0, 0));
    set.rdate(ymd_hms(1997, 9, 18, 9, 0, 0));

    set.exdate(ymd_hms(1997, 9, 4, 9, 0, 0));
    set.exdate(ymd_hms(1997, 9, 11, 9, 0, 0));
    set.exdate(ymd_hms(1997, 9, 18, 9, 0, 0));

    test_recurring_rrule_set(
        set,
        &vec![
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
        ],
    );
}

#[test]
fn setdate_and_exrule() {
    let mut set = RRuleSet::default();

    set.rdate(ymd_hms(1997, 9, 2, 9, 0, 0));
    set.rdate(ymd_hms(1997, 9, 4, 9, 0, 0));
    set.rdate(ymd_hms(1997, 9, 9, 9, 0, 0));
    set.rdate(ymd_hms(1997, 9, 11, 9, 0, 0));
    set.rdate(ymd_hms(1997, 9, 16, 9, 0, 0));
    set.rdate(ymd_hms(1997, 9, 18, 9, 0, 0));

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        by_month: vec![],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_set_pos: vec![],
        by_week_no: vec![],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![],
        by_month_day: vec![],
        by_n_month_day: vec![],
        until: None,
        week_start: Weekday::Mon,
        tz: UTC,
        interval: 1,
        by_easter: None,
    };
    let exrrule = RRule::new(properties).unwrap();
    set.exrule(exrrule);

    test_recurring_rrule_set(
        set,
        &vec![
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
        ],
    );
}

#[test]
fn rrule_and_exdate() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(6),
        by_month: vec![],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_set_pos: vec![],
        by_week_no: vec![],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![],
        by_month_day: vec![],
        by_n_month_day: vec![],
        until: None,
        week_start: Weekday::Mon,
        tz: UTC,
        interval: 1,
        by_easter: None,
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    set.exdate(ymd_hms(1997, 9, 2, 9, 0, 0));
    set.exdate(ymd_hms(1997, 9, 4, 9, 0, 0));
    set.exdate(ymd_hms(1997, 9, 9, 9, 0, 0));

    test_recurring_rrule_set(
        set,
        &vec![
            ymd_hms(1997, 9, 11, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 9, 18, 9, 0, 0),
        ],
    );
}

#[test]
fn rrule_and_exyearly_yearly_big() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(13),
        by_month: vec![9],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_weekday: vec![],
        by_hour: vec![9],
        by_set_pos: vec![],
        by_week_no: vec![],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![],
        by_month_day: vec![2],
        by_n_month_day: vec![],
        until: None,
        week_start: Weekday::Mon,
        tz: UTC,
        interval: 1,
        by_easter: None,
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(10),
        by_month: vec![9],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_weekday: vec![],
        by_hour: vec![9],
        by_set_pos: vec![],
        by_week_no: vec![],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![],
        by_month_day: vec![2],
        by_n_month_day: vec![],
        until: None,
        week_start: Weekday::Mon,
        tz: UTC,
        interval: 1,
        by_easter: None,
    };
    let rrule = RRule::new(properties).unwrap();
    set.exrule(rrule);

    test_recurring_rrule_set(
        set,
        &vec![
            ymd_hms(2007, 9, 2, 9, 0, 0),
            ymd_hms(2008, 9, 2, 9, 0, 0),
            ymd_hms(2009, 9, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn before() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: None,
        by_month: vec![9],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_weekday: vec![],
        by_hour: vec![9],
        by_set_pos: vec![],
        by_week_no: vec![],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![],
        by_month_day: vec![2],
        by_n_month_day: vec![],
        until: None,
        week_start: Weekday::Mon,
        tz: UTC,
        interval: 1,
        by_easter: None,
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(10),
        by_month: vec![9],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_weekday: vec![],
        by_hour: vec![9],
        by_set_pos: vec![],
        by_week_no: vec![],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![],
        by_month_day: vec![2],
        by_n_month_day: vec![],
        until: None,
        week_start: Weekday::Mon,
        tz: UTC,
        interval: 1,
        by_easter: None,
    };
    let rrule = RRule::new(properties).unwrap();
    set.exrule(rrule);

    assert_eq!(
        set.before(ymd_hms(2015, 9, 2, 9, 0, 0), false).unwrap(),
        ymd_hms(2014, 9, 2, 9, 0, 0),
    );
}

#[test]
fn after() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: None,
        by_month: vec![9],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_weekday: vec![],
        by_hour: vec![9],
        by_set_pos: vec![],
        by_week_no: vec![],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![],
        by_month_day: vec![2],
        by_n_month_day: vec![],
        until: None,
        week_start: Weekday::Mon,
        tz: UTC,
        interval: 1,
        by_easter: None,
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(10),
        by_month: vec![9],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_weekday: vec![],
        by_hour: vec![9],
        by_set_pos: vec![],
        by_week_no: vec![],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![],
        by_month_day: vec![2],
        by_n_month_day: vec![],
        until: None,
        week_start: Weekday::Mon,
        tz: UTC,
        interval: 1,
        by_easter: None,
    };
    let rrule = RRule::new(properties).unwrap();
    set.exrule(rrule);

    assert_eq!(
        set.after(ymd_hms(2000, 9, 2, 9, 0, 0), false).unwrap(),
        ymd_hms(2007, 9, 2, 9, 0, 0),
    );
}

#[test]
fn between() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: None,
        by_month: vec![9],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_weekday: vec![],
        by_hour: vec![9],
        by_set_pos: vec![],
        by_week_no: vec![],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![],
        by_month_day: vec![2],
        by_n_month_day: vec![],
        until: None,
        week_start: Weekday::Mon,
        tz: UTC,
        interval: 1,
        by_easter: None,
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(10),
        by_month: vec![9],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_weekday: vec![],
        by_hour: vec![9],
        by_set_pos: vec![],
        by_week_no: vec![],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![],
        by_month_day: vec![2],
        by_n_month_day: vec![],
        until: None,
        week_start: Weekday::Mon,
        tz: UTC,
        interval: 1,
        by_easter: None,
    };
    let rrule = RRule::new(properties).unwrap();
    set.exrule(rrule);

    common::check_occurrences(
        &set.between(
            ymd_hms(2000, 9, 2, 9, 0, 0),
            ymd_hms(2010, 9, 2, 9, 0, 0),
            false,
        ),
        &vec![
            "2007-09-02T09:00:00-00:00",
            "2008-09-02T09:00:00-00:00",
            "2009-09-02T09:00:00-00:00",
        ],
    );
}

#[test]
fn before_70s() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(2),
        by_month: vec![1],
        dt_start: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
        by_weekday: vec![],
        by_hour: vec![9],
        by_set_pos: vec![],
        by_week_no: vec![],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![],
        by_month_day: vec![1],
        by_n_month_day: vec![],
        until: None,
        week_start: Weekday::Mon,
        tz: UTC,
        interval: 1,
        by_easter: None,
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    test_recurring_rrule_set(
        set,
        &vec![ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1961, 1, 1, 9, 0, 0)],
    );
}
