use crate::{RRuleError, WithError};
use std::ops::{
    Bound::{Excluded, Unbounded},
    RangeBounds,
};

/// Collects all dates, but once an error is found it will return the error
/// and not the items that where already found.
pub(crate) fn collect_or_error<T, TZ>(
    iterator: T,
    start: &Option<chrono::DateTime<TZ>>,
    end: &Option<chrono::DateTime<TZ>>,
    inclusive: bool,
    limit: u16,
) -> Result<Vec<chrono::DateTime<TZ>>, RRuleError>
where
    T: Iterator<Item = chrono::DateTime<TZ>> + WithError,
    TZ: chrono::TimeZone,
{
    match collect_with_error(iterator, start, end, inclusive, limit) {
        (_list, Some(err)) => Err(err),
        (list, None) => Ok(list),
    }
}

/// Helper function to collect dates given some filters.
///
/// In case where the iterator ended with an errors the error will be included,
/// otherwise the second value of the return tuple will be `None`.
pub(super) fn collect_with_error<T, TZ>(
    mut iterator: T,
    start: &Option<chrono::DateTime<TZ>>,
    end: &Option<chrono::DateTime<TZ>>,
    inclusive: bool,
    limit: u16,
) -> (Vec<chrono::DateTime<TZ>>, Option<RRuleError>)
where
    T: Iterator<Item = chrono::DateTime<TZ>> + WithError,
    TZ: chrono::TimeZone,
{
    let mut list = vec![];
    let mut err = None;
    // This loop should always end because `.next()` has build in limits
    // Once a limit is tripped it will break in the `None` case.
    while list.len() < limit as usize {
        let next = iterator.next();
        match next {
            Some(value) => {
                if is_in_range(&value, start, end, inclusive) {
                    list.push(value.clone());
                }
                if has_reached_the_end(&value, end, inclusive) {
                    // Date is after end date, so can stop iterating
                    break;
                }
            }
            None => {
                if iterator.has_err() {
                    err = iterator.get_err();
                }
                break;
            }
        }
    }
    // Make sure that the user always know when there are more dates.
    if list.len() > u16::MAX as usize {
        (
            list,
            Some(RRuleError::new_iter_err(format!(
                "List reached maximum limit (`{}`), so there might be more items.",
                u16::MAX
            ))),
        )
    } else {
        (list, err.cloned())
    }
}

/// Checks if `date` is after `end`.
fn has_reached_the_end<TZ: chrono::TimeZone>(
    date: &chrono::DateTime<TZ>,
    end: &Option<chrono::DateTime<TZ>>,
    inclusive: bool,
) -> bool {
    if inclusive {
        match end {
            Some(end) => !(..=end).contains(&date),
            None => false,
        }
    } else {
        match end {
            Some(end) => !(Unbounded, Excluded(end)).contains(date),
            None => false,
        }
    }
}

