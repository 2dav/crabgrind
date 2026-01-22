Loads PDB debug information for a Wine PE image mapping.

This function wraps the `VALGRIND_LOAD_PDB_DEBUGINFO` client request. It
instructs Valgrind to read debug symbols from a Program Database (PDB) file
and associate them with the specified memory region. 

# Arguments
* `fd` – A valid file descriptor for the open PDB file.
* `ptr` – The base address of the PE image mapping.
* `total_size` – The total size of the mapped image.
* `delta` – The adjustment value to apply to symbol addresses.

# Panics
Panics if `fd` is less than 0.

## Note
Requires Valgrind **3.6** (Released 2010).
