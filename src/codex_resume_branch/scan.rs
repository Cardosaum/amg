use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::io::{self, BufRead};

use serde::Deserialize;

use super::prelude::*;

#[derive(Debug)]
pub(super) struct Session {
    pub(super) cwd: PathBuf,
    pub(super) id: String,
    pub(super) source_jsonl: PathBuf,
}

pub(super) fn find_first_session(codexdir: &Path, branch: &str) -> Result<Option<Session>> {
    Ok(SortedWalk::new(codexdir)?
        .filter(|p| is_jsonl(p))
        .find_map(|p| session_from_jsonl(p, branch)))
}

fn is_jsonl(path: &Path) -> bool {
    path.extension() == Some(OsStr::new("jsonl"))
}

fn session_from_jsonl(source_jsonl: PathBuf, branch: &str) -> Option<Session> {
    let line = read_first_line(&source_jsonl).ok().flatten()?;
    let (cwd, id) = parse_session_first_line(&line, branch)?;
    Some(Session {
        cwd,
        id,
        source_jsonl,
    })
}

fn read_first_line(path: &Path) -> io::Result<Option<String>> {
    let f = fs::File::open(path)?;
    let mut lines = io::BufReader::new(f).lines();
    match lines.next() {
        Some(line) => line.map(Some),
        None => Ok(None),
    }
}

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

#[derive(Debug, Deserialize)]
struct Event {
    payload: Option<Payload>,
}

#[derive(Debug, Deserialize)]
struct Payload {
    git: Option<Git>,
    cwd: Option<String>,
    id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Git {
    branch: Option<String>,
}

/// A global lexicographic walk (by full path), roughly matching `fd`'s default output ordering.
struct SortedWalk {
    heap: BinaryHeap<Reverse<PathBuf>>,
}

impl SortedWalk {
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
