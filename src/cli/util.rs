//! Utility functions for paths, environment variables, and common operations.
//!
//! This module provides helper functions for:
//! * Path resolution and validation
//! * Environment variable access
//! * Tmux detection
//! * Home directory resolution

use super::prelude::*;

/// Gets the default Codex directory path.
///
/// Returns `$HOME/.codex` if `$HOME` is set and non-empty.
///
/// # Returns
///
/// Returns [`Result<PathBuf>`] containing the default codex directory path.
///
/// # Errors
///
/// Returns an error if `$HOME` is not set or empty.
pub(super) fn default_codexdir() -> Result<PathBuf> {
    match std::env::var_os(ENV_HOME) {
        Some(home) if !home.is_empty() => Ok(PathBuf::from(home).join(DOT_CODEX_DIR)),
        _ => bail!("CODEX_CODEXDIR is not set and $HOME is empty; please set CODEX_CODEXDIR"),
    }
}

/// Validates that a path exists and is a directory.
///
/// # Arguments
///
/// * `path` - The path to validate
/// * `label` - A human-readable label for error messages (e.g., "repo", "codexdir")
///
/// # Returns
///
/// Returns [`Result<()>`] if the path is a valid directory.
///
/// # Errors
///
/// Returns an error if the path does not exist or is not a directory.
pub(super) fn require_dir(path: &Path, label: &'static str) -> Result<()> {
    if path.is_dir() {
        Ok(())
    } else {
        bail!("{label} is not a directory: {}", path.display());
    }
}

/// Gets the user's home directory path.
///
/// Returns `$HOME` if it's set and non-empty.
///
/// # Returns
///
/// Returns [`Option<PathBuf>`] containing the home directory path, or `None` if not set.
pub(super) fn home_dir() -> Option<PathBuf> {
    std::env::var_os(ENV_HOME)
        .filter(|h| !h.is_empty())
        .map(PathBuf::from)
}

/// Determines whether to use tmux for command execution.
///
/// Returns `true` if tmux should be used, which is when:
/// * `no_tmux` is `false` (tmux is not explicitly disabled)
/// * `$TMUX` environment variable is set and non-empty
///
/// # Arguments
///
/// * `no_tmux` - If `true`, tmux will not be used regardless of environment
///
/// # Returns
///
/// Returns `true` if tmux should be used, `false` otherwise.
pub(super) fn should_use_tmux(no_tmux: bool) -> bool {
    !no_tmux && env_present(ENV_TMUX)
}

/// Checks if an environment variable is present and non-empty.
///
/// # Arguments
///
/// * `name` - The name of the environment variable to check
///
/// # Returns
///
/// Returns `true` if the environment variable is set and non-empty, `false` otherwise.
fn env_present(name: &str) -> bool {
    std::env::var_os(name).is_some_and(|v| !v.is_empty())
}
