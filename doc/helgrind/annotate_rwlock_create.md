Informs Helgrind that a reader-writer lock has been **initialized**.

Intended for user-implemented synchronization primitives that mirror the
behavior of `pthread_rwlock_t`. Helgrind treats `addr` strictly as a unique
identifier (tag) for the lock. It does not dereference this memory address.

See [`annotate_rwlock_destroy`].

# Arguments
* `addr`- A unique address(identifier or tag) of the lock instance. This does not need to be
  a valid pointer, only a unique value for the lifetime of the lock.

## Note
Requires Valgrind **3.6** or higher.
