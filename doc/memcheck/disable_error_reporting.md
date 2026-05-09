Suppression of address error reporting for a specific memory range

Use this to stop Memcheck from reporting address errors (accesses to
"unaddressable" memory) within the specified range. This is distinct from
marking memory as "defined" - it merely silences errors about accesses to
"unaddressable" memory.

See [`enable_error_reporting`](enable_error_reporting) for resuming reporting.

## Note

Requires Valgrind **3.4** or higher.
