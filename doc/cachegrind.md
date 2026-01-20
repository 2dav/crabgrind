# Cachegrind Client Requests
Interface to [Cachegrind Client Requests][vg-docs] defined in `valgrind/cachegrind.h`.

These requests enables isolated profiling by combining the
[--instr-at-start][instr-at-start] flag with [`start_instrumentation`] and [`stop_instrumentation`] to delimit 
specific measurement regions.

## Usage
Nothing happens unless the binary runs under Cachegrind:
> ```text
> :~$ cargo build
> :~$ valgrind --tool=cachegrind target/debug/app
> ```

## Version Requirements
* **Minimum Valgrind:** `3.22`

[vg-docs]: https://valgrind.org/docs/manual/cg-manual.html#cg-manual.clientrequests 
[instr-at-start]: https://courses.cs.vt.edu/~cs3214/fall2011/projects/valgrind/valgrind-3.4.0/docs/html/cl-manual.html#opt.instr-atstart
