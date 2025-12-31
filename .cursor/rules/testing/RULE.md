---
description: "Testing practices: tests as documentation, rstest/rstest_reuse usage, fixtures and templates"
globs:
  - "**/*test*.rs"
  - "**/tests/**/*.rs"
alwaysApply: false
---

# Testing Philosophy

## Tests as Documentation

- Tests act as documentation through clear, descriptive names
- Test names should read like documentation: `validates_existing_directory`, `rejects_nonexistent_path`
- No doc comments in tests - the code itself is the documentation
- Organize tests into logical modules (success, failure, error_messages, fixtures)

## Testing Framework Usage

- Leverage testing frameworks to the max (rstest, rstest_reuse)
- Use fixtures extensively for reusable test setup
- Use templates (`#[template]`) for parameterized test cases
- Apply templates with `#[apply(template_name)]` to avoid duplication
- Use `#[fixture]` for common test data

Example:
```rust
#[template]
#[rstest]
#[case("repo", Some("CODEX_REPO"))]
#[case("codexdir", Some("CODEX_CODEXDIR"))]
fn success_cases(#[case] label: &'static str, #[case] env_var: Option<&'static str>) {}

#[apply(success_cases)]
fn validates_existing_directory(
    #[from(fixtures::temp_dir)] dir: PathBuf,
    #[case] label: &'static str,
    #[case] env_var: Option<&'static str>,
) { ... }
```

