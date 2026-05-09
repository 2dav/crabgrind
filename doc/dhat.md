# DHAT Client Requests

Interface to [DHAT Client Requests][vg-docs] defined in `valgrind/dhat.h`.

## Usage

Nothing happens unless the binary runs under DHAT:

> ```text
> :~$ cargo build
> :~$ valgrind --tool=dhat target/debug/app
> ```

## Version Requirements

- **Minimum Valgrind:** `3.15`

[vg-docs]: https://valgrind.org/docs/manual/dh-manual.html
