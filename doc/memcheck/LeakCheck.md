Configures the scope and output of an immediate leak search.

Valgrind tracks the allocation state between leak checks. Differential modes
(Added, Changed, New) compare the current heap state against the results of the
*previous* leak check invocation.

See [`leak_check`](leak_check).
