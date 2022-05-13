use crate::core::utils::{collect_or_error, collect_with_error};
use crate::core::DateTime;
use crate::{RRuleError, WithError};

pub trait DateFilter
where
    Self: Sized + Iterator<Item = DateTime> + WithError,
{
    /// Returns all the recurrences of the rrule.
    ///
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    fn all(self, limit: u16) -> Result<Vec<DateTime>, RRuleError> {
        collect_or_error(self, &None, &None, true, limit)
    }

    /// Returns all the recurrences of the rrule.
    ///
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    ///
    /// In case the iterator ended with an error, the error will be included,
    /// otherwise the second value of the return tuple will be `None`.
    fn all_with_error(self, limit: u16) -> (Vec<DateTime>, Option<RRuleError>) {
        collect_with_error(self, &None, &None, true, limit)
    }

    /// Returns the last recurrence before the given datetime instance.
    ///
    /// The `inclusive` keyword defines what happens if `before` is a recurrence.
    /// With `inclusive == true`, if `before` itself is a recurrence, it will be returned.
    fn just_before(
        self,
        before: DateTime,
        inclusive: bool,
    ) -> Result<Option<DateTime>, RRuleError> {
        Ok(
            collect_or_error(self, &None, &Some(before), inclusive, u16::MAX)?
                .last()
                .cloned(),
        )
    }

    /// Returns all the recurrences of the rrule before the given date.
    ///
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    ///
    /// In case the iterator ended with an error, the error will be included,
    /// otherwise the second value of the return tuple will be `None`.
    fn all_before_with_error(
        self,
        before: DateTime,
        inclusive: bool,
        limit: u16,
    ) -> (Vec<DateTime>, Option<RRuleError>) {
        collect_with_error(self, &None, &Some(before), inclusive, limit)
    }

    /// Returns the last recurrence after the given datetime instance.
    ///
    /// The `inclusive` keyword defines what happens if `after` is a recurrence.
    /// With `inclusive == true`, if `after` itself is a recurrence, it will be returned.
    fn just_after(self, after: DateTime, inclusive: bool) -> Result<Option<DateTime>, RRuleError> {
        Ok(collect_or_error(self, &Some(after), &None, inclusive, 1)?
            .first()
            .cloned())
    }

    /// Returns all the recurrences of the rrule after the given date.
    ///
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    ///
    /// In case the iterator ended with an error, the error will be included,
    /// otherwise the second value of the return tuple will be `None`.
    fn all_after_with_error(
        self,
        after: DateTime,
        inclusive: bool,
        limit: u16,
    ) -> (Vec<DateTime>, Option<RRuleError>) {
        collect_with_error(self, &Some(after), &None, inclusive, limit)
    }

    /// Returns all the recurrences of the rrule between after and before.
    ///
    /// The `inclusive` keyword defines what happens if after and/or before are
    /// themselves recurrences. With `inclusive == true`, they will be included in the
    /// list, if they are found in the recurrence set.
    fn all_between(
        self,
        start: DateTime,
        end: DateTime,
        inclusive: bool,
    ) -> Result<Vec<DateTime>, RRuleError> {
        collect_or_error(self, &Some(start), &Some(end), inclusive, u16::MAX)
    }

    /// Returns all the recurrences of the rrule after the given date and before the other date.
    ///
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    ///
    /// In case the iterator ended with an error, the error will be included,
    /// otherwise the second value of the return tuple will be `None`.
    fn all_between_with_error(
        self,
        start: DateTime,
        end: DateTime,
        inclusive: bool,
        limit: u16,
    ) -> (Vec<DateTime>, Option<RRuleError>) {
        collect_with_error(self, &Some(start), &Some(end), inclusive, limit)
    }
}
