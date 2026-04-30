Returns the running tool name.

Wraps `VALGRIND_GET_TOOLNAME`.
Returns the running tool name, or [`None`](Option::None) if not running under
Valgrind.

# Arguments

- `buf` – A mutable 64-byte buffer. Valgrind writes the tool name string
    into this memory.

## Note

Requires Valgrind **3.27** or higher.
