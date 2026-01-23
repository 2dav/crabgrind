Updates the address range of a registered stack.

Wraps `VALGRIND_STACK_CHANGE`. Use this if your user-level thread package
implements stack growth, altering the bounds of the stack memory.

# Arguments

- `stack` – The [`StackId`](StackId) returned by
  [`stack_register`](stack_register).
- `new_lowest` – The new lowest addressable byte of the stack.
- `new_highest` – The new highest addressable byte of the stack.

# Reliability Warning

This client request is unreliable and best avoided if possible.

## Note

Requires Valgrind **3.0** or higher.
