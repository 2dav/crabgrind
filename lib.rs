//! `crabgrind` allows Rust programs running under Valgrind to interact with the tools and virtualized
//! environment.
//!
//! [Valgrind's "client request interface"](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq)
//! is primarily accessible through a set of `C` macros in Valgrind's header files. However, these macros
//! cannot be utilized in languages that lack support for C-preprocessor, such as Rust.
//! To address this, `crabgrind` wraps "client request interface" macros with `C` functions and expose
//! this API to Rust programs.
//!
//! This library is essentially a wrapper. It only adds type conversions and some structure, while all
//! the real things happens inside Valgrind.
//!
//! ### Valgrind 3 API coverage
//! - Supported tool-specific client request interface:
//! [valgrind](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq),
//! [callgrind](https://valgrind.org/docs/manual/cl-manual.html),
//! [memcheck](https://valgrind.org/docs/manual/mc-manual.html),
//! [helgrind](https://valgrind.org/docs/manual/hg-manual.html),
//! [massif](https://valgrind.org/docs/manual/ms-manual.html)
//! - [Monitor commands](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.gdbserver-commandhandling) interface
//!
//! ### Quickstart
//! `crabgrind` does not link against Valgrind but instead reads its header files, which must be accessible during build.
//!
//! If you have installed Vallgrind using OS-specific package manager, the paths to the headers are likely
//! to be resolved automatically by [`cc`](https://docs.rs/cc/latest/cc/index.html).
//!
//! In case of manual installation or any `missing file` error, you can set the path to the Valgrind headers location
//! through the `DEP_VALGRIND` environment variable. For example:
//!
//! add dependency `Cargo.toml`
//! ```ignore
//! [dependencies]
//! crabgrind = "0.1"
//! ```
//!
//! use some of the [Valgrind's API](https://docs.rs/crabgrind/latest/crabgrind/#modules)
//! ```no_run
//! use crabgrind as cg;
//!
//! fn main() {
//!     if matches!(cg::run_mode(), cg::RunMode::Native) {
//!         println!("run me under Valgrind");
//!     } else {
//!         cg::println!("Hey, Valgrind!");
//!     }
//! }
//! ```
//! and run under Valgrind,
//!
//! *using [cargo-valgrind](https://github.com/jfrimmel/cargo-valgrind):*
//! > cargo valgrind run
//!
//! *manually:*
//! > cargo build
//!
//! > valgrind ./target/debug/appname
//!
//! ### Examples
//!
//! ##### Print current function stack-trace to the Valgrind log
//! Valgrind provides `VALGRIND_PRINTF_BACKTRACE` macro to print the message with the stack-trace attached,
//! `crabgrind::print_stacktrace` is it's crabbed wrapper.
//! ```rust
//! use crabgrind as cg;
//!
//! #[inline(never)]
//! fn print_trace(){
//!     let mode = cg::run_mode();
//!     cg::print_stacktrace!("current mode: {mode:?}");
//! }
//!
//! print_trace();
//! ```
//!
//! ##### Exclude expensive initialization code from the measurements
//! One way to do this would be to turn off stats collection at stratup with the
//! [`--collect-atstart=no`](https://valgrind.org/docs/manual/cl-manual.html#opt.collect-atstart)
//! callgrind command-line attribute, and enable/disable it from the code with `callgrind::toggle_collect`
//!
//! ```rust
//! use crabgrind as cg;
//!
//! // ... some expensive initialization
//!
//! cg::callgrind::toggle_collect();
//! // code of interest
//! cg::callgrind::toggle_collect();
//!
//! // ... some deinitialization
//! ```
//!
//! ##### Run a closure on the real CPU while running under Valgrind
//! We can run on the real CPU instead of the virtual one using `valgrind::non_simd_call`,
//! refer to `valgrind.h` for details on limitations and various ways to crash.
//!
//! ```rust
//! use crabgrind as cg;
//!
//! let mut state = 0;
//! cg::valgrind::non_simd_call(|tid| {
//!     // uncomment following line to see "the 'impossible' happened"
//!     // println!("tid: {tid}");
//!     state = tid;
//! });
//!
//! println!("tid: {state}");
//! ```
//! ##### Save current memory usage snapshot to a file
//! We'll use `Massif` tool and the [monitor command](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.gdbserver-commandhandling)
//! interface to run the corresponding Massif command.
//! ```rust
//! use crabgrind as cg;
//!
//! let heap = String::from("alloca");
//!
//! if cg::monitor_command("snapshot mem.snapshot").is_ok(){
//!     println!("snapshot is saved to \"mem.snapshot\"");
//! }
//! ```
//!
//! ##### Dump Callgrind counters on a function basis
//! ```rust
//! use crabgrind as cg;
//!
//! fn factorial1(num: u128) -> u128 {
//!     match num {
//!         0 => 1,
//!         1 => 1,
//!         _ => factorial1(num - 1) * num,
//!     }
//! }
//!
//! fn factorial2(num: u128) -> u128 {
//!     (1..=num).product()
//! }
//!
//! cg::callgrind::zero_stats();
//!
//! let a = factorial1(20);
//! cg::callgrind::dump_stats("factorial1");
//!
//! let b = factorial2(20);
//! cg::callgrind::dump_stats("factorial2");
//!
//! assert_eq!(a,b);
//! cg::callgrind::dump_stats(None);
//! ```
//!
//! ### Overhead
//! from [Valgrind docs](https://valgrind.org/docs/manual/manual-core-adv.html)
//! > The code added to your binary has negligible performance impact: on x86, amd64, ppc32, ppc64 and ARM,
//!  the overhead is 6 simple integer instructions and is probably undetectable except in tight loops.
//!
//! > ... the code does nothing when not run on Valgrind, so you are not forced to run your program
//! under Valgrind just because you use the macros in this file.
//!
//! Although your loops should be very tight (like a well-executed dance move) to notice any impact,
//! keep in mind that:
//! - Wrapping each macros in a function implies function call overhead regardless of the run mode. This can potentially impact the performance of your Rust program.
//! - Functions that return `std::result::Result` involve branching, which can also have an impact on performance.
//! - Functions that take strings as parameters internally convert them to `std::ffi::CString`, which can introduce additional overhead.
use std::ffi::c_void;
mod bindings;

