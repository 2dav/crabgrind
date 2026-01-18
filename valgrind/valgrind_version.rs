impl CG_ValgrindClientRequest {
    #[allow(clippy::match_same_arms)]
    pub const fn required_version(self) -> u32 {
        match self {
			Self::CG_RUNNING_ON_VALGRIND => 0,
			Self::CG_VALGRIND_DISCARD_TRANSLATIONS => 0,
			Self::CG_VALGRIND_NON_SIMD_CALL0 => 0,
			Self::CG_VALGRIND_NON_SIMD_CALL1 => 0,
			Self::CG_VALGRIND_NON_SIMD_CALL2 => 0,
			Self::CG_VALGRIND_NON_SIMD_CALL3 => 0,
			Self::CG_VALGRIND_COUNT_ERRORS => 0,
			Self::CG_VALGRIND_MONITOR_COMMAND => 0,
			Self::CG_VALGRIND_CLO_CHANGE => 0,
			Self::CG_VALGRIND_MALLOCLIKE_BLOCK => 0,
			Self::CG_VALGRIND_RESIZEINPLACE_BLOCK => 0,
			Self::CG_VALGRIND_FREELIKE_BLOCK => 0,
			Self::CG_VALGRIND_CREATE_MEMPOOL => 0,
			Self::CG_VALGRIND_DESTROY_MEMPOOL => 0,
			Self::CG_VALGRIND_MEMPOOL_ALLOC => 0,
			Self::CG_VALGRIND_MEMPOOL_FREE => 0,
			Self::CG_VALGRIND_MEMPOOL_TRIM => 0,
			Self::CG_VALGRIND_MOVE_MEMPOOL => 0,
			Self::CG_VALGRIND_MEMPOOL_CHANGE => 0,
			Self::CG_VALGRIND_MEMPOOL_EXISTS => 0,
			Self::CG_VALGRIND_STACK_REGISTER => 0,
			Self::CG_VALGRIND_STACK_DEREGISTER => 0,
			Self::CG_VALGRIND_STACK_CHANGE => 0,
			Self::CG_VALGRIND_LOAD_PDB_DEBUGINFO => 0,
			Self::CG_VALGRIND_MAP_IP_TO_SRCLOC => 0,
			Self::CG_VALGRIND_ENABLE_ERROR_REPORTING => 0,
		}
	}
}
impl CG_CallgrindClientRequest {
    #[allow(clippy::match_same_arms)]
    pub const fn required_version(self) -> u32 {
        match self {
			Self::CG_CALLGRIND_DUMP_STATS => 307,
			Self::CG_CALLGRIND_DUMP_STATS_AT => 307,
			Self::CG_CALLGRIND_ZERO_STATS => 307,
			Self::CG_CALLGRIND_TOGGLE_COLLECT => 307,
			Self::CG_CALLGRIND_START_INSTRUMENTATION => 311,
			Self::CG_CALLGRIND_STOP_INSTRUMENTATION => 311,
		}
	}
}
impl CG_DHATClientRequest {
    #[allow(clippy::match_same_arms)]
    pub const fn required_version(self) -> u32 {
        match self {
			Self::CG_DHAT_AD_HOC_EVENT => 315,
			Self::CG_DHAT_HISTOGRAM_MEMORY => 315,
		}
	}
}
impl CG_MemcheckClientRequest {
    #[allow(clippy::match_same_arms)]
    pub const fn required_version(self) -> u32 {
        match self {
			Self::CG_VALGRIND_MAKE_MEM_NOACCESS => 0,
			Self::CG_VALGRIND_MAKE_MEM_UNDEFINED => 0,
			Self::CG_VALGRIND_MAKE_MEM_DEFINED => 0,
			Self::CG_VALGRIND_MAKE_MEM_DEFINED_IF_ADDRESSABLE => 0,
			Self::CG_VALGRIND_DISCARD => 0,
			Self::CG_VALGRIND_CHECK_MEM_IS_ADDRESSABLE => 0,
			Self::CG_VALGRIND_CHECK_MEM_IS_DEFINED => 0,
			Self::CG_VALGRIND_DO_LEAK_CHECK => 0,
			Self::CG_VALGRIND_COUNT_LEAKS => 0,
			Self::CG_VALGRIND_COUNT_LEAK_BLOCKS => 0,
			Self::CG_VALGRIND_GET_VBITS => 0,
			Self::CG_VALGRIND_SET_VBITS => 0,
			Self::CG_VALGRIND_CREATE_BLOCK => 0,
			Self::CG_VALGRIND_ENABLE_ADDR_ERROR_REPORTING_IN_RANGE => 0,
			Self::CG_VALGRIND_DISABLE_ADDR_ERROR_REPORTING_IN_RANGE => 0,
		}
	}
}
impl CG_HelgrindClientRequest {
    #[allow(clippy::match_same_arms)]
    pub const fn required_version(self) -> u32 {
        match self {
			Self::CG_VALGRIND_HG_CLEAN_MEMORY => 0,
			Self::CG_ANNOTATE_RWLOCK_CREATE => 0,
			Self::CG_ANNOTATE_RWLOCK_DESTROY => 0,
			Self::CG_ANNOTATE_RWLOCK_ACQUIRED => 0,
			Self::CG_ANNOTATE_RWLOCK_RELEASED => 0,
			Self::CG_ANNOTATE_HAPPENS_BEFORE => 0,
			Self::CG_ANNOTATE_HAPPENS_AFTER => 0,
			Self::CG_ANNOTATE_HAPPENS_BEFORE_FORGET_ALL => 0,
		}
	}
}
impl CG_DRDClientRequest {
    #[allow(clippy::match_same_arms)]
    pub const fn required_version(self) -> u32 {
        match self {
			Self::CG_ANNOTATE_BARRIER_INIT => 0,
			Self::CG_DRD_GET_VALGRIND_THREADID => 0,
			Self::CG_DRD_GET_DRD_THREADID => 0,
			Self::CG_ANNOTATE_BENIGN_RACE_SIZED => 0,
			Self::CG_DRD_STOP_IGNORING_VAR => 0,
			Self::CG_DRD_TRACE_VAR => 0,
			Self::CG_DRD_STOP_TRACING_VAR => 0,
			Self::CG_ANNOTATE_IGNORE_READS_BEGIN => 0,
			Self::CG_ANNOTATE_IGNORE_WRITES_BEGIN => 0,
			Self::CG_ANNOTATE_NEW_MEMORY => 0,
			Self::CG_ANNOTATE_THREAD_NAME => 0,
		}
	}
}
impl CG_CachegrindClientRequest {
    #[allow(clippy::match_same_arms)]
    pub const fn required_version(self) -> u32 {
        match self {
			Self::CG_CACHEGRIND_START_INSTRUMENTATION => 322,
			Self::CG_CACHEGRIND_STOP_INSTRUMENTATION => 322,
		}
	}
}
