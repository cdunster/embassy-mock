# Changelog

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2023-08-15

### Documentation

- Add doc-comments to all modules


### Features

- *(executor)* Add MockSpawner support
- *(time)* Add MockTicker support


### Miscellaneous Tasks

- *(ci)* Add workflow for PRs to main
- *(cliff)* Fix glob pattern for tag matching
- *(cliff)* Fix regex for ignored tags
- *(examples)* Add example for the executor
- *(examples)* Add ticker example
- *(nix)* Add cargo-toml-lint
- Warn when missing doc-comments in crate


## [0.0.0] - 2023-08-14

### Documentation

- *(cargo)* Add metadata for publish


### Miscellaneous Tasks

- *(ci)* Add release-plz workflow
- *(cliff)* Add config with custom layout
- *(git)* Ignore code coverage result file
- *(make)* Set tarpaulin as coverage provider
- *(nix)* Add pre-commit hooks
- *(nix)* Add taplo the TOML file formatter
- *(nix)* Add git-cliff the changelog generator
- *(nix)* Add cargo-make
- *(nix)* Add cargo-tarpaulin for code coverage
- *(release-plz)* Use custom cliff config


