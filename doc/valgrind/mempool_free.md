Disassociates a piece of memory from a memory pool.

Wraps `VALGRIND_MEMPOOL_FREE`. Use this when returning memory to the pool.

If the pool was created with the
[`VALGRIND_MEMPOOL_AUTO_FREE`](VALGRIND_MEMPOOL_AUTO_FREE) flag, this call
implicitly frees any nested [`malloclike_block`](malloclike_block) structures
located within `addr`.

## Note

Requires Valgrind **3.0** or higher.
