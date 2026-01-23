Checks if a memory range is addressable.

This performs an immediate check on the A-bits (addressability) for the
specified range. If any byte is not addressable, Valgrind emits an error report.

# Errors

[`OffendingOffset`](OffendingOffset) Contains the location of the first byte
that is not addressable.

## Note

Requires Valgrind **3.2** or higher.
