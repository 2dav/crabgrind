Deregistration of a previously registered stack

Wraps `VALGRIND_STACK_DEREGISTER`. This informs Valgrind that the memory range
associated with `stack` is no longer a stack.

# Arguments

- `stack` - The [`StackId`](StackId) returned by
  [`stack_register`](stack_register).

## Note

Requires Valgrind **3.0** or higher.
