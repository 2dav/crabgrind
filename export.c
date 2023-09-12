#include <valgrind/callgrind.h>
#include <valgrind/cachegrind.h>
#include <valgrind/valgrind.h>
#include <valgrind/memcheck.h>
#include <valgrind/helgrind.h>
#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

size_t running_on_valgrind(){
	return RUNNING_ON_VALGRIND;
}

//
// VALGRIND
//
void vg_discard_translations(void* addr, size_t len){
	VALGRIND_DISCARD_TRANSLATIONS(addr, len);
}

size_t vg_count_errors(){
	return VALGRIND_COUNT_ERRORS;
}

void vg_malloclike_block(void* addr, size_t sizeB, size_t rzB, bool is_zeroed){
	VALGRIND_MALLOCLIKE_BLOCK(addr, sizeB, rzB, is_zeroed);
}

void vg_freelike_block(void* addr, size_t rzB){
	VALGRIND_FREELIKE_BLOCK(addr, rzB);
}

void vg_resizeinplace_block(void* addr, size_t oldSizeB, size_t newSizeB, size_t rzB){
	VALGRIND_RESIZEINPLACE_BLOCK(addr, oldSizeB, newSizeB, rzB);
}

void vg_create_mempool(void* pool, size_t rzB, bool is_zeroed){
	VALGRIND_CREATE_MEMPOOL(pool, rzB, is_zeroed);
}

void vg_create_mempool_ext(void* pool, size_t rzB, bool is_zeroed, int flags){
	VALGRIND_CREATE_MEMPOOL_EXT(pool, rzB, is_zeroed, flags);
}

void vg_destroy_mempool(void* pool){
	VALGRIND_DESTROY_MEMPOOL(pool);
}

void vg_mempool_alloc(void* pool, void* addr, size_t size){
	VALGRIND_MEMPOOL_ALLOC(pool, addr, size);
}

void vg_mempool_free(void* pool, void* addr){
	VALGRIND_MEMPOOL_FREE(pool, addr);
}

void vg_mempool_trim(void* pool, void* addr, size_t size){
	VALGRIND_MEMPOOL_TRIM(pool, addr, size);
}

void vg_move_mempool(void* poolA, void* poolB){
	VALGRIND_MOVE_MEMPOOL(poolA, poolB);
}

void vg_mempool_change(void* pool, void* addrA, void* addrB, size_t size){
	VALGRIND_MEMPOOL_CHANGE(pool, addrA, addrB, size);
}

bool vg_mempool_exists(void* pool){
	return (bool)VALGRIND_MEMPOOL_EXISTS(pool);
}

size_t vg_stack_register(void* start, void* end){
	return VALGRIND_STACK_REGISTER(start, end);
}

void vg_stack_deregister(size_t id){
	VALGRIND_STACK_DEREGISTER(id);
}

void vg_stack_change(size_t id, void* start, void* end){
	VALGRIND_STACK_CHANGE(id, start, end);
}

void vg_load_pdb_debuginfo(int fd, void* ptr, size_t total_size, size_t delta){
	VALGRIND_LOAD_PDB_DEBUGINFO(fd, ptr, total_size, delta);
}

size_t vg_map_ip_to_srcloc(void* addr, void* buf64){
	return VALGRIND_MAP_IP_TO_SRCLOC(addr, buf64);
}

void vg_disable_error_reporting(){
	VALGRIND_DISABLE_ERROR_REPORTING;
}

void vg_enable_error_reporting(){
	VALGRIND_ENABLE_ERROR_REPORTING;
}

void vg_non_simd_call1(size_t (*fn)(size_t, void*), void* arg1){
	VALGRIND_NON_SIMD_CALL1(fn, arg1);
}

size_t vg_print(char* msg){
	return VALGRIND_PRINTF("%s", msg);
}

size_t vg_print_backtrace(char* msg){
	return VALGRIND_PRINTF_BACKTRACE("%s", msg);
}

bool vg_monitor_command(char* cmd){
	return (bool)VALGRIND_MONITOR_COMMAND(cmd);
}

void vg_clo_change(char* opt){
	VALGRIND_CLO_CHANGE(opt);
}

//
// CALLGRIND
//
void cl_dump_stats(){
	CALLGRIND_DUMP_STATS;
}

void cl_dump_stats_at(char* pos_str){
	CALLGRIND_DUMP_STATS_AT(pos_str);
}

void cl_zero_stats(){
	CALLGRIND_ZERO_STATS;
}

void cl_toggle_collect(){
	CALLGRIND_TOGGLE_COLLECT;
}

