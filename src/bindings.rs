#![allow(unused)]
#![allow(non_camel_case_types)]
#![allow(clippy::unreadable_literal)]

use core::ffi::{c_char, c_int};

extern "C" {
    pub fn vg_print(msg: *const c_char) -> c_int;
    pub fn vg_print_backtrace(msg: *const c_char) -> c_int;
    #[doc = include_str!("../doc/valgrind_client_request_expr.md")]
    pub fn valgrind_client_request_expr(
        zzq_default: usize,
        zzq_request: usize,
        zzq_arg1: usize,
        zzq_arg2: usize,
        zzq_arg3: usize,
        zzq_arg4: usize,
        zzq_arg5: usize,
    ) -> usize;
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
