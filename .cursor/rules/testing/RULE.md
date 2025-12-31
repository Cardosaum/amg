---
description: "Testing practices: tests as documentation, rstest usage, fixtures, templates, comprehensive coverage"
globs:
  - "**/*test*.rs"
  - "**/tests/**/*.rs"
alwaysApply: false
---

# Testing Practices

## Tests as Documentation

Tests should be self-documenting through clear, descriptive names. Test names should read like documentation.

### Examples

```rust
// Good: Descriptive test names
#[test]
fn validates_existing_directory() { ... }

#[test]
fn rejects_nonexistent_path() { ... }

#[test]
fn returns_error_with_clear_message() { ... }

// Bad: Vague test names
#[test]
fn test1() { ... }

#[test]
fn test_require_dir() { ... }
```

## Testing Framework Usage

Leverage `rstest` and `rstest_reuse` extensively for parameterized tests and fixtures.

### Parameterized Tests

Use `#[rstest]` with `#[case]` for multiple test scenarios:

```rust
#[rstest]
#[case("resume-branch")]
#[case("rb")]
#[case("resume")]
fn test_subcommand_aliases(#[case] subcommand: &str) {
    let args = parse_args_from(["amg", subcommand, "test-branch", "--repo", "/tmp/repo"]);
    match args.command {
        Commands::ResumeBranch { branch, .. } => {
            assert_eq!(branch, "test-branch");
        }
    }
}
```

### Fixtures

Use `#[fixture]` for reusable test data:

```rust
#[fixture]
fn temp_dir() -> PathBuf {
    let dir = tempfile::tempdir().unwrap();
    dir.path().to_path_buf()
}

#[rstest]
fn test_with_fixture(#[from(temp_dir)] dir: PathBuf) {
    // Use dir in test
}
```

### Templates

Use `#[template]` and `#[apply]` to avoid duplication:

```rust
#[template]
#[rstest]
#[case("main")]
#[case("feature-branch")]
#[case("dev")]
fn branch_names(#[case] branch: &str) {}

#[apply(branch_names)]
fn test_branch_parsing(#[case] branch: &str) {
    // Test implementation
}
```

## Test Organization

Organize tests into logical modules:

- Group related tests together
- Use descriptive module names
- Separate unit tests from integration tests
- Use helper modules for common test utilities

### Examples

```rust
#[cfg(test)]
mod tests {
    mod success_cases {
        use super::*;
        
        #[rstest]
        fn validates_existing_directory() { ... }
    }
    
    mod error_cases {
        use super::*;
        
        #[rstest]
        fn rejects_nonexistent_path() { ... }
    }
    
    mod helpers {
        pub fn parse_args_from<I, T>(args: I) -> Args { ... }
    }
}
```

## Test Coverage

Aim for comprehensive test coverage:

- Test happy paths
- Test error cases
- Test edge cases
- Test boundary conditions
- Test integration between components

## Test Execution

Use the Makefile for CI-grade test execution:

- `make test`: Run all tests with cargo test
- `make test-nextest`: Run tests with nextest (faster, parallel)
- `make ci`: Run all CI checks including tests

## No Doc Comments in Tests

Tests should be self-documenting. Avoid doc comments in test code:

```rust
// Good: Self-documenting test
#[test]
fn validates_existing_directory() {
    // Test implementation
}

// Bad: Unnecessary documentation
/// Verifies that require_dir succeeds for valid directory paths.
/// This test ensures that the function correctly validates existing directories.
#[test]
fn validates_existing_directory() {
    // Same implementation
}
```

## References

- See `tests/cli_help.rs` for examples of organized test modules
- See `src/cli/args.rs` for examples of rstest parameterized tests
- See `Makefile` for test execution targets
