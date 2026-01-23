<div align="center">
	<h1>crabgrind</h1>
	<p><a href="https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq">Valgrind Client Request</a> interface for Rust programs</p>

[crates.io]: https://crates.io/crates/crabgrind
[libs.rs]: https://lib.rs/crates/crabgrind
[documentation]: https://docs.rs/crabgrind
[license]: https://github.com/2dav/crabgrind/blob/main/LICENSE/MIT.LICENSE

[![crates.io](https://img.shields.io/crates/v/crabgrind)][crates.io]
[![libs.rs](https://img.shields.io/badge/libs.rs-crabgrind-orange)][libs.rs]
[![documentation](https://img.shields.io/docsrs/crabgrind)][documentation]
[![license](https://img.shields.io/crates/l/crabgrind)][license]

</div>

`crabgrind` is a small library that enables `Rust` programs to tap into `Valgrind`'s tools and environment.

# Summary

[Valgrind's client request][vg-client.req] mechanism is strictly a `C` implementation detail—it relies
heavily on specific macros and inline assembly. This crate acts as a bridge, translating
those `C` macros into Rust functions in a light way.

We don't re-implement Valgrind's client request internals — that lives inside the Valgrind.
This crate just handles the handshake, structure, and type conversion, all the real things
are done by Valgrind itself.

# `no_std` & `opt-out`

The crate is `no_std` and dependency-free.

If you need your release builds to be free of Valgrind artifacts, enable the `opt-out` feature. 
This optimizes every request into a no-op. You can keep your debugging calls in the source code 
without paying the runtime cost in production.

# Build Configuration

We need to build against local Valgrind installation to read `C` macro definitions and constants.

The build script (`build.rs`) attempts to locate headers in this order:

1.  **Environment Variable:** If `VALGRIND_INCLUDE` is set, it is used as the include path.
2.  **pkg-config:** The system is queried via `pkg-config` for valgrind installation paths.
3.  **Standard Paths:** The compiler falls back to standard include directories.

The crate compiles successfully even without Valgrind installed. In this case,
required values default to zero, and any request will trigger a rust-panic.

# Runtime Safety

These bindings are coupled to the Valgrind version present during compilation.

If a request is invoked at runtime that is unsupported by the active Valgrind
instance (e.g. running under an older Valgrind), the call panics immediately, showing
the version mismatch message and request requirements.

# Example

Add `crabgrind` to `Cargo.toml`

> ```toml
> [dependencies]
> crabgrind = "0.2"
> ```

Use some of [Valgrind's API](https://docs.rs/crabgrind/latest/crabgrind/#modules)

```rust
use crabgrind::{self as cg, valgrind::RunningMode};

fn main() {
    if matches!(cg::valgrind::running_mode(), RunningMode::Native) {
        println!("Run me under Valgrind");
    } else {
        cg::println!("Hey, Valgrind!");
    }
}
```

Run under `Valgrind` 

> ``` bash
> :~$ cargo build
> :~$ valgrind ./target/debug/app
> ```


# License
`crabgrind` is distributed under `MIT` license.

`Valgrind` itself is a GPL2, however `valgrind/*.h` headers are distributed under a BSD-style license, 
so we can use them without worrying about license conflicts.

[vg-client.req]: https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq
