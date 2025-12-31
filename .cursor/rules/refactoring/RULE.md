---
description: "Refactoring guidelines: idiomatic code, separation of concerns, DRY and KISS principles"
alwaysApply: false
---

# Refactoring Guidelines

## Refactoring Principles

When refactoring code, ensure it becomes:

- **Idiomatic**: Follow Rust best practices and standard patterns
- **Industry-grade**: Production-ready, maintainable code
- **DRY**: Eliminate duplication through abstraction
- **KISS**: Prefer simple, straightforward solutions
- **Well-separated**: Each function/module has a single responsibility

## Separation of Concerns

Separate different concerns into distinct parameters. Don't mix labels, environment variables, paths, or other metadata.

### Examples

```rust
// Good: Separate parameters for different concerns
pub fn require_dir(path: &Path, label: &'static str) -> Result<()> {
    if path.is_dir() {
        Ok(())
    } else {
        bail!("{label} is not a directory: {}", path.display());
    }
}

// Bad: Mixed concerns in single parameter
pub fn require_dir(path: &Path, label: &'static str) -> Result<()>
// where label might be "repo (CODEX_REPO)" mixing label and env var info
```

## DRY (Don't Repeat Yourself)

Extract common patterns into reusable functions, constants, or modules.

### Examples

```rust
// Good: Extract helper functions
mod helpers {
    pub fn get_resume_branch_cmd() -> Command { ... }
    pub fn get_argument_by_id(cmd: &Command, id: &str) -> Arg { ... }
    pub fn assert_help_text_exists(help: Option<&StyledStr>, name: &str) -> String { ... }
}

// Bad: Repeated code patterns
fn test_repo_flag() {
    let cmd = Args::command();
    let resume_branch_cmd = cmd.get_subcommands().find(...).expect(...);
    let repo_arg = resume_branch_cmd.get_arguments().find(...).expect(...);
    // ... repeated in every test
}
```

## KISS (Keep It Simple, Stupid)

Prefer simple, straightforward solutions over complex abstractions.

### Examples

```rust
// Good: Simple, direct approach
pub fn should_use_tmux(no_tmux: bool) -> bool {
    !no_tmux && env_present(ENV_TMUX)
}

// Bad: Over-engineered
pub fn should_use_tmux(no_tmux: bool) -> bool {
    TmuxDecisionEngine::new()
        .with_explicit_flag(no_tmux)
        .with_environment_check(ENV_TMUX)
        .evaluate()
}
```

## Refactoring Checklist

Before refactoring:

1. Ensure existing tests pass
2. Understand the current behavior
3. Identify the specific improvement needed

During refactoring:

1. Make small, incremental changes
2. Run tests after each change
3. Maintain backward compatibility
4. Extract helpers for repeated patterns
5. Separate concerns into distinct parameters/types

After refactoring:

1. All tests pass
2. Code is more maintainable
3. No functionality is lost
4. Code follows Rust idioms
5. Documentation is updated if needed

## References

- See `tests/cli_help.rs` for examples of DRY test patterns
- See `src/cli/util.rs` for separation of concerns examples
- See `src/cli/prelude.rs` for DRY import patterns
