Manipulates the accessibility and validity state of a memory range.

This wrapper injects client requests directly into the execution stream to
modify Memcheck's shadow memory. It is useful for custom allocators or
optimizing instrumentation of complex memory operations.

## Example

Enforcing bounds in a custom allocator:

For custom memory management (e.g. arenas, bump allocators), Memcheck sees only
a single large block. Overflows within that block or reading from uninitialized
regions only triggers a generic "uninitialized value" errors.

By explicitly defining accessible regions ([`MemState::Undefined`]) and
restricting access to free space ([`MemState::NoAccess`]), we can turn generic
errors into precise reports.

```rust, no_run
use crabgrind::memcheck as mc;

struct BumpAllocator {
    start: *mut u8,
    offset: usize,
    capacity: usize,
}

impl BumpAllocator {
    fn new(capacity: usize) -> Self {
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                capacity,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
                -1,
                0,
            )
        };

        // Mark entire reserved region as inaccessible.
        // This catches stray reads/writes into the "free" part of the arena.
        mc::mark_memory(ptr as _, capacity, mc::MemState::NoAccess);

        // Name the block for better error messages.
        let block_description = std::ffi::CString::new("BumpAllocator").unwrap();
        mc::create_block(ptr as _, capacity, block_description);

        BumpAllocator {
            start: ptr as *mut u8,
            offset: 0,
            capacity,
        }
    }

    fn alloc(&mut self, size: usize) -> *mut u8 {
        let ptr = unsafe { self.start.add(self.offset) };

        // Mark newly allocated slice as "undefined".
        // This allows the program to write to it, but flags uninitialized reads.
        mc::mark_memory(ptr as _, size, mc::MemState::Undefined);

        self.offset += size;
        ptr
    }
}

fn main() {
    let mut bump = BumpAllocator::new(4096);

    let ptr = bump.alloc(100);
    unsafe { ptr.write(42) }; // Valid write.

    // Invalid access:
    // Attempting to write into the gap or the unallocated heap
    // will cause a precise error, rather than a silent corruption.
    unsafe {
        ptr.add(111).write(0);
    }
}
```

> Run with Memcheck
>
> ```text
> :~$ valgrind --tool=memcheck target/debug/mark_memory
> ```

# Errors

- [`UnaddressableBytes`](UnaddressableBytes) - containing the number of
  unaddressable bytes if `mark` is set to
  [`MemState::DefinedIfAddressable`](MemState::DefinedIfAddressable) and not all
  bytes in the range were addressable

## Note

Requires Valgrind **3.6** for
[`MemState::DefinedIfAddressable`](MemState::DefinedIfAddressable). All other
variants are supported in Valgrind **3.0** or higher.
