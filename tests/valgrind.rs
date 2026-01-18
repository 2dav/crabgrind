use crabgrind::{self as crab, valgrind as vg, valgrind::RunningMode};

use std::{ffi::c_void, process::Output};

mod common;
use common::*;

#[test]
fn running_mode_native() {
    assert_eq!(vg::running_mode(), RunningMode::Native);
}

#[test]
fn running_mode_valgrind() {
    valgrind!(memcheck => {
        assert_eq!(vg::running_mode(), RunningMode::Valgrind)
    });
}

#[test]
fn monitor_command() {
    valgrind!(memcheck --leak-check=no => {
        assert!(vg::monitor_command(cstr!("invalid_command")).is_err());
        assert!(vg::monitor_command(cstr!("leak_check")).is_ok());
    }, |output: Output| {
        let stderr = as_str!(&output.stderr);
        assert!(stderr.contains("LEAK SUMMARY:"));
    });
}

#[test]
fn println_macro() {
    valgrind!(memcheck => {
        crab::println!("cg_println_msg");
        crab::println!("cg_println_msg_{1}_{w:e}_{0:04}", 2, "format", w=4);
        crab::print_stacktrace!("cg_print_stacktrace");
        crab::print_stacktrace!("cg_print_stacktrace_{1}_{w:e}_{0:04}", 2, "format", w=4);
    }, |output: Output| {
        let stderr = as_str!(&output.stderr);
        assert!(stderr.contains("cg_println_msg"));
        assert!(stderr.contains("cg_println_msg_format_4e0_0002"));
        assert!(stderr.contains("cg_print_stacktrace"));
        assert!(stderr.contains("cg_print_stacktrace_format_4e0_0002"));
    });
}

#[test]
fn error_reporting() {
    valgrind!(memcheck => unsafe {
        let before = vg::count_errors();
        oob_read_heap();
        assert_eq!(vg::count_errors(), before + 1);
    });
}

#[test]
fn error_reporting_disable() {
    valgrind!(memcheck => vg::disable_error_reporting,
        |output: Output| {
            let stderr = as_str!(&output.stderr);
            assert!(stderr.contains("VALGRIND_DISABLE_ERROR_REPORTING"));
        }
    );
}

#[test]
fn error_reporting_toggle() {
    valgrind!(memcheck => unsafe {
        let before = vg::count_errors();

        vg::disable_error_reporting();

        oob_read_heap();
        assert_eq!(vg::count_errors(), before);

        vg::enable_error_reporting();

        oob_read_heap();
        assert_eq!(vg::count_errors(), before + 1);
    });
}

#[test]
fn change_clo() {
    valgrind!(memcheck --leak-check=no => {
        vg::change_clo(cstr!("--leak-check=summary"));
    }, |output: Output| {
        let stderr = as_str!(&output.stderr);
        assert!(stderr.contains("Handling new value --leak-check=summary"));
    });
}

#[test]
fn map_ip_to_srcloc() {
    let ip = vg::map_ip_to_srcloc as *const c_void;
    let mut buf = [0u8; 64];

    valgrind!(memcheck => {
        let loc = vg::map_ip_to_srcloc(ip, &mut buf);
        let loc = loc
            .expect("should return non-null loc info")
            .to_str()
            .expect("should be UTF-8 encoded bytes");

        let (fname, line) = loc.split_once(":").expect("<filename>:<line number>");

        assert!(fname.trim().ends_with("valgrind.rs"));
        assert!(line.parse::<u32>().is_ok());
    });
}

#[test]
fn non_simd_call() {
    valgrind!(memcheck => {
        const T: usize = 2;
        let tid = vg::non_simd_call(|tid| tid);
        assert_eq!(tid, T);
        let tid = vg::non_simd_call1(|tid, x| tid + x, 1);
        assert_eq!(tid, T + 1);
        let tid = vg::non_simd_call2(|tid, x, y| tid + x + y, 1, 1);
        assert_eq!(tid, T + 2);
        let tid = vg::non_simd_call3(|tid, x, y, z| tid + x + y + z, 1, 1, 1);
        assert_eq!(tid, T + 3);
    });
}

