use std::ffi::{c_char, c_void};

// valgrind
extern "C" {
    pub fn running_on_valgrind() -> usize;
    pub fn vg_disable_error_reporting();
    pub fn vg_enable_error_reporting();

    pub fn vg_count_errors() -> usize;

    pub fn vg_discard_translations(addr: *mut c_void, len: usize);

    pub fn vg_malloclike_block(addr: *mut c_void, size: usize, rz: usize, is_zeroed: bool);
    pub fn vg_freelike_block(addr: *mut c_void, rz: usize);
    pub fn vg_resizeinplace_block(addr: *mut c_void, old_size: usize, new_size: usize, rz: usize);

    pub fn vg_create_mempool(pool: *mut c_void, rz: usize, is_zeroed: bool);
    pub fn vg_create_mempool_ext(pool: *mut c_void, rz: usize, is_zeroed: bool, flags: u32);
    pub fn vg_destroy_mempool(pool: *mut c_void);
    pub fn vg_mempool_alloc(pool: *mut c_void, addr: *mut c_void, size: usize);
    pub fn vg_mempool_free(pool: *mut c_void, addr: *mut c_void);
    pub fn vg_mempool_trim(pool: *mut c_void, addr: *mut c_void, size: usize);
    pub fn vg_move_mempool(poolA: *mut c_void, poolB: *mut c_void);
    pub fn vg_mempool_change(
        pool: *mut c_void,
        addrA: *mut c_void,
        addrB: *mut c_void,
        size: usize,
    );
    pub fn vg_mempool_exists(pool: *mut c_void) -> bool;

    pub fn vg_stack_register(start: *mut c_void, end: *mut c_void) -> usize;
    pub fn vg_stack_deregister(id: usize);
    pub fn vg_stack_change(id: usize, start: *mut c_void, end: *mut c_void);

    pub fn vg_load_pdb_debuginfo(fd: i32, ptr: *mut c_void, total_size: usize, delta: usize);

    pub fn vg_map_ip_to_srcloc(addr: *mut c_void, buf64: *mut c_void) -> usize;

    pub fn vg_non_simd_call1(f: extern "C" fn(tid: usize, arg1: *mut c_void), arg1: *mut c_void);

    pub fn vg_print(msg: *const c_char) -> usize;
    pub fn vg_print_backtrace(msg: *const c_char) -> usize;

    pub fn vg_monitor_command(cmd: *const c_char) -> bool;

    pub fn vg_clo_change(opt: *const c_char);
}

// callgrind
extern "C" {
    pub fn cl_dump_stats();
    pub fn cl_dump_stats_at(pos_str: *const c_char);
    pub fn cl_zero_stats();

    pub fn cl_toggle_collect();

    pub fn cl_start_instrumentation();
    pub fn cl_stop_instrumentation();
}

// cachegrind
extern "C" {
    pub fn cg_start_instrumentation();
    pub fn cg_stop_instrumentation();
}

// memcheck

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LeakCount {
    pub leaked: usize,
    pub dubious: usize,
    pub reachable: usize,
    pub suppressed: usize,
}

extern "C" {
    pub fn mc_make_mem_noaccess(addr: *mut c_void, len: usize) -> i32;
    pub fn mc_make_mem_undefined(addr: *mut c_void, len: usize) -> i32;
    pub fn mc_make_mem_defined(addr: *mut c_void, len: usize) -> i32;
    pub fn mc_make_mem_defined_if_addressable(addr: *mut c_void, len: usize) -> i32;

    pub fn mc_create_block(addr: *mut c_void, len: usize, desc: *const c_char) -> u32;
    pub fn mc_discard(blkindex: u32) -> u32;
    pub fn mc_check_mem_is_addressable(addr: *mut c_void, len: usize) -> usize;
    pub fn mc_check_mem_is_defined(addr: *mut c_void, len: usize) -> usize;

    pub fn mc_do_leak_check();
    pub fn mc_do_quick_leak_check();
    pub fn mc_do_added_leak_check();
    pub fn mc_do_changed_leak_check();

    pub fn mc_count_leaks() -> LeakCount;
    pub fn mc_count_leak_blocks() -> LeakCount;

    pub fn mc_get_vbits(addr: *mut c_void, bits: *const u8, nbytes: usize) -> u32;
    pub fn mc_set_vbits(addr: *mut c_void, bits: *const u8, nbytes: usize) -> u32;
    pub fn mc_disable_addr_error_reporting_in_range(addr: *mut c_void, len: usize);
    pub fn mc_enable_addr_error_reporting_in_range(addr: *mut c_void, len: usize);
}

// helgrind
extern "C" {
    pub fn hg_clean_memory(addr: *mut c_void, len: usize);
    pub fn hg_annotate_happens_before(addr: *mut c_void);
    pub fn hg_annotate_happens_after(addr: *mut c_void);
    pub fn hg_annotate_new_memory(addr: *mut c_void, size: usize);
    pub fn hg_rwlock_create(lock: *mut c_void);
    pub fn hg_rwlock_destroy(lock: *mut c_void);
    pub fn hg_rwlock_acquired(lock: *mut c_void, is_w: bool);
    pub fn hg_rwlock_released(lock: *mut c_void);
}