macro_rules! raw_call {
    ($f:ident) => { raw_call!($f,) };
    ($f:ident, $($args:tt)*) => {{
        unsafe{ bindings::$f($($args)*) }
    }};
}

/// Current run mode
///
/// see [`run_mode()`]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum RunMode {
    /// on the real CPU
    Native,
    /// under Valgrind emulation
    Valgrind,
    /// under multiple layers of Valgrind emulation
    ValgrindInValgrind(usize),
}

/// Returns the [`RunMode`] app running in
///
/// # Example
/// ```no_run
/// use crabgrind::RunMode;
///
/// match crabgrind::run_mode(){
///     RunMode::Native                 => println!("native CPU"),
///     RunMode::Valgrind               => println!("hey, Valgrind!"),
///     RunMode::ValgrindInValgrind(n)  => println!("Valgrind layers: {n}"),
/// }
/// ```
/// # Implementation
/// `RUNNING_ON_VALGRIND`
#[inline]
pub fn run_mode() -> RunMode {
    match unsafe { bindings::running_on_valgrind() } {
        0 => RunMode::Native,
        1 => RunMode::Valgrind,
        x => RunMode::ValgrindInValgrind(x),
    }
}

#[doc(hidden)]
pub fn __print(msg: String) {
    let cstr = std::ffi::CString::new(msg).unwrap();
    raw_call!(vg_print, cstr.as_ptr());
}

/// Prints to the Valgrind's log.
///
/// Accepts format string similar to [`std::println!`].
///
/// # Example
/// ```no_run
/// if !matches!(crabgrind::run_mode(), crabgrind::RunMode::Native){
///     crabgrind::print!("hello {}", "Valgrind");
/// }
/// ```
///
/// # Implementation
/// `VALGRIND_PRINTF` wrapped with the fixed `"%s"` format.
///
/// # Panics
/// If format string contains null-byte in any position.
#[macro_export]
macro_rules! print{
    ($($arg:tt)+) => { $crate::__print(format!("{}",format_args!($($arg)+)));}
}

/// Prints to the Valgrind's log, with a newline.
///
/// Accepts format string similar to [`std::println!`].
///
/// # Example
/// ```no_run
/// use crabgrind as cg;
///
/// cg::println!("current mode: {:?}", cg::run_mode());
/// ```
///
/// # Implementation
/// `VALGRIND_PRINTF` wrapped with the fixed `"%s"` format.
///
/// # Panics
/// If format string contains null-byte in any position.
#[macro_export]
macro_rules! println{
    ($($arg:tt)+) => { $crate::__print(format!("{}\n",format_args!($($arg)+)));}
}

#[doc(hidden)]
#[inline(always)]
pub fn __print_stacktrace(msg: String) {
    let cstr = std::ffi::CString::new(msg).unwrap();
    raw_call!(vg_print_backtrace, cstr.as_ptr());
}

/// Prints to the Valgrind's log, with the current stacktrace attached.
///
/// Accepts format string similar to [`std::println!`].
///
/// # Example
/// ```no_run
/// use crabgrind as cg;
///
/// #[inline(never)]
/// fn print_trace(){
///     let mode = cg::run_mode();
///     cg::print_stacktrace!("current mode: {mode:?}");
/// }
///
/// print_trace();
/// ```
///
/// # Implementation
/// `VALGRIND_PRINTF_BACKTRACE` wrapped with the fixed `"%s"` format.
///
/// # Panics
/// If format string contains null-byte in any position.
#[macro_export]
macro_rules! print_stacktrace{
    ($($arg:tt)+) => { $crate::__print_stacktrace(format!("{}\n",format_args!($($arg)+)));}
}

/// Execute arbitrary Valgrind [Monitor command](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.gdbserver-commandhandling)
///
/// # Example
/// ```no_run
/// use crabgrind as cg;
///
/// let heap = String::from("alloca");
///
/// if cg::monitor_command("snapshot mem.snapshot").is_ok(){
///     println!("snapshot is saved to \"mem.snapshot\"");
/// }
/// ```
///
/// # Implementation
/// `VALGRIND_MONITOR_COMMAND`
///
/// # Panics
/// If command string contains null-byte in any position.
#[inline]
pub fn monitor_command(cmd: impl AsRef<str>) -> std::io::Result<()> {
    let cmd = std::ffi::CString::new(cmd.as_ref()).unwrap();
    if raw_call!(vg_monitor_command, cmd.as_ptr()) {
        Err(std::io::ErrorKind::NotFound.into())
    } else {
        Ok(())
    }
}

