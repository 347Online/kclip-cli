{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
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
        lib = pkgs.lib;
        rust-toolchain = pkgs.fenix.complete.withComponents [
          "cargo"
          "clippy"
          "rustc"
          "rustfmt"
        ];
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rust-toolchain;
          rustc = rust-toolchain;
        };
        kclip =
          let
            manifest = (lib.importTOML ./Cargo.toml).package;
          in
          (pkgs.naersk.override {
            cargo = rust-toolchain;
            rustc = rust-toolchain;
          }).buildPackage
            {
              pname = manifest.name;
              version = manifest.version;

              src = lib.cleanSource ./.;

              postInstall = ''
                ln -fs $out/bin/kclip $out/bin/kccopy
                ln -fs $out/bin/kclip $out/bin/kcpaste
              '';

              meta.mainProgram = manifest.default-run;
            };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-toolchain
            rust-analyzer-nightly
            kclip
          ];
        };
        packages = {
          inherit kclip;
          default = kclip;
        };
      }
    );
}
