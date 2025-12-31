//! Utility functions for paths, environment variables, and common operations.
//!
//! This module provides helper functions for:
//! * Path resolution and validation
//! * Environment variable access
//! * Tmux detection
//! * Home directory resolution

use super::prelude::*;

/// Gets the default Codex directory path.
///
/// Returns `$HOME/.codex` if `$HOME` is set and non-empty.
///
/// # Returns
///
/// Returns [`Result<PathBuf>`] containing the default codex directory path.
///
/// # Errors
///
/// Returns an error if `$HOME` is not set or empty.
pub(super) fn default_codexdir() -> Result<PathBuf> {
    match std::env::var_os(ENV_HOME) {
        Some(home) if !home.is_empty() => Ok(PathBuf::from(home).join(DOT_CODEX_DIR)),
        _ => bail!("CODEX_CODEXDIR is not set and $HOME is empty; please set CODEX_CODEXDIR"),
    }
}

/// Validates that a path exists and is a directory.
///
/// # Arguments
///
/// * `path` - The path to validate
/// * `label` - A human-readable label for error messages (e.g., "repo", "codexdir")
/// * `env_var` - Optional environment variable name that provided this path (e.g., "CODEX_REPO")
///
/// # Returns
///
/// Returns [`Result<()>`] if the path is a valid directory.
///
/// # Errors
///
/// Returns an error if the path does not exist or is not a directory.
pub(super) fn require_dir(
    path: &Path,
    label: &'static str,
    env_var: Option<&'static str>,
) -> Result<()> {
    if path.is_dir() {
        Ok(())
    } else {
        let msg = match env_var {
            Some(var) => format!(
                "{label} (from {var}) is not a directory: {}",
                path.display()
            ),
            None => format!("{label} is not a directory: {}", path.display()),
        };
        bail!("{msg}");
    }
}

/// Gets the user's home directory path.
///
/// Returns `$HOME` if it's set and non-empty.
///
/// # Returns
///
/// Returns [`Option<PathBuf>`] containing the home directory path, or `None` if not set.
pub(super) fn home_dir() -> Option<PathBuf> {
    std::env::var_os(ENV_HOME)
        .filter(|h| !h.is_empty())
        .map(PathBuf::from)
}

/// Determines whether to use tmux for command execution.
///
/// Returns `true` if tmux should be used, which is when:
/// * `no_tmux` is `false` (tmux is not explicitly disabled)
/// * `$TMUX` environment variable is set and non-empty
///
/// # Arguments
///
/// * `no_tmux` - If `true`, tmux will not be used regardless of environment
///
/// # Returns
///
/// Returns `true` if tmux should be used, `false` otherwise.
pub(super) fn should_use_tmux(no_tmux: bool) -> bool {
    !no_tmux && env_present(ENV_TMUX)
}

