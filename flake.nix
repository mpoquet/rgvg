{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=23.05";
    naersk.url = "github:nix-community/naersk/master";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in rec {
        packages = {
          rgvg = naersk-lib.buildPackage ./.;
        };
        devShells = {
          dev = pkgs.mkShell {
            buildInputs = with pkgs; [ cargo rustc rustfmt pre-commit rustPackages.clippy ];
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          };
          user = pkgs.mkShell {
            buildInputs = [ packages.rgvg pkgs.ripgrep ];
          };
        };
      });
}
