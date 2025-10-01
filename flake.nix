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
        mkPkg =
          let
            manifest = (lib.importTOML ./Cargo.toml).package;
            default-args = {
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
          args:
          (pkgs.naersk.override {
            cargo = rust-toolchain;
            rustc = rust-toolchain;
          }).buildPackage
            (default-args // args);

        kclip-dev = mkPkg {
          release = false;
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-toolchain
            rust-analyzer-nightly
            kclip-dev
          ];
        };
        packages = rec {
          kclip = mkPkg { };
          default = kclip;
        };
      }
    );
}
