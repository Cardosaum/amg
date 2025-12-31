//! Codex command building and configuration.
//!
//! This module constructs Codex commands with appropriate sandbox configuration, including
//! directory access, git repository access, and session resumption.

use super::prelude::*;
use super::process::Cmd;
use super::scan::Session;

/// Builds a Codex command for resuming a session.
///
/// Constructs a command with all necessary flags and arguments for resuming a Codex session,
/// including sandbox configuration, directory access, and session identification.
///
/// # Arguments
///
/// * `repo` - Repository path to grant Codex sandbox access to
/// * `codexdir` - Codex directory containing session files
/// * `session` - The session to resume
/// * `home` - Optional home directory path for adding home-based sandbox directories
///
/// # Returns
///
/// Returns a [`Cmd`] ready to be executed or printed.
///
/// # See Also
///
/// * [`Cmd`] - Command structure
/// * [`Session`] - Session information
pub(super) fn build_codex_cmd(
    repo: &Path,
    codexdir: &Path,
    session: &Session,
    home: Option<&Path>,
) -> Cmd {
    let mut args: Vec<OsString> = [
        "--search",
        "-a",
        "on-failure",
        "-s",
        "workspace-write",
        "--config",
        "model=gpt-5.2-codex",
        "--config",
        "model_reasoning_effort=high",
        "--config",
        "sandbox_workspace_write.network_access=true",
    ]
    .into_iter()
    .map(Into::into)
    .collect();

    // Required adds.
    add_dir(&mut args, repo);
    add_git_dir(&mut args, repo);
    add_dir(&mut args, codexdir);
    add_dir(&mut args, &session.cwd);

    args.extend(["--cd".into(), session.cwd.as_os_str().to_owned()]);

    // Optional adds.
    add_git_dir(&mut args, &session.cwd);
    add_dir_if_dir(&mut args, session.cwd.join(DOT_CODEX_DIR));

    home.into_iter()
        .flat_map(|home| HOME_SANDBOX_DIRS.iter().map(move |rel| home.join(rel)))
        .chain(EXTRA_SANDBOX_DIRS.iter().map(|abs| PathBuf::from(*abs)))
        .for_each(|dir| add_dir_if_dir(&mut args, dir));

    args.extend(["resume".into(), session.id.clone().into()]);

    Cmd {
        program: "codex".into(),
        args,
    }
}

/// Adds a directory to the command arguments.
///
/// Appends `--add-dir` and the directory path to the arguments vector.
///
/// # Arguments
///
/// * `args` - The arguments vector to append to
/// * `dir` - The directory path to add
fn add_dir(args: &mut Vec<OsString>, dir: &Path) {
    args.extend(["--add-dir".into(), dir.as_os_str().to_owned()]);
}

/// Adds a directory to the command arguments if it exists.
///
/// Only adds the directory if it exists and is actually a directory.
///
/// # Arguments
///
/// * `args` - The arguments vector to append to
/// * `dir` - The directory path to add (if it exists)
fn add_dir_if_dir(args: &mut Vec<OsString>, dir: PathBuf) {
    if dir.is_dir() {
        add_dir(args, &dir);
    }
}

/// Adds git directory access for a worktree.
///
/// Resolves the git directory for the given worktree and adds it to the command arguments.
/// Handles both regular git repositories and git worktrees.
///
/// # Arguments
///
/// * `args` - The arguments vector to append to
/// * `worktree` - The git worktree path
///
/// # See Also
///
/// * [`git_dir_for_worktree`] - Git directory resolution logic
fn add_git_dir(args: &mut Vec<OsString>, worktree: &Path) {
    git_dir_for_worktree(worktree)
        .into_iter()
        .for_each(|gitdir| add_dir(args, &gitdir));
}

/// Resolves the git directory for a worktree.
///
/// Handles two cases:
/// * If `<worktree>/.git` is a directory: returns it directly
/// * If it's a file (worktree/linked checkout): parses the `gitdir:` line and returns the target
///
/// # Arguments
///
/// * `worktree` - The git worktree path
///
/// # Returns
///
/// Returns [`Option<PathBuf>`] containing the git directory path, or `None` if it cannot be resolved.
fn git_dir_for_worktree(worktree: &Path) -> Option<PathBuf> {
    let dot_git = worktree.join(DOT_GIT);
    let meta = fs::symlink_metadata(&dot_git).ok()?;

    match (meta.is_dir(), meta.is_file()) {
        (true, _) => Some(dot_git),
        (_, true) => git_dir_from_gitfile(worktree, &dot_git),
        _ => None,
    }
}

/// Extracts the git directory path from a `.git` file (gitfile).
///
/// Parses the `gitdir:` line from a gitfile and resolves the path, handling both relative
/// and absolute paths.
///
/// # Arguments
///
/// * `worktree` - The worktree path (for resolving relative paths)
/// * `dot_git` - The path to the `.git` file
///
/// # Returns
///
/// Returns [`Option<PathBuf>`] containing the resolved git directory path, or `None` if:
/// * The file cannot be read
/// * The file doesn't contain a valid `gitdir:` line
/// * The resolved path doesn't exist or isn't a directory
///
/// # See Also
///
/// * [`git_dir_for_worktree`] - Main git directory resolution function
fn git_dir_from_gitfile(worktree: &Path, dot_git: &Path) -> Option<PathBuf> {
    let content = fs::read_to_string(dot_git).ok()?;
    let gitdir = content
        .lines()
        .next()?
        .trim()
        .strip_prefix("gitdir:")?
        .trim();
    if gitdir.is_empty() {
        return None;
    }

    let p = PathBuf::from(gitdir);
    let p = if p.is_relative() { worktree.join(p) } else { p };
    p.is_dir().then_some(p)
}
