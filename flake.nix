{
  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-parts.url = "flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs: inputs.flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [ "x86_64-linux" ];

    perSystem = { self', pkgs, system, ... }: {
      _module.args.pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [ inputs.rust-overlay.overlays.default ];
      };

      formatter = pkgs.nixpkgs-fmt;

      checks.pre-commit = inputs.pre-commit-hooks.lib.${system}.run {
        src = ./.;
        hooks = {
          # Nix
          nixpkgs-fmt.enable = true;

          # Rust
          rustfmt.enable = true;

          # Git
          commitizen.enable = true;

          # TOML
          taplo.enable = true;
        };
      };

      devShells.default = with pkgs; mkShell {
        packages = [
          (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          self'.checks.pre-commit.enabledPackages
          git-cliff
          cargo-make
          cargo-tarpaulin
          cargo-toml-lint
          cargo-llvm-cov
        ];

        RUST_SRC_PATH = rustPlatform.rustLibSrc;

        inherit (self'.checks.pre-commit) shellHook;
      };
    };
  };
}
