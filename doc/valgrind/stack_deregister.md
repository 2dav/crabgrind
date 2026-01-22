Deregisters a previously registered stack.

Wraps `VALGRIND_STACK_DEREGISTER`. This informs Valgrind that the memory
range associated with `stack` is no longer a stack.

# Arguments
* `stack` – The [`StackId`] returned by [`stack_register`].

# Reliability Warning
This client request is unreliable and best avoided if possible.

## Note
Requires Valgrind **3.0** or higher.
