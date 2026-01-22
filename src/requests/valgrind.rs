use super::client_request;
use crate::{
    ScopeGuard,
    bindings::CG_ValgrindClientRequest as CR,
    requests::{Scope, sealed::Sealed},
};

use core::ffi::{CStr, c_int, c_void};

// <valgrind/valgrind.h>: ".. Returns 1 if command not recognised, 0 otherwise"
const MONITOR_COMMAND_ERROR: usize = 1;
// <valgrind/valgrind.h>: VALGRIND_ENABLE_ERROR_REPORTING macro implementation
const ERROR_REPORTING_ENABLE: usize = usize::MAX; // -1
// <valgrind/valgrind.h>: VALGRIND_DISABLE_ERROR_REPORTING macro implementation
const ERROR_REPORTING_DISABLE: usize = 1;

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

pub type CommandNotFound = ();

pub type RawFd = c_int;
pub type ThreadId = usize;
pub type StackId = usize;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum RunningMode {
    Native,
    Valgrind,
    ValgrindOnValgrind(usize),
}

#[inline(always)]
pub fn running_mode() -> RunningMode {
    match client_request!(CR::CG_RUNNING_ON_VALGRIND) {
        0 => RunningMode::Native,
        1 => RunningMode::Valgrind,
        x => RunningMode::ValgrindOnValgrind(x),
    }
}

/// # Errors
#[inline(always)]
pub fn monitor_command(cmd: impl AsRef<CStr>) -> Result<(), CommandNotFound> {
    match client_request!(CR::CG_VALGRIND_MONITOR_COMMAND, cmd.as_ref().as_ptr()) {
        MONITOR_COMMAND_ERROR => Err(()),
        _ => Ok(()),
    }
}

#[inline(always)]
pub fn disable_reporting() -> ScopeGuard<DisabledReporting> {
    ScopeGuard::new(())
}

#[inline(always)]
pub fn disable_error_reporting() {
    client_request!(CR::CG_VALGRIND_ENABLE_ERROR_REPORTING, ERROR_REPORTING_DISABLE);
}

#[inline(always)]
pub fn enable_error_reporting() {
    client_request!(CR::CG_VALGRIND_ENABLE_ERROR_REPORTING, ERROR_REPORTING_ENABLE);
}

#[inline(always)]
pub fn count_errors() -> usize {
    client_request!(CR::CG_VALGRIND_COUNT_ERRORS)
}

#[inline(always)]
pub fn change_clo(option: impl AsRef<CStr>) {
    client_request!(CR::CG_VALGRIND_CLO_CHANGE, option.as_ref().as_ptr());
}

// TODO: "Throw away all translated (JITed) code for the given address range."
#[inline(always)]
pub fn discard_translations(addr: *const c_void, size: usize) {
    client_request!(CR::CG_VALGRIND_DISCARD_TRANSLATIONS, addr, size);
}

/// # Panics
/// if fd < 0
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

// Given an instruction pointer (IP / program counter), return source-level debug info (file name, function, line number) associated with that IP.
// also write about requirement of debug symbols, thus [profile.release] debug = "full or smth"
#[inline(always)]
#[allow(clippy::needless_lifetimes)]
pub fn map_ip_to_srcloc<'a>(addr: *const c_void, buf: &'a mut [u8; 64]) -> Option<&'a CStr> {
    client_request!(CR::CG_VALGRIND_MAP_IP_TO_SRCLOC, addr, buf.as_mut_ptr());

    // From <valgind/valgrind.h>: "..If no info is found, the first byte is set to zero."
    let is_empty = buf[0] == 0;

    (!is_empty).then(|| {
        // SAFETY: Request definition guarantees the resulting buffer is a null-terminated ascii string
        // limited to 64 bytes.
        unsafe { CStr::from_ptr(buf.as_mut_ptr().cast()) }
    })
}

