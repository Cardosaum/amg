# amg

A command-line tool to manage and resume Codex sessions by git branch name.

## Description

`amg` helps you quickly resume Codex sessions by matching them to git branch names. It searches through Codex session files (JSONL format) and finds the first session that matches your current or specified git branch, then resumes it with the appropriate configuration.

## Installation

### Using cargo-binstall (Recommended - Fastest)

Install pre-built binaries without compiling from source:

```bash
# Install cargo-binstall first (one-time setup)
cargo install cargo-binstall

# Install amg
cargo binstall amg
```

This method downloads pre-built binaries from GitHub Releases, making installation much faster than compiling from source. Binaries are available for:
- Linux (x86_64)
- macOS (Intel and Apple Silicon)
- Windows (x86_64)

### Using Makefile

```bash
git clone https://github.com/Cardosaum/amg.git
cd amg
make install
```

This installs the binary to `$HOME/.cargo/bin` (default Cargo installation directory). Ensure `$HOME/.cargo/bin` is in your PATH (typically done automatically by rustup).

For a custom installation directory:

```bash
make install-custom ROOT=$HOME/.local
```

### Using Cargo

```bash
git clone https://github.com/Cardosaum/amg.git
cd amg
cargo install --path .
```

### From Source (Build Only)

```bash
git clone https://github.com/Cardosaum/amg.git
cd amg
cargo build --release
```

The binary will be available at `target/release/amg`.

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

# Dry run to see what command would be executed
amg rb feature-branch --repo ~/projects/my-repo --dry-run

# Resume without tmux (run inline)
amg resume dev --repo ~/projects/my-repo --no-tmux

# Use environment variables
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

1. **Release PR Creation**: When commits are pushed to `main`, release-plz automatically creates or updates a release Pull Request with:
   - Version bumps in `Cargo.toml` (based on [Conventional Commits](https://www.conventionalcommits.org/))
   - Updated `CHANGELOG.md` (generated from git history)
   - Updated `Cargo.lock` (if dependencies changed)

2. **Release PR Review**: Maintainers review the release PR to ensure:
   - Version bumps are correct (semver based on commit types)
   - Changelog accurately reflects changes
   - All CI checks pass

3. **Publishing**: When the release PR is merged, release-plz automatically:
   - Creates a git tag (format: `amg-v<version>`, e.g., `amg-v0.1.0`)
   - Publishes the crate to [crates.io](https://crates.io)
   - Creates a GitHub release with changelog

### Version Bumping

Versions follow [Semantic Versioning](https://semver.org/) and are determined by commit types:

- **Major version** (`1.0.0` → `2.0.0`): Breaking changes (detected by `cargo-semver-checks` or commits with `BREAKING CHANGE:`)
- **Minor version** (`1.0.0` → `1.1.0`): New features (`feat:` commits)
- **Patch version** (`1.0.0` → `1.0.1`): Bug fixes (`fix:` commits)

### For Contributors

**Important**: All commits must follow the [Conventional Commits](https://www.conventionalcommits.org/) specification for automatic versioning to work correctly.

Commit format: `type(scope): description`

- `feat(cli): add new command` → minor version bump
- `fix(scan): resolve session matching bug` → patch version bump
- `refactor(util): improve error handling` → no version bump (unless breaking)
- `docs: update README` → no version bump
- `chore(deps): update dependencies` → no version bump

See [`.cursor/rules/commits/RULE.md`](.cursor/rules/commits/RULE.md) for detailed commit guidelines.

### Manual Release (if needed)

If you need to trigger a release manually or test locally:

```bash
# Install release-plz
cargo install --locked release-plz

# Update versions and changelog (creates release PR locally)
release-plz release-pr

# Publish to crates.io (after merging release PR)
release-plz release
```

### Secure Token Management with Bitwarden

For local development, you can use [Bitwarden CLI](https://bitwarden.com/help/cli/) to securely manage tokens required by release-plz.

> Note: This repository intentionally does **not** integrate Bitwarden into GitHub Actions. CI releases use GitHub-native auth (the workflow `GITHUB_TOKEN` plus crates.io trusted publishing via OIDC).

#### Quick Start

```bash
# One-time setup
make setup-bitwarden

# Run release-pr with tokens from Bitwarden
make release-pr-bw

# Run release with tokens from Bitwarden
make release-bw
```

#### Setup

1. **Install Bitwarden CLI**:
   ```bash
   brew install bitwarden-cli  # macOS
   # See docs/BITWARDEN_SETUP.md for other platforms
   ```

2. **Run setup script**:
   ```bash
   make setup-bitwarden
   ```

3. **Create vault items** in Bitwarden:
   - `amg-release-github-token`: Store your GitHub Personal Access Token
   - `amg-release-cargo-token` (optional): Store your crates.io token

#### Usage

**Recommended Workflow:**

1. **Unlock Bitwarden and export session** (do this once per terminal session):
   ```bash
   export BW_SESSION=$(bw unlock --raw)
   ```

2. **Run release commands:**
   ```bash
   make release-pr-bw    # Run release-pr with Bitwarden tokens
   make release-bw        # Run release with Bitwarden tokens
   ```

**Alternative: Interactive Unlock**

If you haven't exported `BW_SESSION`, the script will prompt for your password:
```bash
make release-pr-bw    # Will prompt for Bitwarden master password
```

**Using scripts directly:**
```bash
./scripts/release-with-bitwarden.sh release-pr
./scripts/release-with-bitwarden.sh release
```

**Customizing vault item names:**
```bash
export BW_GITHUB_TOKEN_ITEM=your-github-token-item
export BW_CARGO_TOKEN_ITEM=your-cargo-token-item
make release-pr-bw
```

**For Automation / Non-Interactive (Discouraged):**

Avoid `BW_PASSWORD` (it puts your Bitwarden master password in an environment variable). Prefer unlocking once interactively and exporting `BW_SESSION`:

```bash
export BW_SESSION=$(bw unlock --raw)
make release-pr-bw
```

If you must automate login, use API keys:
```bash
export BW_CLIENTID=your_client_id
export BW_CLIENTSECRET=your_client_secret
bw login --apikey
export BW_SESSION=$(bw unlock --raw)
make release-pr-bw
```

For detailed setup instructions and troubleshooting, see [docs/BITWARDEN_SETUP.md](docs/BITWARDEN_SETUP.md).

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass: `make ci`
2. Code is formatted: `make fmt`
3. No clippy warnings: `make lint`
4. **Follow [Conventional Commits](https://www.conventionalcommits.org/)** - This is required for automatic versioning and changelog generation

### Commit Message Format

Use the format: `type(scope): description`

**Types:**
- `feat`: New feature (causes minor version bump)
- `fix`: Bug fix (causes patch version bump)
- `refactor`: Code refactoring (no version bump unless breaking)
- `test`: Adding or updating tests (no version bump)
- `docs`: Documentation changes (no version bump)
- `style`: Code style changes (no version bump)
- `chore`: Maintenance tasks (no version bump)

**Examples:**
- `feat(cli): add shorthand flag for repo argument`
- `fix(scan): resolve session matching bug`
- `docs: update installation instructions`

See [`.cursor/rules/commits/RULE.md`](.cursor/rules/commits/RULE.md) for detailed guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Author

Cardosaum

## Repository

https://github.com/Cardosaum/amg

