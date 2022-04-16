# Change log

All notable changes to this project will be documented in this file.
This project follows the [Semantic Versioning standard](https://semver.org/).

## Version 0.7.2 (2022-04-16)

### Fixed

- Fix [#33](https://github.com/fmeringdal/rust-rrule/issues/33)
- Fix [#34](https://github.com/fmeringdal/rust-rrule/issues/34)
- Many tests were ignored because they were either invalid or the test didn't pass. Most ignored tests are now deleted or the code is fixes so that they pass.
- Better error handling

### Changed

Internal refactorings:

- Added `ParseError` and `ValidationError` which contains more specific errors during the parsing and validation phase respectively.

## Version 0.7.1 (2022-03-07)

### Fixed

- Fix [#48](https://github.com/fmeringdal/rust-rrule/issues/48)

## Version 0.7.0 (2021-xx-xx)

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
- `NWeekdayIdentifier` replaced with new version of `NWeekday`.
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

## Pre version 0.6.0 (2021-07-02)

All changes before 2021-07-02 where not documented.
This is everything before and including: fa8308944a4d2ead0a6ccfa6ee53b76b399e045f
