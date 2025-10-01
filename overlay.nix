self: super:
let
  kclip-cli = super.callPackage ./. { };
in
{
  inherit kclip-cli;
}
