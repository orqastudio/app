// Filesystem helpers for reading and scanning artifact files.
//
// These functions perform I/O against the project's `.orqa/` and `docs/` directories.
// None of them touch the database — database operations live in the access layer's repository.
// This module is part of orqa-engine so it can be used by any access layer (app, daemon, CLI).

use std::path::{Path, PathBuf};

use crate::error::EngineError;
use crate::types::artifact::{Artifact, ArtifactType, ComplianceStatus, DocNode};

/// Write artifact content to disk, creating parent directories as needed.
///
/// Performs filesystem I/O. Errors are propagated as `EngineError::FileSystem`.
pub fn write_artifact_file(full_path: &Path, content: &str) -> Result<(), EngineError> {
    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(full_path, content)?;
    Ok(())
}

/// Build an in-memory `Artifact` struct from a file on disk (no DB record).
///
/// Performs filesystem I/O to read content and metadata. The returned artifact
/// has zeroed IDs — callers that persist it must assign real IDs.
pub fn artifact_from_file(
    full_path: &Path,
    rel_path: String,
    artifact_type: ArtifactType,
) -> Result<Artifact, EngineError> {
    let content = std::fs::read_to_string(full_path)?;
    let file_name = full_path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let name = humanize_name(&file_name);
    let file_size = std::fs::metadata(full_path).ok().map(|m| m.len() as i64);

    Ok(Artifact {
        id: 0,
        project_id: 0,
        artifact_type,
        rel_path,
        name,
        description: None,
        content,
        file_hash: None,
        file_size,
        file_modified_at: None,
        compliance_status: ComplianceStatus::Unknown,
        relationships: None,
        metadata: None,
        created_at: String::new(),
        updated_at: String::new(),
    })
}

/// Map an `ArtifactType` to its `.orqa/` subdirectory path.
///
/// Returns `None` for `Doc` — docs live in `docs/`, not in `.orqa/`.
pub fn governance_dir(root: &Path, artifact_type: &ArtifactType) -> Option<PathBuf> {
    match artifact_type {
        ArtifactType::Agent => Some(root.join(".orqa").join("process").join("agents")),
        ArtifactType::Rule => Some(root.join(".orqa").join("process").join("rules")),
        ArtifactType::Knowledge => Some(root.join(".orqa").join("process").join("knowledge")),
        ArtifactType::Doc => None,
    }
}

/// Recursively scan a directory and build a sorted list of `DocNode` entries.
///
/// Performs filesystem I/O. Hidden entries (starting with `.` or `_`) are skipped.
/// Directories come first (alphabetically), then `.md` files (alphabetically).
pub fn scan_directory(dir: &Path, docs_root: &Path) -> Result<Vec<DocNode>, EngineError> {
    let mut dirs: Vec<(String, PathBuf)> = Vec::new();
    let mut files: Vec<(String, PathBuf)> = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if name.starts_with('.') || name.starts_with('_') {
            continue;
        }

        let path = entry.path();
        if path.is_dir() {
            dirs.push((name.into_owned(), path));
        } else if name.ends_with(".md") {
            files.push((name.into_owned(), path));
        }
    }

    dirs.sort_by(|a, b| a.0.cmp(&b.0));
    files.sort_by(|a, b| a.0.cmp(&b.0));

    let mut nodes = Vec::with_capacity(dirs.len() + files.len());

    for (name, path) in dirs {
        let children = scan_directory(&path, docs_root)?;
        nodes.push(DocNode {
            label: humanize_name(&name),
            path: None,
            children: Some(children),
            frontmatter: None,
            status: None,
            description: None,
            icon: None,
        });
    }

    for (name, path) in files {
        let rel = relative_doc_path(&path, docs_root);
        nodes.push(DocNode {
            label: humanize_name(&name),
            path: Some(rel),
            children: None,
            frontmatter: None,
            status: None,
            description: None,
            icon: None,
        });
    }

    Ok(nodes)
}

