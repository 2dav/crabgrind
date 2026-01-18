#![doc = include_str!("../../doc/cachegrind.md")]
use super::client_request;
use crate::bindings::CG_CachegrindClientRequest as CR;

#[doc = include_str!("../../doc/cachegrind/start_instrumentation.md")]
#[inline(always)]
pub fn start_instrumentation() {
    client_request!(CR::CG_CACHEGRIND_START_INSTRUMENTATION);
}

#[doc = include_str!("../../doc/cachegrind/stop_instrumentation.md")]
#[inline(always)]
pub fn stop_instrumentation() {
    client_request!(CR::CG_CACHEGRIND_STOP_INSTRUMENTATION);
}
