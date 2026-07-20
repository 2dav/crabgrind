#![cfg(not(feature = "valgrind"))]

mod common;
use std::ffi::c_void;

use common::*;
use crabgrind::{memcheck as mc, valgrind as vg};

#[test]
fn count_errors() {
    assert_eq!(vg::count_errors(), 0);
}

#[test]
fn running_mode_native() {
    assert_eq!(vg::running_mode(), vg::RunningMode::Native);
}

#[test]
fn toolname() {
    let mut buf = [0; 64];
    let toolname = vg::toolname(&mut buf);
    assert!(toolname.is_none());
}

#[test]
fn map_ip_to_srcloc() {
    let ip = vg::map_ip_to_srcloc as *const c_void;
    let mut buf = [0u8; 64];

    let loc = vg::map_ip_to_srcloc(ip, &mut buf);
    assert!(loc.is_none());
}

#[test]
fn monitor_command() {
    assert!(vg::monitor_command(cstr!("invalid_command")).is_ok());
}

#[test]
fn println_macro() {
    crabgrind::println!("cg_println_msg");
}

#[test]
fn count_leaks() {
    mc::leak_check(mc::LeakCheck::Quick);
    assert_eq!(mc::count_leaks(), Default::default());
    assert_eq!(mc::count_leak_blocks(), Default::default());
}

#[test]
fn mark_memory() {
    let res = mc::mark_memory(std::ptr::null(), 1, mc::MemState::Undefined);
    assert!(res.is_ok());
}

#[test]
fn vbits() {
    let data = [0u8; 4];
    let mut vbits = [0u8; 4];

    assert!(mc::vbits(data.as_ptr() as _, &mut vbits).is_ok());
}

#[test]
fn valgrind_available() {
    assert!(crabgrind::VALGRIND_AVAILABLE == false);
}
