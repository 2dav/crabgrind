# DRD(Data Race Detector) Client Requests

Interface to [DRD Client Requests][vg-docs] defined in `valgrind/drd.h`.

DRD is a Valgrind tool that detects data races in multithreaded programs by
shadowing every memory access. It tracks the synchronization state of threads.

Client requests allow the program to influence DRD's analysis when static
instrumentation is insufficient. They are primarily used to:

- *Suppress false positives:* Identify intentional races or benign
  unsynchronized accesses (e.g., lock-free counters or statistics).
- *Debug specific regions:* Trace activity on individual variables or memory
  ranges to pinpoint the origin of complex races.
- *Describe custom logic:* Annotate custom memory allocators or custom
  synchronization primitives that DRD does not natively recognize.

## Usage

Nothing happens unless the binary runs under DRD:

> ```text
> :~$ cargo build
> :~$ valgrind --tool=drd target/debug/app
> ```

## Version Requirements

- **Minimum Valgrind:** `3.3`
- **Recommended Valgrind:** `3.5` or higher to use any of DRD requests

[vg-docs]: https://valgrind.org/docs/manual/drd-manual.html#drd-manual.clientreqs
