Resizes and/or moves a block associated with a memory pool.

Wraps `VALGRIND_MOVE_MEMPOOL`. This updates Valgrind's records to reflect
that a block has been moved (e.g. via `realloc`) or resized in place.

## Note
Requires Valgrind **3.0** (Released 2003).
