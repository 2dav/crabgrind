Creates a memory pool definition for Valgrind's tracking tools.

Wraps `VALGRIND_CREATE_MEMPOOL`. This request registers a custom memory
pool allocator. Pools are useful for managing large numbers of fixed-size
blocks or complex allocation schemes.

# Arguments
* `pool` – The address of the pool identifier (usually a pointer to the
  pool structure).
* `redzone` – The size of the redzone (padding) to apply to allocated
  blocks within this pool.
* `is_zeroed` – Indicates whether blocks allocated from this pool are
  zeroed initially.
* `flags` – Optional flags to modify pool behavior.

# Flags
* `None` (or `0`) – Standard pool behavior.
* [`VALGRIND_MEMPOOL_METAPOOL`] – Designates the pool as a "meta-pool".
  This is required if you are carving out "superblocks" via
  [`mempool_alloc`] and then further sub-allocating them using
  [`malloclike_block`]. Without this flag, Valgrind detects overlapping
  blocks and aborts.
* [`VALGRIND_MEMPOOL_AUTO_FREE`] – Must be combined with [`VALGRIND_MEMPOOL_METAPOOL`]. When a
  block is freed via [`mempool_free`], this flag automatically frees all
  second-level blocks (allocated via [`malloclike_block`]) residing within
  that memory region.

## Note
Requires Valgrind **3.0** or higher.
