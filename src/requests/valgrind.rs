#![doc = include_str!("../../doc/valgrind.md")]
use super::{client_request, constants::valgrind::*};
use crate::{
    ScopeGuard,
    bindings::CG_ValgrindClientRequest as CR,
    requests::{Scope, sealed::Sealed},
};

use core::ffi::{CStr, c_int, c_void};

/// Designates the pool as a "meta-pool". See [`create_mempool`]
pub const VALGRIND_MEMPOOL_AUTO_FREE: u8 = 1;
/// Automatically free all second-level blocks. See [`create_mempool`]
pub const VALGRIND_MEMPOOL_METAPOOL: u8 = 2;

#[doc(hidden)]
#[derive(Debug)]
pub struct DisabledReporting;

impl Scope for DisabledReporting {
    type Inner = ();

    #[inline(always)]
    fn enter((): Self::Inner) {
        disable_error_reporting();
    }

    #[inline(always)]
    fn exit((): Self::Inner) {
        enable_error_reporting();
    }
}

/// Monitor Command error - command not recognized
pub type CommandNotFound = ();
/// File descriptor
pub type RawFd = c_int;
/// Valgrind/DRD Thread Identifier.
pub type ThreadId = usize;
/// Valgrind internal address range identifier returned by [`stack_register`]
pub type StackId = usize;

#[doc = include_str!("../../doc/valgrind/RunningMode.md")]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum RunningMode {
    /// Running on the host hardware without Valgrind.
    Native,
    /// Running under a single instance of Valgrind.
    Valgrind,
    /// Running under nested Valgrind instances.
    ValgrindOnValgrind(usize),
}

#[doc = include_str!("../../doc/valgrind/running_mode.md")]
#[inline(always)]
pub fn running_mode() -> RunningMode {
    match client_request!(CR::CG_RUNNING_ON_VALGRIND) {
        RUNNING_MODE_NATIVE => RunningMode::Native,
        RUNNING_MODE_VALGRIND => RunningMode::Valgrind,
        x => RunningMode::ValgrindOnValgrind(x),
    }
}

#[doc = include_str!("../../doc/valgrind/monitor_command.md")]
#[inline(always)]
pub fn monitor_command(cmd: impl AsRef<CStr>) -> Result<(), CommandNotFound> {
    match client_request!(CR::CG_VALGRIND_MONITOR_COMMAND, cmd.as_ref().as_ptr()) {
        MONITOR_COMMAND_ERROR => Err(()),
        _ => Ok(()),
    }
}

#[doc = include_str!("../../doc/valgrind/disable_reporting.md")]
#[inline(always)]
pub fn disable_reporting() -> ScopeGuard<DisabledReporting> {
    ScopeGuard::new(())
}

#[doc = include_str!("../../doc/valgrind/disable_error_reporting.md")]
#[inline(always)]
pub fn disable_error_reporting() {
    client_request!(CR::CG_VALGRIND_ENABLE_ERROR_REPORTING, ERROR_REPORTING_DISABLE);
}

#[doc = include_str!("../../doc/valgrind/enable_error_reporting.md")]
#[inline(always)]
pub fn enable_error_reporting() {
    client_request!(CR::CG_VALGRIND_ENABLE_ERROR_REPORTING, ERROR_REPORTING_ENABLE);
}

#[doc = include_str!("../../doc/valgrind/count_errors.md")]
#[inline(always)]
pub fn count_errors() -> usize {
    client_request!(CR::CG_VALGRIND_COUNT_ERRORS)
}

#[doc = include_str!("../../doc/valgrind/change_clo.md")]
#[inline(always)]
pub fn change_clo(option: impl AsRef<CStr>) {
    client_request!(CR::CG_VALGRIND_CLO_CHANGE, option.as_ref().as_ptr());
}

#[doc = include_str!("../../doc/valgrind/discard_translations.md")]
#[inline(always)]
pub fn discard_translations(addr: *const c_void, size: usize) {
    client_request!(CR::CG_VALGRIND_DISCARD_TRANSLATIONS, addr, size);
}

#[doc = include_str!("../../doc/valgrind/load_pdb_debuginfo.md")]
#[inline(always)]
pub fn load_pdb_debuginfo(fd: RawFd, ptr: *const c_void, total_size: usize, delta: usize) {
    client_request!(
        CR::CG_VALGRIND_LOAD_PDB_DEBUGINFO,
        usize::try_from(fd).expect("file descriptor should be >= 0"),
        ptr,
        total_size,
        delta
    );
}

#[doc = include_str!("../../doc/valgrind/map_ip_to_srcloc.md")]
#[inline(always)]
#[allow(clippy::needless_lifetimes)]
pub fn map_ip_to_srcloc<'a>(addr: *const c_void, buf: &'a mut [u8; 64]) -> Option<&'a CStr> {
    client_request!(CR::CG_VALGRIND_MAP_IP_TO_SRCLOC, addr, buf.as_mut_ptr());
    (!is_empty_srcloc(buf)).then(|| {
        // SAFETY: Request definition guarantees the resulting buffer is a null-terminated ascii string
        // limited to 64 bytes.
        unsafe { CStr::from_ptr(buf.as_mut_ptr().cast()) }
    })
}

#[doc = include_str!("../../doc/valgrind/non_simd_call.md")]
#[inline(always)]
pub fn non_simd_call(f: fn(ThreadId) -> usize) -> usize {
    client_request!(CR::CG_VALGRIND_NON_SIMD_CALL0, f)
}

