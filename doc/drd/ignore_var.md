Activates data race suppression and returns a RAII guard.

Suppression is immediate. The returned guard manages the duration:
detection resumes automatically when the guard is dropped.

This corresponds to the `DRD_IGNORE_VAR` and `DRD_STOP_IGNORING_VAR`
client requests.

# Mechanics
The guard captures the address of `var`. DRD will not report data races
involving this address while the guard is alive. This is intended for scenarios 
involving intentional races (e.g., benign duplicate writes from multiple threads) where 
modifying synchronization logic is impractical.

The lifetime of the guard is tied to `&var`.

Acquiring the guard does **not** hold an active borrow. You may mutate `var` while the guard 
is active.

## Note
Requires Valgrind **3.5** (2009) or higher.
