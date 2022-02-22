{ pkgs ? import <nixpkgs> { } }:
let
  taskwarrior-hooks = import ./default.nix {
    inherit (pkgs) fetchFromGitHub stdenv rustPlatform;
  };

in
pkgs.mkShell {

  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    # taskwarrior-hooks
  ];

}
