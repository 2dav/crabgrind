Maps a code address to a source file name and line number.

This function wraps the `VALGRIND_MAP_IP_TO_SRCLOC` macro. It queries the debug
information available to Valgrind for the specified instruction address and
writes the result into the provided buffer.

# Arguments

- `addr` – The instruction pointer address to resolve.
- `buf` – A mutable 64-byte buffer. Valgrind writes the resulting source
  location string directly into this memory. The result is always
  null-terminated.

# Rust Build Configuration

Rust's release profile strips debug info by default. To use this request in a
release build, debug info must be preserved in `Cargo.toml`:

```toml
[profile.release]
debug = "line-directives-only"
```

See [Cargo book: profiles.debug][profile.debug] for possible options.

## Note

Requires Valgrind **3.6** or higher.

[profile.debug]: https://doc.rust-lang.org/cargo/reference/profiles.html#debug
