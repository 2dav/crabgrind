Indicates if malloc is replaced by the active Valgrind tool.

Wraps `VALGRIND_REPLACES_MALLOC`.
Whether `malloc` is replaced by the tool (e.g., memcheck) or left as-is (e.g.,
cachegrind, callgrind, or if is not running under Valgrind)

## Note

Requires Valgrind **3.27** or higher.
