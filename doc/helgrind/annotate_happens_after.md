Marks the "waiter" side of a custom synchronization event.

Completes the happens-before edge initiated by [`annotate_happens_before`]. 
Helgrind treats all memory accesses performed after this call as happening-after all accesses performed before the
corresponding `_BEFORE` call in other threads.

This must be called **immediately after** the real synchronization action
succeeds (e.g., after returning from a wait on a semaphore or observing a
flag).

# Arguments
* `addr` - An arbitrary machine word acting as a unique ID for the
  synchronization object.

## Note
Requires Valgrind **3.7** (2011) or higher.
