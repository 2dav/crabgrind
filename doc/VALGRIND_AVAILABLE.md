Whether Valgrind headers were available at compile time

This can be used to check for Valgrind support before making requests,
or in `const` contexts (e.g. `cfg`-like static assertions or
constant evaluation).
