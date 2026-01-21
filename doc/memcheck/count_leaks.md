Retrieves leak statistics from the most recent leak check.

This function does not trigger a new scan; it reads the results stored by the
previous leak check operation (e.g., [`leak_check`] or the final summary at exit).

## Note
Requires Valgrind **3.0** (Released 2003).