/// Generate a basic ISO 8601 UTC timestamp without external time crates.
///
/// Used wherever a timestamp string is needed without pulling in the `chrono` or `time` crates.
pub fn now_iso() -> String {
    let now = std::time::SystemTime::now();
    let duration = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();

    let days = secs / 86400;
    let remaining = secs % 86400;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;
    let seconds = remaining % 60;

    let (year, month, day) = days_to_ymd(days);

    format!("{year:04}-{month:02}-{day:02}T{hours:02}:{minutes:02}:{seconds:02}Z")
}

/// Build the relative path from `docs_root` without the `.md` extension.
///
/// Example: `docs/product/vision.md` -> `"product/vision"`.
fn relative_doc_path(file: &Path, docs_root: &Path) -> String {
    let rel = file
        .strip_prefix(docs_root)
        .unwrap_or(file)
        .with_extension("");
    // Normalise to forward slashes (important on Windows)
    rel.to_string_lossy().replace('\\', "/")
}

/// Convert a filename to a human-readable label.
///
/// Strips `.md` / `.sh`, replaces hyphens with spaces, and title-cases each word.
/// Preserves fully uppercase names (e.g. README, CHANGELOG).
pub fn humanize_name(filename: &str) -> String {
    let stem = filename
        .strip_suffix(".md")
        .or_else(|| filename.strip_suffix(".sh"))
        .unwrap_or(filename);
    // Preserve all-caps names like README, CHANGELOG, TODO
    if stem
        .chars()
        .all(|c| c.is_ascii_uppercase() || c == '-' || c == '_')
        && stem.chars().any(|c| c.is_ascii_uppercase())
    {
        return stem.to_string();
    }
    stem.split('-')
        .map(title_case_word)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Title-case a single word (first char uppercase, rest lowercase).
fn title_case_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let mut s = first.to_uppercase().to_string();
            for ch in chars {
                s.extend(ch.to_lowercase());
            }
            s
        }
    }
}

/// Convert days since Unix epoch to (year, month, day).
fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    let mut y = 1970u64;
    let mut remaining = days;

    loop {
        let days_in_year = if is_leap_year(y) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        y += 1;
    }

    let days_in_months: [u64; 12] = if is_leap_year(y) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut m = 0u64;
    for (i, &days_in_month) in days_in_months.iter().enumerate() {
        if remaining < days_in_month {
            m = i as u64 + 1;
            break;
        }
        remaining -= days_in_month;
    }

    (y, m, remaining + 1)
}

/// Check if a year is a leap year.
fn is_leap_year(year: u64) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn humanize_name_basic() {
        assert_eq!(humanize_name("no-stubs.md"), "No Stubs");
    }

    #[test]
    fn humanize_name_preserves_all_caps() {
        assert_eq!(humanize_name("README.md"), "README");
        assert_eq!(humanize_name("CHANGELOG.md"), "CHANGELOG");
    }

    #[test]
    fn humanize_name_shell_script() {
        assert_eq!(humanize_name("pre-commit.sh"), "Pre Commit");
    }

    #[test]
    fn now_iso_format() {
        let ts = now_iso();
        assert!(ts.ends_with('Z'));
        assert_eq!(ts.len(), 20);
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], "T");
        assert_eq!(&ts[13..14], ":");
        assert_eq!(&ts[16..17], ":");
    }

    #[test]
    fn days_to_ymd_epoch() {
        let (y, m, d) = days_to_ymd(0);
        assert_eq!((y, m, d), (1970, 1, 1));
    }

    #[test]
    fn days_to_ymd_known_date() {
        // 2024-01-01 is 19723 days from epoch
        let (y, m, d) = days_to_ymd(19723);
        assert_eq!((y, m, d), (2024, 1, 1));
    }

    #[test]
    fn is_leap_year_checks() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2023));
    }

    #[test]
    fn governance_dir_returns_none_for_doc() {
        let root = Path::new("/tmp/project");
        assert!(governance_dir(root, &ArtifactType::Doc).is_none());
    }

    #[test]
    fn governance_dir_returns_path_for_agent() {
        let root = Path::new("/tmp/project");
        let dir = governance_dir(root, &ArtifactType::Agent);
        assert!(dir.is_some());
        assert!(dir.unwrap().ends_with("process/agents"));
    }
}
