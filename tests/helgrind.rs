use crabgrind::helgrind as hg;

use std::process::Output;

mod common;
use common::*;

#[test]
fn dirty_memory() {
    valgrind!(helgrind => {
        let addr = race_unsafe(|addr|
            unsafe { *addr += 1; }
        );
        print_addr(addr);
    }, |output: Output|{
        let stderr = as_str!(&output.stderr);
        let stdout = as_str!(&output.stdout);

        let var_addr = parse_addr(stdout);

        // HG will output data race reports 'Possible data race ... at 0x<addr>'
        assert!(stderr.contains(var_addr));
    });
}

#[test]
fn clean_memory() {
    valgrind!(helgrind => {
        let addr = race_unsafe(|addr|{
            hg::clean_memory(addr as _, 2);
            unsafe { *addr += 1 };
        });
        print_addr(addr);
    }, |output: Output|{
        let stderr = as_str!(&output.stderr);
        let stdout = as_str!(&output.stdout);

        let var_addr = parse_addr(stdout);

        assert!(!stderr.contains(var_addr));
    });
}

#[test]
fn annotate_happens() {
    valgrind!(helgrind => {
        hg::annotate_happens_after(std::ptr::null());
        hg::annotate_happens_before(std::ptr::null());
        hg::annotate_happens_before_forget_all(std::ptr::null());
    });
}

#[test]
fn annotate_rwlock() {
    valgrind!(helgrind => {
        let var = true;
        hg::annotate_rwlock_create(&var as *const _ as _);
        hg::annotate_rwlock_acquired(&var as *const _ as _, true);
        hg::annotate_rwlock_released(&var as *const _ as _, true);
        hg::annotate_rwlock_acquired(&var as *const _ as _, false);
        hg::annotate_rwlock_released(&var as *const _ as _, false);
        hg::annotate_rwlock_destroy(&var as *const _ as _);
    });
}
