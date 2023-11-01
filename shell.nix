{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell rec {
  packages = [
    rustc
    cargo
    rustfmt
    rust-analyzer
  ] ++ lib.optionals stdenv.isDarwin [
    libiconv
    darwin.apple_sdk.frameworks.AppKit
  ];
  RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = lib.makeLibraryPath packages;
}

