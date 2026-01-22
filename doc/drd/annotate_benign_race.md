Marks a variable as "benign raced", suppressing DRD error reports.

Corresponds to the `ANNOTATE_BENIGN_RACE` client request.

# Mechanics
Instructs DRD to ignore data races detected on `addr`. This asserts that
concurrent accesses to this variable are expected and safe according to
application logic, even if they lack synchronization visible to DRD.

This is commonly used for:
* Lock-free algorithms utilizing atomic CPU instructions (outside of Rust's [`std::sync::atomic`][std-atomic]).
* Statistical counters where precision is not critical.
* "Magic" synchronization (e.g., signal handlers).

**Warning:** Use with extreme caution. Incorrectly marking a truly unsafe race
as benign will suppress valid bug reports.

See also [`annotate_benign_race_sized`] for raw byte ranges.

## Note
Requires Valgrind **3.5** or higher.

[std-atomic]: https://doc.rust-lang.org/std/sync/atomic/
