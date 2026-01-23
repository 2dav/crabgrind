Toggles data race suppression for memory **writes** performed by the current
thread.

Instructs DRD to stop reporting data races involving store operations performed
by the current thread. Suppression is immediate. The returned RAII guard manages
the duration: detection resumes automatically when the guard is dropped.

Wraps the `ANNOTATE_IGNORE_WRITES_BEGIN` and `ANNOTATE_IGNORE_WRITES_END` client
requests.

# Mechanics

This modifies the thread-local DRD state. It applies to **all** write operations
executed by the current thread, regardless of the memory address. This is a
coarse-grained suppression mechanism, distinct from [`ignore_var`](ignore_var)
which targets specific addresses.

**Warning:** Ignoring writes significantly reduces the effectiveness of DRD. Use
with caution, as intentional races on writes are rarer and more prone to causing
subtle memory consistency errors than read races.

## Note

Requires Valgrind **3.5** or higher.
