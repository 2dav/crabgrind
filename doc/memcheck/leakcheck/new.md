Performs a fast summary scan.

Equivalent to [--leak-check=summary][leak-check]. Returns only the total count of
leaked bytes and blocks; no detailed stack traces are generated.

Corresponds to `VALGRIND_DO_NEW_LEAK_CHECK`.

Requies Valgrind **3.4** or higher.

[leak-check]: https://valgrind.org/docs/manual/mc-manual.html#mc-manual.options
