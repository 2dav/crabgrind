Manipulates the accessibility and validity state of a memory range.

This wrapper injects client requests directly into the execution stream to
modify Memcheck's shadow memory. It is useful for custom allocators or
optimizing instrumentation of complex memory operations.

# Errors

- [`UnaddressableBytes`](UnaddressableBytes) - containing the number of
  unaddressable bytes if `mark` is set to
  [`MemState::DefinedIfAddressable`](MemState::DefinedIfAddressable) and not all
  bytes in the range were addressable

## Note

Requires Valgrind **3.6** for
[`MemState::DefinedIfAddressable`](MemState::DefinedIfAddressable). All other
variants are supported in Valgrind **3.0** or higher.
