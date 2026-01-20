#![doc = include_str!("../../doc/drd.md")]
use super::{client_request, valgrind::ThreadId};
use crate::{
    bindings::CG_DRDClientRequest as CR,
    requests::{Scope, ScopeGuard, sealed::Sealed},
};

use core::{
    ffi::{CStr, c_char, c_void},
    marker::PhantomData,
};

// Marker type for the "Suppressing" mode (`DRD_IGNORE_VAR`, `DRD_STOP_IGNORING_VAR`).
//
// See [`ignore_var`]
#[doc(hidden)]
#[derive(Debug)]
pub struct DRDSuppressing<'a, T>(PhantomData<&'a T>);

// Marker type for the "Tracing" mode (`DRD_TRACE_VAR`, `DRD_STOP_TRACING_VAR`).
//
// See [`trace_var`], [`annotate_trace_memory`]
#[doc(hidden)]
#[derive(Debug)]
pub struct DRDTracing<'a, T>(PhantomData<&'a T>);

// Marker type for the "Ignoring Loads" mode (`ANNOTATE_IGNORE_READS_BEGIN`, `ANNOTATE_IGNORE_READS_END`).
//
// See [`annotate_ignore_reads`]
#[doc(hidden)]
#[derive(Debug)]
pub struct DRDIgnoringLoads;

// Marker type for the "Ignoring Stores" mode (`ANNOTATE_IGNORE_WRITES_BEGIN`, `ANNOTATE_IGNORE_WRITES_END`).
//
// See [`annotate_ignore_writes`]
#[doc(hidden)]
#[derive(Debug)]
pub struct DRDIgnoringStores;

impl<T> Scope for DRDTracing<'_, T> {
    type Inner = *const T;

    #[inline(always)]
    fn enter(addr: Self::Inner) {
        client_request!(CR::CG_DRD_TRACE_VAR, addr, size_of::<T>());
    }

    #[inline(always)]
    fn exit(addr: Self::Inner) {
        client_request!(CR::CG_DRD_STOP_TRACING_VAR, addr, size_of::<T>());
    }
}

impl<T> Scope for DRDSuppressing<'_, T> {
    type Inner = *const T;

    #[inline(always)]
    fn enter(addr: Self::Inner) {
        client_request!(CR::CG_ANNOTATE_BENIGN_RACE_SIZED, addr, size_of::<T>());
    }

    #[inline(always)]
    fn exit(addr: Self::Inner) {
        client_request!(CR::CG_DRD_STOP_IGNORING_VAR, addr, size_of::<T>());
    }
}

impl Scope for DRDIgnoringLoads {
    type Inner = *const c_void;

    #[inline(always)]
    fn enter(_: Self::Inner) {
        client_request!(CR::CG_ANNOTATE_IGNORE_READS_BEGIN, false);
    }

    #[inline(always)]
    fn exit(_: Self::Inner) {
        client_request!(CR::CG_ANNOTATE_IGNORE_READS_BEGIN, true);
    }
}

impl Scope for DRDIgnoringStores {
    type Inner = *const c_void;

    #[inline(always)]
    fn enter(_: Self::Inner) {
        client_request!(CR::CG_ANNOTATE_IGNORE_WRITES_BEGIN, false);
    }

    #[inline(always)]
    fn exit(_: Self::Inner) {
        client_request!(CR::CG_ANNOTATE_IGNORE_WRITES_BEGIN, true);
    }
}

#[doc = include_str!("../../doc/drd/valgrind_thread_id.md")]
#[inline(always)]
pub fn valgrind_threadid() -> ThreadId {
    client_request!(CR::CG_DRD_GET_VALGRIND_THREADID)
}

#[doc = include_str!("../../doc/drd/drd_thread_id.md")]
#[inline(always)]
pub fn drd_threadid() -> ThreadId {
    client_request!(CR::CG_DRD_GET_DRD_THREADID)
}

#[doc = include_str!("../../doc/drd/ignore_var.md")]
#[inline(always)]
pub fn ignore_var<T>(var: &T) -> ScopeGuard<DRDSuppressing<'_, T>> {
    ScopeGuard::new(var as _)
}

#[doc = include_str!("../../doc/drd/trace_var.md")]
#[inline(always)]
pub fn trace_var<T>(var: &T) -> ScopeGuard<DRDTracing<'_, T>> {
    ScopeGuard::new(var as _)
}

#[doc = include_str!("../../doc/drd/annotate_trace_memory.md")]
#[inline(always)]
pub fn annotate_trace_memory<'a>(addr: *const c_void) -> ScopeGuard<DRDTracing<'a, c_char>> {
    ScopeGuard::new(addr.cast::<c_char>() as _)
}

#[doc = include_str!("../../doc/drd/annotate_benign_race.md")]
#[inline(always)]
pub fn annotate_benign_race<T>(addr: &T) {
    annotate_benign_race_sized((addr as *const T).cast(), size_of::<T>());
}

#[doc = include_str!("../../doc/drd/annotate_benign_race_sized.md")]
#[inline(always)]
pub fn annotate_benign_race_sized(addr: *const c_void, size: usize) {
    client_request!(CR::CG_ANNOTATE_BENIGN_RACE_SIZED, addr, size);
}

#[doc = include_str!("../../doc/drd/annotate_ignore_reads.md")]
#[inline(always)]
pub fn annotate_ignore_reads() -> ScopeGuard<DRDIgnoringLoads> {
    ScopeGuard::new(core::ptr::null())
}

#[doc = include_str!("../../doc/drd/annotate_ignore_writes.md")]
#[inline(always)]
pub fn annotate_ignore_writes() -> ScopeGuard<DRDIgnoringStores> {
    ScopeGuard::new(core::ptr::null())
}

#[doc = include_str!("../../doc/drd/annotate_new_memory.md")]
#[inline(always)]
pub fn annotate_new_memory(addr: *const c_void, size: usize) {
    client_request!(CR::CG_ANNOTATE_NEW_MEMORY, addr, size);
}

#[doc = include_str!("../../doc/drd/annotate_thread_name.md")]
#[inline(always)]
pub fn annotate_thread_name(name: impl AsRef<CStr>) {
    client_request!(CR::CG_ANNOTATE_THREAD_NAME, name.as_ref().as_ptr());
}

impl<T> Sealed for DRDTracing<'_, T> {}
impl<T> Sealed for DRDSuppressing<'_, T> {}
impl Sealed for DRDIgnoringLoads {}
impl Sealed for DRDIgnoringStores {}
