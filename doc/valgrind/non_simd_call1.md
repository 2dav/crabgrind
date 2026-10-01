Execution of a function on the real CPU, bypassing Valgrind's simulation

Wraps `VALGRIND_NON_SIMD_CALL1`. This transfers control from the simulated CPU
to the host CPU to execute the provided function.

The function `f` **must** accept the current [`ThreadId`](ThreadId) as its first
argument.

```text
fn f(ThreadId, usize) -> usize
```

The value returned by the provided function is propagated to the return value of
the request.

# Reliability Warning

These calls are not entirely reliable. Avoid calling functions that depend on
global variables, libc (e.g., `printf`, [`std::println`][std.println]), or
dynamic linking. Such entanglements frequently cause Valgrind to crash. Use only
for simple, self-contained logic.

# Safety

The callback `f` executes on the real CPU, outside Valgrind's simulation.
The caller must ensure that `f`:

- Does not call any function that Valgrind has intercepted or replaced
  (e.g. `malloc`, `free`, `printf`, or any libc function). Doing so
  corrupts Valgrind's internal state and may crash the process.
- Does not perform heap allocation or deallocation. Allocations inside
  the callback are invisible to Valgrind's leak detector and heap tracker.
- Does not rely on or modify Valgrind's shadow state for any memory it
  touches. Memory writes inside the callback are invisible to all
  Valgrind tools (Memcheck, Helgrind, DRD, Cachegrind, Callgrind).

When not running under Valgrind, the callback is not invoked and this
function returns `0`.

## Note

Requires Valgrind **3.0** or higher.

[std.println]: https://doc.rust-lang.org/std/macro.println.html
