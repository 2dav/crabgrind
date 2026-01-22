Executes a monitor command

This function wraps the `VALGRIND_MONITOR_COMMAND` macro. It sends the
provided string directly to the Valgrind core, which parses and executes it
as if it were entered into the Valgrind monitor (e.g., via `vgdb`).

# Output
If a GDB instance is connected to Valgrind via `vgdb`, the output is sent
according to the `vgdb` output mode. If no connection exists, the output
goes to the Valgrind log.

# Technical Note
Command syntax is verified at runtime. Where a specific client request
exists (e.g., [`memcheck::leak_check`](crate::memcheck::leak_check)), prefer that function over 
this generic wrapper to enforce compile-time argument checking.

# Errors
- [`CommandNotFound`] – The command was not recognized.

## Note
Requires Valgrind **3.6** (Released 2010).
