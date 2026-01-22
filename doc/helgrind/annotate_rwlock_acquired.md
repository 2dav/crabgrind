Informs Helgrind that a reader-writer lock has been **acquired**.

Establishes a happens-before relationship from the previous release of this
lock to the current thread. Helgrind updates its internal lockset state to
reflect that the calling thread holds the lock.

See [`annotate_rwlock_released`].

# Arguments
* `addr` - The unique identifier of the lock.
* `writer_lock` - `true` if the lock was acquired in exclusive (write) mode,
  `false` if acquired in shared (read) mode.

## Note
Requires Valgrind **3.6** or higher.
