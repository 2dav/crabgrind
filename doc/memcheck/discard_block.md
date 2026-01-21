Removes a previously registered memory block description.

Deletes the mapping between a memory range and its custom description string.
Subsequent errors in that region will revert to standard Memcheck reporting.

# Arguments
* `handle` - The [`BlockHandle`] returned by [`create_block`].

# Errors
* [`InvalidBlockHandle`] - provided handle was not valid.

## Note
Requires Valgrind **3.2** (Released 2005).
