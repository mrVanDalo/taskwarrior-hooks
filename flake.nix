{
  description = "taskwarrior hooks (rust)";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.taskwarrior-hookrs = pkgs.callPackage ./default.nix { };
        # nix build
        defaultPackage = self.packages.${system}.taskwarrior-hookrs;
      }
    );


}
