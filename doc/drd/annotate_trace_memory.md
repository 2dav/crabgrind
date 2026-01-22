Reports every memory access that touches a specific address.

This function instructs DRD to log any load or store operation that overlaps the 
**single byte** at `addr`.

Tracing is immediate. The returned guard manages the duration:
tracing stops automatically when the guard is dropped.

Corresponds to the `ANNOTATE_TRACE_MEMORY` client request.

# Mechanics
Contrast this with [`trace_var`], which monitors the entire size of a
variable (`sizeof(T)`). This request is strictly a **1-byte trace**.
It is useful when you need to monitor activity at a specific pointer
location without a known or bounded type size, or when isolating a
specific byte within a larger structure (e.g., a packed field or a
buffer offset).

The lifetime of the guard is tied to caller lifetime.

## Note
Requires Valgrind **3.3** or higher.
