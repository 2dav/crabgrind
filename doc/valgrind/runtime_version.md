Retrieval of the Valgrind version that the executable is running under

Wraps `VALGRIND_RUNNING_VERSION`. The value returned is a decimal value
in the form *Mmm* where 'M' is the major version and 'mm' is the minor version.
Returns 0 if the executable is not running under Valgrind, or is running under
a version older than **3.28**.

## Note

Requires Valgrind **3.28** or higher.
