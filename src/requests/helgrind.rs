use super::client_request;
use crate::bindings::CG_HelgrindClientRequest as CR;

use core::ffi::c_void;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum Annotation {
    HappensBefore,
    HappensBeforeForgetAll,
    HappensAfter,
    New(usize),
    RwLockCreate,
    RwLockDestroy,
    /// 'true' for a writer lock
    RwLockAcquired(bool),
    /// 'true' for a writer lock
    RwLockReleased(bool),
}

#[inline(always)]
pub fn clean_memory(start: *const c_void, len: usize) {
    client_request!(CR::CG_VALGRIND_HG_CLEAN_MEMORY, start, len);
}

#[inline(always)]
pub fn clean_ref<T: Sized>(obj: &T) {
    clean_memory((obj as *const T).cast(), core::mem::size_of::<T>());
}

#[inline(always)]
pub fn annotate_rwlock_create(addr: *const c_void) {
    client_request!(CR::CG_ANNOTATE_RWLOCK_CREATE, addr);
}

#[inline(always)]
pub fn annotate_rwlock_destroy(addr: *const c_void) {
    client_request!(CR::CG_ANNOTATE_RWLOCK_DESTROY, addr);
}

#[inline(always)]
pub fn annotate_rwlock_acquired(addr: *const c_void, writer_lock: bool) {
    client_request!(CR::CG_ANNOTATE_RWLOCK_ACQUIRED, addr, writer_lock);
}

#[inline(always)]
pub fn annotate_rwlock_released(addr: *const c_void, writer_lock: bool) {
    client_request!(CR::CG_ANNOTATE_RWLOCK_RELEASED, addr, writer_lock);
}

#[inline(always)]
pub fn annotate_addr(addr: *const c_void, annotation: Annotation) {
    macro_rules! r {
        ($req:path $(,$args:expr)*) => {{
            client_request!($req, addr $(,$args)*);
        }};
    }

    match annotation {
        Annotation::HappensBefore => r!(CR::CG_ANNOTATE_HAPPENS_BEFORE),
        Annotation::HappensBeforeForgetAll => r!(CR::CG_ANNOTATE_HAPPENS_BEFORE_FORGET_ALL),
        Annotation::HappensAfter => r!(CR::CG_ANNOTATE_HAPPENS_AFTER),
        Annotation::New(len) => clean_memory(addr, len),
        Annotation::RwLockCreate => annotate_rwlock_create(addr),
        Annotation::RwLockDestroy => annotate_rwlock_destroy(addr),
        Annotation::RwLockAcquired(writer) => annotate_rwlock_acquired(addr, writer),
        Annotation::RwLockReleased(writer) => annotate_rwlock_released(addr, writer),
    }
}
