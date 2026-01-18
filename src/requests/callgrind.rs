#![doc = include_str!("../../doc/callgrind.md")]
use super::client_request;
use crate::bindings::CG_CallgrindClientRequest as CR;

use core::ffi::CStr;

#[doc = include_str!("../../doc/callgrind/dump_stats.md")]
#[inline(always)]
pub fn dump_stats<'a>(reason: impl Into<Option<&'a CStr>>) {
    if let Some(reason) = reason.into() {
        client_request!(CR::CG_CALLGRIND_DUMP_STATS_AT, reason.as_ptr());
    } else {
        client_request!(CR::CG_CALLGRIND_DUMP_STATS);
    }
}

#[doc = include_str!("../../doc/callgrind/zero_stats.md")]
#[inline(always)]
pub fn zero_stats() {
    client_request!(CR::CG_CALLGRIND_ZERO_STATS);
}

#[doc = include_str!("../../doc/callgrind/toggle_collect.md")]
#[inline(always)]
pub fn toggle_collect() {
    client_request!(CR::CG_CALLGRIND_TOGGLE_COLLECT);
}

#[doc = include_str!("../../doc/callgrind/start_instrumentation.md")]
#[inline(always)]
pub fn start_instrumentation() {
    client_request!(CR::CG_CALLGRIND_START_INSTRUMENTATION);
}

#[doc = include_str!("../../doc/callgrind/stop_instrumentation.md")]
#[inline(always)]
pub fn stop_instrumentation() {
    client_request!(CR::CG_CALLGRIND_STOP_INSTRUMENTATION);
}