#[doc = include_str!("../../doc/valgrind/non_simd_call1.md")]
#[inline(always)]
pub fn non_simd_call1(f: fn(ThreadId, usize) -> usize, arg1: usize) -> usize {
    client_request!(CR::CG_VALGRIND_NON_SIMD_CALL1, f, arg1)
}

#[doc = include_str!("../../doc/valgrind/non_simd_call2.md")]
#[inline(always)]
pub fn non_simd_call2(f: fn(ThreadId, usize, usize) -> usize, arg1: usize, arg2: usize) -> usize {
    client_request!(CR::CG_VALGRIND_NON_SIMD_CALL2, f, arg1, arg2)
}

#[doc = include_str!("../../doc/valgrind/non_simd_call3.md")]
#[inline(always)]
pub fn non_simd_call3(
    f: fn(ThreadId, usize, usize, usize) -> usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> usize {
    client_request!(CR::CG_VALGRIND_NON_SIMD_CALL3, f, arg1, arg2, arg3)
}

#[doc = include_str!("../../doc/valgrind/malloclike_block.md")]
#[inline(always)]
pub fn malloclike_block(addr: *const c_void, size: usize, redzone: usize, is_zeroed: bool) {
    client_request!(CR::CG_VALGRIND_MALLOCLIKE_BLOCK, addr, size, redzone, is_zeroed);
}

#[doc = include_str!("../../doc/valgrind/resizeinplace_block.md")]
#[inline(always)]
pub fn resizeinplace_block(addr: *const c_void, old_size: usize, new_size: usize, redzone: usize) {
    client_request!(CR::CG_VALGRIND_RESIZEINPLACE_BLOCK, addr, old_size, new_size, redzone);
}

#[doc = include_str!("../../doc/valgrind/freelike_block.md")]
#[inline(always)]
pub fn freelike_block(addr: *const c_void, redzone: usize) {
    client_request!(CR::CG_VALGRIND_FREELIKE_BLOCK, addr, redzone);
}

#[doc = include_str!("../../doc/valgrind/create_mempool.md")]
#[inline(always)]
pub fn create_mempool(
    pool: *const c_void,
    redzone: usize,
    is_zeroed: bool,
    flags: impl Into<Option<u8>>,
) {
    client_request!(
        CR::CG_VALGRIND_CREATE_MEMPOOL,
        pool,
        redzone,
        is_zeroed,
        flags.into().unwrap_or_default()
    );
}

#[doc = include_str!("../../doc/valgrind/mempool_destroy.md")]
#[inline(always)]
pub fn mempool_destroy(pool: *const c_void) {
    client_request!(CR::CG_VALGRIND_DESTROY_MEMPOOL, pool);
}

#[doc = include_str!("../../doc/valgrind/mempool_alloc.md")]
#[inline(always)]
pub fn mempool_alloc(pool: *const c_void, addr: *const c_void, size: usize) {
    client_request!(CR::CG_VALGRIND_MEMPOOL_ALLOC, pool, addr, size);
}

#[doc = include_str!("../../doc/valgrind/mempool_free.md")]
#[inline(always)]
pub fn mempool_free(pool: *const c_void, addr: *const c_void) {
    client_request!(CR::CG_VALGRIND_MEMPOOL_FREE, pool, addr);
}

#[doc = include_str!("../../doc/valgrind/mempool_trim.md")]
#[inline(always)]
pub fn mempool_trim(pool: *const c_void, addr: *const c_void, size: usize) {
    client_request!(CR::CG_VALGRIND_MEMPOOL_TRIM, pool, addr, size);
}

#[doc = include_str!("../../doc/valgrind/move_mempool.md")]
#[inline(always)]
pub fn move_mempool(pool_a: *const c_void, pool_b: *const c_void) {
    client_request!(CR::CG_VALGRIND_MOVE_MEMPOOL, pool_a, pool_b);
}

#[doc = include_str!("../../doc/valgrind/mempool_change.md")]
#[inline(always)]
pub fn mempool_change(
    pool: *const c_void,
    addr_a: *const c_void,
    addr_b: *const c_void,
    size: usize,
) {
    client_request!(CR::CG_VALGRIND_MEMPOOL_CHANGE, pool, addr_a, addr_b, size);
}

#[doc = include_str!("../../doc/valgrind/mempool_exists.md")]
#[inline(always)]
pub fn mempool_exists(pool: *const c_void) -> bool {
    client_request!(CR::CG_VALGRIND_MEMPOOL_EXISTS, pool) > 0
}

#[doc = include_str!("../../doc/valgrind/stack_register.md")]
#[inline(always)]
pub fn stack_register(lowest: *const c_void, highest: *const c_void) -> StackId {
    client_request!(CR::CG_VALGRIND_STACK_REGISTER, lowest, highest)
}

#[doc = include_str!("../../doc/valgrind/stack_deregister.md")]
#[inline(always)]
pub fn stack_deregister(stack: StackId) {
    client_request!(CR::CG_VALGRIND_STACK_DEREGISTER, stack);
}

#[doc = include_str!("../../doc/valgrind/stack_change.md")]
#[inline(always)]
pub fn stack_change(stack: StackId, new_lowest: *const c_void, new_highest: *const c_void) {
    client_request!(CR::CG_VALGRIND_STACK_CHANGE, stack, new_lowest, new_highest);
}

impl Sealed for DisabledReporting {}
