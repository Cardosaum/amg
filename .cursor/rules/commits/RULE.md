---
description: "Commit message conventions: conventional commits format and atomic commit practices"
alwaysApply: true
---

# Commit Practices

## Conventional Commits

- Use conventional commit format: `type(scope): description`
- Types: `feat`, `fix`, `refactor`, `test`, `docs`, `style`, `chore`
- Scope: module or area affected (e.g., `cli`, `util`, `test`)
- Description: concise summary of the change

## Atomic Commits

- One logical change per commit
- Each commit should be self-contained and meaningful
- Separate refactoring from feature additions
- Separate test additions from implementation changes

Examples:
```
feat(cli): add require_dir function with env_var parameter
refactor(cli): separate label and env_var in require_dir
test(cli): add comprehensive unit tests for require_dir using rstest
style(cli): remove unnecessary doc comments from tests
chore: add cursor rules for project preferences
```

