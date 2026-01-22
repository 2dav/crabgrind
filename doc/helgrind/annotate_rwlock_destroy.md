Informs Helgrind that a reader-writer lock is about to be **destroyed**.

Signals that the lock identifier is no longer valid and should be dissociated
from any internal Helgrind state.

See [`annotate_rwlock_create`].

# Arguments
* `addr`- A unique address(identifier or tag) of the lock instance. This does not need to be
  a valid pointer, only a unique value for the lifetime of the lock.

## Note
Requires Valgrind **3.6** or higher.
