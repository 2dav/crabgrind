{ pkgs ? import <nixpkgs> {} }:

let
  llvm = pkgs.llvmPackages;
in
pkgs.mkShell {
  packages = [
    llvm.clang-unwrapped
    llvm.llvm
    llvm.lld
    pkgs.binutils
  ];

  shellHook = ''
    export CLANG="${llvm.clang-unwrapped}/bin/clang"
  '';
}
