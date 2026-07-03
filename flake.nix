{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    fenix.url = "github:nix-community/fenix";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, flake-utils, crane, fenix, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        toolchain = fenix.packages.${system}.latest.toolchain;

        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        commonArgs = {
          src = craneLib.cleanCargoSource ./.;

          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ openssl_3 ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        crate = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
        # For `nix build` & `nix run`:
        packages.default = crate;

        # For `nix develop`:
        devShells.default = craneLib.devShell {
          inputsFrom = [ crate ];

          packages = with pkgs; [
            clippy
            gitFull
            rustfmt
          ];
        };
      }
    );
}
