Convenient, typed wrappers for [`memcheck`](crate::memcheck) requests
that operate on memory ranges

The trait is implemented for `[T]`, so it works out of the box with slices
and types that dereference to slices, such as `Vec<T>`.

Each operation has the same semantics and limitations as its
corresponding raw request; see the linked documentation on each method
for details.

# Example

```rust, no_run
use crabgrind::memcheck as mc;
use crabgrind::memcheck::Memcheck as _;

let range = vec![0u16; 5];

range[3..].mark(mc::MemState::Undefined).unwrap();
assert_eq!(range.check_defined(), Err(3));
```

The equivalent operations using the raw API are:

```rust, no_run
use crabgrind::memcheck as mc;
use core::ffi::c_void;

let range = vec![0u16; 5];

let sub_range = &range[3..];
mc::mark_memory(
    sub_range.as_ptr() as *const c_void,
    size_of_val(sub_range),
    mc::MemState::Undefined,
).unwrap();

assert_eq!(
    mc::check_mem_defined(range.as_ptr() as *const c_void, size_of_val(&range)),
    Err(6),
);
```
