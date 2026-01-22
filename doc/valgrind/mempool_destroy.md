Destroys a previously created memory pool.

Wraps `VALGRIND_DESTROY_MEMPOOL`. This deregisters the pool from
Valgrind's tracking. It implies that all blocks associated with the pool are no longer in use.

## Note
Requires Valgrind **3.0** or higher.
