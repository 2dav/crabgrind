# Memcheck Client Requests
Interface to [Memcheck Client Requests][vg-docs] defined in `valgrind/memcheck.h`.

Memcheck detects memory management errors (invalid access, undefined values, leaks). It
automatically tracks standard heap allocations, but these client requests are necessary to
manually control shadow memory states, perform custom leak detection, or manage non-standard 
memory allocators.

## Functionality
### Shadow Memory
* [`mark_memory`] - Manually set addressability(A-bits) and validity(V-bits) for a range.
* [`check_mem_addressable`] / [`check_mem_defined`] - Assert memory properties and trigger errors on failure.
* [`vbits`] / [`set_vbits`] - Directly read or write the validity bits for low-level introspection.

### Leak Detection
* [`leak_check`] - Force an immediate leak scan (full, summary, or differential).
* [`count_leaks`] / [`count_leak_blocks`] - Retrieve statistics from the most recent scan.

### Reporting
* [`create_block`] - Attach a custom description to a memory range for better error logs.
* [`disable_error_reporting`] - Temporarily silence error reporting for a specific range.

## Usage
Nothing happens unless the binary runs under Memcheck:
> ```text
> :~$ cargo build
> :~$ valgrind --tool=memcheck target/debug/app
> ```

## Version Requirements
* **Minimum Valgrind:** `3.0`
* **Recommended Valgrind:** `3.6` or higher to use any of Memcheck requests

[vg-docs]:https://valgrind.org/docs/manual/mc-manual.html#mc-manual.clientreqs
