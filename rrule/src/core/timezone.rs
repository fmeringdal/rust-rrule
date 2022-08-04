use chrono::FixedOffset;
use chrono::Offset;
use chrono::TimeZone;

use chrono::Local;

/// A wrapper around `chrono_tz::Tz` that is able to represent `Local` timezone also.
///
/// # Usage
///
/// ```
/// use rrule::Tz;
///
/// let utc = Tz::utc();
/// let local = Tz::local();
/// // From `chrono_tz::Tz`
/// let berlin: Tz = chrono_tz::Tz::Europe__Berlin.into();
/// ```
#[derive(Clone, Copy)]
pub enum RRuleTz {
    /// Local timezone
    Local(Local),
    /// Timezone represented by `chrono_tz::Tz`
    Tz(chrono_tz::Tz),
}

impl RRuleTz {
    /// Name of timezone
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            RRuleTz::Local(_) => "Local",
            RRuleTz::Tz(tz) => tz.name(),
        }
    }

    /// Get UTC timezone
    #[must_use]
    pub fn utc() -> Self {
        Self::Tz(chrono_tz::UTC)
    }

    /// Get Local timezone
    #[must_use]
    pub fn local() -> Self {
        Self::Local(Local)
    }

    /// Check if timezone is the Local timezone
    #[must_use]
    pub fn is_local(&self) -> bool {
        match self {
            RRuleTz::Local(_) => true,
            RRuleTz::Tz(_) => false,
        }
    }
}

impl PartialEq for RRuleTz {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Local(_), Self::Local(_)) => true,
            (Self::Tz(l0), Self::Tz(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl From<Local> for RRuleTz {
    fn from(tz: Local) -> Self {
        Self::Local(tz)
    }
}

impl From<chrono_tz::Tz> for RRuleTz {
    fn from(tz: chrono_tz::Tz) -> Self {
        Self::Tz(tz)
    }
}

impl std::fmt::Debug for RRuleTz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local(tz) => tz.fmt(f),
            Self::Tz(tz) => tz.fmt(f),
        }
    }
}

impl std::fmt::Display for RRuleTz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local(_tz) => write!(f, "Local"),
            Self::Tz(tz) => tz.fmt(f),
        }
    }
}

#[derive(Clone, Copy)]
pub enum RRuleOffset {
    Local(FixedOffset),
    Tz(<chrono_tz::Tz as TimeZone>::Offset),
}

impl std::fmt::Debug for RRuleOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local(offset) => offset.fmt(f),
            Self::Tz(offset) => offset.fmt(f),
        }
    }
}

impl std::fmt::Display for RRuleOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local(offset) => offset.fmt(f),
            Self::Tz(offset) => offset.fmt(f),
        }
    }
}

impl Offset for RRuleOffset {
    fn fix(&self) -> FixedOffset {
        match self {
            RRuleOffset::Local(tz) => tz.fix(),
            RRuleOffset::Tz(tz) => tz.fix(),
        }
    }
}

impl TimeZone for RRuleTz {
    type Offset = RRuleOffset;

    fn from_offset(offset: &Self::Offset) -> Self {
        match offset {
            RRuleOffset::Local(offset) => Self::Local(Local::from_offset(offset)),
            RRuleOffset::Tz(offset) => Self::Tz(chrono_tz::Tz::from_offset(offset)),
        }
    }

    fn offset_from_local_date(
        &self,
        local: &chrono::NaiveDate,
    ) -> chrono::LocalResult<Self::Offset> {
        match self {
            Self::Local(tz) => tz
                .from_local_date(local)
                .map(|date| RRuleOffset::Local(*date.offset())),
            Self::Tz(tz) => tz
                .from_local_date(local)
                .map(|date| RRuleOffset::Tz(*date.offset())),
        }
    }

    fn offset_from_local_datetime(
        &self,
        local: &chrono::NaiveDateTime,
    ) -> chrono::LocalResult<Self::Offset> {
        match self {
            Self::Local(tz) => tz
                .from_local_datetime(local)
                .map(|date| RRuleOffset::Local(*date.offset())),
            Self::Tz(tz) => tz
                .from_local_datetime(local)
                .map(|date| RRuleOffset::Tz(*date.offset())),
        }
    }

    fn offset_from_utc_date(&self, utc: &chrono::NaiveDate) -> Self::Offset {
        match self {
            Self::Local(tz) => RRuleOffset::Local(*tz.from_utc_date(utc).offset()),
            Self::Tz(tz) => RRuleOffset::Tz(*tz.from_utc_date(utc).offset()),
        }
    }

    fn offset_from_utc_datetime(&self, utc: &chrono::NaiveDateTime) -> Self::Offset {
        match self {
            Self::Local(tz) => RRuleOffset::Local(*tz.from_utc_datetime(utc).offset()),
            Self::Tz(tz) => RRuleOffset::Tz(*tz.from_utc_datetime(utc).offset()),
        }
    }
}
