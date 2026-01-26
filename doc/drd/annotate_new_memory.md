Notifies DRD that a custom memory allocator has initialized a new memory range.

Wraps the `ANNOTATE_NEW_MEMORY` client request.

# Mechanics

DRD automatically tracks memory state by intercepting standard allocation
functions (`malloc`, `free`, `new`, `delete`). If the client program uses a
custom memory allocator (e.g. memory pools, arenas, or direct `mmap`), DRD
treats those memory regions as *unallocated* or *inaccessible*.

This request explicitly informs DRD that the range `[addr, addr+size)` has been
allocated by the application and is now in use. It prevents DRD from reporting
"Invalid read/write" errors on valid custom-allocated memory and ensures race
detection is active for the new block.

## Example

Tracking races in custom shared memory:

The following example creates a raw memory mapping (simulating a shared memory
segment) and introduces a race condition. DRD will report the race only after
the memory is annotated.

```rust, no_run
use libc::{PROT_READ, PROT_WRITE, MAP_ANONYMOUS, MAP_PRIVATE, MAP_FAILED};
use crabgrind::drd;

fn main() {
    const PAGE_SIZE: usize = 4096;

    // Allocate raw memory. DRD does not track this automatically.
    let ptr = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            PAGE_SIZE, 
            PROT_READ | PROT_WRITE, MAP_ANONYMOUS | MAP_PRIVATE,
            -1, 0
        )
    };

    // Tell DRD to track this memory range.
    drd::annotate_new_memory(ptr as _, PAGE_SIZE);

    // Simulate some race
    let base_addr = ptr as usize;
    let handle1 = thread::spawn(move || unsafe {
        *(base_addr as *mut u8) = 1; // Unprotected write
    });
    let handle2 = thread::spawn(move || unsafe {
        *(base_addr as *mut u8) = 2; // Unprotected write (Race with Thread 1)
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    unsafe { libc::munmap(ptr, PAGE_SIZE) };
}
```

> Run with DRD
>
> ```text
> :~$ valgrind --tool=drd target/debug/annotate_new_memory
> ```
>
> Because range is registered, DRD detects the conflicting access and reports a
> data race:
>
> ```text
> ...
> Conflicting load by thread 3 at 0x04d53ee6 size 1
> ...
>```

## Note

Requires Valgrind **3.5** or higher.
