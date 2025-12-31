use super::prelude::*;
use super::process::Cmd;
use super::scan::Session;

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

fn add_dir(args: &mut Vec<OsString>, dir: &Path) {
    args.extend(["--add-dir".into(), dir.as_os_str().to_owned()]);
}

fn add_dir_if_dir(args: &mut Vec<OsString>, dir: PathBuf) {
    if dir.is_dir() {
        add_dir(args, &dir);
    }
}

fn add_git_dir(args: &mut Vec<OsString>, worktree: &Path) {
    git_dir_for_worktree(worktree)
        .into_iter()
        .for_each(|gitdir| add_dir(args, &gitdir));
}

/// Resolve the git directory for `worktree`.
///
/// - If `<worktree>/.git` is a directory: return it.
/// - If it's a file (worktree/linked checkout): parse `gitdir: ...` and return the target dir.
fn git_dir_for_worktree(worktree: &Path) -> Option<PathBuf> {
    let dot_git = worktree.join(DOT_GIT);
    let meta = fs::symlink_metadata(&dot_git).ok()?;

    match (meta.is_dir(), meta.is_file()) {
        (true, _) => Some(dot_git),
        (_, true) => git_dir_from_gitfile(worktree, &dot_git),
        _ => None,
    }
}

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
