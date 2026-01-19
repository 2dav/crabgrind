Activates access tracing and returns a RAII guard.

Tracing is immediate. The returned guard manages the duration:
tracing stops automatically when the guard is dropped.

This corresponds to the `DRD_TRACE_VAR` and `DRD_STOP_TRACING_VAR`
client requests.

# Mechanics
When active, DRD logs every load and store operation performed on the
address range of `var`. This generates verbose output but provides a
granular trace of access history, which is handy for diagnosing
complex race conditions reported by DRD.

**Warning:** This is an expensive operation in terms of performance and
log volume on the Valgrind side. Use only for targeted debugging.

The lifetime of the guard is tied to `&var`.

Acquiring the guard does **not** hold an active borrow. You may mutate `var` while the guard 
is active.

## Note
Requires Valgrind **3.3** (2007) or higher.
