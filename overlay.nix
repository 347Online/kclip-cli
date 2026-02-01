final: prev: {
  kclip-cli = prev.callPackage ./package.nix {
    rustPlatform = prev.rustPlatform;
    lib = prev.lib;
  };
}
