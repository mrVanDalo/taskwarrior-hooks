{ pkgs ?  import <nixpkgs> {} }:
pkgs.mkShell {

  # needed pkgs
  # -----------
  buildInputs = with pkgs; [
    rustc cargo rustfmt
  ];

}
