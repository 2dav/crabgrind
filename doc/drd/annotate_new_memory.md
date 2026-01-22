Notifies DRD that a custom memory allocator has initialized a new memory range.

Wraps the `ANNOTATE_NEW_MEMORY` client request.

# Mechanics
DRD automatically tracks memory state by intercepting standard allocation
functions (`malloc`, `free`, `new`, `delete`). If the client program uses
a custom memory allocator (e.g., memory pools, arenas, or direct `mmap`), 
DRD treats those memory regions as *unallocated* or *inaccessible*.

This request explicitly informs DRD that the range `[addr, addr+size)` has
been allocated by the application and is now in use. It prevents DRD from
reporting "Invalid read/write" errors on valid custom-allocated memory and
ensures race detection is active for the new block.

## Note
Requires Valgrind **3.5** or higher.
