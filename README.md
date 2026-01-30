<div style="text-align: center;" align="center">

# `crabgrind`

### [Valgrind Client Request][vg-client.req] interface for Rust

[![crates.io](https://img.shields.io/crates/v/crabgrind)][crates.io]
[![libs.rs](https://img.shields.io/badge/libs.rs-crabgrind-orange)][libs.rs]
[![documentation](https://img.shields.io/docsrs/crabgrind)][documentation]
[![license](https://img.shields.io/crates/l/crabgrind)][license]

</div>

## Summary

`crabgrind` is a small library that enables Rust programs to tap into
Valgrind's tools and environment.

It exposes full set of [Valgrind's client requests][vg-client.req] in Rust,
manages the structure, type conversions and enforces static typing where
possible.

## Usage

**Minimum Supported Rust Version:** 1.64

First, add `crabgrind` as a dependency in `Cargo.toml`

```toml
[dependencies]
crabgrind = "0.2"
```

> Note: This crate is `no_std` and dependency-free

### Build Configuration

The crate needs access to a local Valgrind installation(or at least its headers)
in order to read C macro definitions, constants, and supported requests.

The build script (`build.rs`) attempts to locate headers in this order:

1. **Environment Variable:** If `VALGRIND_INCLUDE` is set, it's value is added
   to the search paths.
1. **pkg-config:** The system is queried via `pkg-config`.
1. **Compiler defaults:** No additional include paths are provided, and the
   compiler’s default include paths are used.

If headers cannot be located, the crate will still compile without errors,
however any request will panic at runtime.

### Example

Use some of the [Client Requests][crabgrind.modules]:

```rust, no_run
use crabgrind::valgrind::{running_mode, RunningMode};

fn main() {
    assert_eq!(
        running_mode(), RunningMode::Valgrind,
        ":~$ valgrind {}", std::env::current_exe().unwrap().display()
    );

    crabgrind::println!("Hey, Valgrind!");
}
```

And run under Valgrind

> ```bash
> :~$ cargo build
> :~$ valgrind ./target/debug/app
> ```

## Features

If you need your builds to be free of Valgrind artifacts, enable the `opt-out`
feature. This turns every request into no-op.

> ```toml
> crabgrind = { version = "0.2", features = ["opt-out"] }
> ```

## More Examples

- [Valgrind: Deterministic regression testing(e.g. CI or unit tests)](https://docs.rs/crabgrind/latest/crabgrind/valgrind/fn.count_errors.html#example)
- [Callgrind: Profiling specific code blocks in isolation](https://docs.rs/crabgrind/latest/crabgrind/callgrind/fn.toggle_collect.html#example)
- [Callgrind: Clearing setup costs to isolate some operation](https://docs.rs/crabgrind/latest/crabgrind/callgrind/fn.zero_stats.html#example)
- [Memcheck: Checking for memory leaks at runtime(e.g. CI or unit tests)](https://docs.rs/crabgrind/latest/crabgrind/memcheck/fn.leak_check.html#example)
- [Memcheck: Enforcing bounds in a custom allocator](https://docs.rs/crabgrind/latest/crabgrind/memcheck/fn.mark_memory.html#example)
- [Memcheck: Checking for memory 'definedness' feat. MaybeUninit](https://docs.rs/crabgrind/latest/crabgrind/memcheck/fn.check_mem_defined.html#example)
- [DHAT: Tracking data volumes](https://docs.rs/crabgrind/latest/crabgrind/dhat/fn.ad_hoc_event.html#example)
- [DRD: Tracking races in a custom shared memory](https://docs.rs/crabgrind/latest/crabgrind/drd/fn.annotate_new_memory.html#example)
- [DRD: Tracing memory accesses over some memory](https://docs.rs/crabgrind/latest/crabgrind/drd/fn.trace_var.html#example)

## Implementation

[Valgrind's client request][vg-client.req] mechanism is a C implementation
detail, exposed strictly via C macros. Since Rust does not support C
preprocessor, these macros cannot be used directly.

`crabgrind` wraps the foundational `VALGRIND_DO_CLIENT_REQUEST_EXPR` macro via
FFI binding. All higher-level client requests are implemented in Rust on top of
this binding.

The overhead per request, compared to using C macros directly is strictly the
cost of a single function call.

The implementation is independent of any specific Valgrind version. Instead,
mismatches between requests and local Valgrind instance are handled at
compile-time.

## Runtime Behavior

We are coupled to the Valgrind version present during compilation.

If a request is invoked at runtime that is unsupported by the active Valgrind
instance (e.g. running under an older Valgrind), the call panics immediately,
showing the version mismatch message and request requirements.

If your application is running **without** Valgrind, these
requests execute as harmless machine code. They will not panic or segfault, and
overhead is probably undetectable except in a tight loops.

## License

`crabgrind` is distributed under `MIT` license.

`Valgrind` itself is a GPL3, however `valgrind/*.h` headers are distributed
under a BSD-style license, so we can use them without worrying about license
conflicts.

[crabgrind.modules]: https://docs.rs/crabgrind/latest/crabgrind/#modules
[crates.io]: https://crates.io/crates/crabgrind
[documentation]: https://docs.rs/crabgrind
[libs.rs]: https://lib.rs/crates/crabgrind
[license]: https://github.com/2dav/crabgrind/blob/main/LICENSE/MIT.LICENSE
[vg-client.req]: https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq
