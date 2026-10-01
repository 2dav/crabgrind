#![doc = include_str!("../../doc/memcheck.md")]
use super::{client_request, constants::memcheck::*};
use crate::{
    ScopeGuard,
    requests::{Scope, sealed::Sealed},
};
use core::{
    ffi::{CStr, c_void},
    marker::PhantomData,
    mem::{size_of, size_of_val},
    ops::Deref,
};

#[cfg(feature = "valgrind")]
use crate::bindings::CG_MemcheckClientRequest as CR;

/// Identifier for a custom memory block description.
///
/// Returned by [`create_block`] and used to remove the association with [`discard_block`]
pub type BlockHandle = usize;

/// A handle that was invalid or not found during a discard operation.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct InvalidBlockHandle(pub BlockHandle);

impl Deref for InvalidBlockHandle {
    type Target = BlockHandle;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Error indicating client-request was called when not running under Valgrind.
///
/// See [`mark_memory`](mark_memory)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NoValgrind;

#[doc = include_str!("../../doc/memcheck/OffendingOffset.md")]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct OffendingOffset(pub usize);

impl Deref for OffendingOffset {
    type Target = usize;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for OffendingOffset {
    #[inline(always)]
    fn from(value: usize) -> Self {
        Self(value)
    }
}

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

#[doc = include_str!("../../doc/memcheck/Memcheck.md")]
#[allow(clippy::missing_errors_doc)]
pub trait Memcheck {
    /// Manipulation of accessibility and validity state for a memory region
    ///
    /// This is the typed counterpart to [`mark_memory`].
    fn mark(&self, mark: MemState) -> Result<(), NoValgrind>;

    /// Check of memory range addressability
    ///
    /// This is the typed counterpart to [`check_mem_addressable`]. On error,
    /// [`OffendingOffset`] contains the index of the first offending `T` rather
    /// than the byte offset.
    fn check_addressable(&self) -> Result<(), OffendingOffset>;

    /// Check of memory range addressability and definedness
    ///
    /// This is the typed counterpart to [`check_mem_defined`]. On error,
    /// [`OffendingOffset`] contains the index of the first offending `T` rather
    /// than the byte offset.
    fn check_defined(&self) -> Result<(), OffendingOffset>;

    /// Retrieval of validity (V) bits for a memory range
    ///
    /// This is the typed counterpart to [`vbits`].
    fn vbits(&self, dest: &mut [u8]) -> Result<(), VBitsError>;

    /// Setting of validity (V) bits for a memory range
    ///
    /// This is the typed counterpart to [`set_vbits`].
    fn set_vbits(&self, vbits: &[u8]) -> Result<(), VBitsError>;

    /// Association of a custom name with a memory range
    ///
    /// This is the typed counterpart to [`create_block`].
    fn create_block<C>(&self, desc: &C) -> BlockHandle
    where
        C: AsRef<CStr> + ?Sized;

    /// Temporary disabling of error reporting for a memory range
    ///
    /// This is the typed counterpart to [`disable_reporting`].
    fn disable_reporting(&self) -> ScopeGuard<DisabledReporting<'_>>;
}

impl<T> Memcheck for [T] {
    #[inline(always)]
    fn mark(&self, mark: MemState) -> Result<(), NoValgrind> {
        mark_memory(self.as_ptr().cast(), size_of_val(self), mark)
    }

    #[inline(always)]
    fn check_addressable(&self) -> Result<(), OffendingOffset> {
        check_mem_addressable(self.as_ptr().cast(), size_of_val(self))
            .map_err(|e| e.checked_div(size_of::<T>()).unwrap_or(e.0).into())
    }

    #[inline(always)]
    fn check_defined(&self) -> Result<(), OffendingOffset> {
        check_mem_defined(self.as_ptr().cast(), size_of_val(self))
            .map_err(|e| e.checked_div(size_of::<T>()).unwrap_or(e.0).into())
    }

    #[inline(always)]
    fn vbits(&self, dest: &mut [u8]) -> Result<(), VBitsError> {
        vbits(self.as_ptr().cast(), dest)
    }

    #[inline(always)]
    fn set_vbits(&self, vbits: &[u8]) -> Result<(), VBitsError> {
        set_vbits(self.as_ptr().cast(), vbits)
    }

