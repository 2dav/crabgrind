#!/usr/bin/env bash
set -euo pipefail

rm -rf object && mkdir -p object object/linux object/freebsd object/solaris object/darwin object/android

build() {
    local path="$1"
    local target="$2"
    local name="$3"

    echo "Building $name ($target)"

	"$CLANG" \
		--target="$target" \
		-ffreestanding \
		-c symbols.c \
		-o "object/${path}/${name}.o"
}

#
# Linux
#
build linux i686-unknown-linux-gnu			i686
build linux powerpc-unknown-linux-gnu		powerpc
build linux x86_64-unknown-linux-gnu		x86_64
build linux powerpc64-unknown-linux-gnu		powerpc64
build linux aarch64-unknown-linux-gnu		aarch64
build linux powerpc64le-unknown-linux-gnu	ppc64le
build linux s390x-unknown-linux-gnu			s390x							
build linux armv7-unknown-linux-gnueabihf	armv7							
build linux mips-unknown-linux-gnu			mips							
build linux mips64-unknown-linux-gnuabi64	mips64						
build linux riscv64-unknown-linux-gnu		riscv64						
#
# FreeBSD
#
build freebsd i686-unknown-freebsd11.3		i686
build freebsd x86_64-unknown-freebsd11.3	x86_64
build freebsd aarch64-unknown-freebsd14.0	aarch64
#
# Solaris
#
build solaris i686-pc-solaris2.11		i686 
build solaris x86_64-pc-solaris2.11		x86_64 
#
# Android
#
build android armv7a-linux-androideabi21	arm 
build android aarch64-linux-android21		arm64 
build android mipsel-linux-android21		mips 
build android i686-linux-android21			i686 
#
# Mach-O
#
build darwin i386-apple-macosx10.8		i386
build darwin x86_64-apple-macosx10.8	x86_64
