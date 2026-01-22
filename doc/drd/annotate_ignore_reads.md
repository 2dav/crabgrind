Toggles data race suppression for memory **reads** performed by the current thread.

Instructs DRD to stop reporting data races involving load operations performed by the current thread.
Suppression is immediate. The returned RAII guard manages the duration:
detection resumes automatically when the guard is dropped.

Wraps the `ANNOTATE_IGNORE_READS_BEGIN` and `ANNOTATE_IGNORE_READS_END` client
requests.

# Mechanics
This modifies the thread-local DRD state. It applies to **all** read
operations executed by the current thread, regardless of the memory address.
This is a coarse-grained suppression mechanism, distinct from [`ignore_var`]
which targets specific addresses.

Common use cases include benign races in lock-free algorithms or debugging
noisy output from known-safe racy reads.

## Note
Requires Valgrind **3.5** or higher.
