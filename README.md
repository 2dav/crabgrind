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

`crabgrind` wraps various Valgrind macros in C functions, compiles and links against
the resulting binary, and exposes an unsafe interface to allow Rust programs running under Valgrind to
interact with the tools and environment. 

This library is indeed a wrapper, the only thing it adds are the type conversions and some structure,
all the real things are happening inside Valgrind.

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
[massif](https://valgrind.org/docs/manual/ms-manual.html)
- [Monitor commands](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.gdbserver-commandhandling) interface

### Quickstart
`crabgrind` doesn't links against Valgrind, but reads it's header files, so they must be accessible 
to build the project. 

If headers resides at the `/usr/include/valgrind`, [`cc`](https://docs.rs/cc/latest/cc/index.html), the build
tool `crabgrind` uses, will find them.

If you have installed Vallgrind manually, you can set `DEP_VALGRIND` environment variable to the appropriate path, 
its value, if specified, will be directly passed to [`cc::Build::include`](https://docs.rs/cc/latest/cc/struct.Build.html#method.include).
> env DEP_VALGRIND=/usr/include cargo build


Add the following to your `Cargo.toml` file:
```toml
[dependencies]
crabgrind = "^0.1"
```

Next, use some of the [Valgrind's API](https://docs.rs/crabgrind/latest/crabgrind/#modules)
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
And run your application under Valgrind, either with handy [cargo-valgrind](https://github.com/jfrimmel/cargo-valgrind) 
> cargo valgrind run

or manually

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

however,
- wrapping each macros in a function implies function call overhead regardless of the run mode
- functions that returns `std::result::Result` involve branching
- functions that takes strings as a parameters internally converts them to `std::ffi::CString`

If you wish to compile out all (crab)Valgrind from the binary, you can wrap `crabgrind` calls with 
the feature-gate.

### Safety
No

### Development
Tests must be run under Valgrind, as of now [`cargo-valgrind`](https://github.com/jfrimmel/cargo-valgrind)
fits nicely, it allows to compile and run tests under Valgrind in one command
> cargo valgrind test

### License
`crabgrind` is distributed under the same license terms as the `Valgrind` that is GPL version 2.
