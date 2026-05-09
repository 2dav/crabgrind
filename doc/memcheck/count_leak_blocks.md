Retrieval of leaked block counts from the most recent leak check

This function behaves identically to [`count_leaks`](count_leaks), but populates
the [`LeaksCount`](LeaksCount) fields with the block counts rather than byte
counts.

## Note

Requires Valgrind **3.2** or higher.
