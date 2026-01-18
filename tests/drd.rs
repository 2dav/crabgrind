use crabgrind::drd as dd;

use std::process::Output;

mod common;
use common::*;

#[test]
fn thread_id() {
    valgrind!(drd --verbose => {
        assert_eq!(dd::drd_threadid(), 2);
        assert_eq!(dd::valgrind_threadid(), 2);
    });
}

#[test]
fn ignore_var() {
    valgrind!(drd --read-var-info=yes --first-race-only=yes => {
        let var = contention(|addr| dd::ignore_var(unsafe{ &*addr }));
        print_addr(var);
    }, |output: Output|{
        let stderr = as_str!(&output.stderr);
        let stdout = as_str!(&output.stdout);

        let var_addr = parse_addr(stdout);

        assert!(!stderr.contains(var_addr));
    });
}

#[test]
fn trace_var() {
    valgrind!(drd => {
        let i:i32 = 0;
        let _guard = dd::trace_var(&i);
        unsafe { std::ptr::read_volatile(&i as *const _) };
    }, |output: Output| {
        let stderr = as_str!(&output.stderr);
        assert!(DRD_TRACE_RE.is_match(stderr));
    });
}

#[test]
fn trace_var_toggle() {
    valgrind!(drd => {
        let i:i32 = 0;
        let guard = dd::trace_var(&i);
        drop(guard);
        unsafe { std::ptr::read_volatile(&i as *const _) };
    }, |output: Output| {
        let stderr = as_str!(&output.stderr);
        assert!(!DRD_TRACE_RE.is_match(stderr));
    });
}

#[test]
fn annotate_reads_and_writes() {
    valgrind!(drd => {
        let mut i:i32 = 0;
        dd::annotate_ignore_read_and_writes(true);
        unsafe { std::ptr::read_volatile(&i as *const _) };
        unsafe { std::ptr::write_volatile(&mut i as *mut _, 2) };
    }, |output: Output| {
        let stderr = as_str!(&output.stderr);
        let num_errors = &ERROR_SUMMARY_RE.captures(stderr).unwrap()[1];
        let num_errors = num_errors.parse::<u16>().unwrap();
        assert!(num_errors < 2);
    });
}
