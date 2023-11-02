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
  ] ++ lib.optionals stdenv.isLinux [
    pkg-config
    cairo
    gdk-pixbuf
    pango
    atk
    gtk3.dev
  ];
  RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = lib.makeLibraryPath packages;
}

