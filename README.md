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

`crabgrind` allows Rust programs running under Valgrind to interact with the tools and virtualized 
environment.

[Valgrind's "client request interface"](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq) 
is primarily accessible through a set of `C` macros in Valgrind's header files. However, these macros 
cannot be utilized in languages that lack support for C-preprocessor, such as Rust. 
To address this, `crabgrind` wraps "client request interface" macros with `C` functions and expose
this API to Rust programs.

This library is essentially a wrapper. It only adds type conversions and some structure, while all 
the real things happens inside Valgrind.

## Table of Contents
- [Table of Contents](#table-of-contents)
- [Valgrind 3 API Coverage](#valgind-3-api-coverage)
- [Quickstart](#quickstart)
- [Examples](#examples)
- [Overhead](#overhead)
- [Safety](#safety)
- [Development](#development)
- [License](#license)

### Valgrind 3 API coverage
- Supported tool-specific client request interface: 
[valgrind](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq),
[callgrind](https://valgrind.org/docs/manual/cl-manual.html),
[memcheck](https://valgrind.org/docs/manual/mc-manual.html),
[helgrind](https://valgrind.org/docs/manual/hg-manual.html),
[massif](https://valgrind.org/docs/manual/ms-manual.html),
[cachegrind](https://valgrind.org/docs/manual/cg-manual.html#cg-manual.clientrequests)
- [Monitor commands](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.gdbserver-commandhandling) interface

### Quickstart
`crabgrind` does not link against Valgrind but instead reads its header files, which must be accessible during build.

If you have installed Vallgrind using OS-specific package manager, the paths to the headers are likely 
to be resolved automatically by [`cc`](https://docs.rs/cc/latest/cc/index.html). 

In case of manual installation or any `missing file` error, you can set the path to the Valgrind headers location
through the `DEP_VALGRIND` environment variable. For example:

```bash
DEP_VALGRIND=/usr/include cargo build
```

add dependency `Cargo.toml`
```toml
[dependencies]
crabgrind = "0.1"
```

use some of the [Valgrind's API](https://docs.rs/crabgrind/latest/crabgrind/#modules)
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
and run under Valgrind, 

*using [cargo-valgrind](https://github.com/jfrimmel/cargo-valgrind):*
> cargo valgrind run

*manually:*
> cargo build

> valgrind ./target/debug/appname

### Examples
- [Print current function stack-trace to the Valgrind log](https://docs.rs/crabgrind/latest/crabgrind/#print-current-function-stack-trace-to-the-valgrind-log)
- [Exclude expensive initialization code from the measurements](https://docs.rs/crabgrind/latest/crabgrind/#exclude-expensive-initialization-code-from-the-measurements)
- [Run a closure on the real CPU while running under Valgrind](https://docs.rs/crabgrind/latest/crabgrind/#run-a-closure-on-the-real-cpu-while-running-under-valgrind)
- [Save current memory usage snapshot to a file](https://docs.rs/crabgrind/latest/crabgrind/#save-current-memory-usage-snapshot-to-a-file)
- [Dump Callgrind counters on a function basis](https://docs.rs/crabgrind/latest/crabgrind/#dump-callgrind-counters-on-a-function-basis)

### Overhead
from [Valgrind docs](https://valgrind.org/docs/manual/manual-core-adv.html)
> The code added to your binary has negligible performance impact: on x86, amd64, ppc32, ppc64 and ARM,
 the overhead is 6 simple integer instructions and is probably undetectable except in tight loops.

> ... the code does nothing when not run on Valgrind, so you are not forced to run your program
under Valgrind just because you use the macros in this file.

Although your loops should be very tight (like a well-executed dance move) to notice any impact, 
keep in mind that:
- Wrapping each macros in a function implies function call overhead regardless of the run mode. This can potentially impact the performance of your Rust program.
- Functions that return `std::result::Result` involve branching, which can also have an impact on performance.
- Functions that take strings as parameters internally convert them to `std::ffi::CString`, which can introduce additional overhead.

### Safety
No

### License
`crabgrind` is distributed under `MIT` license.

`Valgrind` itself is a GPL2, however `valgrind/*.h` headers are distributed under a BSD-style license, 
so we can use them without worrying about license conflicts.
