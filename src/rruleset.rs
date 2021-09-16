use crate::datetime::DTime;
use crate::options::RRuleParseError;
use crate::rrule::RRule;
use crate::rrulestr::build_rruleset;
use chrono::prelude::*;
use chrono_tz::Tz;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct RRuleSet {
    pub rrule: Vec<RRule>,
    pub rdate: Vec<DTime>,
    pub exrule: Vec<RRule>,
    pub exdate: Vec<DTime>,
    pub dtstart: Option<DTime>,
}

impl RRuleSet {
    pub fn rrule(&mut self, rrule: RRule) {
        self.rrule.push(rrule);
    }

    pub fn exrule(&mut self, rrule: RRule) {
        self.exrule.push(rrule);
    }

    pub fn rdate(&mut self, rdate: DTime) {
        self.rdate.push(rdate);
    }

    pub fn exdate(&mut self, exdate: DTime) {
        self.exdate.push(exdate);
    }

    /// Returns all the recurrences of the rruleset
    pub fn all(&self) -> Vec<DateTime<Tz>> {
        self.into_iter().collect()
    }

    /// Returns the last recurrence before the given datetime instance.
    /// The inc keyword defines what happens if dt is an recurrence.
    /// With inc == true, if dt itself is an recurrence, it will be returned.
    pub fn before(&self, dt: DateTime<Tz>, inc: bool) -> Option<DateTime<Tz>> {
        self.into_iter()
            .take_while(|d| if inc { *d <= dt } else { *d < dt })
            .last()
    }

    /// Returns the last recurrence after the given datetime instance.
    /// The inc keyword defines what happens if dt is an recurrence.
    /// With inc == true, if dt itself is an recurrence, it will be returned.
    pub fn after(&self, dt: DateTime<Tz>, inc: bool) -> Option<DateTime<Tz>> {
        self.into_iter()
            .find(|d| !(if inc { *d <= dt } else { *d < dt }))
    }

    /// Returns all the recurrences of the rrule between after and before.
    /// The inc keyword defines what happens if after and/or before are
    /// themselves recurrences. With inc == true, they will be included in the
    /// list, if they are found in the recurrence set.
    pub fn between(
        &self,
        after: DateTime<Tz>,
        before: DateTime<Tz>,
        inc: bool,
    ) -> Vec<DateTime<Tz>> {
        self.into_iter()
            .skip_while(|d| if inc { *d <= after } else { *d < after })
            .take_while(|d| if inc { *d <= before } else { *d < before })
            .collect()
    }
}

impl FromStr for RRuleSet {
    type Err = RRuleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        build_rruleset(s)
    }
}

#[cfg(test)]
mod test_iter_set {
    use super::*;
    use crate::options::*;
    use chrono_tz::UTC;

