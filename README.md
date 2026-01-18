<div align="center">
	<h1>crabgrind</h1>
	<p><a href="https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq">Valgrind Client Request</a> interface for Rust programs</p>

[crates.io]: https://crates.io/crates/crabgrind
[libs.rs]: https://lib.rs/crates/crabgrind
[documentation]: https://docs.rs/crabgrind
[license]: https://github.com/2dav/crabgrind/blob/main/LICENSE

[![crates.io](https://img.shields.io/crates/v/crabgrind)][crates.io]
[![libs.rs](https://img.shields.io/badge/libs.rs-crabgrind-orange)][libs.rs]
[![documentation](https://img.shields.io/docsrs/crabgrind)][documentation]
[![license](https://img.shields.io/crates/l/crabgrind)][license]

</div>

`crabgrind` is a small library that enables `Rust` programs to tap into `Valgrind`'s tools and virtualized environment.

`Valgrind` offers a ["client request interface"](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq) that is accessible through `C` macros in its header files.
However, these macros can’t be used in languages fortunate enough to lack `C` preprocessor support, such as `Rust`. To address this,`crabgrind` wraps those macros in `C` functions and expose this API via FFI.

Essentially, `crabgrind` acts as a thin wrapper. It adds some type conversions and structure, but all the real things are done by `Valgrind` itself.

### Compatibility
`crabgrind` usually builds against the latest `Valgrind` releases, even if some new APIs aren't available—at least it compiles. However, some releases may introduce breaking changes. So, if you run into build errors or need a specific new feature, check out the compatibility table.

| Valgrind | crabgrind |
|----------|-----------|
| 3.23     | 0.1.11    |
| 3.22     | 0.1.10    |
| 3.21     | 0.1.9     |

## Quickstart
`crabgrind` does not link against `Valgrind` but instead reads its header files, which must be accessible during build.

If you have installed `Valgrind` using OS-specific package manager, the paths to the headers are likely to be resolved automatically by [`cc`](https://docs.rs/cc/latest/cc/index.html). 

In case of manual installation, you can set the path to the `Valgrind` headers location through the `VALGRIND_INCLUDE` environment variable. For example:

```bash
VALGRIND_INCLUDE=/usr/include cargo build
```

Next, add dependency to `Cargo.toml`
```toml
[dependencies]
crabgrind = "0.1"
```

Then, use some of [Valgrind's API](https://docs.rs/crabgrind/latest/crabgrind/#modules)
```rust
use crabgrind as cg;

fn main() {
    if matches!(cg::run_mode(), cg::RunMode::Native) {
        println!("run me under Valgrind");
    } else {
        cg::println!("Hey, Valgrind!");
    }
}
```
and run under `Valgrind` 

``` bash
cargo build
valgrind ./target/debug/appname
```

and finally, for more details and code examples, be sure to check out the
[documentation](https://docs.rs/crabgrind/latest/crabgrind).

### License
`crabgrind` is distributed under `MIT` license.

`Valgrind` itself is a GPL2, however `valgrind/*.h` headers are distributed under a BSD-style license, 
so we can use them without worrying about license conflicts.
