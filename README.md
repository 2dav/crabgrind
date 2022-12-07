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

### Valgrind 3 API coverage
- Supported tool-specific client request interface: 
	- [valgrind](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq)
	- [callgrind](https://valgrind.org/docs/manual/cl-manual.html)
	- [memcheck](https://valgrind.org/docs/manual/mc-manual.html)
	- [helgrind](https://valgrind.org/docs/manual/hg-manual.html)
	- [massif](https://valgrind.org/docs/manual/ms-manual.html)
- [Monitor commands](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.gdbserver-commandhandling) interface

### Quickstart
`crabgrind` imports macros from Valgrind's header files, therefore you must have Valgrind
installed to build the project. 

Add the following to your `Cargo.toml` file:
```rust
[dependencies]
crabgrind = "^0.1"
```

### Examples
> Print some message to the Valgrind log
```rust
use crabgrind as cg;

if matches!(cg::run_mode(), cg::RunMode::Native) {
    println!("run me under Valgrind");
} else {
    cg::println!("Hey, Valgrind!");
}
```

> Exclude expensive (de)initialization code from the measurements

One way to do this would be to turn off stats collection at stratup with the
[`--collect-atstart=no`](https://valgrind.org/docs/manual/cl-manual.html#opt.collect-atstart)
callgrind command-line attribute, and enable/disable it from the code with `callgrind::toggle_collect`

```rust
use crabgrind as cg;

// ... some expensive initialization

cg::callgrind::toggle_collect();
// code of interest
cg::callgrind::toggle_collect();

// ... some deinitialization
```

> Run a closure on the real CPU while running under Valgrind

We can run on the real CPU instead of the virtual one using `valgrind::non_simd_call`,
refer to `valgrind.h` for details on limitations and various ways to crash.

```rust
use crabgrind as cg;

let mut state = 0;
cg::valgrind::non_simd_call(|tid| {
    // uncomment following line to see "the 'impossible' happened"
    // println!("tid: {tid}");
    state = tid;
});

println!("tid: {state}");
```
> Save current memory usage snapshot to a file

We'll use `Massif` tool and the [monitor command](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.gdbserver-commandhandling)
interface to run the corresponding Massif command.
```rust
use crabgrind as cg;

let heap = String::from("alloca");

if cg::monitor_command("snapshot mem.snapshot").is_ok(){
    println!("snapshot is saved to \"mem.snapshot\"");
}
```

> Print current function stack-trace to the Valgrind's log

Valgrind provides `VALGRIND_PRINTF_BACKTRACE` macro to print the message with the stack-trace attached,
`crabgrind::print_stacktrace` is it's crabbed wrapper.
```rust
use crabgrind as cg;

#[inline(never)]
fn print_trace(){
    let mode = cg::run_mode();
    cg::print_stacktrace!("current mode: {mode:?}");
}

print_trace();
```

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

### License
`crabgrind` is distributed under the same license terms as the `Valgrind` that is GPL version 2.
