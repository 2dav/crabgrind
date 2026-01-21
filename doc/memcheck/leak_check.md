Forces a memory leak check at the current execution point.

This request bypasses the normal end-of-process leak detection, allowing
for granular inspection of memory management in specific subsystems or transient states.

See also [`count_leaks`].

# Behavior
The results are emitted directly to the Valgrind output channel (stderr or xml).
Differential modes (`Added`, `Changed`, `New`) rely on state saved from the
immediately preceding leak check.

## Note
Minimum required Valgrind version:
* `Full`  - **3.0**
* `Quick` - **3.2** 
* `Added`, `Changed`, `New` - **3.4**
