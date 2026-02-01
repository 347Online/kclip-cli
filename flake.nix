{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/25.11";
    flake-utils.url = "github:numtide/flake-utils";
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
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            inputs.fenix.overlays.default
            inputs.naersk.overlays.default
          ];
        };
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
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-toolchain
            rust-analyzer
            kclip-dev
          ];
        };
        packages = rec {
          kclip-cli = kclip-release;
          default = kclip-cli;
        };
        overlays.default = ./overlay.nix;
      }
    );
}