    #[inline(always)]
    fn create_block<C>(&self, desc: &C) -> BlockHandle
    where
        C: AsRef<CStr> + ?Sized,
    {
        create_block(self.as_ptr().cast(), size_of_val(self), desc)
    }

    #[inline(always)]
    fn disable_reporting(&self) -> ScopeGuard<DisabledReporting<'_>> {
        disable_reporting(self.as_ptr().cast(), size_of_val(self))
    }
}

#[doc = include_str!("../../doc/memcheck/mark_memory.md")]
#[allow(clippy::match_same_arms)]
#[inline(always)]
pub fn mark_memory(addr: *const c_void, size: usize, mark: MemState) -> Result<(), NoValgrind> {
    #[cfg(not(feature = "valgrind"))]
    return Ok(());

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

    if result == MAKE_MEM_OK { Ok(()) } else { Err(NoValgrind) }
}

macro_rules! check_mem {
    ($req:path, $addr:expr, $size:expr) => {
        match client_request!($req, $addr, $size) {
            CHECK_MEM_OK => Ok(()),
            x => Err(x.checked_sub($addr as usize).expect("Valgrind contract violation.").into()),
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
    let (a1, a2) = match check {
        LeakCheck::Full => LEAK_CHECK_FULL,
        LeakCheck::Added => LEAK_CHECK_ADDED,
        LeakCheck::Quick => LEAK_CHECK_QUICK,
        LeakCheck::Changed => LEAK_CHECK_CHANGED,
        LeakCheck::New => LEAK_CHECK_NEW,
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
            VBITS_NO_VALGRIND => Err(VBitsError::NoValgrind),
            VBITS_LEGACY => Err(VBitsError::LegacyAlignment),
            VBITS_UNADDRESSABLE => Err(VBitsError::Unaddressable),
            x => Err(VBitsError::Unknown(u8::try_from(x).expect("Return code must fit `u8`"))),
        }
    };
}

#[doc = include_str!("../../doc/memcheck/vbits.md")]
#[inline(always)]
pub fn vbits(addr: *const c_void, dest: &mut [u8]) -> Result<(), VBitsError> {
    #[cfg(not(feature = "valgrind"))]
    return Ok(());

    vbits!(CR::CG_VALGRIND_GET_VBITS, addr, dest)
}

#[doc = include_str!("../../doc/memcheck/set_vbits.md")]
#[inline(always)]
pub fn set_vbits(addr: *const c_void, vbits: &[u8]) -> Result<(), VBitsError> {
    #[cfg(not(feature = "valgrind"))]
    return Ok(());

    vbits!(CR::CG_VALGRIND_SET_VBITS, addr, vbits)
}

#[doc = include_str!("../../doc/memcheck/create_block.md")]
#[inline(always)]
pub fn create_block<C>(addr: *const c_void, size: usize, desc: &C) -> BlockHandle
where
    C: AsRef<CStr> + ?Sized,
{
    let desc = desc.as_ref().as_ptr();
    client_request!(CR::CG_VALGRIND_CREATE_BLOCK, addr, size, desc)
}

#[doc = include_str!("../../doc/memcheck/discard_block.md")]
#[inline(always)]
pub fn discard_block(handle: BlockHandle) -> Result<(), InvalidBlockHandle> {
    match client_request!(CR::CG_VALGRIND_DISCARD, handle) {
        DISCARD_MEM_OK => Ok(()),
        _ => Err(InvalidBlockHandle(handle)),
    }
}

#[doc = include_str!("../../doc/memcheck/disable_reporting.md")]
#[inline(always)]
pub fn disable_reporting<'a>(
    addr: *const c_void,
    size: usize,
) -> ScopeGuard<DisabledReporting<'a>> {
    ScopeGuard::new((addr, size))
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

impl core::fmt::Display for InvalidBlockHandle {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Invalid block handle:{}", self.0)
    }
}

impl core::fmt::Display for NoValgrind {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("No Valgrind")
    }
}

impl core::fmt::Display for OffendingOffset {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Offending offset {}", self.0)
    }
}

// `has_core_error` is set by build.rs
#[cfg(has_core_error)]
impl core::error::Error for VBitsError {}
#[cfg(has_core_error)]
impl core::error::Error for InvalidBlockHandle {}
#[cfg(has_core_error)]
impl core::error::Error for NoValgrind {}
#[cfg(has_core_error)]
impl core::error::Error for OffendingOffset {}

impl Sealed for DisabledReporting<'_> {}
