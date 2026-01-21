Performs a complete leak scan.

Equivalent to the Valgrind command line option [--leak-check=full][leak-check].
Outputs all leaked blocks, complete with allocation stack traces.

Corresponds to `VALGRIND_DO_LEAK_CHECK`.

Requies Valgrind **3.0** or higher.

[leak-check]: https://valgrind.org/docs/manual/mc-manual.html#mc-manual.options
