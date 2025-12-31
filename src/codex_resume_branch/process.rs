use super::prelude::*;

#[derive(Debug, Clone)]
pub(super) struct Cmd {
    pub(super) program: OsString,
    pub(super) args: Vec<OsString>,
}

impl Cmd {
    pub(super) fn as_shell_string(&self) -> String {
        std::iter::once(self.program.as_os_str())
            .chain(self.args.iter().map(OsString::as_os_str))
            .map(sh_quote_lossy)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

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

fn exit_code(status: ExitStatus) -> ExitCode {
    match status.code().and_then(|c| u8::try_from(c).ok()) {
        Some(code) => ExitCode::from(code),
        None => ExitCode::FAILURE,
    }
}

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
