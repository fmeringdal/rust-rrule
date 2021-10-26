mod common;

use chrono::TimeZone;
use chrono_tz::UTC;
use common::{test_recurring_rrule_set, ymd_hms};
use rrule::{DateFilter, Frequency, NWeekday, RRule, RRuleProperties, RRuleSet, Weekday};

#[test]
fn rrule_and_exrule() {
    let mut set = RRuleSet::default();

    let properties1 = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(6),
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    let rrule = RRule::new(properties1).unwrap();
    set.rrule(rrule);
    let properties2 = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(3),
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    let exrule = RRule::new(properties2).unwrap();
    set.exrule(exrule);

    test_recurring_rrule_set(
        set,
        &[
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
        &[
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
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    let exrrule = RRule::new(properties).unwrap();
    set.exrule(exrrule);

    test_recurring_rrule_set(
        set,
        &[
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
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    set.exdate(ymd_hms(1997, 9, 2, 9, 0, 0));
    set.exdate(ymd_hms(1997, 9, 4, 9, 0, 0));
    set.exdate(ymd_hms(1997, 9, 9, 9, 0, 0));

    test_recurring_rrule_set(
        set,
        &[
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
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(10),
        by_month: vec![9],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.exrule(rrule);

    test_recurring_rrule_set(
        set,
        &[
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
        by_month: vec![9],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(10),
        by_month: vec![9],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.exrule(rrule);

    assert_eq!(
        set.just_before(ymd_hms(2015, 9, 2, 9, 0, 0), false)
            .unwrap()
            .unwrap(),
        ymd_hms(2014, 9, 2, 9, 0, 0),
    );
}

#[test]
fn after() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        by_month: vec![9],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(10),
        by_month: vec![9],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.exrule(rrule);

    assert_eq!(
        set.just_after(ymd_hms(2000, 9, 2, 9, 0, 0), false)
            .unwrap()
            .unwrap(),
        ymd_hms(2007, 9, 2, 9, 0, 0),
    );
}

#[test]
fn between() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        by_month: vec![9],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(10),
        by_month: vec![9],
        dt_start: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.exrule(rrule);

    common::check_occurrences(
        &set.all_between(
            ymd_hms(2000, 9, 2, 9, 0, 0),
            ymd_hms(2010, 9, 2, 9, 0, 0),
            false,
        )
        .unwrap(),
        &[
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
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1961, 1, 1, 9, 0, 0)],
    );
}

#[test]
fn secondly_with_interval_1() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(2),
        dt_start: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 1, 1, 9, 0, 1)],
    );
}

#[test]
fn secondly_with_interval_2() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Secondly,
        count: Some(2),
        dt_start: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
        interval: 2,
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 1, 1, 9, 0, 2)],
    );
}

#[test]
fn minutely_with_interval_1() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(2),
        dt_start: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
        by_second: vec![0],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 1, 1, 9, 1, 0)],
    );
}

#[test]
fn minutely_with_interval_2() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Minutely,
        count: Some(2),
        dt_start: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 1, 1, 9, 2, 0)],
    );
}

#[test]
fn hourly_with_interval_1() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(2),
        dt_start: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 1, 1, 10, 0, 0)],
    );
}

#[test]
fn hourly_with_interval_2() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Hourly,
        count: Some(2),
        dt_start: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
        by_minute: vec![0],
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 1, 1, 11, 0, 0)],
    );
}

#[test]
fn daily_with_interval_1() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(2),
        dt_start: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 1, 2, 9, 0, 0)],
    );
}

#[test]
fn daily_with_interval_2() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Daily,
        count: Some(2),
        dt_start: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 1, 3, 9, 0, 0)],
    );
}

#[test]
fn weekly_with_interval_1() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(2),
        dt_start: UTC.ymd(1960, 1, 4).and_hms(9, 0, 0), // 4th is Monday
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 4, 9, 0, 0), ymd_hms(1960, 1, 11, 9, 0, 0)],
    );
}

#[test]
fn weekly_with_interval_2() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Weekly,
        count: Some(2),
        dt_start: UTC.ymd(1960, 1, 4).and_hms(9, 0, 0), // 4th is Monday
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        interval: 2,
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 4, 9, 0, 0), ymd_hms(1960, 1, 18, 9, 0, 0)],
    );
}

#[test]
fn monthly_with_interval_1() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(2),
        dt_start: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 2, 1, 9, 0, 0)],
    );
}

#[test]
fn monthly_with_interval_2() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Monthly,
        count: Some(2),
        dt_start: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 2,
        by_month_day: vec![1],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 3, 1, 9, 0, 0)],
    );
}

#[test]
fn yearly_with_interval_1() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(2),
        dt_start: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1],
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1961, 1, 1, 9, 0, 0)],
    );
}

#[test]
fn yearly_with_interval_2() {
    let mut set = RRuleSet::default();

    let properties = RRuleProperties {
        freq: Frequency::Yearly,
        count: Some(2),
        dt_start: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1],
        interval: 2,
        ..Default::default()
    };
    let rrule = RRule::new(properties).unwrap();
    set.rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1962, 1, 1, 9, 0, 0)],
    );
}
