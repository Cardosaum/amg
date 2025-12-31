use clap::{Parser, Subcommand};

use super::prelude::*;

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
    #[command(visible_alias = "rb")]
    #[command(visible_alias = "resume")]
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
    use rstest::rstest;
    use std::path::PathBuf;

    fn parse_args_from<I, T>(args: I) -> Args
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        Args::parse_from(args)
    }

    #[rstest]
    #[case("resume-branch")]
    #[case("rb")]
    #[case("resume")]
    fn test_subcommand_aliases(#[case] subcommand: &str) {
        let args = parse_args_from([
            "codex_resume_branch",
            subcommand,
            "test-branch",
            "--repo",
            "/tmp/repo",
        ]);
        match args.command {
            Commands::ResumeBranch { branch, .. } => {
                assert_eq!(branch, "test-branch");
            }
        }
    }

    #[rstest]
    #[case("main")]
    #[case("feature-branch")]
    #[case("dev")]
    #[case("test/branch")]
    fn test_branch_names(#[case] branch_name: &str) {
        let args = parse_args_from([
            "codex_resume_branch",
            "resume-branch",
            branch_name,
            "--repo",
            "/tmp/repo",
        ]);
        match args.command {
            Commands::ResumeBranch { branch, .. } => {
                assert_eq!(branch, branch_name);
            }
        }
    }

    #[rstest]
    #[case("/tmp/repo")]
    #[case("/home/user/project")]
    #[case("/var/tmp/test-repo")]
    fn test_repo_paths(#[case] repo_path: &str) {
        let args = parse_args_from([
            "codex_resume_branch",
            "resume-branch",
            "main",
            "--repo",
            repo_path,
        ]);
        match args.command {
            Commands::ResumeBranch { repo, .. } => {
                assert_eq!(repo, PathBuf::from(repo_path));
            }
        }
    }

    #[rstest]
    #[case(None)]
    #[case(Some("/tmp/.codex"))]
    #[case(Some("/home/user/.codex"))]
    fn test_codexdir_option(#[case] codexdir: Option<&str>) {
        let mut cmd_args = vec![
            "codex_resume_branch",
            "resume-branch",
            "main",
            "--repo",
            "/tmp/repo",
        ];
        if let Some(dir) = codexdir {
            cmd_args.push("--codexdir");
            cmd_args.push(dir);
        }

        let args = parse_args_from(cmd_args);
        match args.command {
            Commands::ResumeBranch {
                codexdir: result, ..
            } => {
                assert_eq!(result, codexdir.map(PathBuf::from));
            }
        }
    }

    #[rstest]
    #[case("--dry-run", true, false)]
    #[case("-n", true, false)]
    #[case("--no-tmux", false, true)]
    #[case("--dry-run", true, false)]
    fn test_flags(
        #[case] flag: &str,
        #[case] expected_dry_run: bool,
        #[case] expected_no_tmux: bool,
    ) {
        let args = parse_args_from([
            "codex_resume_branch",
            "resume-branch",
            "main",
            "--repo",
            "/tmp/repo",
            flag,
        ]);
        match args.command {
            Commands::ResumeBranch {
                dry_run, no_tmux, ..
            } => {
                assert_eq!(dry_run, expected_dry_run);
                assert_eq!(no_tmux, expected_no_tmux);
            }
        }
    }

    #[rstest]
    #[case("resume-branch", "main", "/tmp/repo", None, false, false)]
    #[case("rb", "feature", "/home/repo", Some("/tmp/.codex"), true, false)]
    #[case("resume", "dev", "/var/repo", None, false, true)]
    #[case("resume-branch", "test", "/tmp/repo", Some("/home/.codex"), true, true)]
    fn test_all_options_combinations(
        #[case] subcommand: &str,
        #[case] branch: &str,
        #[case] repo: &str,
        #[case] codexdir: Option<&str>,
        #[case] dry_run: bool,
        #[case] no_tmux: bool,
    ) {
        let mut cmd_args = vec!["codex_resume_branch", subcommand, branch, "--repo", repo];

        if let Some(dir) = codexdir {
            cmd_args.push("--codexdir");
            cmd_args.push(dir);
        }

        if dry_run {
            cmd_args.push("--dry-run");
        }

        if no_tmux {
            cmd_args.push("--no-tmux");
        }

        let args = parse_args_from(cmd_args);
        match args.command {
            Commands::ResumeBranch {
                branch: result_branch,
                repo: result_repo,
                codexdir: result_codexdir,
                dry_run: result_dry_run,
                no_tmux: result_no_tmux,
            } => {
                assert_eq!(result_branch, branch);
                assert_eq!(result_repo, PathBuf::from(repo));
                assert_eq!(result_codexdir, codexdir.map(PathBuf::from));
                assert_eq!(result_dry_run, dry_run);
                assert_eq!(result_no_tmux, no_tmux);
            }
        }
    }
}
