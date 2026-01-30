Retrieves the number of errors recorded by the Valgrind tool.

This function wraps the `VALGRIND_COUNT_ERRORS` macro. It returns the current
count of errors encountered during execution.

# Tool Specifics

The return value depends entirely on the active tool. Tools that report errors
(e.g. Memcheck) return a non-zero count. Tools that do not report errors in the
traditional sense (e.g., Cachegrind) will always return zero.

# Usage

This is primarily useful in automated test harnesses. When combined with the
`--log-fd=-1` option, Valgrind runs silently (suppressing standard output),
allowing the client program to inspect the error count programmatically.

# Example

Deterministic regression testing:

Running Valgrind is often a manual step: run the binary, scroll through the log
hoping for "ERROR SUMMARY: 0 errors". [`count_errors`] request allows to
automate assertions that fail, if Valgrind detects any issues.

```rust, no_run
use crabgrind::valgrind as vg;

#[inline(never)]
unsafe fn buggy_ffi(lets_fail: bool) {
    if lets_fail {
        // Simulate a fault Valgrind would catch (e.g. read out of bounds),
        // this triggers an error count increment.
        let v = vec![0u8; 3];
        let p = v.as_ptr().add(v.len());
        std::ptr::read_volatile(p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buggy_ffi() {
        let errors_before = vg::count_errors();

        unsafe { buggy_ffi(false) };
        assert_eq!(errors_before, vg::count_errors());

        unsafe { buggy_ffi(true) };
        assert!(vg::count_errors() > errors_before);
    }
}
```

> Run with Memcheck
>
> ```text
> :~$ valgrind --tool=memcheck target/debug/count_errors
> ```

## Note

Requires Valgrind **3.0** or higher.
