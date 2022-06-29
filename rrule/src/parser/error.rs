use thiserror::Error;

// TODO: clarify difference between field and property

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParseError {
    #[error("`{0}` is not a valid timezone.")]
    InvalidTimezone(String),
    #[error("`{value}` is not a valid datetime format for `{field}`.")]
    InvalidDateTime { value: String, field: String },
    #[error("{field}:{value} is not a valid datetime in local timezone.")]
    InvalidDateTimeInLocalTimezone { value: String, field: String },
    #[error("{field}:{value} is not a valid datetime in local timezone. This value is ambiguous and can be `{date1}` or `{date2}`")]
    DateTimeInLocalTimezoneIsAmbiguous {
        value: String,
        field: String,
        date1: String,
        date2: String,
    },
    #[error("`{0}` is not a valid frequency.")]
    InvalidFrequency(String),
    #[error("`{0}` is not a valid weekday. Valid values are `MO`, `TU`, `WE`, `TH`, `FR`, `SA` and `SU`.")]
    InvalidWeekday(String),
    #[error("`{0}` is not a valid weekday start. Valid values are `MO`, `TU`, `WE`, `TH`, `FR`, `SA` and `SU`.")]
    InvalidWeekdayStart(String),
    #[error("`{0}` is not a valid BYEASTER value.")]
    InvalidByEaster(String),
    #[error("`{0}` is not a valid INTERVAL value.")]
    InvalidInterval(String),
    #[error("`{0}` is not a valid COUNT value.")]
    InvalidCount(String),
    #[error("`{0}` is not a valid BYHOUR value. Expected a comma separated list of values in range 0..=23, e.g. `-1,3,4`")]
    InvalidByHour(String),
    #[error("`{0}` is not a valid BYWEEKNO value. Expected a comma separated list of values in range -53..=53, e.g. `-1,30,53`")]
    InvalidByWeekNo(String),
    #[error("`{0}` is not a valid BYYEARDAY value. Expected a comma separated list of values in range -366..=366, e.g. `-100,`")]
    InvalidByYearDay(String),
    #[error("`{0}` is not a valid BYMONTHDAY value. Expected a comma separated list of values in range -31..=31, e.g. `-30,10`")]
    InvalidByMonthDay(String),
    #[error("`{0}` is not a valid BYMONTH value. Expected a comma separated list of values in range 1..=12, e.g. `6,9,10`")]
    InvalidByMonth(String),
    #[error("`{0}` is not a valid BYMINUTE value. Expected a comma separated list of values in range 0..=59, e.g. `0,15,30,45`")]
    InvalidByMinute(String),
    #[error("`{0}` is not a valid BYSECOND value. Expected a comma separated list of values in range 0..=59, e.g. `0,15,30,45`")]
    InvalidBySecond(String),
    #[error("`{0}` is not a valid BYSETPOS value. Expected a comma separated list of integers, e.g. `-3,1`")]
    InvalidBySetPos(String),
    #[error("The field `{0}` was found twice.")]
    DuplicatedField(String),
    #[error("The property `{0}` was not found and it is required.")]
    MissingProperty(String),
    #[error("Unsupported RFC property `{0}`, valid rules are `RRULE` and `EXRULE`.")]
    UnsupportedRFCProperty(String),
    #[error("Unsupported property `{0}`, valid properties are `RRULE`, `EXRULE`, `RDATE`, `EXDATE` and `DTSTART`.")]
    UnsupportedProperty(String),
    #[error("Unexpected field: `{0}`.")]
    UnexpectedField(String),
    #[error("Unsupported value `{0}` in line `{1}`.")]
    UnsupportedValueInRuleLine(String, String),
    #[error("`{0}` is not a valid rrule.")]
    InvalidRule(String),
    #[error("`{0}` contains two or more rrules, expected only one.")]
    TooManyRulesInLine(String),
    #[error("`{0}` is not a supported character.")]
    UnsupportedCharacter(String),
}
