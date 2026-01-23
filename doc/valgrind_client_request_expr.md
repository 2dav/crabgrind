Direct mapping to the internal `VALGRIND_DO_CLIENT_REQUEST_EXPR` macro.

This is the raw mechanism behind the higher-level client request wrappers. It is
exposed publicly for testing purposes or deep exploration. If you are using
high-level abstractions, you do not need this.

# Safety

You really know what you are doing. Passing invalid request codes or malformed
arguments will crash the program or corrupt Valgrind's state. This function
performs no validation.

# Returns

The result of the client request, or `zzq_default` if Valgrind isn't present.

## Note

Requires Valgrind **3.0** or higher.
