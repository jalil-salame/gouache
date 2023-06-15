{
  description = "";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    crane.inputs.flake-utils.follows = "flake-utils";
    crane.inputs.rust-overlay.follows = "rust-overlay";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.inputs.flake-utils.follows = "flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    crane,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {inherit system overlays;};

      rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
      src = craneLib.cleanCargoSource ./.;
      nativeBuildInputs = [rustToolchain];
      cargoArtifacts = craneLib.buildDepsOnly {inherit src nativeBuildInputs;};
      bin = craneLib.buildPackage {inherit src nativeBuildInputs cargoArtifacts;};
    in {
      formatter = pkgs.alejandra;

      checks = {inherit bin;};
      packages = {
        inherit bin;
        default = bin;
      };
      apps.default = flake-utils.lib.mkApp {drv = bin;};
      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks.${system};
        nativeBuildInputs = [rustToolchain];
      };
    });
}
