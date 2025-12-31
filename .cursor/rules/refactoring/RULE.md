---
description: "Refactoring guidelines: idiomatic code, separation of concerns, DRY and KISS principles"
alwaysApply: false
---

# Refactoring Guidelines

## Refactoring Principles

- Refactor to be idiomatic, industry-grade, production-ready
- Separate concerns properly (e.g., env var vs label as separate parameters)
- Make code DRY and KISS
- When refactoring, ensure proper argument separation instead of mixing concerns

Example:
```rust
// Good: Separate parameters for different concerns
pub fn require_dir(path: &Path, label: &'static str, env_var: Option<&'static str>) -> Result<()>

// Bad: Mixed concerns in single parameter
pub fn require_dir(path: &Path, label: &'static str) -> Result<()>
// where label might be "repo (CODEX_REPO)" mixing label and env var info
```

