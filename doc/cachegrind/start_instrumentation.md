Starting of Cachegrind instrumentation

Executes the `CACHEGRIND_START_INSTRUMENTATION` client request, which instructs
Valgrind to begin (or resume) collecting cache simulation and branch prediction
data.

By default, Cachegrind instruments the entire program from the start. To profile
a specific section of code, you can run Valgrind with
[--instr-atstart=no][instr-atstart] and use this function (and its
counterpart, [`stop_instrumentation`](stop_instrumentation)) to delimit the
region of interest.

## Note

Requires Valgrind **3.22** or higher.

[instr-atstart]: https://valgrind.org/docs/manual/cg-manual.html#opt.instr-atstart
