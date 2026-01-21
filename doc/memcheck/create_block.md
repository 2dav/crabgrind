Associates a custom name with a memory range.

Registering a block allows Memcheck to report errors (invalid accesses, uninitialized reads)
using a specific description string rather than the generic allocation stack trace.

This modifies error reporting context only; it does not change the accessibility,
definedness, or any other physical properties of the memory.

# Arguments
* `addr` - The starting address of the memory range.
* `size` - The length of the range in bytes.
* `desc` - A null-terminated C string describing the block.

## Note
Requires Valgrind **3.2** (Released 2005).
