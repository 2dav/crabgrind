Defined only if they are already addressable.

Accessibility is not altered; bytes that are addressable become defined,
while non-addressable bytes remain unchanged.
Corresponds to `VALGRIND_MAKE_MEM_DEFINED_IF_ADDRESSABLE`.

Requies Valgrind **3.6** or higher.
