Stops Cachegrind instrumentation if currently enabled.

Executes the `CACHEGRIND_STOP_INSTRUMENTATION` client request, causing
Valgrind to pause collection of cache simulation and branch prediction data.

Can be used in conjunction with [`start_instrumentation`] and the [--instr-at-start][instr-at-start]
flag to delimit specific regions for profiling.

## Note
Requires Valgrind **3.22**(2023) or higher.

[instr-at-start]: https://courses.cs.vt.edu/~cs3214/fall2011/projects/valgrind/valgrind-3.4.0/docs/html/cl-manual.html#opt.instr-atstart
