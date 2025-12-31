// -----------------------------
// Constants (keep paths out of logic)
// -----------------------------
pub(super) const DOT_CODEX_DIR: &str = ".codex";
pub(super) const DOT_GIT: &str = ".git";

pub(super) const ENV_HOME: &str = "HOME";
pub(super) const ENV_TMUX: &str = "TMUX";

pub(super) const HOME_SANDBOX_DIRS: [&str; 4] = [
    ".cargo",
    ".rustup",
    "Library/Caches/Mozilla.sccache",
    ".npm",
];

pub(super) const EXTRA_SANDBOX_DIRS: [&str; 2] = ["/tmp", "/var/folders"];
