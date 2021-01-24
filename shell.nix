{ pkgs ? import <nixpkgs> { } }:
let
  rustChannel = pkgs.rust-bin.nightly."2021-01-24";

  rust = rustChannel.rust.override { extensions = [ "rust-src" ]; };
in pkgs.mkShell {
  name = "rust-shell";
  nativeBuildInputs = with pkgs; [ rust-analyzer rust ];
}
