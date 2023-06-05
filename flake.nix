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
          rgvg = naersk-lib.buildPackage {
            src = pkgs.lib.sourceByRegex ./. [
              "^Cargo\.toml"
              "^Cargo\.lock"
              "^src"
              "^src/.*\.rs"
              "^src/bin"
              "^src/bin/.*\.rs"
              "^src/input"
              "^src/input/.*\.rs"
            ];
          };
        };
        devShells = {
          dev = pkgs.mkShell {
            buildInputs = with pkgs; [ cargo rustc rustfmt pre-commit rustPackages.clippy ];
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          };
          user = pkgs.mkShell {
            buildInputs = [ packages.rgvg pkgs.gnugrep pkgs.ripgrep pkgs.ugrep ];
          };
        };
      });
}