/// Disable error reporting for this thread
///
/// Behaves in a stack like way, so you can safely call this multiple times provided that
/// [`enable_error_reporting()`] is called the same number of times to re-enable reporting.  
///
/// The first call of this macro disables reporting.  Subsequent calls have no effect except
/// to increase the number of [`enable_error_reporting()`] calls needed to re-enable reporting.  
///
/// Child threads do not inherit this setting from their parents -- they are always created with
/// reporting enabled.
///
/// # Example
/// ```no_run
/// use crabgrind as cg;
///
/// cg::disable_error_reporting();
///
/// unsafe {
///     let b = Box::new([0]);
///     println!("{}", b.get_unchecked(1));
/// };
/// assert_eq!(cg::count_errors(), 0);
/// ```
///
/// # Implementation
/// `VALGRIND_DISABLE_ERROR_REPORTING`
#[inline]
pub fn disable_error_reporting() {
    raw_call!(vg_disable_error_reporting);
}

/// Re-enable error reporting for this thread
///
/// see [`disable_error_reporting()`] docs
///
/// # Implementation
/// `VALGRIND_ENABLE_ERROR_REPORTING`
#[inline]
pub fn enable_error_reporting() {
    raw_call!(vg_enable_error_reporting);
}

/// Returns the number of errors found so far by Valgrind
///
/// # Example
/// ```no_run
/// use crabgrind as cg;
///
/// unsafe {
///     let b = Box::new([0]);
///     println!("{}", b.get_unchecked(1));
/// };
///
/// assert_eq!(cg::count_errors(), 1);
/// ```
///
/// # Implementation
/// `VALGRIND_COUNT_ERRORS`
#[inline]
pub fn count_errors() -> usize {
    raw_call!(vg_count_errors)
}

/// Change the value of a dynamic command line option.
///
/// see [`official docs`](https://valgrind.org/docs/manual/manual-core.html#manual-core.dynopts)
/// for details.
///
/// # Example
/// ```no_run
/// use crabgrind as cg;
///
/// cg::change_cli_option("--leak-check=no");
/// std::mem::forget(String::from("see you in the void"));
/// ```
///
/// # Implementation
/// `VALGRIND_CLO_CHANGE`
///
/// # Panics
/// If command string contains null-byte in any position.
#[inline]
pub fn change_cli_option(opt: impl AsRef<str>) {
    let cstr = std::ffi::CString::new(opt.as_ref()).unwrap();
    raw_call!(vg_clo_change, cstr.as_ptr());
}

pub mod valgrind {
    //! [`Valgrind requests`](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq)
    use std::os::unix::prelude::RawFd;

    use super::*;

    pub type ThreadId = usize;

    /// Discards translations of code in the specified address range
    ///
    /// see [official docs](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq)
    /// for details.
    ///
    /// # Implementation
    /// `VALGRIND_DISCARD_TRANSLATIONS`
    #[inline]
    pub fn discard_translations(addr: *mut c_void, len: usize) {
        raw_call!(vg_discard_translations, addr, len);
    }

    /// Load PDB debug info for Wine PE image_map
    ///
    /// # Implementation
    /// `VALGRIND_LOAD_PDB_DEBUGINFO`
    #[inline]
    pub fn load_pdb_debuginfo(fd: RawFd, ptr: *mut c_void, total_size: usize, delta: usize) {
        raw_call!(vg_load_pdb_debuginfo, fd, ptr, total_size, delta);
    }

    /// Map a code address to a source file name and line number
    ///
    /// `buf64` must point to a 64-byte buffer in the caller's address space.  
    /// The result will be dumped in there and is guaranteed to be zero terminated.  
    /// If no info is found, the first byte is set to zero.
    ///
    /// # Implementation
    /// `VALGRIND_MAP_IP_TO_SRCLOC`
    #[inline]
    pub fn map_ip_to_srcloc(addr: *mut c_void, buf64: *mut c_void) -> usize {
        raw_call!(vg_map_ip_to_srcloc, addr, buf64)
    }

    extern "C" fn _closure_adapter<F>(tid: ThreadId, f: *mut c_void)
    where
        F: FnMut(ThreadId),
    {
        if let Err(err) = std::panic::catch_unwind(|| unsafe {
            debug_assert!(!f.is_null(), "closure pointer is null");
            debug_assert_eq!(
                f as usize & (std::mem::align_of::<F>() - 1),
                0,
                "unexpected closure pointer"
            );

            (*f.cast::<F>())(tid)
        }) {
            let panic_info = err
                .downcast::<String>()
                .map(|v| *v)
                .or_else(|e| e.downcast::<&str>().map(|v| v.to_string()))
                .unwrap_or_else(|_| "unknown panic source".to_string());

            eprintln!("closure code panicked with: {panic_info:?}");

            std::process::abort();
        }
    }

    /// Runs a closure on the real CPU.
    ///
    /// Closure receives a [`ThreadId`] as the parameter, that is the Valgrind's notion of thread
    /// identifier and there may not be relationship between [`ThreadId`] and rust's [`std::thread::ThreadId`].
    ///
    /// Refer to the `valgrind.h` for details and limitations.
    ///
    /// # Example
    /// ```no_run
    /// use crabgrind as cg;
    ///
    /// let mut thread_id = 0;
    /// cg::valgrind::non_simd_call(|tid| {
    ///     thread_id = tid;
    /// });
    /// println!("{thread_id}");
    /// ```
    ///
    /// # Implementation
    /// `VALGRIND_NON_SIMD_CALL1`
    ///
    /// # Panics
    /// It's safe to panic in the closure code in that this won't cause a UB on stack unwinding.
    #[inline]
    pub fn non_simd_call<F>(f: F)
    where
        F: FnMut(ThreadId),
    {
        let boxed = Box::into_raw(Box::new(f));
        raw_call!(vg_non_simd_call1, _closure_adapter::<F>, boxed.cast());
        let _ = unsafe { Box::from_raw(boxed) };
    }
}

