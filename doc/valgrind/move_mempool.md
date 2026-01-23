Informs Memcheck that a memory pool anchor has moved.

Updates the internal mapping of a custom allocator's header. Use this when the
structure managing the pool is reallocated, changing its address.

# Mechanics

Memcheck identifies pools by their anchor address. If you `realloc` the header,
this request tells Memcheck to track the pool at the new location.

This operation only updates the administrative handle. It does not modify the
accessibility or defined-ness bits of the memory chunks within the pool.

## Note

Requires Valgrind **3.7** or higher.
