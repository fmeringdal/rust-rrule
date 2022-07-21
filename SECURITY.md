## Security

<a name="safety"></a>
We try to make the crate safe for use by arbitrary users/public APIs,
but we can not guarantee the security of it at this point.

The 3 problems we are currently mostly worried about are:

### Denial of Service (DoS) by panic

If the user is able to trigger a case where is it able to panic
the code it could crash the application or server.
This can be limited by using [catch_unwind](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html).
Make sure to read the documentation! This does not work if the
[`panic = "abort"`](https://doc.rust-lang.org/cargo/reference/profiles.html#panic)
setting is set to abort.
It is not advised to reuse `RRule`, `RRuleSet` or iterators after it has panicked.

Disabling overflow checks using compiler flags might result in unexpected results and crashes.
So this is strongly discouraged.

When the [validation limits](#validation_limits) are disabled this problem will be much more
prevalent. Numbers might overflow in some cases.

### Denial of Service (DoS) by CPU exhaustion

The spec allows for infinitely recurring events or searches for a datetime that meets the
requirements but does not exist. There are various protections for this built into the crate.
But in order to hit these limits it might take a few seconds depending on the CPU speed.

This problem can be mitigated by spawning the process in a separate thread and stopping the thread
if it hits the timeout. On decent CPUs this might not be a big issue.

Note that by disabling the [validation limits](#validation_limits) this problem will be
made MUCH more significant.

### Denial of Service (DoS) by memory exhaustion

The spec allows for infinitely recurring events. Thus, the iterator might be practically infinite.
So when not setting a limit over the iterator it might create a list of events that practically
never ends. And thus will continue until it crashes or hangs the system.

This problem can be easily mitigated by limiting the amount of events expected.
This is also the reason why the `rrule.all(limit)` function takes a limit.