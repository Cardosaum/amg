---
description: "Enforce CI checks with make ci before all commits"
alwaysApply: true
---

# CI Checks Workflow

**Always use `make ci` to verify code quality and conformance before committing changes.**

See @Makefile for the CI pipeline definition.

## Required Workflow

When making code changes, you MUST:

1. **Run `make ci` before every commit**
   - Runs formatting checks, linting (clippy), code checks, and tests
   - Never commit code that fails `make ci`
   - Fix all reported issues before proceeding

2. **Use `make ci` as the single source of truth**
   - Do NOT use individual commands like `cargo fmt`, `cargo clippy`, or `cargo test` separately
   - Ensures all checks run in the correct order with proper configuration
   - Guarantees consistency with CI/CD pipeline

3. **If `make ci` fails:**
   - Fix all reported issues immediately
   - Re-run `make ci` to confirm fixes
   - Only commit after `make ci` passes completely

4. **After `make ci` passes:**
   - Ensure commit messages follow conventional commits format (see @commits)
   - Each commit must be atomic and self-contained
   - Each commit must pass `make ci` independently

## Integration with Commit Standards

- All commits must follow conventional commits format (see @commits)
- Each commit must pass `make ci` independently
- Never commit without running `make ci` first

