//! ()
#![no_std]

mod bindings;
mod requests;

pub use requests::{ScopeGuard, cachegrind, callgrind, dhat, drd, helgrind, memcheck, valgrind};

pub use bindings::valgrind_client_request_expr;

/// Valgrind version this crate was compiled against.
pub const VALGRIND_VERSION: (u32, u32) =
    (bindings::__VALGRIND_MAJOR__, bindings::__VALGRIND_MINOR__);

#[doc(hidden)]
#[inline(always)]
pub fn __print(t: impl AsRef<core::ffi::CStr>) {
    unsafe { bindings::vg_print(t.as_ref().as_ptr()) };
}

#[doc(hidden)]
#[inline(always)]
pub fn __print_stacktrace(t: impl AsRef<core::ffi::CStr>) {
    unsafe { bindings::vg_print_backtrace(t.as_ref().as_ptr()) };
}
