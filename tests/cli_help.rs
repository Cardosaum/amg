/// Integration tests to ensure CLI flags and commands have proper help text strings.
///
/// These tests verify that:
/// - All commands and flags have non-empty help text
/// - Help text content matches expected strings
/// - Help output is well-formed and contains expected sections
use amg::cli::Args;
use clap::{Arg, Command, CommandFactory};

mod helpers {
    use super::*;

    /// Get the resume subcommand from the main command.
    pub fn get_resume_branch_cmd() -> Command {
        Args::command()
            .get_subcommands()
            .find(|sub| sub.get_name() == "resume")
            .expect("Should have 'resume' subcommand")
            .clone()
    }

    /// Get an argument by its ID from a command.
    /// Checks both positional and non-positional arguments.
    pub fn get_argument_by_id(cmd: &Command, id: &str) -> Arg {
        cmd.get_arguments()
            .find(|arg| arg.get_id().as_str() == id)
            .or_else(|| {
                cmd.get_positionals()
                    .find(|arg| arg.get_id().as_str() == id)
            })
            .unwrap_or_else(|| panic!("Should have '{id}' argument"))
            .clone()
    }

    /// Assert that help text exists and is non-empty.
    pub fn assert_help_text_exists(help: Option<&clap::builder::StyledStr>, name: &str) -> String {
        let help_text = help
            .unwrap_or_else(|| panic!("{name} should have help text"))
            .to_string();
        assert!(
            !help_text.is_empty(),
            "{name} help text should not be empty"
        );
        help_text
    }

    /// Assert that help text contains expected strings.
    pub fn assert_help_text_contains(help_text: &str, expected: &str, name: &str) {
        assert!(
            help_text.contains(expected),
            "{name} help text should contain '{expected}', got: {help_text}",
        );
    }

    /// Assert that a flag has the expected short form.
    pub fn assert_flag_has_short(arg: &Arg, expected: char, name: &str) {
        assert_eq!(
            arg.get_short(),
            Some(expected),
            "{name} flag should have short flag '{expected}'"
        );
    }

    /// Assert that a flag has the expected long form.
    pub fn assert_flag_has_long(arg: &Arg, expected: &str, name: &str) {
        assert_eq!(
            arg.get_long(),
            Some(expected),
            "{name} flag should have long flag '{expected}'"
        );
    }
}

mod constants {
    // Subcommand names
    pub const RESUME_BRANCH: &str = "resume";

    // Argument IDs
    pub const ARG_BRANCH: &str = "branch";
    pub const ARG_REPO: &str = "repo";
    pub const ARG_CODEXDIR: &str = "codexdir";
    pub const ARG_DRY_RUN: &str = "dry_run";
    pub const ARG_NO_TMUX: &str = "no_tmux";

    // Expected help text snippets
    pub const ABOUT_MAIN: &str = "Manage and resume Codex sessions";
    pub const ABOUT_RESUME_BRANCH: &str = "Resume the first Codex session";
    pub const ABOUT_PAYLOAD_BRANCH: &str = "payload.git.branch";
    pub const HELP_BRANCH: &str = "Git branch to resume";
    pub const HELP_REPO: &str = "Repo to grant Codex sandbox access";
    pub const HELP_CODEXDIR: &str = "Codex directory";
    pub const HELP_JSONL: &str = "JSONL sessions";
    pub const HELP_DRY_RUN: &str = "Print the exact command";
    pub const HELP_NO_TMUX: &str = "TMUX";
    pub const HELP_NO_TMUX_ACTION: &str = "do NOT open";

    // Flag forms
    pub const SHORT_REPO: char = 'r';
    pub const SHORT_DRY_RUN: char = 'n';
    pub const LONG_REPO: &str = "repo";
    pub const LONG_CODEXDIR: &str = "codexdir";
    pub const LONG_DRY_RUN: &str = "dry-run";
    pub const LONG_NO_TMUX: &str = "no-tmux";

    // Aliases
    pub const ALIAS_RB: &str = "rb";
}

use constants::*;
use helpers::*;

#[cfg(test)]
mod main_command {
    use super::*;

    #[test]
    fn has_about_text() {
        let cmd = Args::command();
        let about = cmd.get_about();
        let about_text = assert_help_text_exists(about, "Main command");
        assert_help_text_contains(&about_text, ABOUT_MAIN, "Main command");
    }
}

#[cfg(test)]
mod subcommands {
    use super::*;

    #[test]
    fn resume_branch_has_help_text() {
        let cmd = get_resume_branch_cmd();
        let about = cmd.get_about();
        let about_text = assert_help_text_exists(about, "resume subcommand");
        assert_help_text_contains(&about_text, ABOUT_RESUME_BRANCH, "resume");
        assert_help_text_contains(&about_text, ABOUT_PAYLOAD_BRANCH, "resume");
    }

    #[test]
    fn resume_branch_has_aliases() {
        let cmd = get_resume_branch_cmd();
        let aliases: Vec<_> = cmd.get_visible_aliases().collect();
        assert!(
            aliases.contains(&ALIAS_RB),
            "resume should have '{ALIAS_RB}' alias"
        );
        assert_eq!(
            aliases.len(),
            1,
            "resume should have exactly one alias '{ALIAS_RB}'"
        );
    }
}

