Activates full Callgrind instrumentation if not already switched on

Enables detailed collection of function calls and memory accesses. If cache simulation
is enabled, the simulated cache is flushed to ensure a clean state. This results in
an artificial warm-up phase immediately after the call, characterized by cache misses
that would not occur in a steady-state execution.

This mechanism allows you to skip irrelevant code sections (like initialization) by
starting the program with [--instr-atstart=no][instr-at-start], and then enabling instrumentation only
for the target execution path. When instrumentation is paused (via [`stop_instrumentation`]), 
    the program runs at the minimum slowdown, comparable to [Nulgrind][nulgrind].

## Note
Requires Valgrind **3.11**(2017) or higher.

[nulgrind]: https://valgrind.org/docs/manual/nl-manual.html
[instr-at-start]: https://courses.cs.vt.edu/~cs3214/fall2011/projects/valgrind/valgrind-3.4.0/docs/html/cl-manual.html#opt.instr-atstart
