Records a DHAT ad-hoc event.

Wraps `DHAT_AD_HOC_EVENT` to tell Valgrind something happened at this
instruction pointer.

The `weight` argument is an arbitrary unit of measurement that you define:

- [`None`](Option::None) (which defaults to weight of `1`) treats all calls as
  equal occurrences.
- [`usize`](usize) applies a specific weight, to mark certain events as 'more
  significant' than others.

DHAT aggregates these weights in the profile, showing them as the total "units"
attributed to specific call stacks.

## Example

Tracking Data Volumes.

[Ad-hoc mode][ad-hoc-profiling] generates a flame graph where execution points
are weighted by custom values rather than memory traffic. This can be used for
identifying code paths handling the majority of data volume.

```rust
use crabgrind::dhat;

// This function logs the length of the input slice. In the DHAT profile,
// this aggregates the total bytes processed per call-site.
fn process_chunk(data: &[u8]) {
    dhat::ad_hoc_event(data.len());

    // simulate processing...
    std::hint::black_box(data);
}

fn main() {
    // A: High frequency, small payloads.
    // 1000 packets of 64 bytes.
    for _ in 0..1_000 {
        process_chunk(&[0u8; 64]);
    }

    // B: Low frequency, massive payloads.
    // 10 packets of 1 MB.
    for _ in 0..10 {
        process_chunk(&[0u8; 1_000_000]);
    }
}
```

> Run with DHAT configured for [ad-hoc profiling][ad-hoc-profiling]
>
> ```text
> :~$ valgrind --tool=dhat --mode=ad-hoc target/debug/ad_hoc_event
> ```
>
> Resulting dhat profile can be quickly examined using the [dhat-viewer][dhat.viewer]
> utility, distributed with Valgrind.
>
> ```text
> ...
> Total: 10,064,000 units (100%, 17,612,264.18/Minstr) in 1,010 events
> Total: 10,000,000 units (99.4%, 17,500,262.5/Minstr) in 10 events
> Total:     64,000 units (0.64%, 112,001.68/Minstr) in 1,000 events
> ...
> ```

## Note

Requires Valgrind **3.15** or higher.

This function produces output only when running under Valgrind with DHAT
configured for [ad-hoc profiling][ad-hoc-profiling]:

> ```text
> :~$ valgrind --tool=dhat --mode=ad-hoc ...
> ```

[ad-hoc-profiling]: https://valgrind.org/docs/manual/dh-manual.html#dh-manual.ad-hoc-profiling
[dhat.viewer]: https://valgrind.org/docs/manual/dh-manual.html#dh-manual.viewer
