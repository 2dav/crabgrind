Starts Cachegrind instrumentation if currently disabled.

Executes the `CACHEGRIND_START_INSTRUMENTATION` client request.
It instructs the Valgrind to begin(or resume) collecting cache simulation and
branch prediction data.

# Usage Context

By default, Cachegrind instruments the entire program from the start.
To profile a specific section of code, you can run Valgrind with [`--instr-at-start=no`][instr-at-start] and 
use this function (and its counterpart, [`stop_instrumentation`]) to delimit the region of interest.

## Note
Requires Valgrind **3.22**(2023) or higher.

[instr-at-start]: https://courses.cs.vt.edu/~cs3214/fall2011/projects/valgrind/valgrind-3.4.0/docs/html/cl-manual.html#opt.instr-atstart
