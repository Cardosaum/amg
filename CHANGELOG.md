# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/Cardosaum/amg/releases/tag/{package}-v{version}) - 2025-12-31

### Added

- *(build)* add install targets to Makefile
- add shorthand flag for repo argument
- make subcommand aliases visible in help
- add tracing dependencies for structured logging
- add codex_resume_branch tool

### Fixed

- correct release-plz.toml configuration schema
- resolve CI failures
- resolve CI failures

### Other

- add comments to .gitignore for better maintainability
- *(ci)* add release-plz pipeline for automated releases
- *(rules)* enhance testing rule with patterns
- *(rules)* add concrete refactoring guidance
- *(rules)* enhance commits rule with examples
- *(rules)* improve code-style rule with best practices
- add MIT and Apache 2.0 license files
- refactor cursor rules to follow official best practices
- add rstest_reuse dependency for test templates
- *(cli)* separate label and env_var parameters in require_dir
- add documentation metadata to Cargo.toml
- add documentation for utility modules
- add documentation for scan, process, and codex_cmd modules
- add module and function documentation for CLI
- add crate-level documentation
- remove fmt-check dependency from test targets
- expose Args and Commands for testing
- improve CLI help tests with DRY, KISS principles
- enforce formatting checks in Makefile targets
- add integration test for code formatting
- add production-grade GitHub Actions workflow
- update references after module rename
- add comprehensive README
- update Makefile binary name
- add Makefile with CI-grade test harness
- expand prelude module and use it consistently
- add rstest dev dependency
- add prelude module for common imports
- add clippy and rust lint configuration
- add nextest configuration
- convert CLI to subcommands structure
- add package metadata to Cargo.toml
