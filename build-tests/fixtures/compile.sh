#!/usr/bin/env bash
set -euo pipefail

CLANG="${CLANG:-clang}"

for compiler in \
    "$CLANG" \
    "$GCC_I686" \
    "$GCC_X86_64" \
    "$GCC_AARCH64" \
    "$GCC_ARMV7" \
    "$GCC_POWERPC64" \
    "$GCC_RISCV64"
do
    if [[ ! -x "$compiler" ]]; then
        echo "ERROR: Required compiler not executable: $compiler" >&2
        exit 1
    fi

done

rm -rf object

compile_gcc() {
    local path="$1"
    local name="$2"
    local compiler="$3"

    echo "Building GCC $path/$name"

    mkdir -p "object/gcc/$path"

    "$compiler" \
        -ffreestanding \
        -c symbols.c \
        -o "object/gcc/$path/$name.o"
}

compile_clang() {
    local path="$1"
    local target="$2"
    local name="$3"

    echo "Building Clang $path/$name ($target)"

    mkdir -p "object/clang/$path"

    "$CLANG" \
        --target="$target" \
        -ffreestanding \
        -c symbols.c \
        -o "object/clang/$path/$name.o"
}

build_linux() {
    local target="$1"
    local name="$2"
    local gcc="${3:-}"

    if [[ -n "$gcc" ]]; then
        compile_gcc linux "$name" "$gcc"
    fi

    compile_clang linux "$target" "$name"
}

#
# Linux
#
build_linux i686-unknown-linux-gnu             i686       "$GCC_I686"
build_linux powerpc-unknown-linux-gnu          powerpc
build_linux x86_64-unknown-linux-gnu           x86_64     "$GCC_X86_64"
build_linux powerpc64-unknown-linux-gnu        powerpc64  "$GCC_POWERPC64"
build_linux aarch64-unknown-linux-gnu          aarch64    "$GCC_AARCH64"
build_linux powerpc64le-unknown-linux-gnu      ppc64le
build_linux s390x-unknown-linux-gnu            s390x
build_linux armv7-unknown-linux-gnueabihf      armv7      "$GCC_ARMV7"
build_linux mips-unknown-linux-gnu             mips
build_linux mips64-unknown-linux-gnuabi64      mips64
build_linux riscv64-unknown-linux-gnu          riscv64    "$GCC_RISCV64"

#
# FreeBSD: Clang only
#
compile_clang freebsd i686-unknown-freebsd11.3 i686
compile_clang freebsd x86_64-unknown-freebsd11.3 x86_64
compile_clang freebsd aarch64-unknown-freebsd14.0 aarch64

#
# Solaris
#
compile_clang solaris i686-pc-solaris2.11 i686
compile_clang solaris x86_64-pc-solaris2.11 x86_64

#
# Android
#
compile_clang android armv7a-linux-androideabi21 arm
compile_clang android aarch64-linux-android21 arm64
compile_clang android mipsel-linux-android21 mips
compile_clang android i686-linux-android21 i686

#
# Darwin
#
compile_clang darwin i386-apple-macosx10.8 i386
compile_clang darwin x86_64-apple-macosx10.8 x86_64