void cl_start_instrumentation(){
	CALLGRIND_START_INSTRUMENTATION;
}

void cl_stop_instrumentation(){
	CALLGRIND_STOP_INSTRUMENTATION;
}

//
// CACHEGRIND
//
void cg_start_instrumentation(){
    CACHEGRIND_START_INSTRUMENTATION;
}

void cg_stop_instrumentation(){
    CACHEGRIND_STOP_INSTRUMENTATION;
}

//
// MEMCHECK
//
int mc_make_mem_noaccess(void* addr, size_t len){
	return VALGRIND_MAKE_MEM_NOACCESS(addr, len);
}
int mc_make_mem_undefined(void* addr, size_t len){
	return VALGRIND_MAKE_MEM_UNDEFINED(addr, len);
}
int mc_make_mem_defined(void* addr, size_t len){
	return VALGRIND_MAKE_MEM_DEFINED(addr, len);
}
int mc_make_mem_defined_if_addressable(void* addr, size_t len){
	return VALGRIND_MAKE_MEM_DEFINED_IF_ADDRESSABLE(addr, len);
}
int mc_create_block(void* addr, size_t len, const char* desc){
	return VALGRIND_CREATE_BLOCK(addr, len, desc);
}
int mc_discard(int blkindex){
	return VALGRIND_DISCARD(blkindex);
}
size_t mc_check_mem_is_addressable(void* addr, size_t len){
	return VALGRIND_CHECK_MEM_IS_ADDRESSABLE(addr, len);
}
size_t mc_check_mem_is_defined(void* addr, size_t len){
	return VALGRIND_CHECK_MEM_IS_DEFINED(addr, len);
}
/*size_t mc_check_value_is_defined(void* addr){*/
/*	return VALGRIND_CHECK_VALUE_IS_DEFINED(addr);*/
/*}*/
void mc_do_leak_check(){
	VALGRIND_DO_LEAK_CHECK;
}
void mc_do_quick_leak_check(){
	VALGRIND_DO_QUICK_LEAK_CHECK;
}
void mc_do_added_leak_check(){
	VALGRIND_DO_ADDED_LEAK_CHECK;
}
void mc_do_changed_leak_check(){
	VALGRIND_DO_CHANGED_LEAK_CHECK;
}

typedef struct {
	size_t leaked, dubious, reachable, suppressed;
} leaks_count;

leaks_count mc_count_leaks(){
	leaks_count leaks;
	VALGRIND_COUNT_LEAKS(leaks.leaked, leaks.dubious, leaks.reachable, leaks.suppressed);
	return leaks;
}

leaks_count mc_count_leak_blocks(){
	leaks_count leaks;
	VALGRIND_COUNT_LEAK_BLOCKS(leaks.leaked, leaks.dubious, leaks.reachable, leaks.suppressed);
	return leaks;
}

int mc_get_vbits(void* addr, char* bits, size_t nbytes){
	return VALGRIND_GET_VBITS(addr, bits, nbytes);
}

int mc_set_vbits(void* addr, char* bits, size_t nbytes){
	return VALGRIND_SET_VBITS(addr, bits, nbytes);
}

void mc_disable_addr_error_reporting_in_range(void* addr, size_t len){
	VALGRIND_DISABLE_ADDR_ERROR_REPORTING_IN_RANGE(addr, len);
}

void mc_enable_addr_error_reporting_in_range(void* addr, size_t len){
	VALGRIND_ENABLE_ADDR_ERROR_REPORTING_IN_RANGE(addr, len);
}

//
// HELGRIND
//
void hg_clean_memory(void* addr, size_t len){
	VALGRIND_HG_CLEAN_MEMORY(addr, len);
}

void hg_annotate_happens_before(void* addr){
	ANNOTATE_HAPPENS_BEFORE(addr);
}

void hg_annotate_happens_after(void* addr){
	ANNOTATE_HAPPENS_AFTER(addr);
}

void hg_annotate_new_memory(void* addr, size_t size){
	ANNOTATE_NEW_MEMORY(addr, size);
}

void hg_rwlock_create(void* lock){
	ANNOTATE_RWLOCK_CREATE(lock);
}

void hg_rwlock_destroy(void* lock){
	ANNOTATE_RWLOCK_DESTROY(lock);
}

void hg_rwlock_acquired(void* lock, bool is_write){
	ANNOTATE_RWLOCK_ACQUIRED(lock, is_write);
}

void hg_rwlock_released(void* lock){
	ANNOTATE_RWLOCK_RELEASED(lock, 0);
}
