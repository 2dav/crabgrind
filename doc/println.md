Prints formatted text to the Valgrind output channel.

This macro wraps the C client request `VALGRIND_PRINTF`. It behaves
similarly to [`std::println`](std.println) in terms of syntax, accepting a format
string and a list of arguments.

# Mechanics
The formatted string is sent to the Valgrind log (typically stderr).
Output is buffered in the client process until the Valgrind tool
initializes, at which point the buffer is flushed.

**Note:** If the process runs **without** Valgrind, this macro does
nothing (it is a no-op). It does not print to stdout or stderr.

## Note
Requires Valgrind **3.0** (Released 2003).

[std.println]: https://doc.rust-lang.org/nightly/std/macro.println.html
