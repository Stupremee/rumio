{ pkgs ? import <nixpkgs> { } }:
let
  rustChannel = pkgs.rust-bin.stable.latest;

  rust = rustChannel.rust.override { extensions = [ "rust-src" ]; };
in pkgs.mkShell {
  name = "rust-shell";
  nativeBuildInputs = with pkgs; [
    rust-analyzer
    rust
    cargo-expand
    cargo-release
  ];
}
