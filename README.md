# RRule in rust

[![Build](https://travis-ci.com/fmeringdal/rust_rrule.svg?branch=main)](https://travis-ci.com/fmeringdal/rust_rrule)
[![codecov](https://codecov.io/gh/fmeringdal/rust_rrule/branch/main/graph/badge.svg)](https://codecov.io/gh/fmeringdal/rust_rrule)
[![crates.io](https://img.shields.io/crates/v/rrule.svg)](https://crates.io/crates/rrule)

A pure Rust (partial) implementation of recurrence rules as defined in the iCalendar RFC.

# Usage

```rust
extern crate rrule;

use rrule::build_rrule;

let mut rrule_set = build_rule("RRULE:UNTIL=19990404T110000Z;DTSTART;TZID=America/New_York:19990104T110000Z;FREQ=WEEKLY;BYDAY=TU,WE");

// Get all occurrences of the rrule set
let occurences = rrule_set.all();
```

# Documentation and more examples

[Documentation and more examples](https://docs.rs/rrule)

# Inspired by

- [python-dateutil library](http://labix.org/python-dateutil/)
- [rrule.js](https://github.com/jakubroztocil/rrule)

# Todos

- Validations
- tests for minutes and seconds frequencies
- cache
