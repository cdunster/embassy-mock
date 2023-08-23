# Changelog

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2023-08-23

### Documentation

- *(readme)* Fix link to license on badge


## [0.2.0] - 2023-08-23

### Bug Fixes

- *(cliff)* Disable skipping of tags


### Documentation

- *(executor)* Add more doc-comments
- *(readme)* Add badges to the top
- *(ticker)* Add more doc-comments
- *(timer)* Add more doc-comments


### Features

- *(ticker)* Add mock for `every()` function
- *(time)* Add MockTimer with `after()` support


### Miscellaneous Tasks

- *(ci)* Add codecov workflow for pushes to main
- *(ci)* Upload coverage report to Codecov
- *(ci)* Show more info in code coverage report
- *(ci)* Add step in test job to run all tests
- *(ci)* Add required component for llvm-cov
- *(cliff)* Conditional breaking description
- *(cliff)* Add `BREAKING` tag before message
- *(codecov)* Set check for coverage delta
- *(examples)* Remove concrete-timer example
- *(examples)* Add example using concrete types
- *(examples)* Using MockTimer without trait
- *(examples)* Add timer example using `after()`
- *(make)* Change coverage report format
- *(make)* Show relative path for code coverage
- *(make)* Override coverage task, use llvm-cov
- Add support for llvm-cov code coverage tool


### Refactor

- [**BREAKING**] *(executor)* Remove const generic
- [**BREAKING**] *(ticker)* Remove const generic
- Use snafu instead of thiserror


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


