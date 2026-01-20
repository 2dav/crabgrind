#![doc = include_str!("../../doc/helgrind.md")]
use super::client_request;
use crate::bindings::CG_HelgrindClientRequest as CR;

use core::ffi::c_void;

#[doc = include_str!("../../doc/helgrind/clean_memory.md")]
#[inline(always)]
pub fn clean_memory(start: *const c_void, size: usize) {
    client_request!(CR::CG_VALGRIND_HG_CLEAN_MEMORY, start, size);
}

/// Resets Helgrind's tracking state for a reference.
///
/// See [`clean_memory`]
#[inline(always)]
pub fn clean_ref<T>(obj: &T) {
    clean_memory((obj as *const T).cast(), core::mem::size_of::<T>());
}

#[doc = include_str!("../../doc/helgrind/annotate_rwlock_create.md")]
#[inline(always)]
pub fn annotate_rwlock_create(addr: *const c_void) {
    client_request!(CR::CG_ANNOTATE_RWLOCK_CREATE, addr);
}

#[doc = include_str!("../../doc/helgrind/annotate_rwlock_destroy.md")]
#[inline(always)]
pub fn annotate_rwlock_destroy(addr: *const c_void) {
    client_request!(CR::CG_ANNOTATE_RWLOCK_DESTROY, addr);
}

#[doc = include_str!("../../doc/helgrind/annotate_rwlock_acquired.md")]
#[inline(always)]
pub fn annotate_rwlock_acquired(addr: *const c_void, writer_lock: bool) {
    client_request!(CR::CG_ANNOTATE_RWLOCK_ACQUIRED, addr, writer_lock);
}

#[doc = include_str!("../../doc/helgrind/annotate_rwlock_released.md")]
#[inline(always)]
pub fn annotate_rwlock_released(addr: *const c_void, writer_lock: bool) {
    client_request!(CR::CG_ANNOTATE_RWLOCK_RELEASED, addr, writer_lock);
}

#[doc = include_str!("../../doc/helgrind/annotate_happens_before.md")]
#[inline(always)]
pub fn annotate_happens_before(addr: *const c_void) {
    client_request!(CR::CG_ANNOTATE_HAPPENS_BEFORE, addr);
}

#[doc = include_str!("../../doc/helgrind/annotate_happens_after.md")]
#[inline(always)]
pub fn annotate_happens_after(addr: *const c_void) {
    client_request!(CR::CG_ANNOTATE_HAPPENS_AFTER, addr);
}

#[doc = include_str!("../../doc/helgrind/annotate_happens_before_forget_all.md")]
#[inline(always)]
pub fn annotate_happens_before_forget_all(addr: *const c_void) {
    client_request!(CR::CG_ANNOTATE_HAPPENS_BEFORE_FORGET_ALL, addr);
}
