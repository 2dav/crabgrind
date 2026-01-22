Generic scoped RAII guard for client requests.

This struct manages the lifecycle of Valgrind requests, meant to be used in pairs, defining 
a logical span.

The guard internally maps to the following Valgrind client request pairs:
*  **Suppression:** `DRD_IGNORE_VAR` / `DRD_STOP_IGNORING_VAR` 
*  **Tracing:**  `DRD_TRACE_VAR` / `ANNOTATE_TRACE_MEMORY` / `DRD_STOP_TRACING_VAR` 
*  **Global Suppression**: `ANNOTATE_IGNORE_READS_BEGIN` / `ANNOTATE_IGNORE_READS_END`, `ANNOTATE_IGNORE_WRIES_BEGIN` / `ANNOTATE_IGNORE_WRITES_END`
*  **Error reporting**: `VALGRIND_ENABLE_ADDR_ERROR_REPORTING_IN_RANGE` / `VALGRIND_DISABLE_ADDR_ERROR_REPORTING_IN_RANGE`, `VALGRIND_ENABLE_ERROR_REPORTING` / `VALGRIND_DISABLE_ERROR_REPORTING`

The guard never captures the active borrow (`&T`).
Consequently, the borrow checker permits `&mut` access to the variable while the guard is active.

# Safety
The guard operates only raw memory addresses, not Rust variable bindings.
If the variable is moved or dropped while the guard exists, suppression/tracing remains
active on the **original** address.

Some requests (like [`memcheck::disable_reporting`]) affect the whole thread. They should
logically be singletons. We can't enforce this in the type system, so it's on you to
ensure you aren't running multiple global guards at once.

Likewise, nothing stops you from creating two guards for the same operation over same memory 
address. That will almost certainly break things.
