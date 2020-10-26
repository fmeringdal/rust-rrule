<h1 align="center">RRule.rs</h1>
<p align="center">A pure and efficient Rust implementation (partial)  of recurrence rules as defined in the iCalendar RFC..</p>
<p align="center">
  <a href="https://travis-ci.com/fmeringdal/rust_rrule"><img src="https://travis-ci.com/fmeringdal/rust_rrule.svg?branch=main" /></a>
  <a href="https://codecov.io/gh/fmeringdal/rust_rrule"><img src="https://codecov.io/gh/fmeringdal/rust_rrule/branch/main/graph/badge.svg" /></a>
  <a href="https://crates.io/crates/rrule"><img src="https://img.shields.io/crates/v/rrule.svg" /></a>
</p>


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
