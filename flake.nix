{
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    utils.url = "github:gytis-ivaskevicius/flake-utils-plus";
    fenix.url = "github:nix-community/fenix";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, utils, fenix, naersk }:
  with builtins;
  utils.lib.systemFlake {
    inherit self inputs;

		channels.nixpkgs = { input = nixpkgs; };

    defaultPackage = {
      x86_64-linux = ((naersk.lib.x86_64-linux.override {
        inherit (fenix.packages.x86_64-linux.minimal) cargo rustc;
	    }).buildPackage {
        src = ./.;
        nativeBuildInputs = [ nixpkgs.legacyPackages.x86_64-linux.pandoc ];
      });
    };

		devShellBuilder = channels:
		  with channels.nixpkgs;
		  mkShell {
		    buildInputs = [
		      pandoc
		      diesel-cli
		    ];
		  };
  };
}
