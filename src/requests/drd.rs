use super::{client_request, helgrind as hg, valgrind::ThreadId};
use crate::bindings::CG_DRDClientRequest as CR;

use core::{
    ffi::{CStr, c_char, c_void},
    marker::PhantomData,
    mem::size_of,
};

/// Defines the behavior for a DRD scope operation.
///
/// See [`ignore_var`], [`trace_var`], [`annotate_trace_memory`]
pub trait DRDRegionMode: sealed::Sealed {
    #[doc(hidden)]
    fn enter(addr: usize, size: usize);
    #[doc(hidden)]
    fn exit(addr: usize, size: usize);
}

#[doc = include_str!("../../doc/drd/DRDRegionGuard.md")]
#[clippy::has_significant_drop]
#[derive(Debug)]
#[must_use = "The guard activates immediately upon creation. Dropping it instantly reverts the operation."]
pub struct DRDRegionGuard<'a, T, M: DRDRegionMode> {
    addr: usize,
    _scope: PhantomData<&'a ()>,
    _marker: PhantomData<(T, M)>,
}

// Marker type for the "Ignoring" mode (`DRD_IGNORE_VAR`).
#[doc(hidden)]
#[derive(Debug)]
pub struct Ignoring;
// Marker type for the "Tracing" mode (`DRD_TRACE_VAR`).
#[doc(hidden)]
#[derive(Debug)]
pub struct Tracing;

impl DRDRegionMode for Ignoring {
    #[inline(always)]
    fn enter(addr: usize, size: usize) {
        client_request!(CR::CG_ANNOTATE_BENIGN_RACE_SIZED, addr, size);
    }

    #[inline(always)]
    fn exit(addr: usize, size: usize) {
        client_request!(CR::CG_DRD_STOP_IGNORING_VAR, addr, size);
    }
}

impl DRDRegionMode for Tracing {
    #[inline(always)]
    fn enter(addr: usize, size: usize) {
        client_request!(CR::CG_DRD_TRACE_VAR, addr, size);
    }

    #[inline(always)]
    fn exit(addr: usize, size: usize) {
        client_request!(CR::CG_DRD_STOP_TRACING_VAR, addr, size);
    }
}

impl<T, M: DRDRegionMode> DRDRegionGuard<'_, T, M> {
    #[inline(always)]
    fn new(addr: *const T) -> Self {
        let addr = addr as usize;

        M::enter(addr, size_of::<T>());

        Self { addr, _marker: PhantomData, _scope: PhantomData }
    }
}

impl<T, M: DRDRegionMode> Drop for DRDRegionGuard<'_, T, M> {
    #[inline]
    fn drop(&mut self) {
        M::exit(self.addr, size_of::<T>());
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
pub fn ignore_var<T>(var: &T) -> DRDRegionGuard<'_, T, Ignoring> {
    DRDRegionGuard::new(var as *const _)
}

#[doc = include_str!("../../doc/drd/trace_var.md")]
#[inline(always)]
pub fn trace_var<T>(var: &T) -> DRDRegionGuard<'_, T, Tracing> {
    DRDRegionGuard::new(var as *const _)
}

#[doc = include_str!("../../doc/drd/annotate_trace_memory.md")]
#[inline(always)]
pub fn annotate_trace_memory<'a>(addr: *const c_void) -> DRDRegionGuard<'a, c_char, Tracing> {
    DRDRegionGuard::new(addr.cast())
}

#[inline(always)]
pub fn annotate_happens_before(addr: *const c_void) {
    hg::annotate_addr(addr, hg::Annotation::HappensBefore);
}

#[inline(always)]
pub fn annotate_happens_after(addr: *const c_void) {
    hg::annotate_addr(addr, hg::Annotation::HappensAfter);
}

#[inline(always)]
pub fn annotate_rwlock_create(addr: *const c_void) {
    hg::annotate_addr(addr, hg::Annotation::RwLockCreate);
}

#[inline(always)]
pub fn annotate_rwlock_destroy(addr: *const c_void) {
    hg::annotate_addr(addr, hg::Annotation::RwLockDestroy);
}

#[inline(always)]
pub fn annotate_rwlock_acquired(addr: *const c_void, writer: bool) {
    hg::annotate_addr(addr, hg::Annotation::RwLockAcquired(writer));
}

#[inline(always)]
pub fn annotate_rwlock_released(addr: *const c_void, writer: bool) {
    hg::annotate_addr(addr, hg::Annotation::RwLockReleased(writer));
}

#[inline(always)]
pub fn annotate_benign_race<T: Sized>(addr: &T) {
    annotate_benign_race_sized((addr as *const T).cast(), size_of::<T>());
}

#[inline(always)]
pub fn annotate_benign_race_sized(addr: *const c_void, size: usize) {
    client_request!(CR::CG_ANNOTATE_BENIGN_RACE_SIZED, addr, size);
}

#[inline(always)]
pub fn annotate_ignore_reads(ignore: bool) {
    client_request!(CR::CG_ANNOTATE_IGNORE_READS_BEGIN, !ignore);
}

#[inline(always)]
pub fn annotate_ignore_writes(ignore: bool) {
    client_request!(CR::CG_ANNOTATE_IGNORE_WRITES_BEGIN, !ignore);
}

#[inline(always)]
pub fn annotate_ignore_read_and_writes(ignore: bool) {
    annotate_ignore_reads(ignore);
    annotate_ignore_writes(ignore);
}

#[inline(always)]
pub fn annotate_new_memory(addr: *const c_void, size: usize) {
    client_request!(CR::CG_ANNOTATE_NEW_MEMORY, addr, size);
}

#[inline(always)]
pub fn annotate_thread_name(name: impl AsRef<CStr>) {
    client_request!(CR::CG_ANNOTATE_THREAD_NAME, name.as_ref().as_ptr());
}

// TODO: `ANNOTATE_BARRIER_*` placeholders
// As of Valgrind 3.26 `ANNOTATE_BARRIER_*` Client Requests remains unimplemented, and are
// planned for future version of DRD.

#[doc(hidden)]
#[inline(always)]
pub fn annotate_barrier_init(barrier_t: *const c_void, count: u32, reinitialization_allowed: bool) {
    client_request!(CR::CG_ANNOTATE_BARRIER_INIT, barrier_t, count, reinitialization_allowed);
}

#[doc(hidden)]
#[inline(always)]
pub fn annotate_barrier_destroy(barrier_t: *const c_void) {
    client_request!(CR::CG_ANNOTATE_BARRIER_INIT, barrier_t);
}

#[doc(hidden)]
#[inline(always)]
pub fn annotate_barrier_wait_before(barrier_t: *const c_void) {
    client_request!(CR::CG_ANNOTATE_BARRIER_INIT, barrier_t);
}

#[doc(hidden)]
#[inline(always)]
pub fn annotate_barrier_wait_after(barrier_t: *const c_void) {
    client_request!(CR::CG_ANNOTATE_BARRIER_INIT, barrier_t);
}

mod sealed {
    pub trait Sealed {}
}
impl sealed::Sealed for Ignoring {}
impl sealed::Sealed for Tracing {}
