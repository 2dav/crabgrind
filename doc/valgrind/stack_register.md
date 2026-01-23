Registers a new stack memory range with Valgrind.

Wraps `VALGRIND_STACK_REGISTER`. This informs Valgrind that the memory range
from `lowest` to `highest` functions as a unique stack. It returns a
[`StackId`](StackId) used to identify this range in subsequent calls.

# Reliability Warning

This client request is unreliable and best avoided if possible.

## Note

Requires Valgrind **3.0** or higher.
