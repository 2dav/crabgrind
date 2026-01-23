{ pkgs ? import <nixpkgs> { } }:
let
  overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
in 
pkgs.callPackage ( { stdenv, mkShell }: mkShell {
  strictDeps = true;
  nativeBuildInputs = with pkgs; [
    rustup
    rustPlatform.bindgenHook
    python3

	pkg-config
	rust-analyzer
	cargo-cross
	cspell
	markdownlint-cli
	mdformat
	just

	valgrind
  ];

  buildInputs = with pkgs; [
	valgrind.dev
  ];

  RUSTC_VERSION = overrides.toolchain.channel;
  # VALGRIND_INCLUDE = "${pkgs.valgrind.dev}/include";

  shellHook = ''
	export RUSTUP_HOME=$HOME/.rustup
    export CARGO_HOME=$HOME/.cargo
	export PATH=$PATH:$CARGO_HOME/bin
    export PATH=$PATH:$RUSTUP_HOME/toolchains/$RUSTC_VERSION-${stdenv.hostPlatform.rust.rustcTarget}/bin

    # kinda fix for NixOS-specific ld-wrapper + Docker path resolution issues
	export CROSS_CONTAINER_OPTS="-v $RUSTUP_HOME:$RUSTUP_HOME:z,ro";
  '';
}){}