/// Helper function to determine if a date is within a given range.
pub(super) fn is_in_range<TZ: chrono::TimeZone>(
    date: &chrono::DateTime<TZ>,
    start: &Option<chrono::DateTime<TZ>>,
    end: &Option<chrono::DateTime<TZ>>,
    inclusive: bool,
) -> bool {
    // Should it include or not include the start and/or end date?
    if inclusive {
        match (start, end) {
            (Some(start), Some(end)) => (start..=end).contains(&date),
            (Some(start), None) => (start..).contains(&date),
            (None, Some(end)) => (..=end).contains(&date),
            (None, None) => true,
        }
    } else {
        match (start, end) {
            (Some(start), Some(end)) => (Excluded(start), Excluded(end)).contains(date),
            (Some(start), None) => (Excluded(start), Unbounded).contains(date),
            (None, Some(end)) => (Unbounded, Excluded(end)).contains(date),
            (None, None) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use chrono_tz::UTC;

    #[test]
    fn in_range_exclusive_start_to_end() {
        let inclusive = false;
        let start = UTC.ymd(2021, 10, 1).and_hms(8, 0, 0);
        let end = UTC.ymd(2021, 10, 1).and_hms(10, 0, 0);

        // In middle
        assert!(is_in_range(
            &UTC.ymd(2021, 10, 1).and_hms(9, 0, 0),
            &Some(start),
            &Some(end),
            inclusive
        ));
        // To small
        assert!(!is_in_range(
            &UTC.ymd(2021, 10, 1).and_hms(7, 0, 0),
            &Some(start),
            &Some(end),
            inclusive
        ));
        // To big
        assert!(!is_in_range(
            &UTC.ymd(2021, 10, 1).and_hms(11, 0, 0),
            &Some(start),
            &Some(end),
            inclusive
        ));
        // Equal to end
        assert!(!is_in_range(&end, &Some(start), &Some(end), inclusive));
        // Equal to start
        assert!(!is_in_range(&start, &Some(start), &Some(end), inclusive));
    }

    #[test]
    fn in_range_exclusive_start() {
        let inclusive = false;
        let start = UTC.ymd(2021, 10, 1).and_hms(8, 0, 0);

        // Just after
        assert!(is_in_range(
            &UTC.ymd(2021, 10, 1).and_hms(9, 0, 0),
            &Some(start),
            &None,
            inclusive
        ));
        // To small
        assert!(!is_in_range(
            &UTC.ymd(2021, 10, 1).and_hms(7, 0, 0),
            &Some(start),
            &None,
            inclusive
        ));
        // Bigger
        assert!(is_in_range(
            &UTC.ymd(2021, 10, 2).and_hms(8, 0, 0),
            &Some(start),
            &None,
            inclusive
        ));
        // Equal to start
        assert!(!is_in_range(&start, &Some(start), &None, inclusive));
    }

    #[test]
    fn in_range_exclusive_end() {
        let inclusive = false;
        let end = UTC.ymd(2021, 10, 1).and_hms(10, 0, 0);

        // Just before
        assert!(is_in_range(
            &UTC.ymd(2021, 10, 1).and_hms(9, 0, 0),
            &None,
            &Some(end),
            inclusive
        ));
        // Smaller
        assert!(is_in_range(
            &UTC.ymd(2021, 9, 20).and_hms(10, 0, 0),
            &None,
            &Some(end),
            inclusive
        ));
        // Bigger
        assert!(!is_in_range(
            &UTC.ymd(2021, 10, 2).and_hms(8, 0, 0),
            &None,
            &Some(end),
            inclusive
        ));
        // Equal to end
        assert!(!is_in_range(&end, &None, &Some(end), inclusive));
    }

    #[test]
    fn in_range_exclusive_all() {
        let inclusive = false;

        // Some date
        assert!(is_in_range(
            &UTC.ymd(2021, 10, 1).and_hms(9, 0, 0),
            &None,
            &None,
            inclusive
        ));
        // Smaller
        assert!(is_in_range(
            &UTC.ymd(2021, 9, 20).and_hms(10, 0, 0),
            &None,
            &None,
            inclusive
        ));
        // Bigger
        assert!(is_in_range(
            &UTC.ymd(2021, 10, 2).and_hms(8, 0, 0),
            &None,
            &None,
            inclusive
        ));
    }

    // ---------------- inclusive -----------------------

    #[test]
    fn in_range_inclusive_start_to_end() {
        let inclusive = true;
        let start = UTC.ymd(2021, 10, 1).and_hms(8, 0, 0);
        let end = UTC.ymd(2021, 10, 1).and_hms(10, 0, 0);

        // In middle
        assert!(is_in_range(
            &UTC.ymd(2021, 10, 1).and_hms(9, 0, 0),
            &Some(start),
            &Some(end),
            inclusive
        ));
        // To small
        assert!(!is_in_range(
            &UTC.ymd(2021, 10, 1).and_hms(7, 0, 0),
            &Some(start),
            &Some(end),
            inclusive
        ));
        // To big
        assert!(!is_in_range(
            &UTC.ymd(2021, 10, 1).and_hms(11, 0, 0),
            &Some(start),
            &Some(end),
            inclusive
        ));
        // Equal to end
        assert!(is_in_range(&end, &Some(start), &Some(end), inclusive));
        // Equal to start
        assert!(is_in_range(&start, &Some(start), &Some(end), inclusive));
    }

    #[test]
    fn in_range_inclusive_start() {
        let inclusive = true;
        let start = UTC.ymd(2021, 10, 1).and_hms(8, 0, 0);

        // Just after
        assert!(is_in_range(
            &UTC.ymd(2021, 10, 1).and_hms(9, 0, 0),
            &Some(start),
            &None,
            inclusive
        ));
        // To small
        assert!(!is_in_range(
            &UTC.ymd(2021, 10, 1).and_hms(7, 0, 0),
            &Some(start),
            &None,
            inclusive
        ));
        // Bigger
        assert!(is_in_range(
            &UTC.ymd(2021, 10, 2).and_hms(8, 0, 0),
            &Some(start),
            &None,
            inclusive
        ));
        // Equal to start
        assert!(is_in_range(&start, &Some(start), &None, inclusive));
    }

    #[test]
    fn in_range_inclusive_end() {
        let inclusive = true;
        let end = UTC.ymd(2021, 10, 1).and_hms(10, 0, 0);

        // Just before
        assert!(is_in_range(
            &UTC.ymd(2021, 10, 1).and_hms(9, 0, 0),
            &None,
            &Some(end),
            inclusive
        ));
        // Smaller
        assert!(is_in_range(
            &UTC.ymd(2021, 9, 20).and_hms(10, 0, 0),
            &None,
            &Some(end),
            inclusive
        ));
        // Bigger
        assert!(!is_in_range(
            &UTC.ymd(2021, 10, 2).and_hms(8, 0, 0),
            &None,
            &Some(end),
            inclusive
        ));
        // Equal to end
        assert!(is_in_range(&end, &None, &Some(end), inclusive));
    }

    #[test]
    fn in_range_inclusive_all() {
        let inclusive = true;

        // Some date
        assert!(is_in_range(
            &UTC.ymd(2021, 10, 1).and_hms(9, 0, 0),
            &None,
            &None,
            inclusive
        ));
        // Smaller
        assert!(is_in_range(
            &UTC.ymd(2021, 9, 20).and_hms(10, 0, 0),
            &None,
            &None,
            inclusive
        ));
        // Bigger
        assert!(is_in_range(
            &UTC.ymd(2021, 10, 2).and_hms(8, 0, 0),
            &None,
            &None,
            inclusive
        ));
    }
}
