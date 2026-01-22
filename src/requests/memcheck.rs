#![doc = include_str!("../../doc/memcheck.md")]

use super::client_request;
use crate::{
    ScopeGuard,
    bindings::CG_MemcheckClientRequest as CR,
    requests::{Scope, sealed::Sealed},
};
use core::{
    ffi::{CStr, c_void},
    marker::PhantomData,
};

// vg-docs/mc-manual.clientreqs: "They return -1, when run on Valgrind and 0 otherwise."
const MAKE_MEM_OK: usize = usize::MAX;
// vg-docs/mc-manual.clientreqs: "... returns zero if the relevant property holds; ... Always returns 0 when not run on Valgrind."
const CHECK_MEM_OK: usize = 0;
// <valgrind/memchek.h>: "Returns 1 for an invalid handle, 0 for a valid handle."
const DISCARD_MEM_OK: usize = 0;
// <valgrind/memcheck.h>: "1   success"
const VBITS_OK: usize = 1;

/// Identifier for a custom memory block description.
///
/// Returned by [`create_block`] and used to remove the association with [`discard_block`]
pub type BlockHandle = usize;

/// A handle that was invalid or not found during a discard operation.
pub type InvalidBlockHandle = BlockHandle;

#[doc = include_str!("../../doc/memcheck/UnaddressableBytes.md")]
pub type UnaddressableBytes = usize;

#[doc = include_str!("../../doc/memcheck/OffendingOffset.md")]
pub type OffendingOffset = usize;

#[doc(hidden)]
#[derive(Debug)]
pub struct DisabledReporting<'a>(PhantomData<&'a ()>);

impl Scope for DisabledReporting<'_> {
    type Inner = (*const c_void, usize);

    #[inline(always)]
    fn enter((addr, size): Self::Inner) {
        disable_error_reporting(addr.cast(), size);
    }

    #[inline(always)]
    fn exit((addr, size): Self::Inner) {
        enable_error_reporting(addr.cast(), size);
    }
}

#[doc = include_str!("../../doc/memcheck/MemState.md")]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum MemState {
    #[doc = include_str!("../../doc/memcheck/memstate/noaccess.md")]
    NoAccess,
    #[doc = include_str!("../../doc/memcheck/memstate/undefined.md")]
    Undefined,
    #[doc = include_str!("../../doc/memcheck/memstate/defined.md")]
    Defined,
    #[doc = include_str!("../../doc/memcheck/memstate/defined_if_addressable.md")]
    DefinedIfAddressable,
}

#[doc = include_str!("../../doc/memcheck/LeakCheck.md")]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum LeakCheck {
    #[doc = include_str!("../../doc/memcheck/leakcheck/full.md")]
    #[default]
    Full,
    #[doc = include_str!("../../doc/memcheck/leakcheck/added.md")]
    Added,
    #[doc = include_str!("../../doc/memcheck/leakcheck/quick.md")]
    Quick,
    #[doc = include_str!("../../doc/memcheck/leakcheck/changed.md")]
    Changed,
    #[doc = include_str!("../../doc/memcheck/leakcheck/new.md")]
    New,
}

impl LeakCheck {
    /// Performs a leak check.
    ///
    /// This method is a combination of [`leak_check`] and [`count_leaks`]
    #[inline]
    pub fn check(self) -> LeaksCount {
        leak_check(self);
        count_leaks()
    }

    /// Performs a leak check.
    ///
    /// This method is a combination of [`leak_check`] and [`count_leak_blocks`]
    #[inline]
    pub fn check_blocks(self) -> LeaksCount {
        leak_check(self);
        count_leak_blocks()
    }
}

#[doc = include_str!("../../doc/memcheck/LeaksCount.md")]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct LeaksCount {
    /// Bytes that are definitely lost (no pointers to the start) or indirectly lost.
    ///
    /// This value represents the sum of direct and indirect leaks.
    pub leaked: usize,

    /// Bytes that are "possibly lost".
    ///
    /// Typically involves pointers to the middle of a heap block rather than the start,
    /// suggesting interior pointers that Memcheck cannot verify with 100% certainty.
    pub dubious: usize,

    /// Bytes that are still reachable.
    ///
    /// Pointers to the start of these blocks were found at program exit or during the check.
    pub reachable: usize,

    /// Bytes that were suppressed by a suppression file.
    ///
    /// These are leaks matching suppression rules specified in the Valgrind configuration.
    pub suppressed: usize,
}

/// V-bit (validity bit) manipulation errors.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum VBitsError {
    /// Not running under Valgrind
    NoValgrind,
    /// (Legacy) arrays not 4‑byte aligned or length not multiple of 4
    LegacyAlignment,
    /// Some of the memory is not addressable
    Unaddressable,
    /// Unknown VALGRIND_*_VBITS error code
    Unknown(u8),
}

#[doc = include_str!("../../doc/memcheck/mark_memory.md")]
#[inline(always)]
pub fn mark_memory(
    addr: *const c_void,
    size: usize,
    mark: MemState,
) -> Result<(), UnaddressableBytes> {
    macro_rules! r {
        ($req:path) => {
            client_request!($req, addr, size)
        };
    }

    let result = match mark {
        MemState::NoAccess => r!(CR::CG_VALGRIND_MAKE_MEM_NOACCESS),
        MemState::Undefined => r!(CR::CG_VALGRIND_MAKE_MEM_UNDEFINED),
        MemState::Defined => r!(CR::CG_VALGRIND_MAKE_MEM_DEFINED),
        MemState::DefinedIfAddressable => r!(CR::CG_VALGRIND_MAKE_MEM_DEFINED_IF_ADDRESSABLE),
    };

    (result == MAKE_MEM_OK).then_some(()).ok_or(result)
}

