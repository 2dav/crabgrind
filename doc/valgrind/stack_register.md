Registration of a new stack memory range with Valgrind

Wraps `VALGRIND_STACK_REGISTER`. This informs Valgrind that the memory range
from `lowest` to `highest` functions as a unique stack. It returns a
[`StackId`](StackId) used to identify this range in subsequent calls.

## Note

Requires Valgrind **3.0** or higher.
