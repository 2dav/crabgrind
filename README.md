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
`crabgrind` imports macros from Valgrind's header files, so they must be accessible to build the project.

If you installed Valgrind using your OS-specific package manager, the header files will be placed at the paths
according to your OS's conventions, and most likely [`cc`](https://docs.rs/cc/latest/cc/index.html), the build
tool `crabgrind` uses, will find them.

If you have installed Vallgrind manually or having any issues, you can set `DEP_VALGRIND` environment variable to the appropriate path, if
one is specified, its value will be directly passed to [`cc::Build::include`](https://docs.rs/cc/latest/cc/struct.Build.html#method.include).
> env DEP_VALGRIND=/path/to/valgrind cargo build

Add the following to your `Cargo.toml` file:
```toml
[dependencies]
crabgrind = "^0.1"
```

Next, use some of the [Valgrind's API](https://docs.rs/crabgrind/0.1.5/crabgrind/#modules)
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
- [Print current function stack-trace to the Valgrind log](#print-current-function-stack-trace-to-the-valgrind-log)
- [Exclude expensive initialization code from the measurements](#exclude-expensive-initialization-code-from-the-measurements)
- [Run a closure on the real CPU while running under Valgrind](#run-a-closure-on-the-real-cpu-while-running-under-valgrind)
- [Save current memory usage snapshot to a file](#save-current-memory-usage-snapshot-to-a-file)
- [Dump Callgrind counters on a function basis](#dump-callgrind-counters-on-a-function-basis)

#### Print current function stack-trace to the Valgrind log
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

#### Exclude expensive initialization code from the measurements
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

#### Run a closure on the real CPU while running under Valgrind
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
#### Save current memory usage snapshot to a file
We'll use `Massif` tool and the [monitor command](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.gdbserver-commandhandling)
interface to run the corresponding Massif command.
```rust
use crabgrind as cg;

let heap = String::from("alloca");

if cg::monitor_command("snapshot mem.snapshot").is_ok(){
    println!("snapshot is saved to \"mem.snapshot\"");
}
```

#### Dump Callgrind counters on a function basis
```rust
use crabgrind as cg;

fn factorial1(num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}

fn factorial2(num: u128) -> u128 {
    (1..=num).product()
}

cg::zero_stats();

let a = factorial1(20);
cg::dump_stats("factorial1");

let b = factorial2(20);
cg::dump_stats("factorial2");

assert_eq!(a,b);
cg::dump_stats(None);
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

### Development
Tests must be run under Valgrind, as of now [`cargo-valgrind`](https://github.com/jfrimmel/cargo-valgrind)
fits nicely, it allows to compile and run tests under Valgrind in one command
> cargo valgrind test

### License
`crabgrind` is distributed under the same license terms as the `Valgrind` that is GPL version 2.
