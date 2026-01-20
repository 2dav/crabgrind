# Helgrind Client Requests
Interface to [Helgrind Client Requests][vg-docs] defined in `valgrind/helgrind.h`.

Helgrind detects data races in multithreaded programs. It natively understands
standard POSIX threading primitives, but it relies on these client requests to
understand the semantics of user-defined synchronization primitives, memory
recycling schemes, or custom atomic operations.

## Functionality
These requests inform Helgrind about the following:
### Memory State
* [`clean_memory`] / [`clean_ref`] - Resets tracking for a memory range (useful for allocators).

### User-Defined Locks
* [`annotate_rwlock_create`] - Register a new reader-writer lock instance.
* [`annotate_rwlock_destroy`] - Deregister a lock instance.
* [`annotate_rwlock_acquired`] - Signal that a lock has been acquired.
* [`annotate_rwlock_released`] - Signal that a lock is about to be released.

### Custom Synchronization
* [`annotate_happens_before`] - Mark a "signaller" event.
* [`annotate_happens_after`] - Mark a corresponding "waiter" event.
* [`annotate_happens_before_forget_all`] - Reset synchronization state for a specific object.

## Usage
Nothing happens unless the binary runs under Helgrind:
> ```text
> :~$ cargo build
> :~$ valgrind --tool=helgrind target/debug/app
> ```

## Version Requirements
* **Minimum Valgrind:** `3.2` (required for [`clean_memory`])
* **Recommended Valgrind:** `3.7` or higher to use any of Helgrind requests

[vg-docs]: https://valgrind.org/docs/manual/hg-manual.html#hg-manual.client-requests 
