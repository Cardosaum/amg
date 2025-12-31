//! Process execution and tmux integration.
//!
//! This module provides functionality for executing commands, either directly or through
//! tmux. It handles command construction, shell quoting, and process management.

use super::prelude::*;

/// Represents a command to be executed.
///
/// Contains the program name and its arguments, which can be converted to a shell string
/// or executed directly.
#[derive(Debug, Clone)]
pub(super) struct Cmd {
    /// The program to execute.
    pub(super) program: OsString,
    /// The command-line arguments.
    pub(super) args: Vec<OsString>,
}

impl Cmd {
    /// Converts the command to a shell-quoted string representation.
    ///
    /// All arguments are properly quoted for safe shell execution. Single quotes are used
    /// for quoting, with proper escaping for arguments containing quotes.
    ///
    /// # Returns
    ///
    /// Returns a [`String`] containing the shell-quoted command.
    pub(super) fn as_shell_string(&self) -> String {
        std::iter::once(self.program.as_os_str())
            .chain(self.args.iter().map(OsString::as_os_str))
            .map(sh_quote_lossy)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Executes a command in a new tmux window.
///
/// Creates a new tmux window with the specified working directory and executes the command
/// in that window.
///
/// # Arguments
///
/// * `start_dir` - The working directory for the new tmux window
/// * `cmd` - The command to execute
///
/// # Returns
///
/// Returns [`Result<()>`] indicating success or failure.
///
/// # Errors
///
/// Returns an error if:
/// * The `tmux` command cannot be executed
/// * The tmux command fails (non-zero exit status)
pub(super) fn run_tmux_new_window(start_dir: &Path, cmd: &Cmd) -> Result<()> {
    debug!(
        program = ?cmd.program,
        args = ?cmd.args,
        start_dir = %start_dir.display(),
        "spawning tmux new-window"
    );
    let status = Command::new("tmux")
        .arg("new-window")
        .arg("-c")
        .arg(start_dir)
        .arg(&cmd.program)
        .args(&cmd.args)
        .status()
        .context("failed to launch tmux new-window")?;

    if status.success() {
        Ok(())
    } else {
        bail!("tmux exited with status {status}");
    }
}

/// Creates a command that would execute in a new tmux window.
///
/// This is used for dry-run mode to show what command would be executed.
///
/// # Arguments
///
/// * `start_dir` - The working directory for the new tmux window
/// * `cmd` - The command to wrap
///
/// # Returns
///
/// Returns a [`Cmd`] representing the tmux command that would be executed.
pub(super) fn tmux_new_window_cmd(start_dir: &Path, cmd: &Cmd) -> Cmd {
    let mut args: Vec<OsString> = vec![
        "new-window".into(),
        "-c".into(),
        start_dir.as_os_str().to_owned(),
        cmd.program.clone(),
    ];
    args.extend(cmd.args.iter().cloned());
    Cmd {
        program: "tmux".into(),
        args,
    }
}

/// Executes a command in the specified directory.
///
/// Runs the command synchronously and returns its exit code.
///
/// # Arguments
///
/// * `cwd` - The working directory for the command
/// * `cmd` - The command to execute
///
/// # Returns
///
/// Returns [`Result<ExitCode>`] containing the command's exit code.
///
/// # Errors
///
/// Returns an error if:
/// * The command cannot be executed
/// * Process creation fails
pub(super) fn run_in_dir(cwd: &Path, cmd: &Cmd) -> Result<ExitCode> {
    debug!(
        program = ?cmd.program,
        args = ?cmd.args,
        cwd = %cwd.display(),
        "spawning command"
    );
    let status = Command::new(&cmd.program)
        .args(&cmd.args)
        .current_dir(cwd)
        .status()?;
    Ok(exit_code(status))
}

/// Converts an [`ExitStatus`] to an [`ExitCode`].
///
/// Converts an [`ExitStatus`] to an [`ExitCode`].
///
/// Returns [`ExitCode::FAILURE`] if the status code cannot be converted to a `u8`,
/// otherwise returns the corresponding [`ExitCode`].
fn exit_code(status: ExitStatus) -> ExitCode {
    match status.code().and_then(|c| u8::try_from(c).ok()) {
        Some(code) => ExitCode::from(code),
        None => ExitCode::FAILURE,
    }
}

/// Quotes a string for safe shell execution.
///
/// Uses single quotes for quoting, with proper escaping for strings containing quotes.
/// Empty strings are quoted as `''`.
fn sh_quote_lossy(s: &OsStr) -> String {
    let s = s.to_string_lossy();
    if s.is_empty() {
        return "''".to_owned();
    }
    if !s.contains('\'') {
        return format!("'{s}'");
    }
    format!("'{}'", s.replace('\'', "'\\''"))
}
