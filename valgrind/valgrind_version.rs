impl CG_ValgrindClientRequest {
    #[allow(clippy::match_same_arms)]
    pub const fn required_version(self) -> u32 {
        match self {
			Self::CG_RUNNING_ON_VALGRIND => 300,
			Self::CG_VALGRIND_DISCARD_TRANSLATIONS => 300,
			Self::CG_VALGRIND_NON_SIMD_CALL0 => 300,
			Self::CG_VALGRIND_NON_SIMD_CALL1 => 300,
			Self::CG_VALGRIND_NON_SIMD_CALL2 => 300,
			Self::CG_VALGRIND_NON_SIMD_CALL3 => 300,
			Self::CG_VALGRIND_COUNT_ERRORS => 300,
			Self::CG_VALGRIND_MONITOR_COMMAND => 306,
			Self::CG_VALGRIND_CLO_CHANGE => 310,
			Self::CG_VALGRIND_MALLOCLIKE_BLOCK => 300,
			Self::CG_VALGRIND_RESIZEINPLACE_BLOCK => 300,
			Self::CG_VALGRIND_FREELIKE_BLOCK => 300,
			Self::CG_VALGRIND_CREATE_MEMPOOL => 300,
			Self::CG_VALGRIND_DESTROY_MEMPOOL => 300,
			Self::CG_VALGRIND_MEMPOOL_ALLOC => 300,
			Self::CG_VALGRIND_MEMPOOL_FREE => 300,
			Self::CG_VALGRIND_MEMPOOL_TRIM => 300,
			Self::CG_VALGRIND_MOVE_MEMPOOL => 300,
			Self::CG_VALGRIND_MEMPOOL_CHANGE => 300,
			Self::CG_VALGRIND_MEMPOOL_EXISTS => 300,
			Self::CG_VALGRIND_STACK_REGISTER => 300,
			Self::CG_VALGRIND_STACK_DEREGISTER => 300,
			Self::CG_VALGRIND_STACK_CHANGE => 300,
			Self::CG_VALGRIND_LOAD_PDB_DEBUGINFO => 306,
			Self::CG_VALGRIND_MAP_IP_TO_SRCLOC => 306,
			Self::CG_VALGRIND_ENABLE_ERROR_REPORTING => 300,
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
			Self::CG_VALGRIND_MAKE_MEM_NOACCESS => 300,
			Self::CG_VALGRIND_MAKE_MEM_UNDEFINED => 300,
			Self::CG_VALGRIND_MAKE_MEM_DEFINED => 300,
			Self::CG_VALGRIND_MAKE_MEM_DEFINED_IF_ADDRESSABLE => 306,
			Self::CG_VALGRIND_CHECK_MEM_IS_ADDRESSABLE => 302,
			Self::CG_VALGRIND_CHECK_MEM_IS_DEFINED => 302,
			Self::CG_VALGRIND_DO_LEAK_CHECK => 304,
			Self::CG_VALGRIND_COUNT_LEAKS => 300,
			Self::CG_VALGRIND_COUNT_LEAK_BLOCKS => 302,
			Self::CG_VALGRIND_GET_VBITS => 300,
			Self::CG_VALGRIND_SET_VBITS => 300,
			Self::CG_VALGRIND_CREATE_BLOCK => 302,
			Self::CG_VALGRIND_DISCARD => 302,
			Self::CG_VALGRIND_ENABLE_ADDR_ERROR_REPORTING_IN_RANGE => 304,
			Self::CG_VALGRIND_DISABLE_ADDR_ERROR_REPORTING_IN_RANGE => 304,
		}
	}
}
impl CG_HelgrindClientRequest {
    #[allow(clippy::match_same_arms)]
    pub const fn required_version(self) -> u32 {
        match self {
			Self::CG_VALGRIND_HG_CLEAN_MEMORY => 302,
			Self::CG_ANNOTATE_RWLOCK_CREATE => 306,
			Self::CG_ANNOTATE_RWLOCK_DESTROY => 306,
			Self::CG_ANNOTATE_RWLOCK_ACQUIRED => 306,
			Self::CG_ANNOTATE_RWLOCK_RELEASED => 306,
			Self::CG_ANNOTATE_HAPPENS_BEFORE => 307,
			Self::CG_ANNOTATE_HAPPENS_AFTER => 307,
			Self::CG_ANNOTATE_HAPPENS_BEFORE_FORGET_ALL => 307,
		}
	}
}
impl CG_DRDClientRequest {
    #[allow(clippy::match_same_arms)]
    pub const fn required_version(self) -> u32 {
        match self {
			Self::CG_DRD_GET_VALGRIND_THREADID => 303,
			Self::CG_DRD_GET_DRD_THREADID => 303,
			Self::CG_ANNOTATE_BENIGN_RACE_SIZED => 305,
			Self::CG_DRD_STOP_IGNORING_VAR => 305,
			Self::CG_DRD_TRACE_VAR => 303,
			Self::CG_DRD_STOP_TRACING_VAR => 303,
			Self::CG_ANNOTATE_IGNORE_READS_BEGIN => 305,
			Self::CG_ANNOTATE_IGNORE_WRITES_BEGIN => 305,
			Self::CG_ANNOTATE_NEW_MEMORY => 305,
			Self::CG_ANNOTATE_THREAD_NAME => 305,
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
