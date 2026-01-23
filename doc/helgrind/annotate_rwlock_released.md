Informs Helgrind that a reader-writer lock is about to be **released**.

Signals that the calling thread is relinquishing ownership of the lock.
Subsequent acquires will establish happens-before edges from this point.

See [`annotate_rwlock_acquired`](annotate_rwlock_acquired).

# Arguments

- `addr` - The unique identifier of the lock.
- `writer_lock` - `true` if the lock was held in exclusive (write) mode, `false`
  if held in shared (read) mode.

## Note

Requires Valgrind **3.6** or higher.
