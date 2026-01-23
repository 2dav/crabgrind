Prints formatted text to the Valgrind output channel with a stack trace.

This macro wraps the C client request `VALGRIND_PRINTF_BACKTRACE`. It behaves
similarly to [`std::println`](std.println) in terms of syntax, accepting a
format string and a list of arguments.

# Mechanics

The formatted string is sent to the Valgrind log. Immediately following the
message, Valgrind prints the current stack trace (backtrace) to the same log.

**Note:** If the process runs **without** Valgrind, this macro does nothing (it
is a no-op). No message or stack trace is generated.

## Note

Requires Valgrind **3.0** or higher.
