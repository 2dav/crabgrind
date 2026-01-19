# DRD(Data Race Detector) Client Requests
Interface to [DRD Client Requests][vg-docs] defined in `valgrind/drd.h`.

DRD is a Valgrind tool that detects data races in multithreaded programs by
shadowing every memory access. It tracks the synchronization state of threads.

Client requests allow the program to influence DRD's analysis when static
instrumentation is insufficient. They are primarily used to:
* *Suppress false positives:* Identify intentional races or benign
  unsynchronized accesses (e.g., lock-free counters or statistics).
* *Debug specific regions:* Trace activity on individual variables or
  memory ranges to pinpoint the origin of complex races.
* *Describe custom logic:* Annotate custom memory allocators or custom
  synchronization primitives that DRD does not natively recognize.

## Requests categories
* Thread Id:
  [`valgrind_threadid`], [`drd_threadid`]
* Race Suppression:
  [`ignore_var`], [`annotate_ignore_reads`], [`annotate_ignore_writes`], [`annotate_benign_race`], [`annotate_benign_race_sized`]
* Memory Tracing:
  [`trace_var`], [`annotate_trace_memory`]
* Metadata:
  [`annotate_thread_name`], [`annotate_new_memory`]

## Usage
Nothing happens unless the binary runs under DRD:
> ```text
> :~$ cargo build
> :~$ valgrind --tool=drd target/debug/app
> ```

## Version Requirements
* **Minimum Valgrind:** `3.5`

[vg-docs]: https://valgrind.org/docs/manual/drd-manual.html#drd-manual.clientreqs
