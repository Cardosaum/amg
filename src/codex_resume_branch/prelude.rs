// Re-export commonly used types and functions for convenient access across modules.
// All items here are used by modules that import from this prelude.

// Error handling
pub(super) use anyhow::{Context, Result, bail};

// Standard library - FFI
pub(super) use std::ffi::{OsStr, OsString};

// Standard library - Path
pub(super) use std::path::{Path, PathBuf};

// Standard library - Process
pub(super) use std::process::{Command, ExitCode, ExitStatus};

// Standard library - File system
pub(super) use std::fs;

// Standard library - IO
pub(super) use std::io::{self, BufRead};

// Standard library - Collections
pub(super) use std::cmp::Reverse;
pub(super) use std::collections::BinaryHeap;

// Logging
pub(super) use tracing::{debug, error, info};

// Re-export internal constants for convenient access across modules.
pub(super) use super::constants::{
    DOT_CODEX_DIR, DOT_GIT, ENV_HOME, ENV_TMUX, EXTRA_SANDBOX_DIRS, HOME_SANDBOX_DIRS,
};
