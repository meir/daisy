{
  description = "Daisy, a static site generator written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        packages = rec {
          daisy =
            let
              rustPlatform = pkgs.makeRustPlatform {
                cargo = rust;
                rustc = rust;
              };
              version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
            in
            rustPlatform.buildRustPackage {
              version = "${version}-flake";
              pname = "daisy";

              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
            };

          default = daisy;
        };

        devShells = {
          default =
            with pkgs;
            mkShell {
              buildInputs = [
                libiconv
                openssl
                pkg-config
                rust
              ];
            };
          build =
            with pkgs;
            mkShell {
              buildInputs = [ self.packages.${system}.daisy ];
            };
        };
      }
    );
}
