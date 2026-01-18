Forces [access count][access-counts] histograms for large memory blocks.

Standard DHAT limits histogram blocks to `1024` bytes to limit overhead.
This request overrides that limit, enabling tracking up to `25600` bytes (25KB).

Place this immediately after the allocator returns.

**Access Counts:** Start at zero. Initialization writes performed before this
call are ignored.

## Note
Requires Valgrind **3.15** (2019) or higher.

[access-counts]: https://valgrind.org/docs/manual/dh-manual.html#dh-access-counts
