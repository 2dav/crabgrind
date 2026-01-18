Resets the profile counters to zero

Immediately clears all event counters (instruction fetches, cache misses, etc.)
without writing a dump file. It is useful for discarding data from setup phases or 
previous iterations in a benchmarking loop.

# Example
Clearing setup costs to isolate the operation:
```rust
use crabgrind::callgrind;
use std::collections::HashMap;
use std::ffi::CStr;

let mut map:HashMap<u8, u8> = HashMap::with_capacity(1024);
let reason = CStr::from_bytes_with_nul(b"HashMap::insert\0").unwrap();

// Reset counters to ignore initialization overhead
callgrind::zero_stats();

// Perform operation we want to measure
map.insert(0, 0);

// Dump isolated data
callgrind::dump_stats(reason);
```
> ```text
> :~$ valgrind --tool=callgrind target/debug/zero_stats
> 
> --666538--   Zeroing costs...
> --666538--   ...done
> --666538-- Start dumping at BB 223561 (Client Request: HashMap::insert)...
> --666538-- Dump to ./callgrind.out.666538.2
> --666538-- Dumping done.
> ```

## Note
Requires Valgrind **3.7**(2013) or higher.
