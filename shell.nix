{ pkgs ? import <nixpkgs> { } }:
let
  overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
  latestValgrind = pkgs.valgrind.overrideAttrs (old: rec {
    version = "3.27.0";

    src = pkgs.fetchurl {
      url = "https://www.sourceware.org/pub/valgrind/valgrind-${version}.tar.bz2";
	  sha256 = "sha256-W1k33oJX7o9RaY6nG5cRrc6YBhqgfapKaF78OvkhW+8=";
      # sha256 = pkgs.lib.fakeSha256;
    };
  });
in 
pkgs.callPackage ( { stdenv, mkShell }: mkShell {
  strictDeps = true;
  nativeBuildInputs = with pkgs; [
	pkg-config
    rustup
    rustPlatform.bindgenHook
	rust-analyzer
    python3

	cargo-cross
	cspell
	markdownlint-cli
	just

	latestValgrind
  ];

  buildInputs = with pkgs; [
	latestValgrind.dev
  ];

  RUSTC_VERSION = overrides.toolchain.channel;
  # VALGRIND_INCLUDE = "${latestValgrind.dev}/include";

  shellHook = ''
	export CARGO_HOME=''${CARGO_HOME:-$HOME/.cargo}
	export RUSTUP_HOME=''${RUSTUP_HOME:-$HOME/.rustup}
	export PATH=$PATH:$CARGO_HOME/bin
    export PATH=$PATH:$RUSTUP_HOME/toolchains/$RUSTC_VERSION-${stdenv.hostPlatform.rust.rustcTarget}/bin

    # kinda fix for NixOS-specific ld-wrapper + Docker path resolution issues
	export CROSS_CONTAINER_OPTS="-v $RUSTUP_HOME:$RUSTUP_HOME:z,ro";
  '';
}){}
