# Change log
All notable changes to this project will be documented in this file.
This project follows the [Semantic Versioning standard](https://semver.org/).

## Version 0.6.0 (2021-xx-xx)

### Added
- Added `forbid(unsafe_code)` into repo. We don't ever need unsafe code.

### Changed
- Renamed `Frequenzy` to `Frequency`
- Add limit to `all()`, prevent infinite loops.
- Added/Improved error handling for RRule iterator.

### Deprecated

### Removed

### Fixed
- Replaced panic on incorrect datetime with error.
- Fix timezone conversions when `TZID` and `Z` is present.
- Stabilized `RRule` parsing from string. Fixed possible panics.

### Security

## Pre version 0.5.9 (2021-07-02)
All changes before 2021-07-02 where not documented.
This is everything before and including: fa8308944a4d2ead0a6ccfa6ee53b76b399e045f