Records a DHAT ad-hoc event.

Wraps `DHAT_AD_HOC_EVENT` to tell Valgrind something happened at this instruction pointer.

The `weight` argument is an arbitrary unit of measurement that you define:
* [`None`] (which defaults to a weight of `1`) treats all calls as equal occurrences.
* [`usize`] applies a specific weight, to mark certain events as 'more significant' than others.

DHAT aggregates these weights in the profile, showing them as the total "units" attributed 
to specific call stacks.

## Note
Requires Valgrind **3.15** or higher.

This function produces output only when running under Valgrind with DHAT
 configured for [ad-hoc profiling][ad-hoc-profiling]:
> ```text
> :~$ valgrind --tool=dhat --mode=ad-hoc ...
> ```

[ad-hoc-profiling]: https://valgrind.org/docs/manual/dh-manual.html#dh-manual.ad-hoc-profiling
