//! Command-line interface implementation for amg.
//!
//! This module provides the main CLI functionality for managing and resuming Codex sessions.
//! It handles argument parsing, session scanning, command building, and process execution.
//!
//! ## Architecture
//!
//! The CLI is organized into several submodules:
//!
//! * Command-line argument parsing using `clap`
//! * Session scanning and matching logic
//! * Codex command building
//! * Process execution and tmux integration
//! * Utility functions for paths, environment variables, etc.
//! * Constants and configuration values
//! * Logging initialization
//!
//! ## Entry Point
//!
//! The main entry point is [`entry`], which initializes logging, parses arguments, and dispatches
//! to the appropriate subcommand handler.
//!
//! ## Example
//!
//! ```no_run
//! use amg::cli::entry;
//!
//! fn main() -> std::process::ExitCode {
//!     entry()
//! }
//! ```

mod args;
mod codex_cmd;
mod constants;
mod logging;
mod prelude;
mod process;
mod scan;
mod util;

// Re-export Args and Commands for testing
pub use args::{Args, Commands};

use prelude::*;

/// Main entry point for the CLI application.
///
/// Initializes logging, parses command-line arguments, and executes the appropriate subcommand.
/// Returns an [`ExitCode`] indicating success or failure.
///
/// # Returns
///
/// * [`ExitCode::SUCCESS`] - Command executed successfully
/// * [`ExitCode::FAILURE`] - Command failed (errors are logged to stderr)
///
/// # Examples
///
/// ```no_run
/// use amg::cli::entry;
///
/// fn main() -> std::process::ExitCode {
///     entry()
/// }
/// ```
pub fn entry() -> ExitCode {
    logging::init_tracing();
    match run() {
        Ok(code) => code,
        Err(err) => {
            error!("{err:#}");
            ExitCode::FAILURE
        }
    }
}

/// Internal function that runs the CLI logic.
///
/// Parses arguments and dispatches to the appropriate subcommand handler.
///
/// # Returns
///
/// Returns [`Result<ExitCode>`] indicating success or failure.
///
/// # Errors
///
/// Returns an error if:
/// * Argument parsing fails
/// * Subcommand execution fails
fn run() -> Result<ExitCode> {
    let args = args::parse_args();

    match args.command {
        args::Commands::ResumeBranch {
            branch,
            repo,
            codexdir,
            dry_run,
            no_tmux,
        } => run_resume_branch(branch, repo, codexdir, dry_run, no_tmux),
    }
}

/// Handles the `resume-branch` subcommand.
///
/// Searches for a matching Codex session and either prints the command (dry-run) or executes it.
///
/// # Arguments
///
/// * `branch` - Git branch name to match against session files
/// * `repo` - Repository path to grant Codex sandbox access to
/// * `codexdir` - Optional Codex directory path (defaults to `$HOME/.codex`)
/// * `dry_run` - If `true`, print the command without executing it
/// * `no_tmux` - If `true`, disable automatic tmux window creation
///
/// # Returns
///
/// Returns [`Result<ExitCode>`] indicating success or failure.
///
/// # Errors
///
/// Returns an error if:
/// * The repository or codexdir is not a valid directory
/// * No matching session is found for the branch
/// * Session directory validation fails
/// * Command execution fails
///
/// # See Also
///
/// * [`scan::find_first_session`] - Session matching logic
/// * [`codex_cmd::build_codex_cmd`] - Command building
/// * [`process::run_tmux_new_window`] - Tmux execution
/// * [`process::run_in_dir`] - Inline execution
fn run_resume_branch(
    branch: String,
    repo: PathBuf,
    codexdir: Option<PathBuf>,
    dry_run: bool,
    no_tmux: bool,
) -> Result<ExitCode> {
    let codexdir = codexdir.map(Ok).unwrap_or_else(util::default_codexdir)?;

    util::require_dir(&repo, "repo", Some("CODEX_REPO"))?;
    util::require_dir(&codexdir, "codexdir", Some("CODEX_CODEXDIR"))?;

    let session = scan::find_first_session(&codexdir, &branch)?.with_context(|| {
        format!(
            "No matching session found for branch {:?} under {}",
            branch,
            codexdir.display()
        )
    })?;
    util::require_dir(&session.cwd, "session cwd", None)?;

    let cmd = codex_cmd::build_codex_cmd(&repo, &codexdir, &session, util::home_dir().as_deref());

    info!(
        id = %session.id,
        cwd = %session.cwd.display(),
        source_jsonl = %session.source_jsonl.display(),
        "matched session"
    );

    enum Action {
        Print(process::Cmd),
        RunTmux(process::Cmd),
        RunInline(process::Cmd),
    }

    let use_tmux = util::should_use_tmux(no_tmux);
    let action = match (dry_run, use_tmux) {
        (true, true) => Action::Print(process::tmux_new_window_cmd(&session.cwd, &cmd)),
        (true, false) => Action::Print(cmd),
        (false, true) => Action::RunTmux(cmd),
        (false, false) => Action::RunInline(cmd),
    };

    match action {
        Action::Print(cmd) => {
            let command = cmd.as_shell_string();
            info!(command = %command, "dry-run");
            println!("{command}");
            Ok(ExitCode::SUCCESS)
        }
        Action::RunTmux(cmd) => {
            debug!("running via tmux new-window");
            process::run_tmux_new_window(&session.cwd, &cmd)?;
            Ok(ExitCode::SUCCESS)
        }
        Action::RunInline(cmd) => {
            debug!("running inline");
            process::run_in_dir(&session.cwd, &cmd).context("failed to run codex")
        }
    }
}
