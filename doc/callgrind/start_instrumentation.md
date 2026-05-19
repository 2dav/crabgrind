Starting of Callgrind instrumentation

Enables detailed collection of function calls and memory accesses. If cache
simulation is enabled, the simulated cache is flushed to ensure a clean state.
This results in an artificial warm-up phase immediately after the call,
characterized by cache misses that would not occur in a steady-state execution.

This mechanism allows you to skip irrelevant code sections (like initialization)
by starting the program with [--instr-atstart=no][instr-atstart], and then
enabling instrumentation only for the target execution path. When
instrumentation is paused (via [`stop_instrumentation`](stop_instrumentation)),
the program runs with minimal overhead, comparable to [Nulgrind].

## Note

Requires Valgrind **3.11** or higher.

[instr-atstart]: https://valgrind.org/docs/manual/cl-manual.html#opt.instr-atstart
[nulgrind]: https://valgrind.org/docs/manual/nl-manual.html
