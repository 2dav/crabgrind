Changes the value of a dynamically changeable command line option at runtime.

This function wraps the `VALGRIND_CLO_CHANGE` macro. It modifies the
specified option as if it were passed on the command line at startup.

See [Dynamically Change Options][dynopts] for list of changeable options.

# Behavior
Valgrind validates the option string at runtime. If the option is unknown,
does not support dynamic changes, or is syntactically incorrect, Valgrind
outputs a warning message to the log.

## Note
Requires Valgrind **3.10** or higher.

[dynopts]: https://valgrind.org/docs/manual/manual-core.html#manual-core.dynopts
