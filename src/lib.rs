//! # amg - Resume Codex sessions by git branch
//!
//! A command-line tool to manage and resume Codex sessions by matching them to git branch names.
//!
//! ## Overview
//!
//! `amg` helps you quickly resume Codex sessions by searching through Codex session files (JSONL format)
//! and finding the first session that matches your current or specified git branch, then resumes it
//! with the appropriate configuration.
//!
//! ## Usage
//!
//! The main entry point is [`cli::entry`], which parses command-line arguments and executes the
//! appropriate subcommand.
//!
//! ## Example
//!
//! ```no_run
//! use amg::cli::entry;
//!
//! fn main() -> std::process::ExitCode {
//!     entry()
//! }
//! ```
//!
//! ## Modules
//!
//! * [`cli`] - Command-line interface implementation
//!
//! ## See Also
//!
//! * [Repository](https://github.com/Cardosaum/amg)
//! * [README](https://github.com/Cardosaum/amg/blob/main/README.md)

#![warn(missing_docs)]

pub mod cli;
