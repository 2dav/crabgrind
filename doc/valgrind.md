# Valgrind Client Requests
Interface to [Valgrind Client Requests][vg-docs] defined in `valgrind/valgrind.h`.

These requests interact with the Valgrind runtime and scheduler, applicable regardless of
the specific tool (Memcheck, Cachegrind, etc.) in use. They handle execution environment
queries, dynamic instrumentation changes, and custom memory management.

## Version Requirements
* **Minimum Valgrind:** `3.0`
* **Recommended Valgrind:** `3.10` or higher to use any of Valgrind core requests

[vg-docs]: https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq
