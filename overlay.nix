self: super:
let
  kclip-cli = self.callPackage ./. { };
in
{
  inherit kclip-cli;
}
