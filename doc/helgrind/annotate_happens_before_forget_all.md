Resets the synchronization state associated with a specific object ID.

Instructs Helgrind to discard all pending happens-before relationships
established for `addr`. This returns the object to its original state,
effectively "forgetting" that any signal has been sent.

This is required for cleanup logic. Calling this ensures that Helgrind
does not hold internal resources indefinitely for a synchronization object
that is no longer in use or is being recycled.

# Arguments
* `addr` - An arbitrary machine word acting as a unique ID for the
  synchronization object.

## Note
Requires Valgrind **3.7** (2011) or higher.
