# Callgrind Client Requests

Interface to [Callgrind Client Requests][vg-docs] defined in
`valgrind/callgrind.h`. Use these to control the profiler, instrumentation
state, and dump [Callgrind] counters without restarting the process.

## Usage

Nothing happens unless the binary runs under Callgrind:

> ```text
> :~$ cargo build
> :~$ valgrind --tool=callgrind target/debug/app
> ```

## Version Requirements

- **Minimum Valgrind:** `3.7`
- **Recommended Valgrind:** `3.11` or higher to use any of Callgrind requests

[callgrind]: https://valgrind.org/docs/manual/cl-manual.html
[vg-docs]: https://valgrind.org/docs/manual/cl-manual.html#cl-manual.clientrequests
