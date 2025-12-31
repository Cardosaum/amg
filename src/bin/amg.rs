//! Binary entry point for the `amg` command-line tool.
//!
//! This is the main entry point when running `amg` as a binary. It delegates to the
//! library's [`cli::entry`] function.

/// Main entry point for the amg binary.
///
/// Delegates to [`amg::cli::entry`] to handle all CLI logic.
fn main() -> std::process::ExitCode {
    amg::cli::entry()
}
