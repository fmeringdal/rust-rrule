# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixes

- Fixed typos in security docs 
- Fixed an inifinite loop issue in the iteration phase where the counter date increment method did not increment the counter date and it was unable to make progress. This was solved by using a custom date time implementation for the counter date.
- Fixes issue where iterations that passed a daylight saving time had incorrect hour.

## 0.9.0 (2022-07-18)

### Changed

- MSRV is bumped to `v1.57.0` from `v1.56.1`
- Internal `Time` type has been replaced by `chrono::NaiveTime`
- Iterator module has been refactored
- The `parser` module has been rewritten from scratch to be more Rust idiomatic and use terminology more consistent with the RFC. The end result of the `parser` phase is now a `Grammar`.
- Added `cargo clippy -- -D warnings` back to the CI

### Removed

- `RRule` no longer implements `FromStr` or `Deserialize`. Use `RRuleSet` for these use-cases.

### Fixes

- The `DTSTART` and `UNTIL` values were not synced before this release. They are now synced according to the RFC.
- Fix #61 where `collect_with_error` would not return an error in the case where `RRuleSet` iteration had an error.

## 0.8.0 (2022-06-21)

### Changed

- `RRule` represents only RRULE in the iCalendar specification. And `RRuleSet` is for the whole iCalendar string.
- `RRule` has two stages, `Unvalidated` and `Validated`. When you initialize it, it is `Unvalidated` and by calling the `validate` method, it will change to `Validated`.
- All fields of `RRule` and `RRuleSet` are private, instead there are a set of new methods to get and set values of fields on both structs. It's been done to have a safer API.
- `tz` is removed from all structs. It was a duplicated copy of the timezone inside the `dt_start`. Instead, you can control it by having a proper timezone for [`RRuleSet::dt_start`].

### Removed

- Removed `dt_start` and `dt_end` from `RRule`.
- There is no `Default` for `RRuleSet`, instead use `new(UTC.ymd(1970, 1, 1).and_hms(0, 0, 0))` method to have the same behavior.
- Removed `RRuleProperties`, instead you can use `RRule<Unvalidated>`.
- `DateFilter` trait has been removed, since now we can only iterate over `RRuleSet` and the methods are inside `RRuleSet` itself.
- The iterator over `RRule` is not public anymore.

## 0.7.3 (2022-05-05)

### Added

- There is a new `serde` feature to enable serialization and deserialization on `RRuleProperties` and `RRule`.
- `RRuleProperties` and `RRule` structs now implement `Display` and `FromStr` to convert from and to strings.

### Changed

- `dt_start` and `tz` moved from `RRuleProperties` to `RRule`
 
## 0.7.2 (2022-04-16)

### Fixed

- Fix [#33](https://github.com/fmeringdal/rust-rrule/issues/33)
- Fix [#34](https://github.com/fmeringdal/rust-rrule/issues/34)
- Many tests were ignored because they were either invalid or the test didn't pass. Most ignored tests are now deleted or the code is fixed so that they pass.
- Better error handling

### Changed

Internal refactorings:

- Added `ParseError` and `ValidationError` which contains more specific errors during the parsing and validation phase respectively.

## 0.7.1 (2022-03-07)

### Fixed

- Fix [#48](https://github.com/fmeringdal/rust-rrule/issues/48)

## 0.7.0 (2021-02-04)

### Added

- Added `forbid(unsafe_code)` into repo. We don't ever need unsafe code.
- Added additional testing crates: `rrule-debugger` and `rrule-afl-fuzz`.
  (only used internally and for debugging)
- Added `examples` folder and moved some code from documentation into examples.
- New and improved error handling consolidated into one object, `RRuleError`.
- Added new trait `DateFilter` for implementing methods like:
  `all_with_error`, `all_before_with_error`, `all_after_with_error` and `all_between_with_error` and moved/added methods like `all`, `just_before`, etc.
- Added arbitrary limits for safety reasons.
  See [ReadMe](README.md#validation_limits) for more info.
- Improved `rrule` command line tool.
- New `WithError` trait, implemented for `RRuleIter` and `RRuleSetIter`.

### Changed

- License change, from MIT to (MIT or Apache 2.0). (#27)
- Massive code restructuring. Split into `core`, `iter`, `parser` and `validator`.
- Renamed `Frequenzy` to `Frequency`.
- Add limit to `all()`, prevent infinite loops.
- `ParsedOptions` and `Options` are merged together into `RRuleProperties`. (#22)
- `Options` functions changed, for example `byminute` -> `by_minute`.
- `by_easter` is now opt-in with feature flag `by-easter`. (#26)
- `RRule` can only be crated using `new` function with a valid `RRuleProperties`.
- `RRule.option` is no longer public, but can be read by using `get_properties()`.
- `RRuleIter` and `RRuleSetIter` are now part of the public API.
- `NWeekday` has totally changed, but serves the same purpose.
- Updated `chrono-tz` from `0.5.3` to `0.6.0`.
- Function `all` was moved to `DateFilter` and returns a `Result` now.

### Deprecated

### Removed

- `RRuleSet::new()` replaced with `RRuleSet::default()`.
- `ParsedOptions` and `Options` are now combined as `RRuleProperties`. (#22)
- `NWeekdayIdentifier` replaced with new of `NWeekday`.
- `by_n_weekday` field removed from `ParsedOptions`, combined into `by_weekday`.
- `concat` and `build` in `Options` are removed, no longer needed.
- Removed `serde` dependency. (#21)
- Removed function `between`, use `DateFilter::all_between` instead.
- Removed function `after`, use `DateFilter::just_after` instead.
- Removed function `before`, use `DateFilter::just_before` instead.

### Fixed

- Replaced panic on incorrect datetime with error.
- Fix timezone conversions when `TZID` and `Z` is present.
- Stabilized `RRule` parsing from string. Fixed a lot of possible panics.

### Security

- Added security notice to [README.md](README.md).
- The validation of `RRule` improved the stability of the crate a lot.

## Pre 0.6.0 (2021-07-02)

All changes before 2021-07-02 where not documented.
This is everything before and including: fa8308944a4d2ead0a6ccfa6ee53b76b399e045f
