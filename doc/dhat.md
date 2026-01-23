# DHAT Client Requests

Interface to DHAT Client Requests defined in `valgrind/dhat.h`.

Refer to the [DHAT Valgrind documentation][vg-docs] for tool configuration and
profile interpretation.

## Usage

Nothing happens unless the binary runs under DHAT:

> ```text
> :~$ cargo build
> :~$ valgrind --tool=dhat target/debug/app
> ```

## Version Requirements

- **Minimum Valgrind:** `3.15`

[vg-docs]: https://valgrind.org/docs/manual/dh-manual.html
