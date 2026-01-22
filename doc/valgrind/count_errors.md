Retrieves the number of errors recorded by the Valgrind tool.

This function wraps the `VALGRIND_COUNT_ERRORS` macro. It returns the
current count of errors encountered during execution.

# Tool Specifics
The return value depends entirely on the active tool. Tools that report
errors (e.g. Memcheck) return a non-zero count. Tools that do not report
errors in the traditional sense (e.g., Cachegrind) will always return zero.

# Usage
This is primarily useful in automated test harnesses. When combined with
the `--log-fd=-1` option, Valgrind runs silently (suppressing standard output),
allowing the client program to inspect the error count programmatically.

## Note
Requires Valgrind **3.0** (Released 2003).
