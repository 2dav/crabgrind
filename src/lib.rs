#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(
    not(valgrind_available),
    allow(unused, missing_docs, clippy::needless_pass_by_value, clippy::redundant_else)
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

#[cfg(feature = "opt-out")]
compile_error!("`opt-out` was removed (v0.3). Use `default-features = false`.");

#[doc(hidden)]
#[cfg(feature = "alloc")]
pub extern crate alloc;

#[cfg(feature = "valgrind")]
mod bindings;
mod requests;

pub use requests::{cachegrind, callgrind, dhat, drd, helgrind, memcheck, valgrind, ScopeGuard};

/// Valgrind version this crate was compiled against.
pub const VALGRIND_VERSION: (u32, u32) = valgrind_version();
#[doc = include_str!("../doc/VALGRIND_AVAILABLE.md")]
pub const VALGRIND_AVAILABLE: bool = cfg!(valgrind_available);

/// Printing to the Valgrind output channel
#[inline(always)]
pub fn vg_print<C>(cstr: &C)
where
    C: AsRef<core::ffi::CStr> + ?Sized,
{
    #[cfg(valgrind_available)]
    unsafe {
        bindings::vg_print(cstr.as_ref().as_ptr())
    };
}

/// Printing to the Valgrind output channel with a stack trace attached
#[inline(always)]
pub fn vg_print_stacktrace<C>(cstr: &C)
where
    C: AsRef<core::ffi::CStr> + ?Sized,
{
    #[cfg(valgrind_available)]
    unsafe {
        bindings::vg_print_backtrace(cstr.as_ref().as_ptr())
    };
}

const fn valgrind_version() -> (u32, u32) {
    #[cfg(valgrind_available)]
    return (
        bindings::CR::CG___VALGRIND_MAJOR__.code(),
        bindings::CR::CG___VALGRIND_MINOR__.code(),
    );
    #[cfg(not(valgrind_available))]
    (0, 0)
}

#[doc(hidden)]
#[cfg(valgrind_available)]
mod imp {
    #[cfg(feature = "alloc")]
    #[doc = include_str!("../doc/println.md")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    #[macro_export]
    macro_rules! println{
        ($($arg:tt)+) => {{
            let msg = $crate::alloc::format!("{}\n\0", format_args!($($arg)+));

            let msg = unsafe { core::ffi::CStr::from_ptr(msg.as_bytes().as_ptr().cast()) };
            $crate::vg_print(msg);
        }};
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
        }};
    }
}

#[doc(hidden)]
#[cfg(not(valgrind_available))]
mod imp {
    #[cfg(feature = "alloc")]
    #[doc = include_str!("../doc/println.md")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    #[macro_export]
    macro_rules! println {
        ($($arg:tt)+) => {
            ()
        };
    }

    #[cfg(feature = "alloc")]
    #[doc = include_str!("../doc/println_stacktrace.md")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    #[macro_export]
    macro_rules! print_stacktrace {
        ($($arg:tt)+) => {
            ()
        };
    }
}
