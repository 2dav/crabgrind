use std::process::Output;

use crabgrind::cachegrind as cg;

mod common;
use common::*;

#[test]
fn start_instrumentation() {
    valgrind!(cachegrind --verbose => {
        // triggers warning 'instrumentation is already running'
        cg::start_instrumentation();
    }, |output: Output| {
        let stderr = as_str!(&output.stderr);
        assert!(stderr.contains("warning: CACHEGRIND_START_INSTRUMENTATION called"));
    });
}

#[test]
fn stop_instrumentation() {
    valgrind!(cachegrind --verbose => {
        cg::stop_instrumentation();
        // second call triggers warning 'instrumentation is already stopped'
        cg::stop_instrumentation();
    }, |output: Output| {
        let stderr = as_str!(&output.stderr);
        assert!(stderr.contains("warning: CACHEGRIND_STOP_INSTRUMENTATION called"));
    });
}
