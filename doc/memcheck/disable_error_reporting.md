Suppresses reporting of addressing errors for a specific memory range.

Use this to stop Memcheck from reporting illegal address errors (accessing
unaddressable memory) within the specified range.

This is distinct from marking memory as defined; it merely silences errors
related to whether the addresses can be legally accessed.

See [`enable_error_reporting`] for resuming reporting. 

## Note
Requires Valgrind **3.4** or higher.
