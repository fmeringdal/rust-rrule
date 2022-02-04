<h1 align="center">RRule.rs</h1>
<p align="center">A pure and efficient Rust implementation of recurrence rules as defined in the iCalendar RFC.</p>
<p align="center">
  <a href="https://travis-ci.com/fmeringdal/rust_rrule"><img src="https://travis-ci.com/fmeringdal/rust_rrule.svg?branch=main" /></a>
  <a href="https://github.com/fmeringdal/rust-rrule/actions"><img src="https://img.shields.io/github/checks-status/fmeringdal/rust-rrule/main" /></a>
  <a href="https://codecov.io/gh/fmeringdal/rust_rrule"><img src="https://codecov.io/gh/fmeringdal/rust_rrule/branch/main/graph/badge.svg" /></a>
  <a href="https://crates.io/crates/rrule"><img src="https://img.shields.io/crates/v/rrule.svg" /></a>
  <a href="https://docs.rs/rrule/latest/rrule/"><img src="https://img.shields.io/badge/docs-rrule-blue" /></a>
</p>

## Warning

This crate is not production ready yet. Dates and recurrence rules are quite complicated and
takes time to get right. Even though this crate is well tested (high code coverage), there are still
tests missing regarding edge cases in stuff like DST, time zone and rfc_string parsing
(contributions are very welcome!).
Use at your own risk!

## Specification

This crate follows the [iCalendar (RFC-5545) specification][ICal_spec] for the "Recurrence Rule".
The Recurrence Rule spec corresponds to the `RRule` object in this crate.
In addition, it allows for adding the ["DTSTART" property][DTSTART_property] separated by a newline.

The crate allows for a "BYEASTER" filter. But this is opt-in with the feature flag `"by-easter"`.

### RRuleSet