pub mod callgrind {
    //! [`Callgrind requests`](https://courses.cs.vt.edu/~cs3214/fall2011/projects/valgrind/valgrind-3.4.0/docs/html/cl-manual.html#cl-manual.clientrequests)
    use super::*;

    /// Dump current state of cost centers, and zero them afterwards
    ///
    /// If `reason` parameter is specified, this string will be written as a description field into
    /// the profile data dump.
    ///
    /// # Example
    /// ```no_run
    /// use crabgrind as cg;
    ///
    /// fn factorial1(num: u128) -> u128 {
    ///     match num {
    ///         0 => 1,
    ///         1 => 1,
    ///         _ => factorial1(num - 1) * num,
    ///     }
    /// }
    ///
    /// fn factorial2(num: u128) -> u128 {
    ///     (1..=num).product()
    /// }
    ///
    /// cg::callgrind::zero_stats();
    ///
    /// let a = factorial1(20);
    /// cg::callgrind::dump_stats("factorial1");
    ///
    /// let b = factorial2(20);
    /// cg::callgrind::dump_stats("factorial2");
    ///
    /// assert_eq!(a,b);
    /// cg::callgrind::dump_stats(None);
    /// ```
    ///
    /// # Implementation
    /// `CALLGRIND_DUMP_STATS` or `CALLGRIND_DUMP_STATS_AT`
    ///
    /// # Panics
    /// If `reason` is specified and contains null-byte in any position.
    #[inline]
    pub fn dump_stats<'a>(reason: impl Into<Option<&'a str>>) {
        match reason.into() {
            None => raw_call!(cl_dump_stats),
            Some(reason) => {
                let cstr = std::ffi::CString::new(reason).unwrap();
                raw_call!(cl_dump_stats_at, cstr.as_ptr())
            }
        };
    }

    /// Zero current stats
    ///
    /// # Implementation
    /// `CALLGRIND_ZERO_STATS`
    #[inline]
    pub fn zero_stats() {
        raw_call!(cl_zero_stats);
    }

    /// Toggles collection state
    ///
    /// The collection state specifies whether the happening of events should be noted or if
    /// they are to be ignored. Events are noted by increment of counters in a cost center.
    ///
    /// # Example
    /// run with `valgrind --tool==callgrind --collect-atstart=no ...`
    /// ```no_run
    /// use crabgrind as cg;
    ///
    /// let xs = (0..10 << 10).into_iter().collect::<Vec<u32>>();
    ///
    /// cg::callgrind::toggle_collect();
    /// let i = xs.binary_search(&(10 << 10 >> 1));
    /// cg::callgrind::toggle_collect();
    /// ```
    ///
    /// # Implementation
    /// `CALLGRIND_TOGGLE_COLLECT`
    #[inline]
    pub fn toggle_collect() {
        raw_call!(cl_toggle_collect);
    }

    /// Start full callgrind instrumentation if not already switched on
    ///
    /// When cache simulation is done, it will flush the simulated cache;
    /// this will lead to an artificial cache warmup phase afterwards with cache misses which
    /// would not have happened in reality.
    ///
    /// Use this to bypass Callgrind aggregation for uninteresting code parts.
    /// To start Callgrind in this mode to ignore the setup phase, use the option `--instr-atstart=no`.
    ///
    /// # Example
    /// ```no_run
    /// use crabgrind as cg;
    ///
    /// let xs = (0..10 << 10).into_iter().collect::<Vec<u32>>();
    ///
    /// cg::callgrind::start_instrumentation();
    /// let i = xs.binary_search(&(10 << 10 >> 1));
    /// cg::callgrind::dump_stats(None);
    /// ```
    /// also see documentation for [`stop_instrumentation()`]
    ///
    /// # Implementation
    /// `CALLGRIND_START_INSTRUMENTATION`
    #[inline]
    pub fn start_instrumentation() {
        raw_call!(cl_start_instrumentation);
    }

    /// Stop full callgrind instrumentation if not already switched off
    ///
    /// This flushes Valgrinds translation cache, and does no additional instrumentation afterwards,
    /// which effectivly will run at the same speed as the "none" tool (ie. at minimal slowdown).
    ///
    /// also see documentation for [`start_instrumentation()`]
    ///
    /// # Implementation
    /// `CALLGRIND_STOP_INSTRUMENTATION`
    #[inline]
    pub fn stop_instrumentation() {
        raw_call!(cl_stop_instrumentation);
    }
}

pub mod cachegrind {
    //! [`Cachegrind requests`](https://valgrind.org/docs/manual/cg-manual.html#cg-manual.clientrequests)
    use super::*;

