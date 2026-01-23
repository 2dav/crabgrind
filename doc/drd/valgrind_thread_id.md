Retrieves the Valgrind-assigned thread ID for the current thread context.

This corresponds to the `DRD_GET_VALGRIND_THREADID` client request. It queries
the unique identifier assigned by the Valgrind core to the thread executing this
request.

# Mechanics

Valgrind internally manages thread IDs distinct from OS thread IDs. These IDs
are **1-based** (the first thread is 1).

**Warning:** IDs are recycled. When a thread terminates, its ID may be reused
for a new thread.

## Note

Requires Valgrind **3.3** or higher.
