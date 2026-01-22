Sets validity (V) bits for a memory range.

Copies V-bit metadata from `vbits` into Memcheck's shadow memory for the range
`[addr, addr + len)`. This overwrites the current validity state tracked by Memcheck.

# Arguments
* `addr` - Pointer to the start of the memory range to modify.
* `vbits` - Buffer containing the V-bit data. The length determines the number of bytes modified.

# Warning
Use with extreme caution. Incorrect V-bit data can silence legitimate errors
or cause Memcheck to report false positives. It is generally recommended to
only set V-bits that were previously retrieved via [`vbits`].

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
