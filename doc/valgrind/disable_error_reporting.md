Disables error reporting for the current thread.

This function wraps the `VALGRIND_DISABLE_ERROR_REPORTING` macro.

# Behavior

The first call stops Valgrind from reporting errors for this thread. Subsequent
calls increment the internal counter but do not further alter the reporting
state. Reporting remains suppressed until a matching number of calls to
[`enable_error_reporting`](enable_error_reporting) are made.

# Threading

The suppression state is thread-local. Child threads do **not** inherit the
disabled state from their parents; they are always created with error reporting
enabled.

## Note

Requires Valgrind **3.0** or higher.