    /// Start full cachegrind instrumentation if not already switched on
    ///
    /// When cache simulation is done, it will flush the simulated cache;
    /// this will lead to an artificial cache warmup phase afterwards with cache misses which
    /// would not have happened in reality.
    ///
    /// Use this to bypass Cachegrind aggregation for uninteresting code parts.
    /// To start Callgrind in this mode to ignore the setup phase, use the option `--instr-at-start=no`.
    ///
    /// # Example
    /// ```no_run
    /// use crabgrind as cg;
    ///
    /// let xs = (0..10 << 10).into_iter().collect::<Vec<u32>>();
    ///
    /// cg::cachegrind::start_instrumentation();
    /// let i = xs.binary_search(&(10 << 10 >> 1));
    /// cg::cachegrind::stop_instrumentation();
    /// ```
    /// also see documentation for [`cachegrind::stop_instrumentation()`]
    ///
    /// # Implementation
    /// `CACHEGRIND_START_INSTRUMENTATION`
    #[inline]
    pub fn start_instrumentation() {
        raw_call!(cg_start_instrumentation);
    }

    /// Stop full cachegrind instrumentation if not already switched off
    ///
    /// This flushes Valgrind's translation cache, and does no additional instrumentation afterwards,
    /// which effectively will run at the same speed as the "none" tool (ie. at minimal slowdown).
    ///
    /// also see documentation for [`cachegrind::start_instrumentation()`]
    ///
    /// # Implementation
    /// `CACHEGRIND_STOP_INSTRUMENTATION`
    #[inline]
    pub fn stop_instrumentation() {
        raw_call!(cg_stop_instrumentation);
    }
}

pub mod memcheck {
    //! [`Memcheck requests`](https://valgrind.org/docs/manual/mc-manual.html#mc-manual.clientreqs)
    use super::*;
    pub use bindings::LeakCount;

    pub type BlockDescHandle = u32;

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
    #[non_exhaustive]
    pub enum Error {
        InvalidHandle,
        NotAddressable(usize),
        NoValgrind,
        UnalignedArrays,
    }

