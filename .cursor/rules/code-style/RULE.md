---
description: "Code style standards: self-documenting code, KISS, DRY, separation of concerns, idiomatic Rust"
alwaysApply: true
---

# Code Style Standards

## Core Principles

- **Self-documenting code**: Code should read like documentation through clear naming
- **KISS**: Keep It Simple, Stupid - prefer straightforward solutions
- **DRY**: Don't Repeat Yourself - eliminate duplication
- **Idiomatic Rust**: Write production-ready, industry-grade code
- **Separation of concerns**: Each function/module should have a single responsibility

## Code as Documentation

Avoid doc comments unless strictly necessary. Code should be self-documenting through:
- Clear function and variable names
- Descriptive test names
- Logical structure and organization

### Examples

```rust
// Good: Self-documenting code
fn require_dir(path: &Path, label: &'static str) -> Result<()> {
    if path.is_dir() {
        Ok(())
    } else {
        bail!("{label} is not a directory: {}", path.display());
    }
}

// Bad: Unnecessary documentation
/// Verifies that the given path exists and is a directory.
/// Returns Ok if valid, Err otherwise.
fn require_dir(path: &Path, label: &'static str) -> Result<()> {
    // ... same implementation
}
```

## Separation of Concerns

Separate different concerns into distinct parameters. Don't mix labels, environment variables, or other metadata.

### Examples

```rust
// Good: Separate parameters for different concerns
pub fn require_dir(path: &Path, label: &'static str) -> Result<()>

// Bad: Mixed concerns in single parameter
pub fn require_dir(path: &Path, label: &'static str) -> Result<()>
// where label might be "repo (CODEX_REPO)" mixing label and env var info
```

## Code Quality Standards

- Write idiomatic Rust following standard patterns
- Use `Result` types for error handling (prefer `anyhow::Result`)
- Use `Option` for nullable values
- Leverage Rust's type system for safety
- Prefer `PathBuf` and `Path` for file system operations
- Use `clap` for CLI argument parsing
- Use `tracing` for structured logging

## Documentation

Only add documentation when it provides value:
- Module-level docs (`//!`) explaining purpose and architecture
- Function docs (`///`) for public APIs with non-obvious behavior
- Include Arguments, Returns, Errors sections where useful
- Use intra-doc links for related types
- Avoid redundant or obvious documentation

### Examples

```rust
// Good: Useful documentation
/// Finds the first Codex session matching the given branch name.
///
/// # Arguments
/// * `codexdir` - The Codex directory to search in
/// * `branch` - The git branch name to match against
///
/// # Returns
/// Returns `Some(Session)` if a matching session is found, `None` otherwise.
pub fn find_first_session(codexdir: &Path, branch: &str) -> Result<Option<Session>>

// Bad: Redundant documentation
/// Gets the default codex directory.
/// Returns the default codex directory path.
fn default_codexdir() -> Result<PathBuf> // Obvious from function name and return type
```

## References

- See `src/cli/util.rs` for examples of separation of concerns
- See `src/cli/mod.rs` for module-level documentation patterns
- See `src/cli/args.rs` for CLI argument parsing patterns
