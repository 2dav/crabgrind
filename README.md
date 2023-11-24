Eliminating function call overhead using [linker-plugin-lto](https://doc.rust-lang.org/rustc/linker-plugin-lto.html) 
`rustc` flag. This requires fixing `C` compiler and linker to the ones from LLVM toolchain.

```bash
rustc -vV
clang -v
ld.lld -v
```
```
rustc 1.74.0 (79e9716c9 2023-11-13)
host: x86_64-unknown-linux-gnu
release: 1.74.0
LLVM version: 17.0.4

clang version 16.0.6

LLD 16.0.6
```

using this code as an example
```rust
fn main() {
    crabgrind::run_mode();
}
```
compiling with ~linker-plugin-lto branch gives the following assembly: 
```asm
mov     [rsp+var_38], 1001h
mov     [rsp+var_30], 0
mov     [rsp+var_28], 0
mov     [rsp+var_20], 0
mov     [rsp+var_18], 0
mov     [rsp+var_10], 0
lea     rax, [rsp+var_38]
xor     edx, edx
rol     rdi, 3
rol     rdi, 0Dh
rol     rdi, 3Dh
rol     rdi, 33h
xchg    rbx, rbx
mov     [rsp+var_40], rdx
mov     rax, [rsp+var_40]
retn
```
which is essentially the `valgrind/valgrind.h` `VALGRIND_DO_CLIENT_REQUEST_EXPR` macro instructions
filled with `VG_USERREQ__RUNNING_ON_VALGRIND`(0x1001) param. 
