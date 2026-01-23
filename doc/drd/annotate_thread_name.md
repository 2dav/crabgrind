Assigns a human-readable name to the current thread.

Wraps the `ANNOTATE_THREAD_NAME` client request.

# Mechanics

DRD identifies threads primarily by numeric IDs. This request attaches a label
to the current thread context. The name appears in data race reports, trace
output, and error messages.

This operation affects only the thread executing the request.

## Note

Requires Valgrind **3.5** or higher.
