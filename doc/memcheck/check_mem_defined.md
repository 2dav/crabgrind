Checks if a memory range is both addressable and defined.

This performs an immediate check on both A-bits (addressability) and V-bits
(definedness/initialized values) for the specified range. If any byte is not
addressable or contains undefined data, Valgrind emits an error report.

## Example

```rust, no_run
use std::mem::MaybeUninit;
use crabgrind::memcheck;

let mut var = MaybeUninit::<[u8; 2]>::uninit();
let mut vbits = [0u8; 2];

macro_rules! V {
    ($p:expr) => {{
        memcheck::vbits($p as _, vbits.as_mut_slice()).unwrap();
        &vbits[..]
    }};
    () => { V!(var.as_ptr()) };
}

assert_eq!(memcheck::check_mem_defined(var.as_ptr().cast(), 2), Err(0));
assert_eq!(V!(), [0xFF, 0xFF], "all bytes should be 'undefined' at this point");

// Initialize first byte
unsafe { (*var.as_mut_ptr())[0] = 42 };

assert_eq!(memcheck::check_mem_defined(var.as_ptr().cast(), 2), Err(1));
assert_eq!(V!(), [0x0, 0xFF], "only first byte is 'addressable' and 'defined'");

unsafe { assert_eq!(V!(var.assume_init().as_ptr()), [0x0, 0xFF]) };
```

> Run with Memcheck
>
> ```text
> :~$ valgrind --tool=memcheck target/debug/mem_defined
> ```

## Errors

[`OffendingOffset`](OffendingOffset) Contains the offset of the first byte that
is not defined.

## Note

Requires Valgrind **3.2** or higher.
