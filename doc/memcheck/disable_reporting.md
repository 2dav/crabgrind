Temporary disabling of error reporting for a memory range

This function suppresses addressing errors for the provided byte range and
returns a guard. Reporting remains disabled until the guard is dropped.

When this guard is dropped, error reporting is automatically re-enabled for the
associated memory range. This ensures that reporting is restored even if the
function panics or exits early.

See also [`disable_error_reporting`](disable_error_reporting),
[`enable_error_reporting`](enable_error_reporting).

## Note

Requires Valgrind **3.4** or higher.
