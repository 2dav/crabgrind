# Object File Fixtures

This directory contains cross-compiled object-file fixtures used to test the correctness of constant and symbol extraction across supported object formats, architectures, bitnesses, and endianness.

These files are relocatable object files (.o) inspected directly by the test suite.

## Regenerating

```bash
nix-shell --run ./compile.sh
```
## Supported fixture matrix

### ELF / Linux

| Architecture | Target triple | Bits | Endianness |
|---|---|---:|---|
| x86 | i686-unknown-linux-gnu | 32 | Little |
| x86-64 | x86_64-unknown-linux-gnu | 64 | Little |
| ARMv7 | armv7-unknown-linux-gnueabihf | 32 | Little |
| AArch64 | aarch64-unknown-linux-gnu | 64 | Little |
| PowerPC | powerpc-unknown-linux-gnu | 32 | Big |
| PowerPC64 | powerpc64-unknown-linux-gnu | 64 | Big |
| PowerPC64LE | powerpc64le-unknown-linux-gnu | 64 | Little |
| S390X | s390x-unknown-linux-gnu | 64 | Big |
| MIPS32 | mips-unknown-linux-gnu | 32 | Big |
| MIPS64 | mips64-unknown-linux-gnuabi64 | 64 | Big |
| RISC-V 64 | riscv64-unknown-linux-gnu | 64 | Little |

### ELF / FreeBSD

| Architecture | Target triple | Bits | Endianness |
|---|---|---:|---|
| x86 | i686-unknown-freebsd11.3 | 32 | Little |
| x86-64 | x86_64-unknown-freebsd11.3 | 64 | Little |
| AArch64 | aarch64-unknown-freebsd14.0 | 64 | Little |

### ELF / Solaris

| Architecture | Target triple | Bits | Endianness |
|---|---|---:|---|
| x86 | i686-pc-solaris2.11 | 32 | Little |
| x86-64 | x86_64-pc-solaris2.11 | 64 | Little |

### ELF / Android

| Architecture | Target triple | Bits | Endianness |
|---|---|---:|---|
| ARMv7 | armv7a-linux-androideabi21 | 32 | Little |
| AArch64 | aarch64-linux-android21 | 64 | Little |
| MIPS32 | mipsel-linux-android21 | 32 | Little |
| x86 | i686-linux-android21 | 32 | Little |

### Mach-O / Darwin

| Architecture | Target triple | Bits | Endianness |
|---|---|---:|---|
| i386 | i386-apple-macosx10.8 | 32 | Little |
| x86-64 | x86_64-apple-macosx10.8 | 64 | Little |

## What is tested
| Symbol | Purpose |
|---|---|
| CG_TEST_ZERO_SIZE | Zero-sized global object symbol |
| CG_TEST_ZERO | Zero-valued constant |
| CG_TEST_MAX | Maximum test constant |
| CG_TEST_A, CG_TEST_B | Symbols in a shared data section |
| CG_TEST_PADDING | Explicit padding and symbol offsets |
| CG_TEST_SECTION_A, CG_TEST_SECTION_B | Symbols in separate custom sections |
| CG_TEST_ALIGN_16 | 16-byte alignment |
| CG_TEST_ALIGN_32 | 32-byte alignment |
| CG_TEST_HI, CG_TEST_LO, CG_TEST_MI | Constants at different offsets |
| CG_TEST_DUP | Duplicate-value handling |
