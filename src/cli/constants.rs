//! Constants and configuration values.
//!
//! This module centralizes all constant values used throughout the CLI, keeping paths
//! and configuration out of the main logic.

/// Default Codex directory name (relative to home directory).
pub(super) const DOT_CODEX_DIR: &str = ".codex";

/// Git directory name.
pub(super) const DOT_GIT: &str = ".git";

/// Environment variable name for the home directory.
pub(super) const ENV_HOME: &str = "HOME";

/// Environment variable name for tmux session detection.
pub(super) const ENV_TMUX: &str = "TMUX";

/// Home directory subdirectories to include in Codex sandbox.
///
/// These directories are added to the sandbox if they exist in the user's home directory.
pub(super) const HOME_SANDBOX_DIRS: [&str; 4] = [
    ".cargo",
    ".rustup",
    "Library/Caches/Mozilla.sccache",
    ".npm",
];

/// Additional absolute paths to include in Codex sandbox.
///
/// These directories are added to the sandbox if they exist.
pub(super) const EXTRA_SANDBOX_DIRS: [&str; 2] = ["/tmp", "/var/folders"];
