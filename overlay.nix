self: super:
let
  kclip-cli = self.callPackage ./package.nix {
    rustPlatform = super.rustPlatform;
    lib = super.lib;
  };
in
{
  inherit kclip-cli;
}
