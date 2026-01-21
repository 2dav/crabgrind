Addressable but containing undefined data.

The bytes may be read or written, but their values are considered uninitialized.
Memcheck will report an error if these values influence program behavior.
Corresponds to `VALGRIND_MAKE_MEM_UNDEFINED`.

Requies Valgrind **3.0** or higher.
