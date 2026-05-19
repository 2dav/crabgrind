Stopping of Cachegrind instrumentation

Executes the `CACHEGRIND_STOP_INSTRUMENTATION` client request, causing Valgrind
to pause collection of cache simulation and branch prediction data.

Can be used in conjunction with [`start_instrumentation`](start_instrumentation)
and the [--instr-atstart][instr-atstart] flag to delimit specific regions for
profiling.

## Note

Requires Valgrind **3.22** or higher.

[instr-atstart]: https://valgrind.org/docs/manual/cg-manual.html#opt.instr-atstart
