{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = [ 
	pkgs.valgrind.dev
	pkgs.python3
  ];
}
