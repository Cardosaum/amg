use std::path::PathBuf;

use clap::Parser;

/// Resume the first Codex session whose first JSONL line has `.payload.git.branch == <branch>`.
///
/// Usage:
///     codex_resume_branch <branch>
///
/// Required environment variables:
///     CODEX_REPO=/path/to/repo
///
/// Optional environment variables:
///     CODEX_CODEXDIR=/path/to/.codex   (defaults to $HOME/.codex)
#[derive(Parser, Debug)]
#[command(name = "codex_resume_branch")]
pub(super) struct Args {
    /// Git branch to resume (matches `.payload.git.branch` in the first JSONL line).
    pub(super) branch: String,

    /// Repo to grant Codex sandbox access to.
    #[arg(long, env = "CODEX_REPO")]
    pub(super) repo: PathBuf,

    /// Codex directory containing JSONL sessions (defaults to `$HOME/.codex`).
    #[arg(long, env = "CODEX_CODEXDIR")]
    pub(super) codexdir: Option<PathBuf>,

    /// Print the exact command that would be executed and exit without running.
    /// (If `$TMUX` is set and `--no-tmux` is not, this prints the `tmux new-window ...` command.)
    #[arg(long, short = 'n')]
    pub(super) dry_run: bool,

    /// If `$TMUX` is set, do NOT open a new tmux window; run inline instead.
    #[arg(long)]
    pub(super) no_tmux: bool,
}

pub(super) fn parse_args() -> Args {
    Args::parse()
}
