Check of `malloc` replacement by the active Valgrind tool

Wraps `VALGRIND_REPLACES_MALLOC`. Returns `true` if the active Valgrind tool
replaces `malloc` (e.g., Memcheck). Returns `false` if the tool does not replace
`malloc` (e.g., Cachegrind, Callgrind) or if not running under Valgrind.

## Note

Requires Valgrind **3.27** or higher.
