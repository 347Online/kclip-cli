final: prev: {
  kclip-cli = final.callPackage ./package.nix {
    rustPlatform = prev.rustPlatform;
    lib = prev.lib;
  };
}
