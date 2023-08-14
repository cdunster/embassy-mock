{
  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-utils.url = "flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = { self, ... }@inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = inputs.nixpkgs.legacyPackages.${system}.appendOverlays [
          inputs.rust-overlay.overlays.default
        ];
        rustFromFile = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        checks = {
          pre-commit = inputs.pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              nixpkgs-fmt.enable = true;
              rustfmt.enable = true;
              commitizen.enable = true;
              taplo.enable = true;
            };
          };
        };

        devShells.default = with pkgs; mkShell {
          packages = [
            rustFromFile
            taplo
          ];

          RUST_SRC_PATH = rustPlatform.rustLibSrc;

          shellHook = ''
            ${(self.checks.${system}.pre-commit).shellHook}
          '';
        };
      });
}

