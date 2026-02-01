{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/25.11";
    flake-parts.url = "github:hercules-ci/flake-parts";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.fenix.follows = "fenix";
    };
  };

  outputs =
    inputs@{
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ];

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];

      flake.overlays.default = import ./overlay.nix;

      perSystem =
        {
          system,
          pkgs,
          ...
        }:
        let
          rust-toolchain = pkgs.fenix.complete.withComponents [
            "cargo"
            "clippy"
            "rustc"
            "rustfmt"
          ];

          kclip-release = pkgs.callPackage ./package.nix {
            rustPlatform = pkgs.rustPlatform;
            lib = pkgs.lib;
          };
          kclip-dev =
            (pkgs.naersk.override {
              cargo = rust-toolchain;
              rustc = rust-toolchain;
            }).buildPackage
              {
                inherit (kclip-release)
                  pname
                  version
                  src
                  postInstall
                  meta
                  ;
              };
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.fenix.overlays.default
              inputs.naersk.overlays.default
            ];
          };

          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
              rust-toolchain
              rust-analyzer
              kclip-dev
            ];
          };

          formatter = pkgs.nixfmt-tree;

          packages = rec {
            kclip-cli = kclip-release;
            default = kclip-cli;
          };
        };
    };
}
