Informs Valgrind that a custom allocator has freed a heap block.

Wraps `VALGRIND_FREELIKE_BLOCK`. This serves as the counterpart to [`malloclike_block`], marking 
the memory as unaddressable and recording the deallocation for leak checking.

# Arguments
* `addr` – The starting address of the block to free.
* `redzone` – The redzone size that was used when the block was allocated.
  This must match the value passed to [`malloclike_block`].

# Behavior
* Marks the memory block as unaddressable.
* Records the deallocation.

# Allocator Internals
If your allocator writes to a block *after* freeing it (e.g., to zero it
out or maintain internal free-lists), you must use `MAKE_MEM_UNDEFINED` 
(see [`memcheck::mark_memory`](crate::memcheck::mark_memory)) before writing. 
Accessing freed memory without this will trigger invalid write errors.

## Note
Requires Valgrind **3.0** or higher.
