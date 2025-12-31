# amg

[![CI](https://github.com/Cardosaum/amg/actions/workflows/ci.yml/badge.svg)](https://github.com/Cardosaum/amg/actions/workflows/ci.yml)
[![Release](https://github.com/Cardosaum/amg/actions/workflows/release-plz.yml/badge.svg)](https://github.com/Cardosaum/amg/actions/workflows/release-plz.yml)
[![Crates.io](https://img.shields.io/crates/v/amg)](https://crates.io/crates/amg)
[![docs.rs](https://docs.rs/amg/badge.svg)](https://docs.rs/amg)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-APACHE)

> [!WARNING]
> **Early Development**: This project is in early development. APIs and behavior may change without notice.

A command-line tool to manage and resume Codex sessions by git branch name.

## Description

`amg` helps you quickly resume Codex sessions by matching them to git branch names. It searches through Codex session files (JSONL format) and finds the first session that matches your current or specified git branch, then resumes it with the appropriate configuration.

## Installation

### Using cargo-binstall (Recommended)

```bash
cargo install cargo-binstall
cargo binstall amg
```

Pre-built binaries are available for Linux (x86_64), macOS (Intel/Apple Silicon), and Windows (x86_64).

### Using Cargo

```bash
cargo install --git https://github.com/Cardosaum/amg.git
```

### From Source

```bash
git clone https://github.com/Cardosaum/amg.git
cd amg
cargo build --release
```

The binary will be at `target/release/amg`. To install: `make install` (installs to `$HOME/.cargo/bin`).

## Usage

### Basic Usage

Resume a Codex session for a specific branch:

```bash
amg resume-branch <branch-name> --repo /path/to/repo
```

Or use the shorter aliases:

```bash
amg rb <branch-name> --repo /path/to/repo
amg resume <branch-name> --repo /path/to/repo
```

### Environment Variables

You can set environment variables to avoid passing flags every time:

```bash
export CODEX_REPO=/path/to/your/repo
export CODEX_CODEXDIR=/path/to/.codex  # Optional, defaults to $HOME/.codex
```

Then simply run:

```bash
amg resume-branch main
```

### Options

- `--repo <REPO>`: Repository path to grant Codex sandbox access to (required, or set `CODEX_REPO`)
- `--codexdir <DIR>`: Codex directory containing JSONL sessions (optional, defaults to `$HOME/.codex`, or set `CODEX_CODEXDIR`)
- `-n, --dry-run`: Print the command that would be executed without running it
- `--no-tmux`: Disable automatic tmux window creation (if `$TMUX` is set)

### Examples

```bash
# Resume session for 'main' branch
amg resume-branch main --repo ~/projects/my-repo

# Dry run
amg rb feature-branch --repo ~/projects/my-repo --dry-run

# Without tmux
amg resume dev --repo ~/projects/my-repo --no-tmux

# With environment variables
export CODEX_REPO=~/projects/my-repo
amg rb main
```

## How It Works

1. Searches through the Codex directory (default: `$HOME/.codex`) for JSONL session files
2. Reads the first line of each JSONL file to extract git branch information
3. Matches sessions where `.payload.git.branch` equals your specified branch name
4. Resumes the first matching session with appropriate sandbox configuration

## Development

### Prerequisites

- Rust 1.91 or later
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
# Using cargo test
cargo test

# Using nextest (faster, parallel)
cargo nextest run

# Or use the Makefile
make test-nextest
```

### Code Quality

The project uses strict linting and formatting:

```bash
# Run all CI checks
make ci

# Format code
make fmt

# Run clippy
make lint

# Check formatting
make fmt-check
```

### Project Structure

```
src/
├── bin/
│   └── amg.rs          # Binary entry point
├── cli/
│   ├── mod.rs          # Main CLI logic
│   ├── args.rs         # CLI argument parsing
│   ├── codex_cmd.rs    # Codex command building
│   ├── scan.rs         # Session scanning
│   ├── process.rs      # Process execution
│   ├── util.rs         # Utility functions
│   ├── constants.rs    # Constants
│   ├── logging.rs      # Logging setup
│   └── prelude.rs      # Common imports
└── lib.rs              # Library root
```

## Releases

This project uses [release-plz](https://release-plz.dev) for automated releases. The release process is fully automated through GitHub Actions.

### How Releases Work

1. **Push to main** → release-plz creates/updates a release PR with version bumps and changelog
2. **Review & merge PR** → release-plz creates tag and publishes to crates.io
3. **Tag creation** → binaries are built and uploaded to GitHub Releases

Versions follow [Semantic Versioning](https://semver.org/) based on commit types:
- `feat:` → minor version bump
- `fix:` → patch version bump
- Breaking changes → major version bump

### Manual Release

```bash
cargo install --locked release-plz
release-plz release-pr  # Create release PR
release-plz release      # Publish (after merging PR)
```

For secure token management with Bitwarden, see [docs/BITWARDEN_SETUP.md](docs/BITWARDEN_SETUP.md).

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass: `make ci`
2. Code is formatted: `make fmt`
3. No clippy warnings: `make lint`
4. Follow [Conventional Commits](https://www.conventionalcommits.org/) for automatic versioning

Commit format: `type(scope): description`

- `feat(cli): add new command` → minor version bump
- `fix(scan): resolve bug` → patch version bump
- `docs: update README` → no version bump

See [`.cursor/rules/commits/RULE.md`](.cursor/rules/commits/RULE.md) for details.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Links

- Repository: https://github.com/Cardosaum/amg
- Author: Cardosaum

