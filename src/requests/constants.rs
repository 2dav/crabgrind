pub mod dhat {
    // <valgrind/dhat.h>:
    // "... If no meaningful weight argument exists, just use 1."
    pub const AD_HOC_EVENT_DEFAULT_WEIGHT: usize = 1;
}

pub mod memcheck {
    // vg-docs/mc-manual.clientreqs:
    // "They return -1, when run on Valgrind and 0 otherwise."
    pub const MAKE_MEM_OK: usize = usize::MAX;

    // vg-docs/mc-manual.clientreqs:
    // "... returns zero if the relevant property holds;
    // ... Always returns 0 when not run on Valgrind."
    pub const CHECK_MEM_OK: usize = 0;

    // <valgrind/memcheck.h>:
    // "Returns 1 for an invalid handle, 0 for a valid handle."
    pub const DISCARD_MEM_OK: usize = 0;

    // <valgrind/memcheck.h>:
    //  0   if not running on valgrind
    //  1   success
    //  2   [previously indicated unaligned arrays;  these are now allowed]
    //  3   if any parts of zzsrc/zzvbits are not addressable.
    pub const VBITS_NO_VALGRIND: usize = 0;
    pub const VBITS_OK: usize = 1;
    pub const VBITS_LEGACY: usize = 2;
    pub const VBITS_UNADDRESSABLE: usize = 3;

    // <valgrind/memcheck.h>
    // VALGRIND_DO_LEAK_CHECK           (VG_USERREQ__DO_LEAK_CHECK, 0, 0, ...
    // VALGRIND_DO_ADDED_LEAK_CHECK     (VG_USERREQ__DO_LEAK_CHECK, 0, 1, ...
    // VALGRIND_DO_CHANGED_LEAK_CHECK   (VG_USERREQ__DO_LEAK_CHECK, 0, 2, ...
    // VALGRIND_DO_NEW_LEAK_CHECK       (VG_USERREQ__DO_LEAK_CHECK, 0, 3, ...
    // VALGRIND_DO_QUICK_LEAK_CHECK     (VG_USERREQ__DO_LEAK_CHECK, 1, 0, ...
    pub const LEAK_CHECK_FULL: (u8, u8) = (0, 0);
    pub const LEAK_CHECK_ADDED: (u8, u8) = (0, 1);
    pub const LEAK_CHECK_QUICK: (u8, u8) = (1, 0);
    pub const LEAK_CHECK_CHANGED: (u8, u8) = (0, 2);
    pub const LEAK_CHECK_NEW: (u8, u8) = (0, 3);
}

pub mod valgrind {
    // <valgrind/valgrind.h>:
    // ".. Returns 1 if command not recognized, 0 otherwise"
    pub const MONITOR_COMMAND_ERROR: usize = 1;

    // <valgrind/valgrind.h>:
    // " Behaves in a stack like
    //   way, so you can safely call this multiple times provided that
    //   VALGRIND_ENABLE_ERROR_REPORTING is called the same number of times
    //   to re-enable reporting.  The first call of this macro disables
    //   reporting.  Subsequent calls have no effect except to increase the
    //   number of VALGRIND_ENABLE_ERROR_REPORTING calls needed to re-enable
    //   reporting. "
    pub const ERROR_REPORTING_ENABLE: usize = usize::MAX; // -1
    pub const ERROR_REPORTING_DISABLE: usize = 1;

    // vg-docs
    pub const RUNNING_MODE_NATIVE: usize = 0;
    pub const RUNNING_MODE_VALGRIND: usize = 1;

    // <valgrind/valgrind.h>: VALGRIND_MAP_IP_TO_SRCLOC
    // "..If no info is found, the first byte is set to zero."
    #[inline(always)]
    pub fn is_empty_srcloc(buf: &[u8]) -> bool {
        buf[0] == 0
    }
}