macro_rules! check_mem {
    ($req:path, $addr:expr, $size:expr) => {
        match client_request!($req, $addr, $size) {
            CHECK_MEM_OK => Ok(()),
            x => Err(x - $addr as usize),
        }
    };
}

#[doc = include_str!("../../doc/memcheck/check_mem_addressable.md")]
#[inline(always)]
pub fn check_mem_addressable(addr: *const c_void, size: usize) -> Result<(), OffendingOffset> {
    check_mem!(CR::CG_VALGRIND_CHECK_MEM_IS_ADDRESSABLE, addr, size)
}

#[doc = include_str!("../../doc/memcheck/check_mem_defined.md")]
#[inline(always)]
pub fn check_mem_defined(addr: *const c_void, size: usize) -> Result<(), OffendingOffset> {
    check_mem!(CR::CG_VALGRIND_CHECK_MEM_IS_DEFINED, addr, size)
}

#[doc = include_str!("../../doc/memcheck/leak_check.md")]
#[inline(always)]
pub fn leak_check(check: LeakCheck) {
    let (a1, a2): (u8, u8) = match check {
        LeakCheck::Full => (0, 0),
        LeakCheck::Added => (0, 1),
        LeakCheck::Quick => (1, 0),
        LeakCheck::Changed => (0, 2),
        LeakCheck::New => (0, 3),
    };

    client_request!(CR::CG_VALGRIND_DO_LEAK_CHECK, a1, a2);
}

#[doc = include_str!("../../doc/memcheck/count_leaks.md")]
#[inline(always)]
pub fn count_leaks() -> LeaksCount {
    let mut leaks = LeaksCount::default();

    client_request!(
        CR::CG_VALGRIND_COUNT_LEAKS,
        core::ptr::addr_of_mut!(leaks.leaked),
        core::ptr::addr_of_mut!(leaks.dubious),
        core::ptr::addr_of_mut!(leaks.reachable),
        core::ptr::addr_of_mut!(leaks.suppressed)
    );

    leaks
}

#[doc = include_str!("../../doc/memcheck/count_leak_blocks.md")]
#[inline(always)]
pub fn count_leak_blocks() -> LeaksCount {
    let mut leaks = LeaksCount::default();

    client_request!(
        CR::CG_VALGRIND_COUNT_LEAK_BLOCKS,
        core::ptr::addr_of_mut!(leaks.leaked),
        core::ptr::addr_of_mut!(leaks.dubious),
        core::ptr::addr_of_mut!(leaks.reachable),
        core::ptr::addr_of_mut!(leaks.suppressed)
    );

    leaks
}

macro_rules! vbits {
    ($req:path, $addr:expr, $slice:expr) => {
        match client_request!($req, $addr, $slice.as_ptr(), $slice.len()) {
            VBITS_OK => Ok(()),
            0 => Err(VBitsError::NoValgrind),
            2 => Err(VBitsError::LegacyAlignment),
            3 => Err(VBitsError::Unaddressable),
            x => Err(VBitsError::Unknown(u8::try_from(x).expect("Return code must fit `u8`"))),
        }
    };
}

#[doc = include_str!("../../doc/memcheck/vbits.md")]
#[inline(always)]
pub fn vbits(addr: *const c_void, dest: &mut [u8]) -> Result<(), VBitsError> {
    vbits!(CR::CG_VALGRIND_GET_VBITS, addr, dest)
}

#[doc = include_str!("../../doc/memcheck/set_vbits.md")]
#[inline(always)]
pub fn set_vbits(addr: *const c_void, vbits: &[u8]) -> Result<(), VBitsError> {
    vbits!(CR::CG_VALGRIND_SET_VBITS, addr, vbits)
}

#[doc = include_str!("../../doc/memcheck/create_block.md")]
#[inline(always)]
pub fn create_block(addr: *const c_void, size: usize, desc: impl AsRef<CStr>) -> BlockHandle {
    let desc = desc.as_ref().as_ptr();
    client_request!(CR::CG_VALGRIND_CREATE_BLOCK, addr, size, desc)
}

#[doc = include_str!("../../doc/memcheck/discard_block.md")]
#[inline(always)]
pub fn discard_block(handle: BlockHandle) -> Result<(), InvalidBlockHandle> {
    match client_request!(CR::CG_VALGRIND_DISCARD, handle) {
        DISCARD_MEM_OK => Ok(()),
        _ => Err(handle),
    }
}

#[doc = include_str!("../../doc/memcheck/disable_reporting.md")]
#[inline(always)]
pub fn disable_reporting(bytes: &[u8]) -> ScopeGuard<DisabledReporting<'_>> {
    ScopeGuard::new((bytes.as_ptr().cast(), bytes.len()))
}

#[doc = include_str!("../../doc/memcheck/enable_error_reporting.md")]
#[inline(always)]
pub fn enable_error_reporting(addr: *const c_void, size: usize) {
    client_request!(CR::CG_VALGRIND_ENABLE_ADDR_ERROR_REPORTING_IN_RANGE, addr, size);
}

#[doc = include_str!("../../doc/memcheck/disable_error_reporting.md")]
#[inline(always)]
pub fn disable_error_reporting(addr: *const c_void, size: usize) {
    client_request!(CR::CG_VALGRIND_DISABLE_ADDR_ERROR_REPORTING_IN_RANGE, addr, size);
}

impl core::fmt::Display for VBitsError {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoValgrind => write!(f, "not running under Valgrind"),
            Self::LegacyAlignment => write!(f, "legacy alignment issue"),
            Self::Unaddressable => write!(f, "memory not addressable"),
            Self::Unknown(x) => write!(f, "unknown VALGRIND_*_VBITS error code: {x}"),
        }
    }
}

impl core::error::Error for VBitsError {}

impl Sealed for DisabledReporting<'_> {}
