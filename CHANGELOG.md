# Changelog

## [Unreleased]

### Added

- Convenient, typed wrappers for `memcheck` requests for primitive scalar types

## [0.4.0] - 2026-10-01

### Added

- Convenient, typed wrappers for `memcheck` requests that operate on memory
  ranges.
- Explicit `alloc` crate support via the `alloc` feature.
- Expose FFI printing functions wrappers, `vg_print` and `vg_print_stacktrace`,
  for printing directly to the Valgrind log in no-alloc environments.

### Changed

- Replaced type aliases for errors with dedicated error types.
- Marked the `non_simd_call_*` functions as unsafe and documented their safety
  contracts.
- Relaxed C-string argument types across the API to accept borrowed values.

### Fixed

- Hardened FFI handling to prevent invalid NUL-terminated strings from being
  interpreted unsafely.
- Various misleading claims and typos throughout documentation.
- Fixed potential UB when `println/println_stacktrace` macros are passed
  format strings containing interior NUL bytes.
- Added defensive checks around Valgrind FFI interactions to improve
  robustness against unexpected responses.

[unreleased]: https://github.com/2dav/crabgrind/compare/b9ad158...HEAD
[0.4.0]: https://github.com/2dav/crabgrind/compare/a8b35cb...b9ad158
