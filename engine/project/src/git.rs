// Git utility functions for the OrqaStudio engine.
//
// Provides structured git state queries for session startup checks. These
// functions wrap std::process::Command so that daemon and CLI code can
// delegate git operations to the engine layer rather than spawning subprocesses
// directly. All functions degrade gracefully — a missing git binary or
// non-git directory returns None rather than propagating an error.

use std::path::Path;
use std::process::Command;

/// Result of a git stash list query.
#[derive(Debug, Clone)]
pub struct StashList {
    /// Raw text from `git stash list`, trimmed. Empty if no stashes.
    pub output: String,
}

impl StashList {
    /// Returns true if there are one or more stashes.
    pub fn is_empty(&self) -> bool {
        self.output.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stash_list_is_empty_when_output_is_empty() {
        let stash = StashList {
            output: String::new(),
        };
        assert!(stash.is_empty());
    }

    #[test]
    fn stash_list_is_not_empty_when_has_output() {
        let stash = StashList {
            output: "stash@{0}: on main: my stash".to_owned(),
        };
        assert!(!stash.is_empty());
    }

    #[test]
    fn stash_list_on_nonexistent_path_returns_none() {
        // Git cannot operate on a nonexistent directory.
        let result = stash_list(std::path::Path::new("/nonexistent/path/that/cannot/exist"));
        assert!(result.is_none());
    }

    #[test]
    fn uncommitted_status_on_nonexistent_path_returns_none() {
        let result =
            uncommitted_status(std::path::Path::new("/nonexistent/path/that/cannot/exist"));
        assert!(result.is_none());
    }
}

/// Result of a git status query on a specific branch.
#[derive(Debug, Clone)]
pub struct UncommittedStatus {
    /// The current branch name.
    pub branch: String,
    /// Number of uncommitted files detected by `git status --porcelain`.
    pub uncommitted_count: usize,
}

/// Query git stash list for the repository rooted at `project_path`.
///
/// Returns None if git is unavailable or the directory is not a git repo.
/// Returns Some with an empty output field if the stash is clean.
pub fn stash_list(project_path: &Path) -> Option<StashList> {
    let out = Command::new("git")
        .args(["stash", "list"])
        .current_dir(project_path)
        .output()
        .ok()?;

    if !out.status.success() {
        return None;
    }

    let text = String::from_utf8_lossy(&out.stdout).trim().to_owned();
    Some(StashList { output: text })
}

/// Query uncommitted file count for the repository rooted at `project_path`.
///
/// Returns None if git is unavailable, the directory is not a git repo, or
/// the current branch cannot be determined. Returns Some with uncommitted_count
/// of zero if the working tree is clean.
pub fn uncommitted_status(project_path: &Path) -> Option<UncommittedStatus> {
    let branch_out = Command::new("git")
        .args(["branch", "--show-current"])
        .current_dir(project_path)
        .output()
        .ok()?;

    if !branch_out.status.success() {
        return None;
    }

    let branch = String::from_utf8(branch_out.stdout)
        .ok()
        .map(|s| s.trim().to_owned())?;

    let status_out = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(project_path)
        .output()
        .ok()?;

    if !status_out.status.success() {
        return None;
    }

    let uncommitted_count = std::str::from_utf8(&status_out.stdout)
        .unwrap_or("")
        .lines()
        .filter(|l| !l.trim().is_empty())
        .count();

    Some(UncommittedStatus {
        branch,
        uncommitted_count,
    })
}
