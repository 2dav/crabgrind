# DRD Client Requests

Interface to [DRD Client Requests][vg-docs] defined in `valgrind/drd.h`.

DRD is a Valgrind tool that detects data races in multithreaded programs by
shadowing every memory access. It tracks the synchronization state of threads.

## Usage

Nothing happens unless the binary runs under DRD:

> ```text
> :~$ cargo build
> :~$ valgrind --tool=drd target/debug/app
> ```

## Version Requirements

- **Minimum Valgrind:** `3.3`
- **Recommended Valgrind:** `3.5` or higher to use any of DRD's requests

[vg-docs]: https://valgrind.org/docs/manual/drd-manual.html#drd-manual.clientreqs
