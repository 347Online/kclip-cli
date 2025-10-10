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
    ln -fs $out/bin/kclip $out/bin/kccopy
    ln -fs $out/bin/kclip $out/bin/kcpaste
    ln -fs $out/bin/kclip $out/bin/kcclear
  '';

  meta.mainProgram = manifest.default-run;
}
