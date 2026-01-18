use crabgrind::{memcheck as mc, valgrind as vg};

use std::{mem::MaybeUninit, process::Output};

mod common;
use common::*;

#[test]
fn leak_check_quick() {
    valgrind!(memcheck --leak-check=no => {
        leak::<10>();
        mc::leak_check(mc::LeakCheck::Quick);
    }, |output: Output|{
        let stderr = as_str!(&output.stderr);
        let definitely_lost = lost(&DEFINITELY_LOST_RE, stderr);
        assert_eq!(definitely_lost, 10);
    });
}

#[test]
fn leak_check_added() {
    valgrind!(memcheck --leak-check=no => {
        leak::<20>();
        mc::leak_check(mc::LeakCheck::Added);
    }, |output: Output|{
        let stderr = as_str!(&output.stderr);
        let definitely_lost_added = lost(&DEFINITELY_LOST_ADDED_RE, stderr);
        assert_eq!(definitely_lost_added, 20);
    });
}

#[test]
fn leak_check_new() {
    valgrind!(memcheck --leak-check=no => {
        leak::<30>();
        mc::leak_check(mc::LeakCheck::Quick);
        leak::<40>();
        mc::leak_check(mc::LeakCheck::New);
    }, |output: Output|{
        let stderr = as_str!(&output.stderr);
        let definitely_lost_new = lost(&DEFINITELY_LOST_ADDED_RE, stderr);
        assert_eq!(definitely_lost_new, 40);
    });
}

#[test]
fn leak_check_changed() {
    valgrind!(memcheck --leak-check=no => {
        leak::<50>();
        mc::leak_check(mc::LeakCheck::Quick);
        leak::<60>();
        mc::leak_check(mc::LeakCheck::Changed);
    }, |output: Output|{
        let stderr = as_str!(&output.stderr);
        let definitely_lost_changed = lost(&DEFINITELY_LOST_ADDED_RE, stderr);
        assert_eq!(definitely_lost_changed, 60);
    });
}

#[test]
fn count_leaks() {
    valgrind!(memcheck --leak-check=no => {
        mc::leak_check(mc::LeakCheck::Quick);
        let lc = mc::count_leaks();
        assert_eq!(lc.leaked, 0);
        assert_ne!(lc.reachable, 0);
        assert_ne!(lc.dubious, 0);

        leak::<10>();
        mc::leak_check(mc::LeakCheck::Quick);
        let lc = mc::count_leaks();
        assert_eq!(lc.leaked, 10);
    });
}

#[test]
fn count_leak_blocks() {
    valgrind!(memcheck --leak-check=no => {
        for _ in 0..13 {
            leak::<1>();
        }

        mc::leak_check(mc::LeakCheck::Quick);
        let lc = mc::count_leak_blocks();
        assert_eq!(lc.leaked, 13);
    });
}

#[test]
fn mark_memory_defined() {
    valgrind!(memcheck => {
        const N:usize = 5;
        let uninit = MaybeUninit::<[u8; N]>::uninit();
        let ptr = uninit.as_ptr() as _;

        mc::mark_memory(ptr, N, mc::MemState::Undefined).unwrap();
        assert_eq!(mc::check_mem_defined(ptr, N), Err(0));

        mc::mark_memory(ptr, 2, mc::MemState::Defined).unwrap();
        assert_eq!(mc::check_mem_defined(ptr, N), Err(2));

        mc::mark_memory(unsafe{ ptr.offset(2) }, N - 2, mc::MemState::DefinedIfAddressable).unwrap();
        assert!(mc::check_mem_defined(ptr, N).is_ok());
    });
}

#[test]
fn mark_memory_addressable() {
    valgrind!(memcheck => {
        const N:usize = 5;
        let uninit = MaybeUninit::<[u8; N]>::uninit();
        let ptr = uninit.as_ptr() as *const core::ffi::c_void;
        let ptr_offset2 = unsafe{ ptr.offset(2) };

        mc::mark_memory(ptr_offset2, N - 2, mc::MemState::NoAccess).unwrap();
        assert_eq!(mc::check_mem_addressable(ptr, N), Err(2));
        assert!(mc::check_mem_addressable(ptr, 2).is_ok());

        mc::mark_memory(ptr_offset2, N - 2, mc::MemState::Defined).unwrap();
        assert!(mc::check_mem_addressable(ptr, N).is_ok());
    });
}

#[test]
fn addr_error_reporting() {
    valgrind!(memcheck => {
        let mut heap = Box::new([0u8;10]);
        mc::mark_memory(heap.as_ptr() as _, 5, mc::MemState::NoAccess).unwrap();

        heap[5] = 1;
        assert_eq!(vg::count_errors(), 0);

        // store inside NO_ACCESS region
        heap[0] = 2;
        assert_eq!(vg::count_errors(), 1);

        // store inside NO_ACCESS region, error reporting disabled
        mc::disable_error_reporting(heap.as_ptr() as _, 2);
        heap[0] = 3;
        assert_eq!(vg::count_errors(), 1);

        // store inside NO_ACCESS region, error reporting re-enabled
        mc::enable_error_reporting(heap.as_ptr() as _, 2);
        heap[0] = 4;
        assert_eq!(vg::count_errors(), 2);
    });
}

#[test]
fn create_discard_block() {
    valgrind!(memcheck => {
        const N:usize = 5;
        let heap = Box::new([0u8; N]);
        let ptr = heap.as_ptr() as _;
        let desc = cstr!("b");

        let id = mc::create_block(ptr, N, desc);
        assert_eq!(id, 0);

        let id = mc::create_block(ptr, N, desc);
        assert_eq!(id, 1);

        assert!(mc::discard_block(0).is_ok());

        let id = mc::create_block(ptr, N, desc);
        assert_eq!(id, 0);

        let id = mc::create_block(ptr, N, desc);
        assert_eq!(id, 2);
    });
}

#[test]
fn vbits() {
    valgrind!(memcheck => {
        let data = [0u8; 4];
        let mut vbits = [0u8; 4];
        let ptr = data.as_ptr() as _;

        mc::mark_memory(ptr, 4, mc::MemState::Undefined).unwrap();
        mc::vbits(ptr, &mut vbits).unwrap();
        assert_eq!(vbits, [0xFF, 0xFF, 0xFF, 0xFF]);

        vbits.copy_from_slice(&[0xFF, 0x00, 0xAA, 0x55]);
        mc::set_vbits(ptr, &vbits).unwrap();
        assert!(mc::check_mem_defined(unsafe{ ptr.offset(1) }, 1).is_ok());

        vbits.fill(0);

        mc::mark_memory(ptr, 1, mc::MemState::NoAccess).unwrap();
        assert_eq!(mc::vbits(ptr, &mut vbits), Err(mc::VBitsError::Unaddressable));

        mc::mark_memory(ptr, 2, mc::MemState::Undefined).unwrap();
        mc::mark_memory(ptr, 1, mc::MemState::Defined).unwrap();

        mc::vbits(ptr, &mut vbits).unwrap();
        assert_eq!(vbits, [0x00, 0xFF, 0xAA, 0x55]);
    });
}

fn lost(re: &regex::Regex, stderr: &str) -> u32 {
    let lost = &re.captures(stderr).expect("nothing lost, definitely")[1];
    lost.parse().unwrap()
}