    impl std::error::Error for Error {}
    unsafe impl Send for Error {}
    unsafe impl Sync for Error {}
    impl std::fmt::Display for Error {
        #[inline]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::InvalidHandle => f.write_str("Invalid memory block description handle"),
                Error::NotAddressable(addr) => {
                    write!(f, "Memory starting from 0x{addr:X} is not addressable")
                }
                Error::NoValgrind => f.write_str("Not running under Valgrind"),
                Error::UnalignedArrays => {
                    f.write_str("[previously indicated unaligned arrays;  these are now allowed]")
                }
            }
        }
    }

    pub type Result<T = ()> = std::result::Result<T, Error>;

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
    pub enum LeakCheck {
        Full,
        Quick,
        Added,
        Changed,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
    pub enum MemState {
        NoAccess,
        Undefined,
        Defined,
        DefinedIfAddressable,
    }

    /// Mark memory state for an address range
    ///
    /// # Memory mark option
    /// **MemState::NoAccess**
    /// - mark address ranges as completely inaccessible
    ///
    /// **MemState::Defined**
    /// - mark address ranges as accessible but containing undefined data
    ///
    /// **MemState::Undefined**
    /// - mark address ranges as accessible and containing defined data
    ///
    /// **MemState::DefinedIfAddressable**
    /// - same as `MemState::Defined` but only affects those bytes that are already addressable
    ///
    /// # Implementation
    /// - [MemState::NoAccess] `VALGRIND_MAKE_MEM_NOACCESS`
    /// - [MemState::Undefined] `VALGRIND_MAKE_MEM_UNDEFINED`
    /// - [MemState::Defined] `VALGRIND_MAKE_MEM_DEFINED`
    /// - [MemState::DefinedIfAddressable] `VALGRIND_MAKE_MEM_DEFINED_IF_ADDRESSABLE`
    #[inline]
    pub fn mark_mem(addr: *mut c_void, len: usize, mark: MemState) -> Result {
        let ret = match mark {
            MemState::NoAccess => raw_call!(mc_make_mem_noaccess, addr, len),
            MemState::Undefined => raw_call!(mc_make_mem_undefined, addr, len),
            MemState::Defined => raw_call!(mc_make_mem_defined, addr, len),
            MemState::DefinedIfAddressable => {
                raw_call!(mc_make_mem_defined_if_addressable, addr, len)
            }
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::NoValgrind)
        }
    }

    /// Create a block-description handle
    ///
    /// The description is an ascii string which is included in any messages pertaining to
    /// addresses within the specified memory range.  Has no other effect on the properties of
    /// the memory range.
    ///
    /// # Implementation
    /// `VALGRIND_CREATE_BLOCK`
    ///
    /// # Panics
    /// If string contains null-byte in any position.
    #[inline]
    pub fn new_block_description(
        addr: *mut c_void,
        len: usize,
        desc: impl AsRef<str>,
    ) -> BlockDescHandle {
        let cstr = std::ffi::CString::new(desc.as_ref()).unwrap();
        raw_call!(mc_create_block, addr, len, cstr.as_ptr())
    }

    /// Discard a block-description-handle
    ///
    /// # Implementation
    /// `VALGRIND_DISCARD`
    #[inline]
    pub fn discard(handle: BlockDescHandle) -> Result {
        if raw_call!(mc_discard, handle) == 0 {
            Ok(())
        } else {
            Err(Error::InvalidHandle)
        }
    }

    /// Check that memory range is addressable
    ///
    /// If suitable addressibility is not established, Valgrind prints an error message and returns
    /// the address of the first offending byte.
    ///
    /// # Implementation
    /// `VALGRIND_CHECK_MEM_IS_ADDRESSABLE`
    #[inline]
    pub fn is_addressable(addr: *mut c_void, len: usize) -> Result {
        match raw_call!(mc_check_mem_is_addressable, addr, len) {
            0 => Ok(()),
            addr => Err(Error::NotAddressable(addr)),
        }
    }

    /// Check that memory range is addressable and defined
    ///
    /// If suitable addressibility and definedness are not established, Valgrind prints an error
    /// message and returns the address of the first offending byte.
    ///
    /// # Implementation
    /// `VALGRIND_CHECK_MEM_IS_DEFINED`
    #[inline]
    pub fn is_defined(addr: *mut c_void, len: usize) -> Result {
        match raw_call!(mc_check_mem_is_defined, addr, len) {
            0 => Ok(()),
            addr => Err(Error::NotAddressable(addr)),
        }
    }

    /// Do a memory leak check
    ///
    /// # Memory check option
    /// **LeakCheck::Full**
    /// - Do a full memory leak check (like --leak-check=full) mid-execution. This is useful for
    /// incrementally checking for leaks between arbitrary places in the program's execution.
    ///
    /// **LeakCheck::Quick**
    /// - Do a summary memory leak check (like --leak-check=summary) mid-execution.
    ///
    /// **LeakCheck::Added**
    /// - Same as `LeakCheck::Full` but only showing the entries for which there was an increase in
    /// leaked bytes or leaked number of blocks since the previous leak search.
    ///
    /// **LeakCheck::Changed**
    /// - Same as `LeakCheck::Added` but showing entries with increased or decreased leaked
    /// bytes/blocks since previous leak search.
    ///
    /// # Implementation
    /// - [LeakCheck::Full]  `VALGRIND_DO_LEAK_CHECK`
    /// - [LeakCheck::Quick]  `VALGRIND_DO_QUICK_LEAK_CHECK`
    /// - [LeakCheck::Added]  `VALGRIND_DO_ADDED_LEAK_CHECK`
    /// - [LeakCheck::Changed]  `VALGRIND_DO_CHANGED_LEAK_CHECK`
    #[inline]
    pub fn leak_check(mode: LeakCheck) {
        match mode {
            LeakCheck::Full => raw_call!(mc_do_leak_check),
            LeakCheck::Quick => raw_call!(mc_do_quick_leak_check),
            LeakCheck::Added => raw_call!(mc_do_added_leak_check),
            LeakCheck::Changed => raw_call!(mc_do_changed_leak_check),
        };
    }

    /// Return number of leaked bytes found by all previous leak checks
    ///
    /// # Implementation
    /// `VALGRIND_COUNT_LEAKS`
    #[inline]
    pub fn leaks_count() -> LeakCount {
        raw_call!(mc_count_leaks)
    }

    /// Return number of leaked blocks found by all previous leak checks
    ///
    /// # Implementation
    /// `VALGRIND_COUNT_LEAK_BLOCKS`
    #[inline]
    pub fn block_leaks_count() -> LeakCount {
        raw_call!(mc_count_leak_blocks)
    }

    /// Get the validity data for address range
    ///
    /// # Implementation
    /// `VALGRIND_GET_VBITS`
    #[inline]
    pub fn vbits(addr: *mut c_void, bits: *const u8, nbytes: usize) -> Result {
        match raw_call!(mc_get_vbits, addr, bits, nbytes) {
            0 => Err(Error::NoValgrind),
            1 => Ok(()),
            2 => Err(Error::UnalignedArrays),
            3 => Err(Error::NotAddressable(0)),
            x => unreachable!("Unexpected return code {}", x),
        }
    }

    /// Set the validity data for address range
    ///
    /// # Implementation
    /// `VALGRIND_SET_VBITS`
    #[inline]
    pub fn set_vbits(addr: *mut c_void, bits: *const u8, nbytes: usize) -> Result {
        match raw_call!(mc_set_vbits, addr, bits, nbytes) {
            0 => Err(Error::NoValgrind),
            1 => Ok(()),
            2 => Err(Error::UnalignedArrays),
            3 => Err(Error::NotAddressable(0)),
            x => unreachable!("Unexpected return code {}", x),
        }
    }

    /// Disable reporting of addressing errors in the specified address range
    ///
    /// # Implementation
    /// `VALGRIND_DISABLE_ADDR_ERROR_REPORTING_IN_RANGE`
    #[inline]
    pub fn disable_error_reporting(addr: *mut c_void, len: usize) {
        raw_call!(mc_disable_addr_error_reporting_in_range, addr, len);
    }

    /// Re-enable reporting of addressing errors in the specified address range
    ///
    /// # Implementation
    /// `VALGRIND_ENABLE_ADDR_ERROR_REPORTING_IN_RANGE`
    #[inline]
    pub fn enable_error_reporting(addr: *mut c_void, len: usize) {
        raw_call!(mc_enable_addr_error_reporting_in_range, addr, len);
    }

    pub mod alloc {
        //! Heap memory functionality
        use super::super::*;

        /// Marks a region of memory as having been allocated by a `malloc()`-like function
        ///
        /// See the comments in `valgrind.h` for information on how to use it.
        ///
        /// # Implementation
        /// `VALGRIND_MALLOCLIKE_BLOCK`
        #[inline]
        pub fn malloc(addr: *mut c_void, size: usize, rz: usize, is_zeroed: bool) {
            raw_call!(vg_malloclike_block, addr, size, rz, is_zeroed);
        }

        /// Partner to [`malloc()`]
        ///
        /// See the comments in `valgrind.h` for information on how to use it.
        ///
        /// # Implementation
        /// `VALGRIND_FREELIKE_BLOCK`
        #[inline]
        pub fn free(addr: *mut c_void, rz: usize) {
            raw_call!(vg_freelike_block, addr, rz);
        }

        /// Informs Memcheck about reallocation
        ///
        /// See the comments in `valgrind.h` for information on how to use it.
        ///
        /// # Implementation
        /// `VALGRIND_RESIZEINPLACE_BLOCK`
        #[inline]
        pub fn resize_inplace(addr: *mut c_void, old_size: usize, new_size: usize, rz: usize) {
            raw_call!(vg_resizeinplace_block, addr, old_size, new_size, rz);
        }
    }

    pub mod mempool {
        //! Memory pools functionality
        //!
        //! refer to [`Memory pools`](https://valgrind.org/docs/manual/mc-manual.html#mc-manual.mempools)
        //! Valgrind manual.
        use super::super::*;

        /// `VALGRIND_MEMPOOL_AUTO_FREE`
        pub const AUTO_FREE: u32 = 1;

        /// `VALGRIND_MEMPOOL_METAPOOL`
        pub const METAPOOL: u32 = 2;

        /// Create a memory pool
        ///
        /// refer to [Memory Pools: describing and working with custom allocators](https://valgrind.org/docs/manual/mc-manual.html#mc-manual.mempools)
        ///
        /// # Implementation
        /// `VALGRIND_CREATE_MEMPOOL` or `VALGRIND_CREATE_MEMPOOL_EXT`
        #[inline]
        pub fn create(
            pool: *mut c_void,
            rz: usize,
            is_zeroed: bool,
            flags: impl Into<Option<u32>>,
        ) {
            match flags.into() {
                None => raw_call!(vg_create_mempool, pool, rz, is_zeroed),
                Some(flags) => raw_call!(vg_create_mempool_ext, pool, rz, is_zeroed, flags),
            };
        }

        /// Destroy a memory pool
        ///
        /// refer to [Memory Pools: describing and working with custom allocators](https://valgrind.org/docs/manual/mc-manual.html#mc-manual.mempools)
        ///
        /// # Implementation
        /// `VALGRIND_DESTROY_MEMPOOL`
        #[inline]
        pub fn destroy(pool: *mut c_void) {
            raw_call!(vg_destroy_mempool, pool);
        }

        /// Associate a piece of memory with a memory pool
        ///
        /// refer to [Memory Pools: describing and working with custom allocators](https://valgrind.org/docs/manual/mc-manual.html#mc-manual.mempools)
        ///
        /// # Implementation
        /// `VALGRIND_MEMPOOL_ALLOC`
        #[inline]
        pub fn alloc(pool: *mut c_void, addr: *mut c_void, size: usize) {
            raw_call!(vg_mempool_alloc, pool, addr, size);
        }

        /// Disassociate a piece of memory from a memory pool
        ///
        /// refer to [Memory Pools: describing and working with custom allocators](https://valgrind.org/docs/manual/mc-manual.html#mc-manual.mempools)
        ///
        /// # Implementation
        /// `VALGRIND_MEMPOOL_FREE`
        #[inline]
        pub fn free(pool: *mut c_void, addr: *mut c_void) {
            raw_call!(vg_mempool_free, pool, addr);
        }

        /// Disassociate any pieces outside a particular range
        ///
        /// refer to [Memory Pools: describing and working with custom allocators](https://valgrind.org/docs/manual/mc-manual.html#mc-manual.mempools)
        ///
        /// # Implementation
        /// `VALGRIND_MEMPOOL_TRIM`
        #[inline]
        pub fn trim(pool: *mut c_void, addr: *mut c_void, size: usize) {
            raw_call!(vg_mempool_trim, pool, addr, size);
        }

        /// Resize and/or move a piece associated with a memory pool
        ///
        /// refer to [Memory Pools: describing and working with custom allocators](https://valgrind.org/docs/manual/mc-manual.html#mc-manual.mempools)
        ///
        /// # Implementation
        /// `VALGRIND_MOVE_MEMPOOL`
        #[inline]
        pub fn move_to(pool_a: *mut c_void, pool_b: *mut c_void) {
            raw_call!(vg_move_mempool, pool_a, pool_b);
        }

        /// Resize and/or move a piece associated with a memory pool
        ///
        /// refer to [Memory Pools: describing and working with custom allocators](https://valgrind.org/docs/manual/mc-manual.html#mc-manual.mempools)
        ///
        /// # Implementation
        /// `VALGRIND_MEMPOOL_CHANGE`
        #[inline]
        pub fn change(pool: *mut c_void, addr_a: *mut c_void, addr_b: *mut c_void, size: usize) {
            raw_call!(vg_mempool_change, pool, addr_a, addr_b, size);
        }

        /// Check mempool existence
        ///
        /// refer to [Memory Pools: describing and working with custom allocators](https://valgrind.org/docs/manual/mc-manual.html#mc-manual.mempools)
        ///
        /// # Implementation
        /// `VALGRIND_MEMPOOL_EXISTS`
        #[inline]
        pub fn is_exists(pool: *mut c_void) -> bool {
            raw_call!(vg_mempool_exists, pool)
        }
    }

    pub mod stack {
        //! Stack memory functionality
        use super::super::*;

        pub type StackId = usize;

        /// Mark a piece of memory as being a stack
        ///
        /// # Implementation
        /// `VALGRIND_STACK_REGISTER`
        #[inline]
        pub fn register(lowest: *mut c_void, highest: *mut c_void) -> StackId {
            raw_call!(vg_stack_register, lowest, highest)
        }

        /// Unmark the piece of memory associated with a [`StackId`] as being a stack
        ///
        /// # Implementation
        /// `VALGRIND_STACK_DEREGISTER`
        #[inline]
        pub fn deregister(id: StackId) {
            raw_call!(vg_stack_deregister, id);
        }

        /// Change the start and end address of the [`StackId`]
        ///
        /// # Implementation
        /// `VALGRIND_STACK_CHANGE`
        #[inline]
        pub fn change(id: StackId, new_lowest: *mut c_void, new_highest: *mut c_void) {
            raw_call!(vg_stack_change, id, new_lowest, new_highest);
        }
    }
}

