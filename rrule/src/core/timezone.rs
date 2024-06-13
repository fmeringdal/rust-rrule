use chrono::Local;

/// A wrapper around `chrono_tz::Tz` that is able to represent `Local` timezone also.
///
/// # Usage
///
/// ```
/// use rrule::Tz;
///
/// let local = Tz::LOCAL;
/// // From `chrono_tz::Tz`
/// let utc: Tz = chrono_tz::Tz::UTC.into();
/// let berlin: Tz = chrono_tz::Tz::Europe__Berlin.into();
/// ```
#[derive(Clone, Copy)]
pub enum Tz {
    /// Local timezone
    Local(Local),
    /// Timezone represented by `chrono_tz::Tz`
    Tz(chrono_tz::Tz),
}

impl Tz {
    /// Name of timezone
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Local(_) => "Local",
            Self::Tz(tz) => tz.name(),
        }
    }

    /// Check if timezone is the Local timezone
    #[must_use]
    pub fn is_local(&self) -> bool {
        match self {
            Self::Local(_) => true,
            Self::Tz(_) => false,
        }
    }

    /// Local timezone
    #[allow(non_upper_case_globals)]
    pub const LOCAL: Self = Self::Local(Local);
}