#[cfg(test)]
mod arguments {
    use super::*;

    #[test]
    fn branch_has_help_text() {
        let cmd = get_resume_branch_cmd();
        let arg = get_argument_by_id(&cmd, ARG_BRANCH);
        let help_text = assert_help_text_exists(arg.get_help(), "branch argument");
        assert_help_text_contains(&help_text, HELP_BRANCH, "branch");
        assert_help_text_contains(&help_text, ABOUT_PAYLOAD_BRANCH, "branch");
    }

    #[test]
    fn repo_has_help_text_and_flags() {
        let cmd = get_resume_branch_cmd();
        let arg = get_argument_by_id(&cmd, ARG_REPO);
        let help_text = assert_help_text_exists(arg.get_help(), "repo flag");
        assert_help_text_contains(&help_text, HELP_REPO, "repo");
        assert_flag_has_short(&arg, SHORT_REPO, "repo");
        assert_flag_has_long(&arg, LONG_REPO, "repo");
    }

    #[test]
    fn codexdir_has_help_text_and_flag() {
        let cmd = get_resume_branch_cmd();
        let arg = get_argument_by_id(&cmd, ARG_CODEXDIR);
        let help_text = assert_help_text_exists(arg.get_help(), "codexdir flag");
        assert_help_text_contains(&help_text, HELP_CODEXDIR, "codexdir");
        assert_help_text_contains(&help_text, HELP_JSONL, "codexdir");
        assert_flag_has_long(&arg, LONG_CODEXDIR, "codexdir");
    }

    #[test]
    fn dry_run_has_help_text_and_flags() {
        let cmd = get_resume_branch_cmd();
        let arg = get_argument_by_id(&cmd, ARG_DRY_RUN);
        let help_text = assert_help_text_exists(arg.get_help(), "dry-run flag");
        assert_help_text_contains(&help_text, HELP_DRY_RUN, "dry-run");
        assert_flag_has_short(&arg, SHORT_DRY_RUN, "dry-run");
        assert_flag_has_long(&arg, LONG_DRY_RUN, "dry-run");
    }

    #[test]
    fn no_tmux_has_help_text_and_flag() {
        let cmd = get_resume_branch_cmd();
        let arg = get_argument_by_id(&cmd, ARG_NO_TMUX);
        let help_text = assert_help_text_exists(arg.get_help(), "no-tmux flag");
        assert_help_text_contains(&help_text, HELP_NO_TMUX, "no-tmux");
        assert_help_text_contains(&help_text, HELP_NO_TMUX_ACTION, "no-tmux");
        assert_flag_has_long(&arg, LONG_NO_TMUX, "no-tmux");
    }
}

#[cfg(test)]
mod help_output {
    use super::*;
    use std::process::Command;

    fn run_help_command(args: &[&str]) -> String {
        let mut cmd_args = vec!["run", "--bin", "amg", "--quiet", "--"];
        cmd_args.extend_from_slice(args);
        cmd_args.push("--help");

        let output = Command::new("cargo")
            .args(cmd_args)
            .output()
            .expect("Failed to execute cargo run");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!(
                "Help command failed with status {:?}. Stderr: {}",
                output.status, stderr
            );
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        // In case cargo outputs compilation messages, also check stderr
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        // Combine stdout and stderr, but prefer stdout (help text should be there)
        if stdout.trim().is_empty() && !stderr.trim().is_empty() {
            stderr
        } else {
            stdout
        }
    }

    #[test]
    fn main_help_is_well_formed() {
        let output = run_help_command(&[]);
        assert!(
            output.contains(ABOUT_MAIN),
            "Help output should contain main command description '{}'. Got output:\n{}",
            ABOUT_MAIN,
            output
        );
        assert!(
            output.contains(RESUME_BRANCH),
            "Help output should mention '{RESUME_BRANCH}' subcommand. Got output:\n{}",
            output
        );
        assert!(
            output.contains("USAGE:") || output.contains("Usage:"),
            "Help output should contain usage section. Got output:\n{}",
            output
        );
    }

    #[test]
    fn resume_branch_help_is_well_formed() {
        let output = run_help_command(&[RESUME_BRANCH]);
        assert!(
            output.contains(ABOUT_RESUME_BRANCH),
            "resume help output should contain description '{}'. Got output:\n{}",
            ABOUT_RESUME_BRANCH,
            output
        );
        assert!(
            output.contains("--repo") || output.contains("-r"),
            "resume help output should mention repo flag. Got output:\n{}",
            output
        );
        assert!(
            output.contains("--codexdir"),
            "resume help output should mention codexdir flag. Got output:\n{}",
            output
        );
        assert!(
            output.contains("--dry-run") || output.contains("-n"),
            "resume help output should mention dry-run flag. Got output:\n{}",
            output
        );
        assert!(
            output.contains("--no-tmux"),
            "resume help output should mention no-tmux flag. Got output:\n{}",
            output
        );
        assert!(
            output.contains("USAGE:") || output.contains("Usage:"),
            "resume help output should contain usage section. Got output:\n{}",
            output
        );
    }
}