    fn ymd_hms(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> DTime {
        UTC.ymd(year, month, day).and_hms(hour, minute, second)
    }

    fn ymd_hms_2(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> DateTime<Tz> {
        UTC.ymd(year, month, day).and_hms(hour, minute, second)
    }

    fn test_recurring(actual_dates: Vec<DateTime<Tz>>, expected_dates: Vec<DateTime<Tz>>) {
        println!("Acutal: {:?}", actual_dates);
        println!("Expected: {:?}", expected_dates);
        assert_eq!(
            actual_dates.len(),
            expected_dates.len(),
            "Expected number of returned dates to be equal to the expected"
        );

        for (actual, exptected) in actual_dates.into_iter().zip(expected_dates) {
            assert_eq!(actual, exptected);
        }
    }

    #[test]
    fn rrule_and_exrule() {
        let mut set = RRuleSet::default();

        let options1 = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(6),
            dtstart: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            ..Default::default()
        };
        let rrule = RRule::new(options1);
        set.rrule(rrule);
        let options2 = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(3),
            dtstart: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![3],
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            ..Default::default()
        };
        let exrule = RRule::new(options2);
        set.exrule(exrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1997, 9, 2, 9, 0, 0),
                ymd_hms_2(1997, 9, 9, 9, 0, 0),
                ymd_hms_2(1997, 9, 16, 9, 0, 0),
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

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1997, 9, 2, 9, 0, 0),
                ymd_hms_2(1997, 9, 9, 9, 0, 0),
                ymd_hms_2(1997, 9, 16, 9, 0, 0),
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

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(3),
            dtstart: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![3],
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            ..Default::default()
        };
        let exrrule = RRule::new(options);
        set.exrule(exrrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1997, 9, 2, 9, 0, 0),
                ymd_hms_2(1997, 9, 9, 9, 0, 0),
                ymd_hms_2(1997, 9, 16, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn rrule_and_exdate() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(6),
            dtstart: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byweekday: vec![1, 3],
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        set.exdate(ymd_hms(1997, 9, 2, 9, 0, 0));
        set.exdate(ymd_hms(1997, 9, 4, 9, 0, 0));
        set.exdate(ymd_hms(1997, 9, 9, 9, 0, 0));

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1997, 9, 11, 9, 0, 0),
                ymd_hms_2(1997, 9, 16, 9, 0, 0),
                ymd_hms_2(1997, 9, 18, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn rrule_and_exyearly_yearly_big() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(13),
            bymonth: vec![9],
            dtstart: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            bymonthday: vec![2],
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(10),
            bymonth: vec![9],
            dtstart: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            bymonthday: vec![2],
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.exrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(2007, 9, 2, 9, 0, 0),
                ymd_hms_2(2008, 9, 2, 9, 0, 0),
                ymd_hms_2(2009, 9, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn before() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            bymonth: vec![9],
            dtstart: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            bymonthday: vec![2],
            interval: 1,
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(10),
            bymonth: vec![9],
            dtstart: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            bymonthday: vec![2],
            interval: 1,
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.exrule(rrule);

        assert_eq!(
            set.before(ymd_hms_2(2015, 9, 2, 9, 0, 0), false).unwrap(),
            ymd_hms_2(2014, 9, 2, 9, 0, 0),
        );
    }

    #[test]
    fn after() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            bymonth: vec![9],
            dtstart: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            bymonthday: vec![2],
            interval: 1,
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(10),
            bymonth: vec![9],
            dtstart: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            bymonthday: vec![2],
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.exrule(rrule);

        assert_eq!(
            set.after(ymd_hms_2(2000, 9, 2, 9, 0, 0), false).unwrap(),
            ymd_hms_2(2007, 9, 2, 9, 0, 0),
        );
    }

    #[test]
    fn between() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            bymonth: vec![9],
            dtstart: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            bymonthday: vec![2],
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(10),
            bymonth: vec![9],
            dtstart: UTC.ymd(1997, 9, 2).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            bymonthday: vec![2],
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.exrule(rrule);

        test_recurring(
            set.between(
                ymd_hms_2(2000, 9, 2, 9, 0, 0),
                ymd_hms_2(2010, 9, 2, 9, 0, 0),
                false,
            ),
            vec![
                ymd_hms_2(2007, 9, 2, 9, 0, 0),
                ymd_hms_2(2008, 9, 2, 9, 0, 0),
                ymd_hms_2(2009, 9, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn before_70s() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(2),
            bymonth: vec![1],
            dtstart: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            bymonthday: vec![1],
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1960, 1, 1, 9, 0, 0),
                ymd_hms_2(1961, 1, 1, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn secondly_with_interval_1() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Secondly,
            count: Some(2),
            dtstart: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1960, 1, 1, 9, 0, 0),
                ymd_hms_2(1960, 1, 1, 9, 0, 1),
            ],
        );
    }

    #[test]
    fn secondly_with_interval_2() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Secondly,
            count: Some(2),
            dtstart: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            interval: 2,
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1960, 1, 1, 9, 0, 0),
                ymd_hms_2(1960, 1, 1, 9, 0, 2),
            ],
        );
    }

    #[test]
    fn minutely_with_interval_1() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Minutely,
            count: Some(2),
            dtstart: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1960, 1, 1, 9, 0, 0),
                ymd_hms_2(1960, 1, 1, 9, 1, 0),
            ],
        );
    }

    #[test]
    fn minutely_with_interval_2() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Minutely,
            count: Some(2),
            dtstart: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            interval: 2,
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1960, 1, 1, 9, 0, 0),
                ymd_hms_2(1960, 1, 1, 9, 2, 0),
            ],
        );
    }

    #[test]
    fn hourly_with_interval_1() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Hourly,
            count: Some(2),
            dtstart: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1960, 1, 1, 9, 0, 0),
                ymd_hms_2(1960, 1, 1, 10, 0, 0),
            ],
        );
    }

    #[test]
    fn hourly_with_interval_2() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Hourly,
            count: Some(2),
            dtstart: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1960, 1, 1, 9, 0, 0),
                ymd_hms_2(1960, 1, 1, 11, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_with_interval_1() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Daily,
            count: Some(2),
            dtstart: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1960, 1, 1, 9, 0, 0),
                ymd_hms_2(1960, 1, 2, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn daily_with_interval_2() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Daily,
            count: Some(2),
            dtstart: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            interval: 2,
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1960, 1, 1, 9, 0, 0),
                ymd_hms_2(1960, 1, 3, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_with_interval_1() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Weekly,
            count: Some(2),
            dtstart: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1960, 1, 1, 9, 0, 0),
                ymd_hms_2(1960, 1, 8, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn weekly_with_interval_2() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Weekly,
            count: Some(2),
            dtstart: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            interval: 2,
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1960, 1, 1, 9, 0, 0),
                ymd_hms_2(1960, 1, 15, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_with_interval_1() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Monthly,
            count: Some(2),
            dtstart: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1960, 1, 1, 9, 0, 0),
                ymd_hms_2(1960, 2, 1, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn monthly_with_interval_2() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Monthly,
            count: Some(2),
            dtstart: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            interval: 2,
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1960, 1, 1, 9, 0, 0),
                ymd_hms_2(1960, 3, 1, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_with_interval_1() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(2),
            dtstart: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1960, 1, 1, 9, 0, 0),
                ymd_hms_2(1960, 2, 1, 9, 0, 0),
            ],
        );
    }

    #[test]
    fn yearly_with_interval_2() {
        let mut set = RRuleSet::default();

        let options = ParsedOptions {
            freq: Frequenzy::Yearly,
            count: Some(2),
            dtstart: UTC.ymd(1960, 1, 1).and_hms(9, 0, 0),
            byhour: vec![9],
            byminute: vec![0],
            bysecond: vec![0],
            interval: 2,
            ..Default::default()
        };
        let rrule = RRule::new(options);
        set.rrule(rrule);

        test_recurring(
            set.all(),
            vec![
                ymd_hms_2(1960, 1, 1, 9, 0, 0),
                ymd_hms_2(1962, 1, 1, 9, 0, 0),
            ],
        );
    }
}
