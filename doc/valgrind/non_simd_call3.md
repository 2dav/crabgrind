Executes a function on the real CPU, bypassing Valgrind's simulation.

Wraps `VALGRIND_NON_SIMD_CALL3`. This transfers control from the simulated
CPU to the host CPU to execute the provided function.

The function `f` **must** accept the current [`ThreadId`] as its first argument.
```text
fn f(ThreadId, usize, usize, usize) -> usize
```

Value returned by the provided function is propagated to return value of request.

# Reliability Warning
These calls are not entirely reliable. Avoid calling functions that depend on
global variables, libc (e.g. `printf`, [`std::println`][std.println]), or dynamic linking. Such
entanglements frequently cause Valgrind to crash. Use only for simple,
self-contained logic.

## Note
Requires Valgrind **3.0** or higher.

[std.println]: https://doc.rust-lang.org/std/macro.println.html
