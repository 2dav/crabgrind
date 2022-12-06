//! [Valgrind Client Request](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq) interface for Rust
//!
//!`crabgrind` wraps various Valgrind client request macros in a C functions, compiles and links against
//! the resulting binary and exposes unsafe interface, allowing Rust programs running under Valgrind to
//! interact with the tools and environment.
//!
//! ### Valgrind 3 API coverage
//! - Supported tool-specific client request interface:
//!     - [valgrind](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq)
//!     - [callgrind](https://valgrind.org/docs/manual/cl-manual.html)
//!     - [memcheck](https://valgrind.org/docs/manual/mc-manual.html)
//!     - [helgrind](https://valgrind.org/docs/manual/hg-manual.html)
//!     - [massif](https://valgrind.org/docs/manual/ms-manual.html)
//! - [Monitor commands](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.gdbserver-commandhandling) interface
//!
//! ### Quickstart
//! `crabgrind` imports macros from Valgrind's header files, therefor you must have Valgrind
//! installed to build the project.
//!
//! Add the following to your `Cargo.toml` file:
//! ```no_run
//! [dependencies]
//! crabgrind = "^0.1"
//! ```
//!
//! ### Examples
//! **Print some message to the Valgrind log**
//! ```no_run
//! use crabgrind as cg;
//!
//! if matches!(cg::run_mode(), cg::RunMode::Native) {
//!     println!("run me under Valgrind");
//! } else {
//!     let msg = std::ffi::CString::new("Hey Valgrind!\n")?;
//!     cg::print(msg);
//! }
//! ```
//!
//! **Exclude expensive (de)initialization code from the measurements**
//!
//! One way to do this would be to turn off stats collection at stratup with the
//! [`--collect-atstart=no`](https://valgrind.org/docs/manual/cl-manual.html#opt.collect-atstart)
//! callgrind command-line attribute, and enable/disable it from the code with [`callgrind::toggle_collect`]
//!
//! ```no_run
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
//! **Run a closure on the real CPU while running under Valgrind**
//!
//! We can run on the real CPU instead of the virtual one using [`valgrind::non_simd_call`],
//! refer to `valgrind.h` for details on limitations and various ways to crash.
//!
//! ```no_run
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
//! **Save current memory usage snapshot to a file**
//!
//! We'll use `Massif` tool and the [monitor command](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.gdbserver-commandhandling)
//! interface to run the corresponding Massif command.
//! ```no_run
//! use crabgrind as cg;
//! use std::ffi::CString;
//!
//! let heap = String::from("alloca");
//!
//! cg::monitor_command(CString::new("snapshot mem.snapshot")?);
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
//! however, wrapping each macros in a function implies function call overhead regardless of the run
//! mode, plus, some of the wrappers returns `std::result::Result` which involves branching.
//! If you wish to compile out all (crab)Valgrind from the binary, you can wrap `crabgrind` calls with
//! the feature-gate.

use std::ffi::{c_void, CStr};

mod bindings;

