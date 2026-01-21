Retrieves the count of leaked blocks from the most recent leak check.

This function behaves identically to [`count_leaks`], but populates the [`LeaksCount`]
fields with the quantity of memory blocks rather than the number of bytes.

## Note
Requires Valgrind **3.2** (Released 2005).
