{
  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-parts.url = "flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs: inputs.flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [ "x86_64-linux" ];

    imports = [
      inputs.git-hooks.flakeModule
    ];

    perSystem = { config, pkgs, system, ... }: {
      _module.args.pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [ inputs.rust-overlay.overlays.default ];
      };

      formatter = pkgs.nixpkgs-fmt;

      pre-commit.settings.hooks = {
        # Nix
        nixpkgs-fmt.enable = true;
        deadnix.enable = true;
        statix.enable = true;

        # Rust
        rustfmt.enable = true;

        # Git
        check-merge-conflicts.enable = true;
        no-commit-to-branch.enable = true;
        commitizen.enable = true;

        # TOML
        taplo.enable = true;

        # Markdown
        markdownlint = {
          enable = true;
          settings.configuration = {
            # Allow duplicate headings if they have different parents (for the changelog)
            no-duplicate-heading.siblings_only = true;
          };
        };
        mdformat.enable = true;

        # Spell checking
        typos.enable = true;

        # Whitespace
        mixed-line-endings.enable = true;
        trim-trailing-whitespace.enable = true;

        # Private keys
        detect-private-keys.enable = true;
      };

      devShells.default = with pkgs; mkShell {
        packages = [
          (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          config.pre-commit.settings.enabledPackages
          git-cliff
          cargo-make
          cargo-llvm-cov
        ];

        RUST_SRC_PATH = rustPlatform.rustLibSrc;

        shellHook = "${config.pre-commit.installationScript}";
      };
    };
  };
}
