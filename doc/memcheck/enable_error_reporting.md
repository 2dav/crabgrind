Resumes reporting of addressing errors for a specific memory range.

Use this to re-enable detection of illegal addresses (unaddressable bytes)
within the specified range after it has been disabled via
[`disable_error_reporting`](disable_error_reporting).

This only affects the reporting of addressing errors (A-bits). Undefined value
errors (V-bits) are unaffected.

## Note

Requires Valgrind **3.4** or higher.
