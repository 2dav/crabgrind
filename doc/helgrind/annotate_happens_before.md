Marks the "signaller" side of a custom synchronization event.

Creates a happens-before edge. Memory accesses prior to this call in the
current thread will be considered visible to any thread that subsequently
executes [`annotate_happens_after`] using the same `addr`.

This must be called **immediately before** the real synchronization action
(e.g., before posting a semaphore or setting a flag).

# Arguments
* `addr` - An arbitrary machine word acting as a unique ID for the
  synchronization object. This is treated as a tag, not a dereferenced pointer.

## Note
Requires Valgrind **3.7** (2011) or higher.
