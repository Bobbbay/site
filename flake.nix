{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, fenix, utils, naersk, nixpkgs }:
    utils.lib.eachDefaultSystem (system: {
      defaultPackage = (naersk.lib.${system}.override {
        inherit (fenix.packages.${system}.minimal) cargo rustc;
      }).buildPackage { src = ./.; };
    });
}
