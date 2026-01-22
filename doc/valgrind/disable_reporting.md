Disables error reporting for the current thread.

When this guard is dropped, error reporting is automatically re-enabled for the associated
memory range. This ensures that reporting is restored even if the function panics or exits early.

See also [`disable_error_reporting`], [`enable_error_reporting`]

## Note
Requires Valgrind **3.0** or higher.
