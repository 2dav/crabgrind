use crabgrind::callgrind as cg;

use std::process::Output;

mod common;
use common::*;

fn factorial(num: u128) -> u128 {
    (1..=num).product()
}

#[test]
fn dump_stats() {
    valgrind!(callgrind --verbose => {
        factorial(10);
        cg::dump_stats(None);
        factorial(10);
        cg::dump_stats(cstr!("DUMP"));
    }, |output: Output| {
        let stderr = as_str!(&output.stderr);
        assert!(stderr.contains("(Client Request)..."));
        assert!(stderr.contains("(Client Request: DUMP)..."));
    });
}

#[test]
fn zero_stats() {
    valgrind!(callgrind --verbose => {
        cg::zero_stats();
        cg::dump_stats(cstr!("HashMap::insert"));
    }, |output: Output| {
        let stderr = as_str!(&output.stderr);
        assert!(stderr.contains("Zeroing costs..."));
    });
}

#[test]
fn no_collect() {
    valgrind!(callgrind --verbose --collect-atstart=no => {
        factorial(10);
    }, |output: Output| {
        let stderr = as_str!(&output.stderr);
        assert!(stderr.contains("Collected : 0"))
    });
}

#[test]
fn toggle_collect() {
    valgrind!(callgrind --verbose --collect-atstart=no => {
        cg::toggle_collect();
        factorial(10);
    }, |output: Output| {
        let stderr = as_str!(&output.stderr);
        assert!(!stderr.contains("Collected : 0"))
    });
}

#[test]
fn instrumentation() {
    valgrind!(callgrind --verbose --instr-atstart=no=> {
        cg::start_instrumentation();
        cg::stop_instrumentation();
    }, |output: Output| {
        let stderr = as_str!(&output.stderr);
        assert!(stderr.contains("Client Request: instrumentation switched OFF"));
        assert!(stderr.contains("Client Request: instrumentation switched ON"));
    });
}
