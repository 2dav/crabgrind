#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(not(feature = "valgrind"), allow(unused, missing_docs, clippy::needless_pass_by_value))]
#![no_std]

#[cfg(feature = "opt-out")]
compile_error!("`opt-out` was removed (v0.3). Use `default-features = false`.");

#[doc(hidden)]
#[cfg(feature = "alloc")]
pub extern crate alloc;

#[cfg(feature = "valgrind")]
mod bindings;
mod requests;

pub use imp::{vg_print, vg_print_stacktrace};
pub use requests::{ScopeGuard, cachegrind, callgrind, dhat, drd, helgrind, memcheck, valgrind};

/// Valgrind version this crate was compiled against.
pub const VALGRIND_VERSION: (u32, u32) = imp::VALGRIND_VERSION;
#[doc = include_str!("../doc/VALGRIND_AVAILABLE.md")]
pub const VALGRIND_AVAILABLE: bool = VALGRIND_VERSION.0 != 0xBEDA_BEDA;

#[doc(hidden)]
#[cfg(feature = "valgrind")]
pub mod imp {

    pub const VALGRIND_VERSION: (u32, u32) =
        (super::bindings::__VALGRIND_MAJOR__, super::bindings::__VALGRIND_MINOR__);

    #[cfg(feature = "alloc")]
    #[doc = include_str!("../doc/println.md")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    #[macro_export]
    macro_rules! println{
        ($($arg:tt)+) => {{
            let msg = $crate::alloc::format!("{}\n\0", format_args!($($arg)+));

            let msg = unsafe { core::ffi::CStr::from_ptr(msg.as_bytes().as_ptr().cast()) };
            $crate::vg_print(msg);
        }}
    }

    #[cfg(feature = "alloc")]
    #[doc = include_str!("../doc/println_stacktrace.md")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    #[macro_export]
    macro_rules! print_stacktrace{
        ($($arg:tt)+) => {{
            let msg = $crate::alloc::format!("{}\0", format_args!($($arg)+));

            let msg = unsafe { core::ffi::CStr::from_ptr(msg.as_bytes().as_ptr().cast()) };
            $crate::vg_print_stacktrace(msg);
        }}
    }

    /// Printing to the Valgrind output channel
    #[inline(always)]
    pub fn vg_print<C>(t: &C)
    where
        C: AsRef<core::ffi::CStr> + ?Sized,
    {
        unsafe { super::bindings::vg_print(t.as_ref().as_ptr()) };
    }

    /// Printing to the Valgrind output channel with a stack trace attached
    #[inline(always)]
    pub fn vg_print_stacktrace<C>(t: &C)
    where
        C: AsRef<core::ffi::CStr> + ?Sized,
    {
        unsafe { super::bindings::vg_print_backtrace(t.as_ref().as_ptr()) };
    }
}

#[cfg(not(feature = "valgrind"))]
mod imp {
    pub const VALGRIND_VERSION: (u32, u32) = (0xBEDA_BEDA, 0xBEDA_BEDA);

    #[cfg(feature = "alloc")]
    #[doc = include_str!("../doc/println.md")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    #[macro_export]
    macro_rules! println {
        ($($arg:tt)+) => {};
    }

    #[cfg(feature = "alloc")]
    #[doc = include_str!("../doc/println_stacktrace.md")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    #[macro_export]
    macro_rules! print_stacktrace {
        ($($arg:tt)+) => {};
    }

    /// Printing to the Valgrind output channel
    #[inline(always)]
    pub fn vg_print<C>(_t: &C)
    where
        C: AsRef<core::ffi::CStr> + ?Sized,
    {
    }

    /// Printing to the Valgrind output channel with a stack trace attached
    #[inline(always)]
    pub fn vg_print_stacktrace<C>(_t: &C)
    where
        C: AsRef<core::ffi::CStr> + ?Sized,
    {
    }
}
