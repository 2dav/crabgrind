Resets Helgrind's tracking state for a specific memory range.

This function instructs Helgrind to discard all existing lock-set and access history
for the addresses defined by `start` and `size`. Post-call, the range is treated
as if it has just been allocated to the calling thread.

# Mechanics
*   **Ownership Assumption**: The calling thread is considered the sole owner of
    this memory range. It may access the memory without triggering synchronization
    warnings.
*   **External Synchronization**: Any other thread attempting to access this range
    must synchronize with the calling thread first; otherwise, Helgrind will report
    a race condition.

# Use Case
This is primarily required for custom memory allocators or memory pools.
When memory is recycled (freed by one thread and subsequently reallocated to
another), Helgrind must "forget" the previous owner's access patterns to prevent
false positives on the new owner.

## Note
Requires Valgrind **3.2** (2006) or higher.
