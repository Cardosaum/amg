use super::prelude::*;

pub(super) fn default_codexdir() -> Result<PathBuf> {
    match std::env::var_os(ENV_HOME) {
        Some(home) if !home.is_empty() => Ok(PathBuf::from(home).join(DOT_CODEX_DIR)),
        _ => bail!("CODEX_CODEXDIR is not set and $HOME is empty; please set CODEX_CODEXDIR"),
    }
}

pub(super) fn require_dir(path: &Path, label: &'static str) -> Result<()> {
    if path.is_dir() {
        Ok(())
    } else {
        bail!("{label} is not a directory: {}", path.display());
    }
}

pub(super) fn home_dir() -> Option<PathBuf> {
    std::env::var_os(ENV_HOME)
        .filter(|h| !h.is_empty())
        .map(PathBuf::from)
}

pub(super) fn should_use_tmux(no_tmux: bool) -> bool {
    !no_tmux && env_present(ENV_TMUX)
}

fn env_present(name: &str) -> bool {
    std::env::var_os(name).is_some_and(|v| !v.is_empty())
}
