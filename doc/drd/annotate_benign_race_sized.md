Marks a raw memory range as "benign raced".

Corresponds to the `ANNOTATE_BENIGN_RACE_SIZED` client request.

# Mechanics
Identical to [`annotate_benign_race`], but operates on an arbitrary
memory range `[addr, addr + size)` rather than a typed reference.
DRD will suppress all race reports involving accesses within this range.

This is useful for suppressing races on slices, buffers, or memory
mapped regions where the type information is irrelevant or unavailable.

## Note
Requires Valgrind **3.5** (2009) or higher.
