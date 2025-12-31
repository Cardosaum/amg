// Re-export commonly used types and functions for convenient access across modules.
// All items here are used by modules that import from this prelude.
pub(super) use anyhow::{Context, Result, bail};
pub(super) use std::ffi::{OsStr, OsString};
pub(super) use std::path::{Path, PathBuf};
pub(super) use std::process::ExitCode;
pub(super) use tracing::{debug, error, info};

// Re-export internal constants for convenient access across modules.
pub(super) use super::constants::{DOT_CODEX_DIR, ENV_HOME, ENV_TMUX};