/// Checks if an environment variable is present and non-empty.
///
/// # Arguments
///
/// * `name` - The name of the environment variable to check
///
/// # Returns
///
/// Returns `true` if the environment variable is set and non-empty, `false` otherwise.
fn env_present(name: &str) -> bool {
    std::env::var_os(name).is_some_and(|v| !v.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use rstest_reuse::*;
    use std::fs;
    use std::io::Write;

    mod fixtures {
        use super::*;
        use std::time::{SystemTime, UNIX_EPOCH};

        fn unique_suffix() -> String {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                .to_string()
        }

        #[fixture]
        pub fn temp_dir() -> PathBuf {
            let temp_dir = std::env::temp_dir().join(format!("amg_test_{}", unique_suffix()));
            fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");
            temp_dir
        }

        #[fixture]
        pub fn temp_file() -> PathBuf {
            let temp_file = std::env::temp_dir().join(format!("amg_test_file_{}", unique_suffix()));
            let mut file = fs::File::create(&temp_file).expect("Failed to create temp file");
            file.write_all(b"test content")
                .expect("Failed to write to temp file");
            drop(file);
            temp_file
        }

        #[fixture]
        pub fn nonexistent_path() -> PathBuf {
            let path =
                std::env::temp_dir().join(format!("amg_test_nonexistent_{}", unique_suffix()));
            if path.exists() {
                fs::remove_dir_all(&path).ok();
            }
            path
        }

        #[fixture]
        pub fn nonexistent_path_with_special_chars() -> PathBuf {
            let base = std::env::temp_dir().join(format!("amg_test_special_{}", unique_suffix()));
            if base.exists() {
                fs::remove_dir_all(&base).ok();
            }
            base.join("path with spaces & symbols!")
        }
    }

    mod success {
        use super::*;

        #[template]
        #[rstest]
        #[case("repo", None)]
        #[case("codexdir", None)]
        #[case("session cwd", None)]
        #[case("repo", Some("CODEX_REPO"))]
        #[case("codexdir", Some("CODEX_CODEXDIR"))]
        #[case("test_dir", Some("TEST_ENV_VAR"))]
        fn success_cases(#[case] label: &'static str, #[case] env_var: Option<&'static str>) {}

        #[apply(success_cases)]
        fn validates_existing_directory(
            #[from(fixtures::temp_dir)] dir: PathBuf,
            #[case] label: &'static str,
            #[case] env_var: Option<&'static str>,
        ) {
            let result = require_dir(&dir, label, env_var);
            assert!(
                result.is_ok(),
                "require_dir should succeed for valid directory with label={label:?}, env_var={env_var:?}"
            );
        }

        #[rstest]
        fn handles_paths_with_special_characters(#[from(fixtures::temp_dir)] base_dir: PathBuf) {
            let special_path = base_dir.join("dir with spaces & symbols!");
            fs::create_dir_all(&special_path).expect("Failed to create special dir");

            let result = require_dir(&special_path, "special", None);
            assert!(
                result.is_ok(),
                "require_dir should succeed for directory with special characters"
            );
        }
    }

    mod failure {
        use super::*;

        #[rstest]
        fn rejects_nonexistent_path(#[from(fixtures::nonexistent_path)] path: PathBuf) {
            let result = require_dir(&path, "nonexistent", None);
            assert!(
                result.is_err(),
                "require_dir should fail for nonexistent path: {}",
                path.display()
            );
        }

        #[rstest]
        fn rejects_file_instead_of_directory(#[from(fixtures::temp_file)] file: PathBuf) {
            let result = require_dir(&file, "test_file", None);
            assert!(
                result.is_err(),
                "require_dir should fail for file: {}",
                file.display()
            );
        }
    }

    mod error_messages {
        use super::*;

        #[template]
        #[rstest]
        #[case("repo", Some("CODEX_REPO"), true)]
        #[case("codexdir", Some("CODEX_CODEXDIR"), true)]
        #[case("session cwd", None, false)]
        #[case("test_label", Some("TEST_VAR"), true)]
        #[case("another_label", None, false)]
        fn error_message_cases(
            #[case] label: &'static str,
            #[case] env_var: Option<&'static str>,
            #[case] should_include_env_var: bool,
        ) {
        }

        #[apply(error_message_cases)]
        fn includes_label_in_error_message(
            #[from(fixtures::nonexistent_path)] path: PathBuf,
            #[case] label: &'static str,
            #[case] env_var: Option<&'static str>,
            #[case] _should_include_env_var: bool,
        ) {
            let result = require_dir(&path, label, env_var);
            assert!(
                result.is_err(),
                "require_dir should fail for nonexistent path"
            );

            let error_msg = result.unwrap_err().to_string();
            assert!(
                error_msg.contains(label),
                "Error message should include label '{label}', got: {error_msg}"
            );
        }

        #[apply(error_message_cases)]
        fn includes_directory_indicator(
            #[from(fixtures::nonexistent_path)] path: PathBuf,
            #[case] label: &'static str,
            #[case] env_var: Option<&'static str>,
            #[case] _should_include_env_var: bool,
        ) {
            let result = require_dir(&path, label, env_var);
            assert!(
                result.is_err(),
                "require_dir should fail for nonexistent path"
            );

            let error_msg = result.unwrap_err().to_string();
            assert!(
                error_msg.contains("is not a directory"),
                "Error message should indicate it's not a directory, got: {error_msg}"
            );
        }

        #[apply(error_message_cases)]
        fn includes_env_var_when_provided(
            #[from(fixtures::nonexistent_path)] path: PathBuf,
            #[case] label: &'static str,
            #[case] env_var: Option<&'static str>,
            #[case] should_include_env_var: bool,
        ) {
            let result = require_dir(&path, label, env_var);
            assert!(
                result.is_err(),
                "require_dir should fail for nonexistent path"
            );

            let error_msg = result.unwrap_err().to_string();
            if should_include_env_var {
                let expected = format!("(from {})", env_var.unwrap());
                assert!(
                    error_msg.contains(&expected),
                    "Error message should include '{expected}', got: {error_msg}"
                );
            } else {
                assert!(
                    !error_msg.contains("(from"),
                    "Error message should not include env_var when None, got: {error_msg}"
                );
            }
        }

        #[rstest]
        fn includes_path_in_error_message(#[from(fixtures::nonexistent_path)] path: PathBuf) {
            let result = require_dir(&path, "test", None);
            assert!(
                result.is_err(),
                "require_dir should fail for nonexistent path"
            );

            let error_msg = result.unwrap_err().to_string();
            let path_str = path.to_string_lossy();
            assert!(
                error_msg.contains(path_str.as_ref()),
                "Error message should include path '{}', got: {}",
                path_str,
                error_msg
            );
        }

        #[rstest]
        fn handles_special_characters_in_path(
            #[from(fixtures::nonexistent_path_with_special_chars)] path: PathBuf,
        ) {
            let result = require_dir(&path, "special", Some("SPECIAL_VAR"));
            assert!(
                result.is_err(),
                "require_dir should fail for nonexistent path"
            );

            let error_msg = result.unwrap_err().to_string();
            assert!(
                error_msg.contains("special"),
                "Error message should include label 'special', got: {error_msg}"
            );
            assert!(
                error_msg.contains("(from SPECIAL_VAR)"),
                "Error message should include env_var '(from SPECIAL_VAR)', got: {error_msg}"
            );
        }
    }
}
