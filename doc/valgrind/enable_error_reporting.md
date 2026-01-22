Re-enables error reporting for the current thread.

This function wraps the `VALGRIND_ENABLE_ERROR_REPORTING` macro. It decrements
the internal suppression counter established by [`disable_error_reporting`].

# Behavior
If the internal counter reaches zero as a result of this call, error
reporting resumes for the current thread.

## Note
Requires Valgrind **3.0** or higher.
