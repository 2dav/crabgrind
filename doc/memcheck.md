# Memcheck Client Requests

Interface to [Memcheck Client Requests][vg-docs] defined in
`valgrind/memcheck.h`.

Memcheck detects memory management errors (invalid access, undefined values,
leaks). It automatically tracks standard heap allocations, but these client
requests are necessary to manually control shadow memory states, perform custom
leak detection, or manage non-standard memory allocators.

## Usage

Nothing happens unless the binary runs under Memcheck:

> ```text
> :~$ cargo build
> :~$ valgrind --tool=memcheck target/debug/app
> ```

## Version Requirements

- **Minimum Valgrind:** `3.0`
- **Recommended Valgrind:** `3.6` or higher to use any of Memcheck requests

[vg-docs]: https://valgrind.org/docs/manual/mc-manual.html#mc-manual.clientreqs