macro_rules! raw_call {
    ($f:ident) => { raw_call!($f,) };
    ($f:ident, $($args:tt)*) => {{
        unsafe{ bindings::$f($($args)*) }
    }};
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum RunMode {
    Native,
    Valgrind,
    ValgrindInValgrind(usize),
}

/// RUNNING_ON_VALGRIND
#[inline]
pub fn run_mode() -> RunMode {
    match unsafe { bindings::running_on_valgrind() } {
        0 => RunMode::Native,
        1 => RunMode::Valgrind,
        x => RunMode::ValgrindInValgrind(x),
    }
}

/// VALGRIND_PRINTF
/// wrapped with the fixed `"%s"` format
#[inline]
pub fn print(msg: impl AsRef<CStr>) -> usize {
    raw_call!(vg_print, msg.as_ref().as_ptr())
}

/// VALGRIND_PRINTF_BACKTRACE
/// wrapped with the fixed `"%s"` format
#[inline]
pub fn print_backtrace(msg: impl AsRef<CStr>) -> usize {
    raw_call!(vg_print_backtrace, msg.as_ref().as_ptr())
}

/// VALGRIND_MONITOR_COMMAND
#[inline]
pub fn monitor_command(cmd: impl AsRef<CStr>) -> bool {
    raw_call!(vg_monitor_command, cmd.as_ref().as_ptr())
}

pub mod valgrind {
    //! [`Valgrind requests`](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq)
    use std::os::unix::prelude::RawFd;

    use super::*;

    /// VALGRIND_DISABLE_ERROR_REPORTING
    #[inline]
    pub fn disable_error_reporting() {
        raw_call!(vg_disable_error_reporting);
    }

    /// VALGRIND_ENABLE_ERROR_REPORTING
    #[inline]
    pub fn enable_error_reporting() {
        raw_call!(vg_enable_error_reporting);
    }

    /// VALGRIND_COUNT_ERRORS
    #[inline]
    pub fn count_errors() -> usize {
        raw_call!(vg_count_errors)
    }

    /// VALGRIND_CLO_CHANGE
    #[inline]
    pub fn cli_option_change(opt: impl AsRef<CStr>) {
        raw_call!(vg_clo_change, opt.as_ref().as_ptr());
    }

    /// VALGRIND_DISCARD_TRANSLATIONS
    #[inline]
    pub fn discard_translations(addr: *mut c_void, len: usize) {
        raw_call!(vg_discard_translations, addr, len);
    }

    /// VALGRIND_LOAD_PDB_DEBUGINFO
    #[inline]
    pub fn load_pdb_debuginfo(fd: RawFd, ptr: *mut c_void, total_size: usize, delta: usize) {
        raw_call!(vg_load_pdb_debuginfo, fd, ptr, total_size, delta);
    }

    /// VALGRIND_MAP_IP_TO_SRCLOC
    #[inline]
    pub fn map_ip_to_srcloc(addr: *mut c_void, buf64: *mut c_void) -> usize {
        raw_call!(vg_map_ip_to_srcloc, addr, buf64)
    }

    extern "C" fn _closure_adapter<F>(tid: usize, f: *mut c_void)
    where
        F: FnMut(usize),
    {
        debug_assert!(!f.is_null(), "closure pointer is null");
        debug_assert_eq!(
            f as usize & (std::mem::align_of::<F>() - 1),
            0,
            "unexpected closure pointer"
        );
        if let Err(err) = std::panic::catch_unwind(|| unsafe { (*f.cast::<F>())(tid) }) {
            let panic_info = err
                .downcast::<String>()
                .map(|v| *v)
                .or_else(|e| e.downcast::<&str>().map(|v| v.to_string()))
                .unwrap_or_else(|_| "unknown panic source".to_string());

            eprintln!("closure code panicked with: {panic_info:?}");

            std::process::abort();
        }
    }

    /// VALGRIND_NON_SIMD_CALL1
    #[inline]
    pub fn non_simd_call<F>(f: F)
    where
        F: FnMut(usize),
    {
        let boxed = Box::into_raw(Box::new(f));
        raw_call!(vg_non_simd_call1, _closure_adapter::<F>, boxed.cast());
        let _ = unsafe { Box::from_raw(boxed) };
    }
}

pub mod callgrind {
    //! [`Callgrind requests`](https://courses.cs.vt.edu/~cs3214/fall2011/projects/valgrind/valgrind-3.4.0/docs/html/cl-manual.html#cl-manual.clientrequests)
    use super::*;

    /// CALLGRIND_DUMP_STATS | CALLGRIND_DUMP_STATS_AT
    #[inline]
    pub fn dump_stats<R: AsRef<CStr>>(reason: impl Into<Option<R>>) {
        match reason.into() {
            None => raw_call!(cg_dump_stats),
            Some(reason) => raw_call!(cg_dump_stats_at, reason.as_ref().as_ptr()),
        };
    }

    /// CALLGRIND_ZERO_STATS
    #[inline]
    pub fn zero_stats() {
        raw_call!(cg_zero_stats);
    }

    /// CALLGRIND_TOGGLE_COLLECT
    #[inline]
    pub fn toggle_collect() {
        raw_call!(cg_toggle_collect);
    }

    /// CALLGRIND_START_INSTRUMENTATION
    #[inline]
    pub fn start_instrumentation() {
        raw_call!(cg_start_instrumentation);
    }

    /// CALLGRIND_STOP_INSTRUMENTATION
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
        /// VALGRIND_DO_LEAK_CHECK
        Full,
        /// VALGRIND_DO_QUICK_LEAK_CHECK
        Quick,
        /// VALGRIND_DO_ADDED_LEAK_CHECK
        Added,
        /// VALGRIND_DO_CHANGED_LEAK_CHECK
        Changed,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
    pub enum MemMark {
        /// VALGRIND_MAKE_MEM_NOACCESS
        NoAccess,
        /// VALGRIND_MAKE_MEM_UNDEFINED
        Undefined,
        /// VALGRIND_MAKE_MEM_DEFINED
        Defined,
        /// VALGRIND_MAKE_MEM_DEFINED_IF_ADDRESSABLE
        DefinedIfAddressable,
    }

    #[inline]
    pub fn mark_mem(addr: *mut c_void, len: usize, mark: MemMark) {
        match mark {
            MemMark::NoAccess => raw_call!(mc_make_mem_noaccess, addr, len),
            MemMark::Undefined => raw_call!(mc_make_mem_undefined, addr, len),
            MemMark::Defined => raw_call!(mc_make_mem_defined, addr, len),
            MemMark::DefinedIfAddressable => {
                raw_call!(mc_make_mem_defined_if_addressable, addr, len)
            }
        };
    }

    /// VALGRIND_CREATE_BLOCK
    #[inline]
    pub fn create_block(addr: *mut c_void, len: usize, desc: impl AsRef<CStr>) -> BlockDescHandle {
        raw_call!(mc_create_block, addr, len, desc.as_ref().as_ptr())
    }

    /// VALGRIND_DISCARD
    #[inline]
    pub fn discard(handle: BlockDescHandle) -> Result {
        if raw_call!(mc_discard, handle) == 0 {
            Ok(())
        } else {
            Err(Error::InvalidHandle)
        }
    }

    /// VALGRIND_CHECK_MEM_IS_ADDRESSABLE
    #[inline]
    pub fn is_addressable(addr: *mut c_void, len: usize) -> Result {
        match raw_call!(mc_check_mem_is_addressable, addr, len) {
            0 => Ok(()),
            addr => Err(Error::NotAddressable(addr)),
        }
    }

    /// VALGRIND_CHECK_MEM_IS_DEFINED
    #[inline]
    pub fn is_defined(addr: *mut c_void, len: usize) -> Result {
        match raw_call!(mc_check_mem_is_defined, addr, len) {
            0 => Ok(()),
            addr => Err(Error::NotAddressable(addr)),
        }
    }

    #[inline]
    pub fn leak_check(mode: LeakCheck) {
        match mode {
            LeakCheck::Full => raw_call!(mc_do_leak_check),
            LeakCheck::Quick => raw_call!(mc_do_quick_leak_check),
            LeakCheck::Added => raw_call!(mc_do_added_leak_check),
            LeakCheck::Changed => raw_call!(mc_do_changed_leak_check),
        };
    }

    /// VALGRIND_COUNT_LEAKS
    #[inline]
    pub fn leaks_count() -> LeakCount {
        raw_call!(mc_count_leaks)
    }

    /// VALGRIND_COUNT_LEAK_BLOCKS
    #[inline]
    pub fn block_leaks_count() -> LeakCount {
        raw_call!(mc_count_leak_blocks)
    }

    /// VALGRIND_GET_VBITS
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

    /// VALGRIND_SET_VBITS
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

    /// VALGRIND_DISABLE_ADDR_ERROR_REPORTING_IN_RANGE
    #[inline]
    pub fn disable_error_reporting(addr: *mut c_void, len: usize) {
        raw_call!(mc_disable_addr_error_reporting_in_range, addr, len);
    }

    /// VALGRIND_ENABLE_ADDR_ERROR_REPORTING_IN_RANGE
    #[inline]
    pub fn enable_error_reporting(addr: *mut c_void, len: usize) {
        raw_call!(mc_enable_addr_error_reporting_in_range, addr, len);
    }

    pub mod alloc {
        use super::super::*;

        /// VALGRIND_MALLOCLIKE_BLOCK
        #[inline]
        pub fn malloc(addr: *mut c_void, size: usize, rz: usize, is_zeroed: bool) {
            raw_call!(vg_malloclike_block, addr, size, rz, is_zeroed);
        }

        /// VALGRIND_FREELIKE_BLOCK
        #[inline]
        pub fn free(addr: *mut c_void, rz: usize) {
            raw_call!(vg_freelike_block, addr, rz);
        }

        /// VALGRIND_RESIZEINPLACE_BLOCK
        #[inline]
        pub fn resize_inplace(addr: *mut c_void, old_size: usize, new_size: usize, rz: usize) {
            raw_call!(vg_resizeinplace_block, addr, old_size, new_size, rz);
        }
    }

    pub mod mempool {
        //! [`Memory pools requests`](https://valgrind.org/docs/manual/mc-manual.html#mc-manual.mempools)
        use super::super::*;

        /// VALGRIND_MEMPOOL_AUTO_FREE
        pub const AUTO_FREE: u32 = 1;

        /// VALGRIND_MEMPOOL_METAPOOL
        pub const METAPOOL: u32 = 2;

        /// VALGRIND_CREATE_MEMPOOL | VALGRIND_CREATE_MEMPOOL_EXT
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

        /// VALGRIND_DESTROY_MEMPOOL
        #[inline]
        pub fn destroy(pool: *mut c_void) {
            raw_call!(vg_destroy_mempool, pool);
        }

        /// VALGRIND_MEMPOOL_ALLOC
        #[inline]
        pub fn alloc(pool: *mut c_void, addr: *mut c_void, size: usize) {
            raw_call!(vg_mempool_alloc, pool, addr, size);
        }

        /// VALGRIND_MEMPOOL_FREE
        #[inline]
        pub fn free(pool: *mut c_void, addr: *mut c_void) {
            raw_call!(vg_mempool_free, pool, addr);
        }

        /// VALGRIND_MEMPOOL_TRIM
        #[inline]
        pub fn trim(pool: *mut c_void, addr: *mut c_void, size: usize) {
            raw_call!(vg_mempool_trim, pool, addr, size);
        }

        /// VALGRIND_MOVE_MEMPOOL
        #[inline]
        pub fn move_to(pool_a: *mut c_void, pool_b: *mut c_void) {
            raw_call!(vg_move_mempool, pool_a, pool_b);
        }

        /// VALGRIND_MEMPOOL_CHANGE
        #[inline]
        pub fn change(pool: *mut c_void, addr_a: *mut c_void, addr_b: *mut c_void, size: usize) {
            raw_call!(vg_mempool_change, pool, addr_a, addr_b, size);
        }

        /// VALGRIND_MEMPOOL_EXISTS
        #[inline]
        pub fn is_exists(pool: *mut c_void) -> bool {
            raw_call!(vg_mempool_exists, pool)
        }
    }

    pub mod stack {
        use super::super::*;

        pub type StackId = usize;

        /// VALGRIND_STACK_REGISTER
        #[inline]
        pub fn register(start: *mut c_void, end: *mut c_void) -> StackId {
            raw_call!(vg_stack_register, start, end)
        }

        /// VALGRIND_STACK_DEREGISTER
        #[inline]
        pub fn deregister(id: StackId) {
            raw_call!(vg_stack_deregister, id);
        }

        /// VALGRIND_STACK_CHANGE
        #[inline]
        pub fn change(id: StackId, start: *mut c_void, end: *mut c_void) {
            raw_call!(vg_stack_change, id, start, end);
        }
    }
}

pub mod helgrind {
    //! [`Helgrind requests`](https://valgrind.org/docs/manual/hg-manual.html#hg-manual.client-requests)
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
    pub enum Annotation {
        /// ANNOTATE_HAPPENS_BEFORE
        HappensBefore,
        /// ANNOTATE_HAPPENS_AFTER
        HappensAfter,
        /// ANNOTATE_NEW_MEMORY
        New(usize),
        /// ANNOTATE_RWLOCK_CREATE
        RwLockCreate,
        /// ANNOTATE_RWLOCK_DESTROY
        RwLockDestroy,
        /// ANNOTATE_RWLOCK_ACQUIRED, 'true' for a writer lock
        RwLockAcquired(bool),
        /// ANNOTATE_RWLOCK_RELEASED
        RwLockReleased,
    }

    /// VALGRIND_HG_CLEAN_MEMORY
    #[inline]
    pub fn clean_memory(addr: *mut c_void, len: usize) {
        raw_call!(hg_clean_memory, addr, len);
    }

    #[inline]
    pub fn annotate_memory(addr: *mut c_void, rel: Annotation) {
        match rel {
            Annotation::HappensBefore => raw_call!(hg_annotate_happens_before, addr),
            Annotation::HappensAfter => raw_call!(hg_annotate_happens_after, addr),
            Annotation::New(size) => raw_call!(hg_annotate_new_memory, addr, size),
            Annotation::RwLockCreate => raw_call!(hg_rwlock_create, addr),
            Annotation::RwLockDestroy => raw_call!(hg_rwlock_destroy, addr),
            Annotation::RwLockAcquired(is_wl) => raw_call!(hg_rwlock_acquired, addr, is_wl),
            Annotation::RwLockReleased => raw_call!(hg_rwlock_released, addr),
        };
    }
}
