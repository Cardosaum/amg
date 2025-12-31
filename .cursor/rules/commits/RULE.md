---
description: "Commit conventions: conventional commits format and atomic commit practices"
alwaysApply: true
---

# Commit Practices

## Conventional Commits Format

Use the format: `type(scope): description`

### Commit Types

- `feat`: New feature
- `fix`: Bug fix
- `refactor`: Code refactoring without changing behavior
- `test`: Adding or updating tests
- `docs`: Documentation changes
- `style`: Code style changes (formatting, whitespace)
- `chore`: Maintenance tasks (dependencies, config)

### Scope

Module or area affected (e.g., `cli`, `util`, `test`, `ci`). Omit scope if change affects multiple areas.

### Description

Concise summary in imperative mood (e.g., "add feature" not "added feature" or "adds feature").

## Atomic Commits

Each commit should represent one logical, self-contained change:

- One feature, fix, or refactoring per commit
- Each commit should compile and pass tests independently
- Separate refactoring from feature additions
- Separate test additions from implementation changes
- Separate documentation from code changes
- Separate dependency updates from code changes

## Examples

### Good: Atomic Commits

```
feat(cli): add shorthand flag for repo argument

Add -r as shorthand for --repo flag for improved usability.

---

test: add integration test for code formatting

Add integration test that enforces code formatting by running
cargo fmt --check. This ensures formatting is verified automatically
as part of the test suite.

---

refactor: improve CLI help tests with DRY, KISS principles

Refactor CLI help text tests to be more idiomatic and production-ready:
- Extract helper functions for common operations
- Add constants module for all repeated strings
- Organize tests into logical modules
- Reduce code duplication by ~25%

---

docs: add crate-level documentation

Add comprehensive crate-level documentation to lib.rs including:
- Overview and purpose
- Usage examples
- Module structure
- Links to repository and README

Enable missing_docs warning to catch undocumented public items.
```

### Bad: Non-Atomic Commits

```
feat: add new feature and fix bugs and update docs

This mixes multiple concerns in one commit.

---

refactor: refactor everything

Too vague, doesn't explain what was refactored or why.
```

## Commit Message Guidelines

- First line should be 50-72 characters
- Use body for detailed explanation (wrap at 72 characters)
- Explain *what* and *why*, not *how* (code shows how)
- Reference issues/PRs when applicable
- Use bullet points in body for multiple changes

## References

- See git history for examples of atomic commits
- Conventional Commits spec: https://www.conventionalcommits.org/
