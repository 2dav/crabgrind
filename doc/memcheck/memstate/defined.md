Addressable and containing defined (initialized) data.

This tells Memcheck the bytes are safe to use without reporting uninitialized value errors.
Corresponds to `VALGRIND_MAKE_MEM_DEFINED`.

Requires Valgrind **3.0** or higher.
