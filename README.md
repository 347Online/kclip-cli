# KClip 📋

KClip is a cross-platform commandline utility for copying to and pasting from the system clipboard, similar to pbccopy/pbpaste on macOS and xclip/wl-clipboard on Linux systems.

## Usage

KClip can be invoked in one of three ways:
- 1. via the `kclip` command - this is the main binary and can be used to either copy or paste with `kclip copy` and `kclip paste`
- 2. when the name of the file is `kccopy`* - reads text from std in and copies it to the system clipboard
- 3. when the name of the file is `kcpaste`* - writes the contents of the system clipboard to stdout

## Compilation

To build KClip you will need an up-to-date version of Rust. KClip has been written against the 2024 edition. You may be able to get KClip to build with older editions by changing the setting in Cargo.toml, however this is not recommended and thus will not receive documentation or support.

### Cargo-based setup

#### Pre-requisites

You will need a working installation of the Rust toolchain, using [Rustup](https://rustup.rs/) is highly recommended

#### Building

To build KClip, simply run

```sh
cargo build --release
```

Like any Rust project, initial compilation will take longer as all associated dependencies are compiled, but later builds will be faster by virtue of incremental compilation.

After building the application, you will need to place the compiled binary in a location on `$PATH`, e.g. /usr/local/bin, however this alone will not provide access to `kccopy` or `kcpaste`.
For that, it is recommended to create symlinks to wherever you placed the kclip binary in a location on `$PATH` as well. If you expect to rebuild KClip often, you can even link to the location of the compiled binary itself, which will always give you access to your latest build.

```sh
# Execute from the base directory of the repository
for x in {kclip,kccopy,kcpaste}; do
  sudo ln -s "$(readlink -f ./target/release/kclip)" "/usr/local/bin/$x"
done
```

### Nix-based setup

This is the primary intended method to build KClip.

#### Pre-requisites

You will need a working Nix installation, the setup for which is beyond the scope of this document. Generally one has three options for acquiring Nix:
* Determinate Systems, via [docs.determinate.systems/determinate-nix](https://docs.determinate.systems/determinate-nix/)
* Nix, via [nixos.org](https://nixos.org/)
* Lix, via [lix.systems](https://lix.systems/)

#### Building

This repository contains a Nix dev shell which you can access with `nix develop` or via [nix-direnv](https://github.com/nix-community/nix-direnv). The dev shell will automatically keep a local development build of KClip on `$PATH` for easier testing while hacking on the program.

To create a release build, simply run `nix build` while in this directory, or `nix build "github:347Online/kclip-cli"` if you do not wish to clone the repo locally. This will create a symlink to the derivation output in your Nix store. You can freely move this symlinks to a location on `$PATH` e.g. /usr/local/bin:

```sh
sudo mv result/bin/* /usr/local/bin/
```

You can also add KClip to a NixOS, nix-darwin, or Home Manager installation:

##### Flake-based setup (recommended)

```nix
# Add to flake inputs
  inputs = {
    kclip = {
      url = "github:347Online/kclip-cli";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

# NixOS / nix-darwin
environment.systemPackages = [
  inputs.kclip.packages.${system}.kclip-cli
];

# Home Manager
home.packages = [
  inputs.kclip.packages.${system}.kclip-cli
];
```

