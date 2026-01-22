Disassociates all pieces of memory outside a specific range from the pool.

Wraps `VALGRIND_MEMPOOL_TRIM`. This effectively "frees" any blocks
tracked by the pool that fall entirely outside the specified address range.

## Note
Requires Valgrind **3.0** or higher.
