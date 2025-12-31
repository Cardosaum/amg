//! Session scanning and matching logic.
//!
//! This module provides functionality to scan Codex session files (JSONL format) and find
//! sessions that match a given git branch name. It performs a lexicographically sorted walk
//! through the Codex directory to find matching sessions.

use serde::Deserialize;

use super::prelude::*;

/// Represents a matched Codex session.
///
/// Contains the information needed to resume a session, including the working directory,
/// session ID, and the source JSONL file path.
#[derive(Debug)]
pub(super) struct Session {
    /// The working directory where the session was created.
    pub(super) cwd: PathBuf,
    /// The unique session identifier.
    pub(super) id: String,
    /// The path to the JSONL file containing this session.
    pub(super) source_jsonl: PathBuf,
}

/// Finds the first Codex session matching the given branch name.
///
/// Scans through all JSONL files in the codex directory in lexicographic order and returns
/// the first session whose first JSONL line has `.payload.git.branch == branch`.
///
/// # Arguments
///
/// * `codexdir` - The Codex directory to search in
/// * `branch` - The git branch name to match against
///
/// # Returns
///
/// Returns [`Result<Option<Session>>`] containing:
/// * `Some(Session)` - If a matching session is found
/// * `None` - If no matching session is found
///
/// # Errors
///
/// Returns an error if:
/// * The codexdir cannot be read
/// * File system operations fail during scanning
///
/// # See Also
///
/// * [`Session`] - Session structure
/// * [`SortedWalk`] - Directory walker implementation
pub(super) fn find_first_session(codexdir: &Path, branch: &str) -> Result<Option<Session>> {
    Ok(SortedWalk::new(codexdir)?
        .filter(|p| is_jsonl(p))
        .find_map(|p| session_from_jsonl(p, branch)))
}

/// Checks if a path has a `.jsonl` extension.
///
/// # Arguments
///
/// * `path` - The path to check
///
/// # Returns
///
/// Returns `true` if the path has a `.jsonl` extension, `false` otherwise.
fn is_jsonl(path: &Path) -> bool {
    path.extension() == Some(OsStr::new("jsonl"))
}

/// Attempts to create a [`Session`] from a JSONL file if it matches the branch.
///
/// Reads the first line of the JSONL file and parses it to extract session information.
/// Returns `Some(Session)` if the branch matches, `None` otherwise.
///
/// # Arguments
///
/// * `source_jsonl` - Path to the JSONL file
/// * `branch` - The git branch name to match against
///
/// # Returns
///
/// Returns [`Option<Session>`] if a matching session is found, `None` otherwise.
///
/// # See Also
///
/// * [`read_first_line`] - Reads the first line of a file
/// * [`parse_session_first_line`] - Parses session data from JSON
fn session_from_jsonl(source_jsonl: PathBuf, branch: &str) -> Option<Session> {
    let line = read_first_line(&source_jsonl).ok().flatten()?;
    let (cwd, id) = parse_session_first_line(&line, branch)?;
    Some(Session {
        cwd,
        id,
        source_jsonl,
    })
}

/// Reads the first line from a file.
///
/// # Arguments
///
/// * `path` - The path to the file to read
///
/// # Returns
///
/// Returns [`io::Result<Option<String>>`] containing:
/// * `Some(String)` - The first line if the file is not empty
/// * `None` - If the file is empty
///
/// # Errors
///
/// Returns an error if the file cannot be opened or read.
fn read_first_line(path: &Path) -> io::Result<Option<String>> {
    let f = fs::File::open(path)?;
    let mut lines = io::BufReader::new(f).lines();
    match lines.next() {
        Some(line) => line.map(Some),
        None => Ok(None),
    }
}

/// Parses the first line of a JSONL session file to extract session information.
///
/// Performs a fast-path check to avoid JSON parsing unless the branch name appears in the line.
/// Then parses the JSON to extract git branch, working directory, and session ID.
///
/// # Arguments
///
/// * `line` - The first line of the JSONL file
/// * `branch` - The git branch name to match against
///
/// # Returns
///
/// Returns [`Option<(PathBuf, String)>`] containing:
/// * `Some((cwd, id))` - If the branch matches and all required fields are present
/// * `None` - If the branch doesn't match or required fields are missing
fn parse_session_first_line(line: &str, branch: &str) -> Option<(PathBuf, String)> {
    // Fast-path: avoid JSON parsing unless the branch appears on the line.
    if !line.contains(branch) {
        return None;
    }

    let Event {
        payload:
            Some(Payload {
                git: Some(Git {
                    branch: Some(got_branch),
                }),
                cwd: Some(cwd),
                id: Some(id),
            }),
    } = serde_json::from_str(line).ok()?
    else {
        return None;
    };

    let cwd = cwd.trim();
    let id = id.trim();
    (got_branch == branch && !cwd.is_empty() && !id.is_empty())
        .then(|| (PathBuf::from(cwd), id.to_owned()))
}

/// JSON deserialization structure for Codex event payload.
#[derive(Debug, Deserialize)]
struct Event {
    /// The event payload containing session information.
    payload: Option<Payload>,
}

/// JSON deserialization structure for Codex payload.
#[derive(Debug, Deserialize)]
struct Payload {
    /// Git-related information.
    git: Option<Git>,
    /// The working directory where the session was created.
    cwd: Option<String>,
    /// The unique session identifier.
    id: Option<String>,
}

/// JSON deserialization structure for git information.
#[derive(Debug, Deserialize)]
struct Git {
    /// The git branch name.
    branch: Option<String>,
}

/// A lexicographically sorted directory walker.
///
/// Performs a depth-first traversal of a directory tree, returning files in lexicographic
/// order by their full path. This roughly matches `fd`'s default output ordering.
///
/// Symlinks are skipped during traversal.
struct SortedWalk {
    /// Binary heap used to maintain sorted order of paths.
    heap: BinaryHeap<Reverse<PathBuf>>,
}

impl SortedWalk {
    /// Creates a new sorted walker starting at the given root directory.
    ///
    /// # Arguments
    ///
    /// * `root` - The root directory to start walking from
    ///
    /// # Returns
    ///
    /// Returns [`Result<SortedWalk>`] containing the walker instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the root directory cannot be read. Other unreadable directories
    /// encountered during traversal are simply skipped.
    fn new(root: &Path) -> Result<Self> {
        // Fail fast for the root dir; other unreadable dirs are simply skipped during traversal.
        fs::read_dir(root)
            .with_context(|| format!("failed to read directory {}", root.display()))?;

        let mut heap = BinaryHeap::new();
        heap.push(Reverse(root.to_owned()));
        Ok(Self { heap })
    }
}

impl Iterator for SortedWalk {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Reverse(path)) = self.heap.pop() {
            let meta = match fs::symlink_metadata(&path) {
                Ok(m) => m,
                Err(_) => continue,
            };
            if meta.file_type().is_symlink() {
                continue;
            }

            if meta.is_dir() {
                let rd = match fs::read_dir(&path) {
                    Ok(rd) => rd,
                    Err(_) => continue,
                };
                rd.flatten().for_each(|e| self.heap.push(Reverse(e.path())));
                continue;
            }

            if meta.is_file() {
                return Some(path);
            }
        }

        None
    }
}