pub mod helgrind {
    //! [`Helgrind requests`](https://valgrind.org/docs/manual/hg-manual.html#hg-manual.client-requests)
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
    pub enum Annotation {
        HappensBefore,
        HappensAfter,
        New(usize),
        RwLockCreate,
        RwLockDestroy,
        /// 'true' for a writer lock
        RwLockAcquired(bool),
        RwLockReleased,
    }

    /// Let `Helgrind` forget everything it know about the specified memory range
    ///
    /// # Implementation
    /// `VALGRIND_HG_CLEAN_MEMORY`
    #[inline]
    pub fn clean_memory(addr: *mut c_void, len: usize) {
        raw_call!(hg_clean_memory, addr, len);
    }

    /// Annotations useful for debugging
    ///
    /// # Annotation options
    /// **Annotation::RwLockCreate**
    /// - Report that a lock has just been created at address LOCK
    ///
    /// **Annotation::RwLockDestroy**
    /// - Report that the lock at address LOCK is about to be destroyed
    ///
    /// **Annotation::RwLockAcquired**
    /// - Report that the lock at address LOCK has just been acquired
    ///
    /// **Annotation::RwLockReleased**
    /// - Report that the lock at address LOCK is about to be released
    ///
    /// **Annotation::HappensAfter** **Annotation::HappensBefore**
    /// - If threads `T1 .. Tn` all do ANNOTATE_HAPPENS_BEFORE(obj) and later (w.r.t. some
    /// notional global clock for the computation) thread `Tm` does ANNOTATE_HAPPENS_AFTER(obj),
    /// then `Helgrind` will regard all memory accesses done by `T1 .. Tn` before the ..BEFORE..
    /// call as happening-before all memory accesses done by `Tm` after the ..AFTER.. call.  
    /// Hence `Helgrind` won't complain about races if `Tm's` accesses afterwards are to the same
    /// locations as accesses before by any of `T1 .. Tn`.
    ///
    /// **Annotation::New**
    /// - Report that a new memory at "address" of size "size" has been allocated
    ///
    ///
    /// # Implementation
    /// - Annotation::RwLockCreate `ANNOTATE_RWLOCK_CREATE`
    /// - Annotation::RwLockDestroy `ANNOTATE_RWLOCK_DESTROY`
    /// - Annotation::RwLockAcquired `ANNOTATE_RWLOCK_ACQUIRED`
    /// - Annotation::RwLockReleased `ANNOTATE_RWLOCK_RELEASED`
    /// - Annotation::HappensAfter `ANNOTATE_HAPPENS_AFTER`
    /// - Annotation::HappensBefore `ANNOTATE_HAPPENS_BEFORE`
    /// - Annotation::New `ANNOTATE_NEW_MEMORY`
    #[inline]
    pub fn annotate_memory(addr: *mut c_void, rel: Annotation) {
        match rel {
            Annotation::RwLockCreate => raw_call!(hg_rwlock_create, addr),
            Annotation::RwLockDestroy => raw_call!(hg_rwlock_destroy, addr),
            Annotation::RwLockAcquired(is_wl) => raw_call!(hg_rwlock_acquired, addr, is_wl),
            Annotation::RwLockReleased => raw_call!(hg_rwlock_released, addr),
            Annotation::HappensAfter => raw_call!(hg_annotate_happens_after, addr),
            Annotation::HappensBefore => raw_call!(hg_annotate_happens_before, addr),
            Annotation::New(size) => raw_call!(hg_annotate_new_memory, addr, size),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::{self as cg, valgrind::ThreadId};

    #[test]
    fn test_run_mode_under_valgrind() {
        assert_eq!(cg::RunMode::Valgrind, cg::run_mode());
    }

    #[test]
    fn print_macros_wont_fail() {
        // we are fine as long as it's not crashing
        let m = "crabgrind";
        cg::print!("{m}");
        cg::println!("het, {m}");
        cg::print_stacktrace!("{m}");
    }

    #[test]
    fn ok_monitor_command() {
        // we are fine as long as it's not crashing
        cg::monitor_command("v.info all_errors").unwrap();
    }

    #[test]
    #[should_panic]
    fn wrong_monitor_command() {
        cg::monitor_command("hey valgringo").unwrap();
    }

    #[test]
    fn disable_error_reporting() {
        cg::disable_error_reporting();

        unsafe {
            let b = Box::new([0]);
            println!("{}", b.get_unchecked(1));
        };
        assert_eq!(cg::count_errors(), 0);
    }

    #[test]
    fn non_simd_call() {
        let mut tid = ThreadId::MAX;
        cg::valgrind::non_simd_call(|id| {
            tid = id;
        });
        assert_ne!(tid, ThreadId::MAX);
    }

    #[test]
    fn change_cli_option() {
        cg::change_cli_option("--leak-check=no");
        std::mem::forget(String::from("leaked"));
    }
}
