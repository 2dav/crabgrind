// Tests that checks requests executes correctly in both cases:
// - native execution (not running under Valgrind)
// - compiled in stubs mode (--no-default-features)
//
use crabgrind::{
    self as cg, cachegrind as ch, callgrind as cl, dhat as dh, drd, helgrind as hg, memcheck as mc,
    valgrind as vg,
};

const NUL: *const std::ffi::c_void = std::ptr::null();

/*
 crabgrind/valgrind
*/

#[test]
fn running_mode() {
    assert!(vg::running_mode().is_native());
}

#[test]
fn toolname() {
    assert!(vg::toolname(&mut [0; 64]).is_none());
}

#[test]
fn replace_malloc() {
    assert_eq!(false, vg::replaces_malloc());
}

#[test]
fn monitor_command() {
    assert!(vg::monitor_command(c"invalid_command").is_ok());
    assert!(vg::monitor_command(c"leak_check").is_ok());
}

#[cfg(feature = "alloc")]
#[test]
fn println_macro() {
    cg::println!("cg_println_msg");
    cg::println!("cg_println_msg_{1}_{w:e}_{0:04}", 2, "format", w = 4);
    cg::print_stacktrace!("cg_print_stacktrace");
    cg::print_stacktrace!("cg_print_stacktrace_{1}_{w:e}_{0:04}", 2, "format", w = 4);
}

#[test]
fn vg_print() {
    cg::vg_print(c"vg_print_msg");
    cg::vg_print_stacktrace(c"vg_print_stacktrace");
}

#[test]
fn error_reporting() {
    assert_eq!(vg::count_errors(), 0);
    vg::disable_error_reporting();
    vg::enable_error_reporting();
}

#[test]
fn map_ip_to_srcloc() {
    assert!(vg::map_ip_to_srcloc(NUL, &mut [0; 64]).is_none());
}

#[test]
fn non_simd_call() {
    unsafe {
        let tid = vg::non_simd_call(|tid| tid);
        assert_eq!(tid, 0);
        let tid = vg::non_simd_call1(|tid, x| tid + x, 1);
        assert_eq!(tid, 0);
        let tid = vg::non_simd_call2(|tid, x, y| tid + x + y, 1, 1);
        assert_eq!(tid, 0);
        let tid = vg::non_simd_call3(|tid, x, y, z| tid + x + y + z, 1, 1, 1);
        assert_eq!(tid, 0);
    }
}

#[test]
fn vg_no_return() {
    vg::change_clo(c"--leak-check=summary");

    // *_block
    vg::malloclike_block(NUL, 128, 0, false);
    vg::malloclike_block(NUL, 128, 0, false);
    vg::freelike_block(NUL, 0);
    vg::resizeinplace_block(NUL, 1, 1, 0);

    // mempool
    vg::create_mempool(NUL, 0, false, None);
    vg::mempool_alloc(NUL, NUL, 1);
    vg::mempool_change(NUL, NUL, NUL, 1);
    vg::mempool_free(NUL, NUL);
    vg::mempool_trim(NUL, NUL, 1);
    vg::mempool_destroy(NUL);

    // stack
    assert_eq!(0, vg::stack_register(NUL, NUL));
    vg::stack_change(0, NUL, NUL);
    vg::stack_deregister(0);

    vg::load_pdb_debuginfo(0, NUL, 0, 0);

    vg::discard_translations(NUL, 0);
}

/*
 crabgrind/memcheck
*/

#[test]
fn leak_check() {
    mc::leak_check(mc::LeakCheck::Full);
    mc::leak_check(mc::LeakCheck::Added);
    mc::leak_check(mc::LeakCheck::Quick);
    mc::leak_check(mc::LeakCheck::Changed);
    mc::leak_check(mc::LeakCheck::New);

    assert_eq!(mc::count_leaks(), Default::default());
    assert_eq!(mc::count_leak_blocks(), Default::default());
}

#[test]
fn memory() {
    #[cfg(feature = "valgrind")]
    assert!(mc::mark_memory(NUL, 1, mc::MemState::Undefined).is_err());
    #[cfg(not(feature = "valgrind"))]
    assert!(mc::mark_memory(NUL, 1, mc::MemState::Undefined).is_ok());

    assert!(mc::check_mem_defined(NUL, 1).is_ok());
    assert!(mc::check_mem_addressable(NUL, 1).is_ok());
}

#[test]
fn create_discard_block() {
    assert_eq!(0, mc::create_block(NUL, 1, c""));
    assert_eq!(0, mc::create_block(NUL, 1, c""));
    assert!(mc::discard_block(0).is_ok());
}

#[test]
fn vbits() {
    let mut vbits = [0u8; 1];

    #[cfg(feature = "valgrind")]
    assert!(mc::vbits(NUL, &mut vbits).is_err());
    #[cfg(not(feature = "valgrind"))]
    assert!(mc::vbits(NUL, &mut vbits).is_ok());

    assert_eq!(vbits, [0x0]);

    #[cfg(feature = "valgrind")]
    assert!(mc::set_vbits(NUL, &vbits).is_err());
    #[cfg(not(feature = "valgrind"))]
    assert!(mc::set_vbits(NUL, &vbits).is_ok());

    assert_eq!(vbits, [0x0]);
}

#[test]
fn mc_no_return() {
    mc::disable_error_reporting(NUL, 1);
    mc::enable_error_reporting(NUL, 1);
}

/*
 crabgrind/cachegrind
*/

#[test]
fn cachegrind_instrumentation() {
    ch::start_instrumentation();
    ch::stop_instrumentation();
}

/*
 crabgrind/drd
*/

#[test]
fn thread_id() {
    assert_eq!(drd::drd_threadid(), 0);
    assert_eq!(drd::valgrind_threadid(), 0);
}

#[test]
fn drd_no_return() {
    drop(drd::ignore_var(&0));
    drop(drd::trace_var(&0));
    drop(drd::annotate_ignore_reads());
    drop(drd::annotate_ignore_writes());
}

/*
 crabgrind/callgrind
*/

#[test]
fn callgrind_no_return() {
    cl::dump_stats(None);
    cl::dump_stats(c"DUMP");
    cl::zero_stats();
    cl::toggle_collect();

    cl::start_instrumentation();
    cl::stop_instrumentation();
}

/*
 crabgrind/dhat
*/

#[test]
fn dhat_no_return() {
    dh::ad_hoc_event(100);
    dh::histogram_memory(NUL);
}

/*
 crabgrind/helgrind
*/

#[test]
fn helgrind_no_return() {
    hg::clean_memory(NUL, 1);

    hg::annotate_happens_after(NUL);
    hg::annotate_happens_before(NUL);
    hg::annotate_happens_before_forget_all(NUL);

    hg::annotate_rwlock_create(NUL);
    hg::annotate_rwlock_acquired(NUL, true);
    hg::annotate_rwlock_released(NUL, true);
    hg::annotate_rwlock_acquired(NUL, false);
    hg::annotate_rwlock_released(NUL, false);
    hg::annotate_rwlock_destroy(NUL);
}
