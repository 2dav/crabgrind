Check of memory range addressability

This performs an immediate check on the A-bits (addressability) for the
specified range. If any byte is not addressable, Valgrind emits an error report.

# Errors

- [`OffendingOffset`](OffendingOffset) - Contains the offset of the first byte
  that is not addressable.

## Note

Requires Valgrind **3.2** or higher.
