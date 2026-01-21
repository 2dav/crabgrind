Number of bytes that could not be modified during a memory marking operation.

Typically returned when a request specifies [`MemState::DefinedIfAddressable`] and part of the
range was not addressable.

See [`mark_memory`]