#[test]
fn malloclike_block() {
    valgrind!(memcheck --leak-check=full --track-origins=yes => {
        let ptr = mmap::<128>();

        vg::malloclike_block(ptr, 128, 0, false);

        unsafe { libc::munmap(ptr, 128) };
    }, |output: Output| {
        let stderr = as_str!(&output.stderr);
        assert!(stderr.contains("128 bytes in 1 blocks are definitely lost"));
    });
}

#[test]
fn freelike_block() {
    valgrind!(memcheck --leak-check=full --track-origins=yes => {
        let ptr = mmap::<128>();

        vg::malloclike_block(ptr, 128, 0, false);

        unsafe { libc::munmap(ptr, 128) };

        vg::freelike_block(ptr, 0);
    }, |output: Output| {
        let stderr = as_str!(&output.stderr);
        assert!(!stderr.contains("blocks are definitely lost"));
    });
}

#[test]
fn resizeinplace_block() {
    valgrind!(memcheck --leak-check=full --track-origins=yes => {
        const N:usize = 128;
        let ptr = mmap::<N>();

        vg::malloclike_block(ptr, N / 2, 1, false);

        let before = vg::count_errors();
        unsafe { ptr.add(N / 2).read_volatile() };
        let after = vg::count_errors();
        assert_eq!(after - before, 1);

        vg::resizeinplace_block(ptr, N / 2, N, 0);

        unsafe { ptr.add(N / 2).read_volatile() };
        assert_eq!(after, vg::count_errors());

        unsafe { libc::munmap(ptr, N) };
        vg::freelike_block(ptr, 0);
    });
}

#[test]
fn mempool() {
    valgrind!(memcheck --leak-check=full --track-origins=yes => {
        let before = vg::count_errors();
        let pool = vec![0u8;8];
        let pool = pool.as_ptr() as *const c_void;
        let a1 = unsafe{ pool.add(1) };
        let a2 = unsafe{ pool.add(2) };

        vg::create_mempool(pool, 0, false, None);
        assert!(vg::mempool_exists(pool));

        vg::mempool_alloc(pool, a1, 1);
        vg::mempool_alloc(pool, a2, 1);

        vg::mempool_change(pool, a1, a2, 1);

        vg::mempool_free(pool, a2);
        vg::mempool_trim(pool, a2, 1);
        vg::move_mempool(pool, pool);
        assert_eq!(vg::count_errors(), before);

        vg::mempool_destroy(pool);
        assert!(!vg::mempool_exists(pool));

        vg::mempool_free(pool, a1);
        vg::mempool_free(pool, a2);
        assert_eq!(vg::count_errors(), before + 2);
    });
}

#[test]
fn stack() {
    valgrind!(memcheck --leak-check=full --track-origins=yes => {
        const N: usize = 256;
        let stack_low = mmap::<N>();
        let stack_high = unsafe { stack_low.offset((N / 2) as _) };

        let id = vg::stack_register(stack_low, stack_high);
        assert_ne!(id, vg::stack_register(stack_low, stack_high));

        let stack_high = unsafe { stack_high.offset((N / 2) as _) };

        vg::stack_change(id, stack_low, stack_high);
        vg::stack_deregister(id);

        unsafe { libc::munmap(stack_low, N) };

        assert_eq!(vg::count_errors(), 0);
    });
}

/*
 * is there any way to test these:
 * VALGRIND_DISCARD_TRANSLATIONS
 * VALGRIND_LOAD_PDB_DEBUGINFO
*/

#[test]
fn load_pdb_debug_info() {
    // "no crash" test
    valgrind!(memcheck => { vg::load_pdb_debuginfo(0, std::ptr::null(), 0, 0); });
}

#[test]
fn discard_translations() {
    // "no crash" test
    valgrind!(memcheck => { vg::discard_translations(std::ptr::null(), 0); });
}
