# Valgrind Client Requests
Interface to [Valgrind Client Requests][vg-docs] defined in `valgrind/valgrind.h`.

These requests interact with the Valgrind runtime and scheduler, applicable regardless of
the specific tool (Memcheck, Cachegrind, etc.) in use. They handle execution environment
queries, dynamic instrumentation changes, and custom memory management.

## Functionality

### Execution Environment
* [`running_mode`] - Detect if running under Valgrind (and nesting depth).
* [`count_errors`] - Retrieve the current error count from the active tool.
* [`monitor_command`] - Inject Valgrind monitor commands programmatically.
* [`change_clo`] - Modify dynamic command line options at runtime.

### Error Reporting Control
* [`disable_error_reporting`] / [`enable_error_reporting`] - Silence error reporting for specific threads or sections.

### Dynamic Code & JITs
* [`discard_translations`] - Invalidate translations for modified code (required for JITs).

### Debugging Information
* [`load_pdb_debuginfo`] - Load Windows PDB symbols for Wine runs.
* [`map_ip_to_srcloc`] - Resolve instruction addresses to source locations.

### Custom Allocators
The core requests provide granular control for memory managers that bypass standard `malloc`.
* **Simple Blocks:** [`malloclike_block`], [`resizeinplace_block`], [`freelike_block`].
* **Memory Pools:** [`create_mempool`], [`mempool_alloc`], [`mempool_free`], [`mempool_trim`], [`mempool_change`].

### Threading
* [`stack_register`] / [`stack_deregister`] / [`stack_change`] - Define stack bounds for user-space threads.

### Low-Level
* [`non_simd_call`], [`non_simd_call1`], [`non_simd_call2`], [`non_simd_call3`] - Execute a function on the real CPU.

## Version Requirements
* **Minimum Valgrind:** `3.0`
* **Recommended Valgrind:** `3.10` or higher to use any of Valgrind core requests

[vg-docs]: https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq
