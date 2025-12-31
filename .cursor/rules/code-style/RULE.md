---
description: "Code style and documentation standards: self-documenting code, KISS principle, separation of concerns"
alwaysApply: true
---

# Code Style and Documentation

## Code as Documentation (KISS)

- Code should be self-documenting through clear naming and structure
- Avoid doc comments unless strictly necessary (almost never)
- Keep It Simple, Stupid (KISS) - prefer simple, straightforward solutions
- Test names and function names should read like documentation

Example:
```rust
// Good: Self-documenting code
fn validates_existing_directory(dir: PathBuf, label: &str) { ... }

// Bad: Unnecessary documentation
/// Verifies that require_dir succeeds for valid directory paths.
/// This test ensures that the function correctly validates existing directories.
fn validates_existing_directory(dir: PathBuf, label: &str) { ... }
```

## Code Quality Standards

- Write idiomatic, industry-grade, production-ready code
- Follow single concern principle - each function/module should do one thing
- Apply DRY (Don't Repeat Yourself) - eliminate duplication
- Proper separation of concerns (e.g., separate env var names from labels)
- Code should be well-written and easy to understand

Example:
```rust
// Good: Proper separation of concerns
pub fn require_dir(path: &Path, label: &'static str, env_var: Option<&'static str>) -> Result<()>

// Bad: Mixed concerns
pub fn require_dir(path: &Path, label: &'static str) -> Result<()>
// where label contains "repo (CODEX_REPO)" mixing label and env var
```