#[inline(always)]
pub fn non_simd_call(f: fn(ThreadId) -> usize) -> usize {
    client_request!(CR::CG_VALGRIND_NON_SIMD_CALL0, f)
}

#[inline(always)]
pub fn non_simd_call1(f: fn(ThreadId, usize) -> usize, arg1: usize) -> usize {
    client_request!(CR::CG_VALGRIND_NON_SIMD_CALL1, f, arg1)
}

#[inline(always)]
pub fn non_simd_call2(f: fn(ThreadId, usize, usize) -> usize, arg1: usize, arg2: usize) -> usize {
    client_request!(CR::CG_VALGRIND_NON_SIMD_CALL2, f, arg1, arg2)
}

#[inline(always)]
pub fn non_simd_call3(
    f: fn(ThreadId, usize, usize, usize) -> usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> usize {
    client_request!(CR::CG_VALGRIND_NON_SIMD_CALL3, f, arg1, arg2, arg3)
}

#[inline(always)]
pub fn malloclike_block(addr: *const c_void, size: usize, redzone: usize, is_zeroed: bool) {
    client_request!(CR::CG_VALGRIND_MALLOCLIKE_BLOCK, addr, size, redzone, is_zeroed);
}

#[inline(always)]
pub fn resizeinplace_block(addr: *const c_void, old_size: usize, new_size: usize, redzone: usize) {
    client_request!(CR::CG_VALGRIND_RESIZEINPLACE_BLOCK, addr, old_size, new_size, redzone);
}

#[inline(always)]
pub fn freelike_block(addr: *const c_void, redzone: usize) {
    client_request!(CR::CG_VALGRIND_FREELIKE_BLOCK, addr, redzone);
}

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

#[inline(always)]
pub fn mempool_destroy(pool: *const c_void) {
    client_request!(CR::CG_VALGRIND_DESTROY_MEMPOOL, pool);
}

#[inline(always)]
pub fn mempool_alloc(pool: *const c_void, addr: *const c_void, size: usize) {
    client_request!(CR::CG_VALGRIND_MEMPOOL_ALLOC, pool, addr, size);
}

#[inline(always)]
pub fn mempool_free(pool: *const c_void, addr: *const c_void) {
    client_request!(CR::CG_VALGRIND_MEMPOOL_FREE, pool, addr);
}

#[inline(always)]
pub fn mempool_trim(pool: *const c_void, addr: *const c_void, size: usize) {
    client_request!(CR::CG_VALGRIND_MEMPOOL_TRIM, pool, addr, size);
}

#[inline(always)]
pub fn move_mempool(pool_a: *const c_void, pool_b: *const c_void) {
    client_request!(CR::CG_VALGRIND_MOVE_MEMPOOL, pool_a, pool_b);
}

#[inline(always)]
pub fn mempool_change(
    pool: *const c_void,
    addr_a: *const c_void,
    addr_b: *const c_void,
    size: usize,
) {
    client_request!(CR::CG_VALGRIND_MEMPOOL_CHANGE, pool, addr_a, addr_b, size);
}

#[inline(always)]
pub fn mempool_exists(pool: *const c_void) -> bool {
    client_request!(CR::CG_VALGRIND_MEMPOOL_EXISTS, pool) > 0
}

#[inline(always)]
pub fn stack_register(lowest: *const c_void, highest: *const c_void) -> StackId {
    client_request!(CR::CG_VALGRIND_STACK_REGISTER, lowest, highest)
}

#[inline(always)]
pub fn stack_deregister(stack: StackId) {
    client_request!(CR::CG_VALGRIND_STACK_DEREGISTER, stack);
}

#[inline(always)]
pub fn stack_change(stack: StackId, new_lowest: *const c_void, new_highest: *const c_void) {
    client_request!(CR::CG_VALGRIND_STACK_CHANGE, stack, new_lowest, new_highest);
}

impl Sealed for DisabledReporting {}
