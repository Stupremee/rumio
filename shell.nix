{ pkgs ? import <nixpkgs> { } }:
let
  rustChannel = pkgs.rust-bin.nightly.latest;

  rust = rustChannel.rust.override { extensions = [ "rust-src" ]; };
in pkgs.mkShell {
  name = "rust-shell";
  nativeBuildInputs = with pkgs; [ rust cargo-expand cargo-release valgrind ];
}
