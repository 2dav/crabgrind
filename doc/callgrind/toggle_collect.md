Toggles the recording of profiling events

Switches the counter collection state on or off. When collection is disabled,
the instrumented code still executes with full overhead, but the counters are not
incremented. This is useful for ignoring specific code sections without the cost
of flushing Valgrind's translation cache (unlike [`stop_instrumentation`]).

You can disable collection at startup using the Callgrind option
[--collect-atstart=no][collect-at-start].


# Example
Profiling specific code blocks in isolation:
```rust
use crabgrind::callgrind;
use std::collections::HashMap;

macro_rules! collect{
    ($expr:expr) => {{
        callgrind::toggle_collect();
        let ret = $expr;
        callgrind::toggle_collect();
        ret
    }}
}

// Profile only the insertion, ignoring setup overhead
let mut map:HashMap<i32, i32> = HashMap::with_capacity(32);

collect!(map.insert(0, 0));
map.shrink_to_fit(); // Ignored by profiler
collect!(map.entry(2).insert_entry(0));
```
> ```text
> :~$ valgrind --tool=callgrind --collect-atstart=no target/debug/toggle_collect
> ```

## Note
Requires Valgrind **3.7** or higher.

[collect-at-start]: https://courses.cs.vt.edu/~cs3214/fall2011/projects/valgrind/valgrind-3.4.0/docs/html/cl-manual.html#opt.collect-atstart
