Checks if a memory range is both addressable and defined.

This performs an immediate check on both A-bits (addressability) and V-bits
(definedness/initialized values) for the specified range. If any byte is not
addressable or contains undefined data, Valgrind emits an error report.

# Errors

[`OffendingOffset`](OffendingOffset) Contains the offset of the first byte that
is not defined.

## Note

Requires Valgrind **3.2** or higher.
