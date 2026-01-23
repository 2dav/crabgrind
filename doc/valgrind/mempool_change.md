Updates Memcheck records for a chunk within a memory pool.

Use this when a specific block inside a custom allocator is moved or resized,
such as during a superblock reallocation or a chunk extension.

# Arguments

- `pool` - The handle for the memory pool.
- `addr_a` - The original address of the chunk.
- `addr_b` - The new address of the chunk.
- `size` - The new size of the chunk (covers `addr_b` to `addr_b + size - 1`).

# Mechanics

Memcheck associates the chunk data previously at `addr_a` with the new range
starting at `addr_b`.

This is a metadata-only update. It does **not** alter memory accessibility
(A-bits) or definedness (V-bits). If the new region overlaps unaddressable
memory, that state remains unaddressable until explicitly marked otherwise.

## Note

Requires Valgrind **3.7** or higher.
