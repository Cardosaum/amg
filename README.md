# amg

A command-line tool to manage and resume Codex sessions by git branch name.

## Description

`amg` helps you quickly resume Codex sessions by matching them to git branch names. It searches through Codex session files (JSONL format) and finds the first session that matches your current or specified git branch, then resumes it with the appropriate configuration.

## Installation

### Using Makefile (Recommended)

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

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass: `make ci`
2. Code is formatted: `make fmt`
3. No clippy warnings: `make lint`
4. Follow conventional commit messages

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Author

Cardosaum

## Repository

https://github.com/Cardosaum/amg

