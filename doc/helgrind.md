# Helgrind Client Requests
Interface to [Helgrind Client Requests][vg-docs] defined in `valgrind/helgrind.h`.

Helgrind detects data races in multithreaded programs. It natively understands
standard POSIX threading primitives, but it relies on these client requests to
understand the semantics of user-defined synchronization primitives, memory
recycling schemes, or custom atomic operations.

## Usage
Nothing happens unless the binary runs under Helgrind:
> ```text
> :~$ cargo build
> :~$ valgrind --tool=helgrind target/debug/app
> ```

## Version Requirements
* **Minimum Valgrind:** `3.2` (required for [`clean_memory`])
* **Recommended Valgrind:** `3.7` or higher to use any of Helgrind requests

[vg-docs]: https://valgrind.org/docs/manual/hg-manual.html#hg-manual.client-requests 
