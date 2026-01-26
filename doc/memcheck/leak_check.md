Forces a memory leak check at the current execution point.

This request bypasses the normal end-of-process leak detection, allowing for
granular inspection of memory management in specific subsystems or transient
states.

See also [`count_leaks`](count_leaks).

# Behavior

The results are emitted directly to the Valgrind output channel (stderr or xml).
Differential modes (`Added`, `Changed`, `New`) rely on state saved from the
immediately preceding leak check.

## Example

Checking for memory leaks at runtime:

```rust, no_run
use crabgrind::memcheck as mc;

fn check() -> mc::LeaksCount {
    mc::leak_check(Default::default());
    mc::count_leaks()
    // or equivalently
    // mc::LeakCheck::Full.check()
}

assert_eq!(check().leaked, 0);
// leak 8 bytes
std::mem::forget(Box::new(0usize));

assert_eq!(check().leaked, 8);
```

> Run with Memcheck
>
> ```text
> :~$ valgrind --tool=memcheck target/debug/leak_check
> ```

## Note

Minimum required Valgrind version:

- `Full` - **3.0**
- `Quick` - **3.2**
- `Added`, `Changed`, `New` - **3.4**
