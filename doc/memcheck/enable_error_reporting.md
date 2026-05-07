Resumption of address error reporting for a specific memory range

Use this to re-enable detection of address errors (unaddressable bytes) within
the specified range after it has been disabled via
[`disable_error_reporting`](disable_error_reporting).

This only affects the reporting of address errors (A-bits). Undefined value
errors (V-bits) are unaffected.

## Note

Requires Valgrind **3.4** or higher.
