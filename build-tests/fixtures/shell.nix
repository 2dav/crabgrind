
{ pkgs ? import <nixpkgs> {} }:

let
  cross = pkgs.pkgsCross;

  compilers = {
    i686      = cross.gnu32.stdenv.cc;
    x86_64    = cross.gnu64.stdenv.cc;
    aarch64   = cross.aarch64-multiplatform.stdenv.cc;
    armv7     = cross.armv7l-hf-multiplatform.stdenv.cc;
    powerpc64 = cross.ppc64.stdenv.cc;
    riscv64   = cross.riscv64.stdenv.cc;
  };

  gccPath = cc: "${cc}/bin/${cc.targetPrefix}gcc";
  llvm = pkgs.llvmPackages;

in
pkgs.mkShell {
  packages = [
    llvm.clang-unwrapped
    llvm.llvm
    llvm.lld
    pkgs.binutils
  ] ++ builtins.attrValues compilers;

  shellHook = ''
	export CLANG="${llvm.clang-unwrapped}/bin/clang"

    export GCC_I686="${gccPath compilers.i686}"
    export GCC_X86_64="${gccPath compilers.x86_64}"
    export GCC_AARCH64="${gccPath compilers.aarch64}"
    export GCC_ARMV7="${gccPath compilers.armv7}"
    export GCC_POWERPC64="${gccPath compilers.powerpc64}"
    export GCC_RISCV64="${gccPath compilers.riscv64}"

    echo "Clang + 6 cached GCC cross-compilers loaded."
  '';
}

