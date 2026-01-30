Forces a Callgrind profile dump for the current thread and resets all counters

Triggers a dump of the current cost center state(instruction fetches, cache
misses, etc.) to the output file. The operation is strictly thread-local:
calling this from one thread has no effect on the profiling data of other
threads. Counters are reset to zero immediately after the dump.

The data is written to the output file (usually `callgrind.out.<N>.<pid>`).

The optional `reason` argument maps to `CALLGRIND_DUMP_STATS_AT`. If provided,
the string is appended to the description field in the profile data dump.
Passing [`None`](Option::None) corresponds to the basic `CALLGRIND_DUMP_STATS`
request.

# Arguments

- `reason` - An optional C-style string [`CStr`](core::ffi::CStr) describing the
  context of the dump.

# Example

Dumping without a specific reason:

```rust
use crabgrind::callgrind;
callgrind::dump_stats(None);
```

> ```text
> :~$ valgrind --tool=callgrind --verbose target/debug/dump_stats
>
> --602275-- Start dumping at BB 223999 (Client Request)...
> --602275-- Dump to ./callgrind.out.602275.2
> --602275-- Dumping done.
> ```

Dumping with a checkpoint marker for analysis:

```rust
use crabgrind::callgrind;
use core::ffi::CStr;

let reason = CStr::from_bytes_with_nul(b"factorial(10)\0").unwrap();
callgrind::dump_stats(reason);
```

> ```text
> :~$ valgrind --tool=callgrind --verbose target/debug/dump_stats
>
> --602275-- Start dumping at BB 223999 (Client Request: factorial(10))...
> --602275-- Dump to ./callgrind.out.602275.2
> --602275-- Dumping done.
> ```

## Note

Requires Valgrind **3.4** or higher.
