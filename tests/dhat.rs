use crabgrind::dhat as dh;
use crabgrind::valgrind::count_errors;

use std::process::Output;

mod common;
use common::*;

#[test]
fn ad_hoc_event() {
    valgrind!(dhat --mode=ad-hoc => {
        dh::ad_hoc_event(100);
        dh::ad_hoc_event(200);
    }, |output: Output| {
        let stderr = as_str!(&output.stderr);
        assert!(stderr.contains("300 units in 2 events"));
    });
}

#[test]
fn histogram_memory() {
    valgrind!(dhat --mode=heap => {
        let ptr = mmap::<2048>();
        dh::histogram_memory(ptr);
        assert_eq!(count_errors(), 0);
        unsafe { libc::munmap(ptr, 2048) };
    });
}
