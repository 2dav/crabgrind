#![doc = include_str!("../../doc/dhat.md")]
use super::client_request;
use crate::bindings::CG_DHATClientRequest as CR;
use core::ffi::c_void;

// dhat.h: "... If no meaningful weight argument exists, just use 1."
const DEFAULT_WEIGHT: usize = 1;

#[doc = include_str!("../../doc/dhat/ad_hoc_event.md")]
#[inline(always)]
pub fn ad_hoc_event(weight: impl Into<Option<usize>>) {
    client_request!(CR::CG_DHAT_AD_HOC_EVENT, weight.into().unwrap_or(DEFAULT_WEIGHT));
}

#[doc = include_str!("../../doc/dhat/histogram_memory.md")]
#[inline(always)]
pub fn histogram_memory(addr: *const c_void) {
    client_request!(CR::CG_DHAT_HISTOGRAM_MEMORY, addr);
}
