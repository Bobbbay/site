{
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    utils.url = "github:gytis-ivaskevicius/flake-utils-plus";
    fenix.url = "github:nix-community/fenix";
    naersk.url = "github:nmattia/naersk";
    deploy-rs.url = "github:serokell/deploy-rs";
  };

  outputs = { self, nixpkgs, utils, fenix, naersk, deploy-rs }:
    with builtins;
    utils.lib.systemFlake {
      inherit self inputs;

      channels.nixpkgs = {
        input = nixpkgs;
        overlaysBuilder = channels: [
          (final: prev: {
            inherit (deploy-rs.packages.${prev.system}) deploy-rs;
          })
        ];
      };

      sharedOverlays = [
        (final: prev: {
          inherit (deploy-rs.packages.${prev.system}) deploy-rs;
        })
      ];

      defaultPackage = {
        x86_64-linux = ((naersk.lib.x86_64-linux.override {
          inherit (fenix.packages.x86_64-linux.minimal) cargo rustc;
        }).buildPackage {
          src = ./.;
        });
      };

      devShellBuilder = channels:
        with channels.nixpkgs;
        mkShell {
          buildInputs = [
            pandoc
            diesel-cli
            nixpkgs-fmt
          ];
        };
    };
}
