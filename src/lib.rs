#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(not(feature = "valgrind"), allow(unused, missing_docs, clippy::needless_pass_by_value))]
#![no_std]

#[cfg(feature = "opt-out")]
compile_error!("`opt-out` was removed (v0.3). Use `default-features = false`.");

#[cfg(feature = "valgrind")]
mod bindings;
mod requests;
pub use requests::{ScopeGuard, cachegrind, callgrind, dhat, drd, helgrind, memcheck, valgrind};

/// Valgrind version this crate was compiled against.
pub const VALGRIND_VERSION: (u32, u32) = imp::VALGRIND_VERSION;
/// Whether Valgrind headers were available when `crabgrind` was compiled.
pub const VALGRIND_AVAILABLE: bool = VALGRIND_VERSION.0 != 0xBEDA_BEDA;

#[doc(hidden)]
#[cfg(feature = "valgrind")]
pub mod imp {
    pub const VALGRIND_VERSION: (u32, u32) =
        (super::bindings::__VALGRIND_MAJOR__, super::bindings::__VALGRIND_MINOR__);

    #[doc = include_str!("../doc/println.md")]
    #[macro_export]
    macro_rules! println{
        ($($arg:tt)+) => {{
            let msg = format!("{}\n\0", format_args!($($arg)+));

            let msg = unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(msg.as_bytes()) };
            $crate::imp::__print(msg);
        }}
    }

    #[doc = include_str!("../doc/println_stacktrace.md")]
    #[macro_export]
    macro_rules! print_stacktrace{
        ($($arg:tt)+) => {{
            let msg = format!("{}\0", format_args!($($arg)+));

            let msg = unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(msg.as_bytes()) };
            $crate::imp::__print_stacktrace(msg);
        }}
    }

    #[inline(always)]
    pub fn __print(t: impl AsRef<core::ffi::CStr>) {
        unsafe { super::bindings::vg_print(t.as_ref().as_ptr()) };
    }

    #[inline(always)]
    pub fn __print_stacktrace(t: impl AsRef<core::ffi::CStr>) {
        unsafe { super::bindings::vg_print_backtrace(t.as_ref().as_ptr()) };
    }
}

#[cfg(not(feature = "valgrind"))]
mod imp {
    pub const VALGRIND_VERSION: (u32, u32) = (0, 0);

    #[doc = include_str!("../doc/println.md")]
    #[macro_export]
    macro_rules! println {
        ($($arg:tt)+) => {};
    }

    #[doc = include_str!("../doc/println_stacktrace.md")]
    #[macro_export]
    macro_rules! print_stacktrace {
        ($($arg:tt)+) => {};
    }
}
