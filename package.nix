{ lib, rustPlatform }:
let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;

  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;

  postInstall = ''
    $out/bin/kclip install $out/bin
  '';

  meta.mainProgram = manifest.default-run;
}
