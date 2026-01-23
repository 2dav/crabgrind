Disables full Callgrind instrumentation if not already switched off

Flushes Valgrind's translation cache, effectively halting further data
collection. Subsequent code execution runs with minimal overhead, comparable to
running under [Nulgrind] (the "none" tool).

This is primarily used to skip uninteresting code sections (such as
initialization routines) to reduce profiling noise and total runtime. It can be
toggled back on via [`start_instrumentation`](start_instrumentation) without
restarting the process.

## Note

Requires Valgrind **3.11** or higher.

See also the Callgrind option [--instr-atstart][instr-at-start].

[instr-at-start]: https://courses.cs.vt.edu/~cs3214/fall2011/projects/valgrind/valgrind-3.4.0/docs/html/cl-manual.html#opt.instr-atstart
[nulgrind]: https://valgrind.org/docs/manual/nl-manual.html
