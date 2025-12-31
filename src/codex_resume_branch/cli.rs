use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Codex session management tool
#[derive(Parser, Debug)]
#[command(name = "codex_resume_branch")]
#[command(about = "Manage and resume Codex sessions")]
pub(super) struct Args {
    #[command(subcommand)]
    pub(super) command: Commands,
}

#[derive(Subcommand, Debug)]
pub(super) enum Commands {
    /// Resume the first Codex session whose first JSONL line has `.payload.git.branch == <branch>`.
    ///
    /// Usage:
    ///     codex_resume_branch resume-branch <branch>
    ///
    /// Required environment variables:
    ///     CODEX_REPO=/path/to/repo
    ///
    /// Optional environment variables:
    ///     CODEX_CODEXDIR=/path/to/.codex   (defaults to $HOME/.codex)
    #[command(alias = "rb")]
    #[command(alias = "resume")]
    ResumeBranch {
        /// Git branch to resume (matches `.payload.git.branch` in the first JSONL line).
        branch: String,

        /// Repo to grant Codex sandbox access to.
        #[arg(long, env = "CODEX_REPO")]
        repo: PathBuf,

        /// Codex directory containing JSONL sessions (defaults to `$HOME/.codex`).
        #[arg(long, env = "CODEX_CODEXDIR")]
        codexdir: Option<PathBuf>,

        /// Print the exact command that would be executed and exit without running.
        /// (If `$TMUX` is set and `--no-tmux` is not, this prints the `tmux new-window ...` command.)
        #[arg(long, short = 'n')]
        dry_run: bool,

        /// If `$TMUX` is set, do NOT open a new tmux window; run inline instead.
        #[arg(long)]
        no_tmux: bool,
    },
}

pub(super) fn parse_args() -> Args {
    Args::parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn parse_args_from<I, T>(args: I) -> Args
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        Args::parse_from(args)
    }

    #[test]
    fn test_resume_branch_full_name() {
        let args = parse_args_from([
            "codex_resume_branch",
            "resume-branch",
            "main",
            "--repo",
            "/tmp/repo",
        ]);
        match args.command {
            Commands::ResumeBranch {
                branch,
                repo,
                codexdir,
                dry_run,
                no_tmux,
            } => {
                assert_eq!(branch, "main");
                assert_eq!(repo, PathBuf::from("/tmp/repo"));
                assert_eq!(codexdir, None);
                assert!(!dry_run);
                assert!(!no_tmux);
            }
        }
    }

    #[test]
    fn test_resume_branch_alias_rb() {
        let args = parse_args_from([
            "codex_resume_branch",
            "rb",
            "feature-branch",
            "--repo",
            "/tmp/repo",
        ]);
        match args.command {
            Commands::ResumeBranch { branch, .. } => {
                assert_eq!(branch, "feature-branch");
            }
        }
    }

    #[test]
    fn test_resume_branch_alias_resume() {
        let args = parse_args_from([
            "codex_resume_branch",
            "resume",
            "dev",
            "--repo",
            "/tmp/repo",
        ]);
        match args.command {
            Commands::ResumeBranch { branch, .. } => {
                assert_eq!(branch, "dev");
            }
        }
    }

    #[test]
    fn test_resume_branch_with_codexdir() {
        let args = parse_args_from([
            "codex_resume_branch",
            "resume-branch",
            "main",
            "--repo",
            "/tmp/repo",
            "--codexdir",
            "/tmp/.codex",
        ]);
        match args.command {
            Commands::ResumeBranch { codexdir, .. } => {
                assert_eq!(codexdir, Some(PathBuf::from("/tmp/.codex")));
            }
        }
    }

    #[test]
    fn test_resume_branch_dry_run() {
        let args = parse_args_from([
            "codex_resume_branch",
            "resume-branch",
            "main",
            "--repo",
            "/tmp/repo",
            "--dry-run",
        ]);
        match args.command {
            Commands::ResumeBranch { dry_run, .. } => {
                assert!(dry_run);
            }
        }
    }

    #[test]
    fn test_resume_branch_dry_run_short() {
        let args = parse_args_from([
            "codex_resume_branch",
            "resume-branch",
            "main",
            "--repo",
            "/tmp/repo",
            "-n",
        ]);
        match args.command {
            Commands::ResumeBranch { dry_run, .. } => {
                assert!(dry_run);
            }
        }
    }

    #[test]
    fn test_resume_branch_no_tmux() {
        let args = parse_args_from([
            "codex_resume_branch",
            "resume-branch",
            "main",
            "--repo",
            "/tmp/repo",
            "--no-tmux",
        ]);
        match args.command {
            Commands::ResumeBranch { no_tmux, .. } => {
                assert!(no_tmux);
            }
        }
    }

    #[test]
    fn test_resume_branch_all_options() {
        let args = parse_args_from([
            "codex_resume_branch",
            "rb",
            "test-branch",
            "--repo",
            "/tmp/repo",
            "--codexdir",
            "/tmp/.codex",
            "--dry-run",
            "--no-tmux",
        ]);
        match args.command {
            Commands::ResumeBranch {
                branch,
                repo,
                codexdir,
                dry_run,
                no_tmux,
            } => {
                assert_eq!(branch, "test-branch");
                assert_eq!(repo, PathBuf::from("/tmp/repo"));
                assert_eq!(codexdir, Some(PathBuf::from("/tmp/.codex")));
                assert!(dry_run);
                assert!(no_tmux);
            }
        }
    }
}
