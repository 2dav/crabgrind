Determination of the current execution environment

This function wraps the `RUNNING_ON_VALGRIND` client request. It returns the
nesting level of Valgrind instances overseeing the current process.

# Example

```rust, ignore
use crabgrind::valgrind::{running_mode, RunningMode};

match running_mode() {
    RunningMode::Native => println!(":~$ valgrind {}", std::env::current_exe().unwrap().display()),
    RunningMode::Valgrind => crabgrind::println!("valgrind"),
    RunningMode::ValgrindOnValgrind(x) => crabgrind::println!("nested {x} instances"),
}
```

>
> ```text
> :~$ valgrind target/debug/running_mode
> ```

## Note

Requires Valgrind **3.0** or higher.
