Discards Valgrind's translations for a specific address range.

This function wraps the `VALGRIND_DISCARD_TRANSLATIONS` macro. It forces
Valgrind to discard any existing cached translations for the specified memory
range.

# Behavior

After this call, execution jumps into the specified address range will trigger
Valgrind to generate fresh translations. This is required when debugging JIT
compilers or handling self-modifying code, as Valgrind otherwise continues
executing the previously translated (and now stale) instructions.

# Performance

Translation invalidation is expensive. Valgrind must locate all relevant
translations within its internal structures, which is a slow operation. Optimize
by writing new code to fresh memory or discarding large chunks of old code in a
single call rather than invalidating frequently.

# Alternatives

For transparent self-modifying-code support, run with `--smc-check=all` or
execute on ppc32/Linux, ppc64/Linux, or ARM/Linux.

## Note

Requires Valgrind **3.0** or higher.
