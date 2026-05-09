Direct mapping to the internal `VALGRIND_DO_CLIENT_REQUEST_EXPR` macro

This is the raw mechanism behind the higher-level client request wrappers. It is
exposed publicly for testing purposes or deep exploration. If you are using
high-level abstractions, you do not need this.

# Safety

This function is intended only for advanced use cases. Passing invalid request
codes or malformed arguments will crash the program or corrupt Valgrind's state.
No validation is performed.

# Returns

The result of the client request, or `zzq_default` if Valgrind isn't present.

## Note

Requires Valgrind **3.0** or higher.
