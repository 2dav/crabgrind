Retrieves validity (V) bits for a memory range.

Copies the V-bit metadata for the range `[addr, addr + len)` into `dest`.
This allows direct inspection of Memcheck's shadow memory state.

# Arguments
* `addr` - Pointer to the start of the memory range to inspect.
* `dest` - Buffer to receive the V-bit data. The length determines the number of bytes inspected.

# Behavior
Each byte of memory in the target range corresponds to 1 bit of validity data in `dest`.
This is a low-level introspection tool; arbitrary modification of shadow state
via [`set_vbits`] or [`mark_memory`] can lead to false negatives or positives in Memcheck.

# Errors
- [`VBitsError::NoValgrind`]
  The client request is not running on Valgrind.
  The function has no effect and returns this error.
- [`VBitsError::LegacyAlignment`]
  Legacy alignment constraint triggered.
  Historically, `addr` and `vbits` were required to be 4-byte aligned,
  and `len` was required to be a multiple of 4. Valgrind 3.8.0 and later
  relaxed this restriction, treating this case as success.
- [`VBitsError::Unaddressable`]
  A memory access failure occurred during the operation.
  This indicates that the source address range or the destination buffer
  (or parts thereof) were not addressable.
- [`VBitsError::Unknown`]
  An unknown error code was returned from Valgrind.
  Contains the raw error code returned by the client request.

## Note
Requires Valgrind **3.0** or higher.
