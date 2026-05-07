Notification to Valgrind of an in-place heap block resize

Wraps `VALGRIND_RESIZEINPLACE_BLOCK`. This handles custom reallocation logic
where the block address remains constant but the size changes.

Call this after the new block is established but before any references to the
old size are discarded.

# Arguments

- `addr` - The address of the block.
- `old_size` - The previous size of the block.
- `new_size` - The new size of the block.
- `redzone` - The redzone size associated with the block.

# Behavior

- Updates Valgrind's size records for the block.
- **Shrink:** Marks the freed tail memory as unaddressable.
- **Grow:** Marks the new tail memory as undefined and establishes a redzone
  past the new end.
- **Overlap:** Preserves definedness (V-bits) for the overlapping region between
  the old and new size.

## Note

Requires Valgrind **3.0** or higher.
