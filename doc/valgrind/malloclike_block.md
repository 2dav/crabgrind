Informs Valgrind that a custom allocator has created a heap block.

Wraps `VALGRIND_MALLOCLIKE_BLOCK`. Valgrind tracks standard allocator calls
automatically, but this macro is required for custom memory management schemes
to ensure accurate leak detection, redzone insertion, and error reporting.

# Arguments

- `addr` – The starting address of the *usable* block (i.e., after any header or
  redzone padding).
- `size` – The size of the usable block in bytes.
- `redzone` – The size of the redzone (padding) in bytes. Use `0` if no redzone
  is applied. Non-zero values are recommended to enable detection of block
  overruns.
- `is_zeroed` – If `true`, the memory is marked as defined (e.g., from a
  `calloc`-like operation). If `false`, it is marked as undefined.

# Behavior

- Registers the block for leak checking.
- Marks the memory range as addressable.

# Placement

Call this immediately after the allocation logic, preferably at the outermost
level of the allocator function to ensure clean stack traces.

## Note

Requires Valgrind **3.0** or higher.