`RRuleSet` allows for a combination or `RRule`s and some other properties.
 - List of [RRules](https://icalendar.org/iCalendar-RFC-5545/3-8-5-3-recurrence-rule.html):
 Allows multiple RRules to be combined. (Union, `A ∪ B`)
 - List of [RDates](https://icalendar.org/iCalendar-RFC-5545/3-8-5-2-recurrence-date-times.html):
 A list of datetime combinations to always include. (Union, `A ∪ B`)
 - List of ExRule (see note below):
 Allows of RRules that are removed from the results. (Complement `A \ B` or `A - B`)
 - List of [ExDate](https://icalendar.org/iCalendar-RFC-5545/3-8-5-1-exception-date-times.html):
 A list of datetime combinations to always exclude. (Complement `A \ B` or `A - B`)

Note: "EXRULE" was originally part of [RFC 2445](https://datatracker.ietf.org/doc/html/rfc2445),
[RFC 5545][ICal_spec] supersedes this specification.
But "EXRULE" works exactly the same als "RRULE" excepts it excludes dates.

If you notice that the implementation differs from the specifications above, please open an issue.

## Library Usage

```rust
use rrule::RRule;

// RRule that starts 2012.02.01 and occurs daily for 3 days.
let rrule: RRule = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3".parse().unwrap();

// Set hard limit in case of infinitely recurring rules.
let limit = 100;
// Get all recurrences of the rrule
let recurrences = rrule.all(limit).unwrap();
assert_eq!(recurrences.len(), 3);
```

See more examples at [docs.rs](https://docs.rs/rrule)

## Command Line Tool Usage
Install the command line tool with:
```bash
cargo install rrule --features="cli-tool"
```

Then run it with:
```bash
rrule "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3"
```

## Safety and Security
<a name="safety"></a>
We try to make the crate safe for use by arbitrary users/public APIs,
but we can not guaranty the security of it at this point.

The 3 problems we are currently mostly worried about are:

### Denial of Service (DoS) by panic
If the user is able to trigger a case where is it able to panic
the code it could crash an application or servers.
This can limit this by using [catch_unwind](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html).
Make sure to read the documentation! This does not work if the
[`panic = "abort"`](https://doc.rust-lang.org/cargo/reference/profiles.html#panic)
setting is set to abort.
It is not advices to reuse `RRule`, `RRuleSet` or iterators after it has panicked.

Disabling overflow checks using compiler flags might result in unexpected results and crashes.
So this is strongly discouraged.

When the [validation limits](#validation_limits) are disabled this problem will be much more
prevalent. Numbers might overflow in some cases.

### Denial of Service (DoS) by CPU exhaustion
The spec allows of infinitely recurring events or by searching of a datetime that meets the
requirements but does not exist. There are various protections for this build into the crate.
But in order to hit these limits it might take a few seconds/minutes depending on the CPU speed.

This problem can be mitigating by spawning the process in a separate thread and stopping the thread
if it hits the timeout. On decent CPUs this might not be a big issue.

Note that by disabling the [validation limits](#validation_limits) this problem will be
made MUCH more significant.

### Denial of Service (DoS) by memory exhaustion
The spec allows of infinitely recurring events. Thus, the iterator might be practically infinite.
So when not setting a limit over the iterator it might create a list of events that practically
never ends. And thus will continue until is crashes or hangs the system.

This problem can be easily mitigated by limiting the amount of events expected.
This is also the reason why `rrule.all(limit)` function takes a limit.

## Limitation and limits

All dates are limited to the range or years +/-262_000[^1] because of [Chrono][Chrono] limits.
See [Chrono's limits for more info](https://github.com/chronotope/chrono#limitations).

Supported timezones are limited to by the timezones that [Chrono-Tz][Chrono-Tz] supports.
This is equivalent to the IANA database.
See [Chrono-Tz's limits for more info](https://github.com/chronotope/chrono-tz/#limiting-the-timezone-table-to-zones-of-interest).

### Validation Limits
<a name="validation_limits"></a>
Because the specifications does give a lot of flexibility this can be [abused very easily](#safety).
In order to prevent most of the abuse we have imposed arbitrary limitation on the crate.

These limitations are reasonable in most use cases, but we do allow other developers to opt-out
if needed.

Limitations:
| Description                      | Arbitrary Limit       | Crate Limit                 |
|----------------------------------|-----------------------|-----------------------------|
| Year range                       | -10_000..=10_000      | -262_000..=262_000 (Chrono) |
| Max interval with freq Yearly    | 10_000 ( 10000 years) | 65_535 (u16::MAX)           |
| Max interval with freq Monthly   |  1_000 (~83 years)    | 65_535 (u16::MAX)           |
| Max interval with freq Weekly    |  1_000 (~19 years)    | 65_535 (u16::MAX)           |
| Max interval with freq Daily     | 10_000 (~27 years)    | 65_535 (u16::MAX)           |
| Max interval with freq Hourly    | 10_000 (~416 days)    | 65_535 (u16::MAX)           |
| Max interval with freq Minutely  | 10_000 (~7 days)      | 65_535 (u16::MAX)           |
| Max interval with freq Secondly  | 50_000 (~13 hours)    | 65_535 (u16::MAX)           |
| Formula loop limit               | 10_000                | 65_535 (u16::MAX)           |
| Iteration limit                  | 100_000               | 4_294_967_295 (u32::MAX)    |

By default, the "Arbitrary Limit" are used. If you instead want to use the "Crate Limit".
Make sure you [understand the risks that come with this](#safety).

<details>
    <summary>Yes, I understand the risks, but I want to disable the arbitrary limits.</summary>

Make sure you actually need this before enabling it.

<span style="color:red;font-weight:bold">**DANGER!**</span><br/>
To disable all arbitrary limits you can enable the `"no-validation-limits"` feature flag.

</details>

## Inspired by

- [python-dateutil library](http://labix.org/python-dateutil/)
- [rrule.js](https://github.com/jakubroztocil/rrule)

## License

The code in this project is licensed under the [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) license.

All contributions to this project will be similarly licensed.

[Chrono]: https://github.com/chronotope/chrono
[Chrono-Tz]: https://github.com/chronotope/chrono-tz/
[ICal_spec]: https://icalendar.org/iCalendar-RFC-5545/3-3-10-recurrence-rule.html
[DTSTART_property]: https://icalendar.org/iCalendar-RFC-5545/3-8-2-4-date-time-start.html

[^1]: See [validation limits](#validation_limits) sections more info.