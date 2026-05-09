Retrieval of the running tool name

Wraps `VALGRIND_GET_TOOLNAME`. Returns the running tool name if under Valgrind,
or [`None`](Option::None) otherwise.

# Arguments

- `buf` - A mutable 64-byte buffer. Valgrind writes the tool name string
  into this memory.

## Note

Requires Valgrind **3.27** or higher.
