# Callgrind Client Requests
Interface to [Callgrind Client Requests][vg-docs] defined in `valgrind/callgrind.h`.
Use these to control the profiler, instrumentation state, and dump [Callgrind][callgrind] 
counters without restarting the process.

## Usage
Nothing happens unless the binary runs under Callgrind:
> ```text
> :~$ cargo build
> :~$ valgrind --tool=callgrind target/debug/app
> ```
## Execution Safety
These requests relies on Valgrind's magic assembly sequences to communicate with Valgrind runtime. 
If your application is running **without** Valgrind, these requests execute as harmless machine code. 
They will not panic or segfault.

## Version Requirements
* **Minimum Valgrind:** `3.7` (for requests like [`dump_stats`] and [`toggle_collect`]).
* **Recommended Valgrind:** `3.11` or higher (required for [`start_instrumentation`] support).

[vg-docs]: https://valgrind.org/docs/manual/cl-manual.html#cl-manual.clientrequests
[callgrind]: https://valgrind.org/docs/manual/cl-manual.html
