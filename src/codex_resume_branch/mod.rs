mod cli;
mod codex_cmd;
mod constants;
mod logging;
mod prelude;
mod process;
mod scan;
mod util;

use prelude::*;

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

fn run() -> Result<ExitCode> {
    let args = cli::parse_args();

    match args.command {
        cli::Commands::ResumeBranch {
            branch,
            repo,
            codexdir,
            dry_run,
            no_tmux,
        } => run_resume_branch(branch, repo, codexdir, dry_run, no_tmux),
    }
}

fn run_resume_branch(
    branch: String,
    repo: PathBuf,
    codexdir: Option<PathBuf>,
    dry_run: bool,
    no_tmux: bool,
) -> Result<ExitCode> {
    let codexdir = codexdir
        .map(Ok)
        .unwrap_or_else(util::default_codexdir)?;

    util::require_dir(&repo, "repo (CODEX_REPO)")?;
    util::require_dir(&codexdir, "codexdir (CODEX_CODEXDIR)")?;

    let session = scan::find_first_session(&codexdir, &branch)?.with_context(|| {
        format!(
            "No matching session found for branch {:?} under {}",
            branch,
            codexdir.display()
        )
    })?;
    util::require_dir(&session.cwd, "session cwd")?;

    let cmd =
        codex_cmd::build_codex_cmd(&repo, &codexdir, &session, util::home_dir().as_deref());

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
