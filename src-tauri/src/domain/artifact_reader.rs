use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use crate::domain::artifact::{
    parse_doc_frontmatter, parse_plan_frontmatter, parse_research_frontmatter, Artifact,
    ArtifactSummary, ArtifactType, ComplianceStatus, DocFrontmatter, DocNode,
};
use crate::domain::paths;
use crate::error::OrqaError;

/// Read a documentation file from `<project_path>/docs/<rel_path>.md` and construct an `Artifact`.
///
/// The `rel_path` must not contain `..` (path traversal is rejected by the caller).
pub fn read_doc(project_path: &Path, rel_path: &str) -> Result<Artifact, OrqaError> {
    let docs_path = project_path.join("docs").join(format!("{rel_path}.md"));

    if !docs_path.exists() {
        return Err(OrqaError::NotFound(format!("doc not found: {rel_path}")));
    }

    let raw_content = std::fs::read_to_string(&docs_path)?;
    let (frontmatter, body) = parse_doc_frontmatter(&raw_content);

    let name = frontmatter.title.clone().unwrap_or_else(|| {
        rel_path
            .split('/')
            .next_back()
            .unwrap_or(rel_path)
            .replace('-', " ")
    });

    let file_size = std::fs::metadata(&docs_path).ok().map(|m| m.len() as i64);
    let fm_json = serde_json::to_value(&frontmatter).ok();

    Ok(Artifact {
        id: 0,
        project_id: 0,
        artifact_type: ArtifactType::Doc,
        rel_path: format!("docs/{rel_path}.md"),
        name,
        description: None,
        content: body,
        file_hash: None,
        file_size,
        file_modified_at: frontmatter.updated.clone(),
        compliance_status: ComplianceStatus::Unknown,
        relationships: None,
        metadata: fm_json,
        created_at: frontmatter.created.unwrap_or_default(),
        updated_at: frontmatter.updated.unwrap_or_default(),
    })
}

/// Read a research document from `<project_path>/.orqa/research/<rel_path>.md` and construct
/// an `Artifact`.
///
/// The `rel_path` must not contain `..` (path traversal is rejected by the caller).
pub fn read_research(project_path: &Path, rel_path: &str) -> Result<Artifact, OrqaError> {
    let file_path = project_path
        .join(paths::RESEARCH_DIR)
        .join(format!("{rel_path}.md"));

    if !file_path.exists() {
        return Err(OrqaError::NotFound(format!(
            "research doc not found: {rel_path}"
        )));
    }

    let raw_content = std::fs::read_to_string(&file_path)?;
    let (frontmatter, body) = parse_research_frontmatter(&raw_content);

    let name = frontmatter
        .category
        .as_deref()
        .map(|c| format!("{} Research", title_case_hyphenated(c)))
        .unwrap_or_else(|| {
            rel_path
                .split('/')
                .next_back()
                .unwrap_or(rel_path)
                .replace('-', " ")
        });

    let description = frontmatter.description.clone();
    let file_size = std::fs::metadata(&file_path).ok().map(|m| m.len() as i64);
    let fm_json = serde_json::to_value(&frontmatter).ok();

    Ok(Artifact {
        id: 0,
        project_id: 0,
        artifact_type: ArtifactType::Doc,
        rel_path: format!("{}/{rel_path}.md", paths::RESEARCH_DIR),
        name,
        description,
        content: body,
        file_hash: None,
        file_size,
        file_modified_at: frontmatter.date.clone(),
        compliance_status: ComplianceStatus::Unknown,
        relationships: None,
        metadata: fm_json,
        created_at: frontmatter.date.clone().unwrap_or_default(),
        updated_at: frontmatter.date.unwrap_or_default(),
    })
}

/// Read an implementation plan from `<project_path>/.orqa/plans/<rel_path>.md` and construct
/// an `Artifact`.
///
/// The `rel_path` must not contain `..` (path traversal is rejected by the caller).
pub fn read_plan(project_path: &Path, rel_path: &str) -> Result<Artifact, OrqaError> {
    let file_path = project_path
        .join(paths::PLANS_DIR)
        .join(format!("{rel_path}.md"));

    if !file_path.exists() {
        return Err(OrqaError::NotFound(format!("plan not found: {rel_path}")));
    }

    let raw_content = std::fs::read_to_string(&file_path)?;
    let (frontmatter, body) = parse_plan_frontmatter(&raw_content);

    let file_name = file_path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let name = frontmatter
        .title
        .clone()
        .unwrap_or_else(|| humanize_name(&file_name));

    let file_size = std::fs::metadata(&file_path).ok().map(|m| m.len() as i64);
    let fm_json = serde_json::to_value(&frontmatter).ok();

    Ok(Artifact {
        id: 0,
        project_id: 0,
        artifact_type: ArtifactType::Doc,
        rel_path: format!("{}/{rel_path}.md", paths::PLANS_DIR),
        name,
        description: None,
        content: body,
        file_hash: None,
        file_size,
        file_modified_at: frontmatter.updated.clone(),
        compliance_status: ComplianceStatus::Unknown,
        relationships: None,
        metadata: fm_json,
        created_at: frontmatter.created.clone().unwrap_or_default(),
        updated_at: frontmatter.updated.unwrap_or_default(),
    })
}

/// Recursively scan a `docs/` directory and build a sorted `DocNode` tree.
///
/// Hidden entries (starting with `.` or `_`) are skipped. Directories come first
/// (alphabetically), then `.md` files (alphabetically).
pub fn scan_doc_tree(base_path: &Path) -> Result<Vec<DocNode>, OrqaError> {
    scan_directory(base_path, base_path)
}

/// Scan the `.orqa/research/` directory and build a sorted `DocNode` tree.
///
/// Uses `ResearchFrontmatter` to derive labels for leaf nodes. Subdirectories produce
/// directory `DocNode` entries with `children`. Returns an empty vec if the directory
/// does not exist (no error).
pub fn scan_research_tree(research_path: &Path) -> Result<Vec<DocNode>, OrqaError> {
    scan_research_directory(research_path, research_path)
}

/// Scan the `.orqa/plans/` directory and build a flat sorted list of plan `DocNode` entries.
///
/// Uses `PlanFrontmatter` to derive labels. Returns an empty vec if the directory does not
/// exist (no error).
pub fn scan_plan_tree(plans_path: &Path) -> Result<Vec<DocNode>, OrqaError> {
    let mut nodes = Vec::new();

    for entry in std::fs::read_dir(plans_path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if name.starts_with('.') || name.starts_with('_') || !name.ends_with(".md") {
            continue;
        }

        let path = entry.path();
        let rel = name.trim_end_matches(".md").to_string();

        let content = std::fs::read_to_string(&path).unwrap_or_default();
        let (fm, _) = parse_plan_frontmatter(&content);

        let label = fm.title.clone().unwrap_or_else(|| humanize_name(&name));

        let doc_fm = DocFrontmatter {
            title: Some(label.clone()),
            category: None,
            tags: fm.tags.clone(),
            created: fm.created.clone(),
            updated: fm.updated.clone(),
        };

        nodes.push(DocNode {
            label,
            path: Some(rel),
            children: None,
            frontmatter: Some(doc_fm),
        });
    }

    nodes.sort_by(|a, b| a.label.cmp(&b.label));
    Ok(nodes)
}

/// Convert a directory entry to an `ArtifactSummary` for a governance artifact type.
///
/// Returns `None` if the entry should be skipped (wrong type, hidden, invalid extension).
pub fn summary_from_entry(
    entry: &DirEntry,
    artifact_type: &ArtifactType,
) -> Result<Option<ArtifactSummary>, OrqaError> {
    let file_name = entry.file_name();
    let name = file_name.to_string_lossy();

    if name.starts_with('.') || name.starts_with('_') {
        return Ok(None);
    }

    let ft = entry.file_type()?;

    let summary = match artifact_type {
        ArtifactType::Skill => {
            if ft.is_dir() && entry.path().join("SKILL.md").exists() {
                Some(ArtifactSummary {
                    id: 0,
                    artifact_type: artifact_type.clone(),
                    rel_path: format!(".orqa/skills/{}/SKILL.md", name),
                    name: humanize_name(&name),
                    description: None,
                    compliance_status: ComplianceStatus::Unknown,
                    file_modified_at: None,
                })
            } else {
                None
            }
        }
        _ if ft.is_file() => {
            let valid = match artifact_type {
                ArtifactType::Agent | ArtifactType::Rule => name.ends_with(".md"),
                ArtifactType::Hook => true,
                _ => false,
            };
            if valid {
                let rel_path = match artifact_type {
                    ArtifactType::Agent => format!(".orqa/agents/{name}"),
                    ArtifactType::Rule => format!(".orqa/rules/{name}"),
                    ArtifactType::Hook => format!(".orqa/hooks/{name}"),
                    _ => return Ok(None),
                };
                Some(ArtifactSummary {
                    id: 0,
                    artifact_type: artifact_type.clone(),
                    rel_path,
                    name: humanize_name(&name),
                    description: None,
                    compliance_status: ComplianceStatus::Unknown,
                    file_modified_at: None,
                })
            } else {
                None
            }
        }
        _ => None,
    };

    Ok(summary)
}

// ---------------------------------------------------------------------------
// Orqa artifact directory readers (milestones, epics, tasks, ideas, decisions, lessons)
// ---------------------------------------------------------------------------

/// Scan a flat `.orqa/<subdir>/` directory and return sorted `ArtifactSummary` entries.
///
/// Each `.md` file is parsed for its frontmatter `id`, `title`, `status`, and `description`.
/// Hidden files (starting with `.` or `_`) and non-`.md` files are skipped.
/// Returns an empty vec when the directory does not exist.
pub fn scan_orqa_artifact_dir(
    dir: &Path,
    dir_label: &str,
) -> Result<Vec<ArtifactSummary>, OrqaError> {
    if !dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut summaries: Vec<ArtifactSummary> = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if name.starts_with('.') || name.starts_with('_') || !name.ends_with(".md") {
            continue;
        }

        if !entry.file_type()?.is_file() {
            continue;
        }

        let path = entry.path();
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        let (id, title, status, description) = extract_basic_frontmatter(&content);

        let display_name = title.or_else(|| id.clone()).unwrap_or_else(|| humanize_name(&name));
        let rel_path = format!("{}/{}", dir_label, name);

        summaries.push(ArtifactSummary {
            id: 0,
            artifact_type: ArtifactType::Doc,
            rel_path,
            name: display_name,
            description,
            compliance_status: status
                .as_deref()
                .map(status_to_compliance)
                .unwrap_or(ComplianceStatus::Unknown),
            file_modified_at: None,
        });
    }

    // Sort by numeric ID extracted from the filename (e.g. MS-001 → 1), then alphabetically.
    summaries.sort_by(|a, b| {
        let na = numeric_id_from_path(&a.rel_path);
        let nb = numeric_id_from_path(&b.rel_path);
        na.cmp(&nb).then_with(|| a.rel_path.cmp(&b.rel_path))
    });

    Ok(summaries)
}

/// Read a single `.orqa/<subdir>/<filename>.md` artifact and construct an `Artifact`.
///
/// `dir` is the full path to the parent directory (e.g. `<project>/.orqa/milestones/`).
/// `dir_label` is the relative prefix stored in `rel_path` (e.g. `.orqa/milestones`).
/// `rel_path` is the caller-supplied filename (without `.md`) or full relative path.
///
/// The caller must have already rejected `..` path traversal.
pub fn read_orqa_artifact(
    dir: &Path,
    dir_label: &str,
    rel_path: &str,
) -> Result<Artifact, OrqaError> {
    // Accept either bare filename ("MS-001") or full relative path (".orqa/milestones/MS-001.md").
    let filename = rel_path
        .split('/')
        .next_back()
        .unwrap_or(rel_path)
        .trim_end_matches(".md");

    let file_path = dir.join(format!("{filename}.md"));

    if !file_path.exists() {
        return Err(OrqaError::NotFound(format!(
            "artifact not found: {rel_path}"
        )));
    }

    let raw_content = std::fs::read_to_string(&file_path)?;
    let (id, title, _status, description) = extract_basic_frontmatter(&raw_content);

    let name = title.or_else(|| id.clone()).unwrap_or_else(|| {
        let fname = file_path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        humanize_name(&fname)
    });

    let file_size = std::fs::metadata(&file_path).ok().map(|m| m.len() as i64);
    let full_rel_path = format!("{}/{filename}.md", dir_label);

    // Strip frontmatter from content so callers receive only the body.
    let body = crate::domain::artifact::extract_frontmatter(&raw_content)
        .1;

    Ok(Artifact {
        id: 0,
        project_id: 0,
        artifact_type: ArtifactType::Doc,
        rel_path: full_rel_path,
        name,
        description,
        content: body,
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

// Milestone

/// Scan the `.orqa/milestones/` directory and return sorted `ArtifactSummary` entries.
pub fn scan_milestones(project_path: &Path) -> Result<Vec<ArtifactSummary>, OrqaError> {
    let dir = project_path.join(paths::MILESTONES_DIR);
    scan_orqa_artifact_dir(&dir, paths::MILESTONES_DIR)
}

/// Read a single milestone file.
pub fn read_milestone(project_path: &Path, rel_path: &str) -> Result<Artifact, OrqaError> {
    let dir = project_path.join(paths::MILESTONES_DIR);
    read_orqa_artifact(&dir, paths::MILESTONES_DIR, rel_path)
}

// Epic

/// Scan the `.orqa/epics/` directory and return sorted `ArtifactSummary` entries.
pub fn scan_epics(project_path: &Path) -> Result<Vec<ArtifactSummary>, OrqaError> {
    let dir = project_path.join(paths::EPICS_DIR);
    scan_orqa_artifact_dir(&dir, paths::EPICS_DIR)
}

/// Read a single epic file.
pub fn read_epic(project_path: &Path, rel_path: &str) -> Result<Artifact, OrqaError> {
    let dir = project_path.join(paths::EPICS_DIR);
    read_orqa_artifact(&dir, paths::EPICS_DIR, rel_path)
}

// Task

/// Scan the `.orqa/tasks/` directory and return sorted `ArtifactSummary` entries.
pub fn scan_tasks(project_path: &Path) -> Result<Vec<ArtifactSummary>, OrqaError> {
    let dir = project_path.join(paths::TASKS_DIR);
    scan_orqa_artifact_dir(&dir, paths::TASKS_DIR)
}

/// Read a single task file.
pub fn read_task(project_path: &Path, rel_path: &str) -> Result<Artifact, OrqaError> {
    let dir = project_path.join(paths::TASKS_DIR);
    read_orqa_artifact(&dir, paths::TASKS_DIR, rel_path)
}

// Idea

/// Scan the `.orqa/ideas/` directory and return sorted `ArtifactSummary` entries.
pub fn scan_ideas(project_path: &Path) -> Result<Vec<ArtifactSummary>, OrqaError> {
    let dir = project_path.join(paths::IDEAS_DIR);
    scan_orqa_artifact_dir(&dir, paths::IDEAS_DIR)
}

/// Read a single idea file.
pub fn read_idea(project_path: &Path, rel_path: &str) -> Result<Artifact, OrqaError> {
    let dir = project_path.join(paths::IDEAS_DIR);
    read_orqa_artifact(&dir, paths::IDEAS_DIR, rel_path)
}

// Decision

/// Scan the `.orqa/decisions/` directory and return sorted `ArtifactSummary` entries.
pub fn scan_decisions(project_path: &Path) -> Result<Vec<ArtifactSummary>, OrqaError> {
    let dir = project_path.join(paths::DECISIONS_DIR);
    scan_orqa_artifact_dir(&dir, paths::DECISIONS_DIR)
}

/// Read a single decision record file.
pub fn read_decision(project_path: &Path, rel_path: &str) -> Result<Artifact, OrqaError> {
    let dir = project_path.join(paths::DECISIONS_DIR);
    read_orqa_artifact(&dir, paths::DECISIONS_DIR, rel_path)
}

// Lesson

/// Scan the `.orqa/lessons/` directory and return sorted `ArtifactSummary` entries.
pub fn scan_lessons_dir(project_path: &Path) -> Result<Vec<ArtifactSummary>, OrqaError> {
    let dir = project_path.join(paths::LESSONS_DIR);
    scan_orqa_artifact_dir(&dir, paths::LESSONS_DIR)
}

/// Read a single lesson file.
pub fn read_lesson_file(project_path: &Path, rel_path: &str) -> Result<Artifact, OrqaError> {
    let dir = project_path.join(paths::LESSONS_DIR);
    read_orqa_artifact(&dir, paths::LESSONS_DIR, rel_path)
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Extract the four most common frontmatter fields without committing to a specific schema.
///
/// Returns `(id, title, status, description)` — all `Option<String>`.
fn extract_basic_frontmatter(
    content: &str,
) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
    use crate::domain::artifact::extract_frontmatter;

    let (fm_text, _) = extract_frontmatter(content);
    let Some(yaml) = fm_text else {
        return (None, None, None, None);
    };

    // Use the serde_yaml Value type to avoid coupling to any specific struct.
    let value: serde_yaml::Value = serde_yaml::from_str(&yaml).unwrap_or(serde_yaml::Value::Null);

    let get_str = |key: &str| -> Option<String> {
        value
            .get(key)
            .and_then(|v| v.as_str())
            .map(str::to_owned)
    };

    (get_str("id"), get_str("title"), get_str("status"), get_str("description"))
}

/// Map a `status` string to a `ComplianceStatus` for display purposes.
///
/// - `done` / `accepted` / `compliant` / `active` → `Compliant`
/// - `non_compliant` / `rejected` / `error` → `NonCompliant`
/// - Anything else → `Unknown`
fn status_to_compliance(status: &str) -> ComplianceStatus {
    match status {
        "done" | "accepted" | "compliant" | "active" | "complete" => ComplianceStatus::Compliant,
        "non_compliant" | "rejected" | "error" => ComplianceStatus::NonCompliant,
        _ => ComplianceStatus::Unknown,
    }
}

/// Extract a numeric suffix from a relative path for natural sort order.
///
/// Example: `.orqa/epics/EPIC-005.md` → `5`, `.orqa/milestones/MS-001.md` → `1`.
/// Falls back to `u64::MAX` if no numeric suffix is found.
fn numeric_id_from_path(rel_path: &str) -> u64 {
    let stem = rel_path
        .split('/')
        .next_back()
        .unwrap_or(rel_path)
        .trim_end_matches(".md");

    // Find the last run of digits in the stem.
    let digits: String = stem
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .chars()
        .rev()
        .collect();

    digits.parse::<u64>().unwrap_or(u64::MAX)
}

/// Recursively scan a directory and build a sorted list of `DocNode` entries.
fn scan_directory(dir: &Path, docs_root: &Path) -> Result<Vec<DocNode>, OrqaError> {
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
        });
    }

    for (name, path) in files {
        let rel = relative_path_without_extension(&path, docs_root);
        let fm = std::fs::read_to_string(&path)
            .ok()
            .map(|content| parse_doc_frontmatter(&content).0);
        let label = fm
            .as_ref()
            .and_then(|f| f.title.clone())
            .unwrap_or_else(|| humanize_name(&name));
        nodes.push(DocNode {
            label,
            path: Some(rel),
            children: None,
            frontmatter: fm,
        });
    }

    Ok(nodes)
}

/// Recursively scan a research directory and build a sorted list of `DocNode` entries.
fn scan_research_directory(dir: &Path, research_root: &Path) -> Result<Vec<DocNode>, OrqaError> {
    let mut dirs: Vec<(String, PathBuf)> = Vec::new();
    let mut files: Vec<(String, PathBuf)> = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if name.starts_with('.') || name.starts_with('_') {
            continue;
        }

        // Skip README at any level — it is a meta-document, not a research document.
        if name.to_ascii_uppercase() == "README.MD" {
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
        let children = scan_research_directory(&path, research_root)?;
        nodes.push(DocNode {
            label: humanize_name(&name),
            path: None,
            children: Some(children),
            frontmatter: None,
        });
    }

    for (name, path) in files {
        let rel = relative_path_without_extension(&path, research_root);
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        let (fm, _) = parse_research_frontmatter(&content);

        let label = fm
            .category
            .as_deref()
            .map(|c| format!("{} Research", title_case_hyphenated(c)))
            .unwrap_or_else(|| humanize_name(&name));

        let doc_fm = DocFrontmatter {
            title: Some(label.clone()),
            category: fm.category,
            tags: Vec::new(),
            created: fm.date.clone(),
            updated: fm.date,
        };

        nodes.push(DocNode {
            label,
            path: Some(rel),
            children: None,
            frontmatter: Some(doc_fm),
        });
    }

    Ok(nodes)
}

/// Build the relative path from a root directory, stripping the `.md` extension.
///
/// Normalises path separators to forward slashes (important on Windows).
///
/// Example: `docs/product/vision.md` with root `docs/` -> `"product/vision"`.
fn relative_path_without_extension(file: &Path, root: &Path) -> String {
    let rel = file.strip_prefix(root).unwrap_or(file).with_extension("");
    rel.to_string_lossy().replace('\\', "/")
}

/// Convert a filename to a human-readable label.
///
/// Strips `.md` / `.sh`, replaces hyphens with spaces, and title-cases each word.
/// Preserves fully uppercase names (e.g. README, CHANGELOG).
pub(crate) fn humanize_name(filename: &str) -> String {
    // Strip any file extension
    let stem = match filename.rfind('.') {
        Some(pos) if pos > 0 => &filename[..pos],
        _ => filename,
    };
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

/// Title-case a hyphenated string (e.g. `"my-category"` -> `"My Category"`).
fn title_case_hyphenated(s: &str) -> String {
    s.split('-')
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_temp_project() -> TempDir {
        tempfile::tempdir().expect("tempdir")
    }

    #[test]
    fn humanize_name_basic() {
        assert_eq!(humanize_name("no-stubs.md"), "No Stubs");
        assert_eq!(humanize_name("coding-standards"), "Coding Standards");
        assert_eq!(humanize_name("README.MD"), "README");
        assert_eq!(humanize_name("pre-commit.sh"), "Pre Commit");
    }

    #[test]
    fn humanize_name_preserves_all_caps() {
        assert_eq!(humanize_name("README"), "README");
        assert_eq!(humanize_name("CHANGELOG"), "CHANGELOG");
        assert_eq!(humanize_name("TODO"), "TODO");
    }

    #[test]
    fn title_case_hyphenated_converts_correctly() {
        assert_eq!(title_case_hyphenated("my-category"), "My Category");
        assert_eq!(title_case_hyphenated("persistence"), "Persistence");
        assert_eq!(title_case_hyphenated("deep-research"), "Deep Research");
    }

    #[test]
    fn read_doc_not_found() {
        let tmp = make_temp_project();
        fs::create_dir_all(tmp.path().join("docs")).expect("create docs dir");

        let result = read_doc(tmp.path(), "missing");
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn read_doc_with_frontmatter() {
        let tmp = make_temp_project();
        let docs = tmp.path().join("docs");
        fs::create_dir_all(&docs).expect("create docs dir");
        fs::write(
            docs.join("vision.md"),
            "---\ntitle: Product Vision\ncreated: 2026-01-01\nupdated: 2026-03-01\n---\n# Vision\nContent here.",
        )
        .expect("write file");

        let artifact = read_doc(tmp.path(), "vision").expect("read_doc");
        assert_eq!(artifact.name, "Product Vision");
        assert_eq!(artifact.rel_path, "docs/vision.md");
        assert_eq!(artifact.artifact_type, ArtifactType::Doc);
        assert!(artifact.content.contains("# Vision"));
    }

    #[test]
    fn read_doc_derives_name_from_path_when_no_title() {
        let tmp = make_temp_project();
        let docs = tmp.path().join("docs");
        fs::create_dir_all(&docs).expect("create docs dir");
        fs::write(docs.join("coding-standards.md"), "# Standards").expect("write file");

        let artifact = read_doc(tmp.path(), "coding-standards").expect("read_doc");
        assert_eq!(artifact.name, "coding standards");
    }

    #[test]
    fn read_research_not_found() {
        let tmp = make_temp_project();
        fs::create_dir_all(tmp.path().join(paths::RESEARCH_DIR)).expect("create research dir");

        let result = read_research(tmp.path(), "missing");
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn read_research_with_frontmatter() {
        let tmp = make_temp_project();
        let research = tmp.path().join(paths::RESEARCH_DIR);
        fs::create_dir_all(&research).expect("create research dir");
        fs::write(
            research.join("persistence.md"),
            "---\ncategory: persistence\ndate: 2026-02-01\ndescription: DB research\n---\nBody here.",
        )
        .expect("write file");

        let artifact = read_research(tmp.path(), "persistence").expect("read_research");
        assert_eq!(artifact.name, "Persistence Research");
        assert_eq!(
            artifact.rel_path,
            format!("{}/persistence.md", paths::RESEARCH_DIR)
        );
        assert_eq!(artifact.description.as_deref(), Some("DB research"));
    }

    #[test]
    fn read_plan_not_found() {
        let tmp = make_temp_project();
        fs::create_dir_all(tmp.path().join(paths::PLANS_DIR)).expect("create plans dir");

        let result = read_plan(tmp.path(), "missing");
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn read_plan_with_frontmatter() {
        let tmp = make_temp_project();
        let plans = tmp.path().join(paths::PLANS_DIR);
        fs::create_dir_all(&plans).expect("create plans dir");
        fs::write(
            plans.join("phase-1.md"),
            "---\ntitle: Phase 1 Plan\ncreated: 2026-01-01\nupdated: 2026-03-01\n---\n# Phase 1",
        )
        .expect("write file");

        let artifact = read_plan(tmp.path(), "phase-1").expect("read_plan");
        assert_eq!(artifact.name, "Phase 1 Plan");
        assert_eq!(
            artifact.rel_path,
            format!("{}/phase-1.md", paths::PLANS_DIR)
        );
    }

    #[test]
    fn scan_doc_tree_empty_dir() {
        let tmp = make_temp_project();
        let docs = tmp.path().join("docs");
        fs::create_dir_all(&docs).expect("create docs dir");

        let nodes = scan_doc_tree(&docs).expect("scan");
        assert!(nodes.is_empty());
    }

    #[test]
    fn scan_doc_tree_with_files() {
        let tmp = make_temp_project();
        let docs = tmp.path().join("docs");
        fs::create_dir_all(&docs).expect("create docs dir");
        fs::write(docs.join("alpha.md"), "# Alpha").expect("write alpha");
        fs::write(docs.join("beta.md"), "# Beta").expect("write beta");

        let nodes = scan_doc_tree(&docs).expect("scan");
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].label, "Alpha");
        assert_eq!(nodes[1].label, "Beta");
    }

    #[test]
    fn scan_doc_tree_skips_hidden() {
        let tmp = make_temp_project();
        let docs = tmp.path().join("docs");
        fs::create_dir_all(&docs).expect("create docs dir");
        fs::write(docs.join("visible.md"), "# Visible").expect("write");
        fs::write(docs.join(".hidden.md"), "# Hidden").expect("write hidden");

        let nodes = scan_doc_tree(&docs).expect("scan");
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].label, "Visible");
    }

    #[test]
    fn scan_research_tree_skips_readme() {
        let tmp = make_temp_project();
        let research = tmp.path().join(paths::RESEARCH_DIR);
        fs::create_dir_all(&research).expect("create research dir");
        fs::write(research.join("README.md"), "# Readme").expect("write readme");
        fs::write(
            research.join("topic.md"),
            "---\ncategory: topic\n---\nContent.",
        )
        .expect("write topic");

        let nodes = scan_research_tree(&research).expect("scan");
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].label, "Topic Research");
    }

    #[test]
    fn scan_plan_tree_sorted_by_label() {
        let tmp = make_temp_project();
        let plans = tmp.path().join(paths::PLANS_DIR);
        fs::create_dir_all(&plans).expect("create plans dir");
        fs::write(
            plans.join("z-plan.md"),
            "---\ntitle: Zebra Plan\n---\nContent.",
        )
        .expect("write z");
        fs::write(
            plans.join("a-plan.md"),
            "---\ntitle: Alpha Plan\n---\nContent.",
        )
        .expect("write a");

        let nodes = scan_plan_tree(&plans).expect("scan");
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].label, "Alpha Plan");
        assert_eq!(nodes[1].label, "Zebra Plan");
    }

    // -----------------------------------------------------------------------
    // scan_orqa_artifact_dir and read_orqa_artifact tests
    // -----------------------------------------------------------------------

    #[test]
    fn scan_orqa_artifact_dir_missing_dir_returns_empty() {
        let tmp = make_temp_project();
        let result = scan_orqa_artifact_dir(&tmp.path().join("nonexistent"), ".orqa/milestones")
            .expect("should not error");
        assert!(result.is_empty());
    }

    #[test]
    fn scan_orqa_artifact_dir_parses_frontmatter() {
        let tmp = make_temp_project();
        let dir = tmp.path().join(".orqa").join("milestones");
        fs::create_dir_all(&dir).expect("create dir");
        fs::write(
            dir.join("MS-001.md"),
            "---\nid: MS-001\ntitle: Dogfooding\nstatus: active\n---\n# Milestone 1",
        )
        .expect("write file");

        let summaries =
            scan_orqa_artifact_dir(&dir, ".orqa/milestones").expect("scan");
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].name, "Dogfooding");
        assert_eq!(summaries[0].rel_path, ".orqa/milestones/MS-001.md");
    }

    #[test]
    fn scan_orqa_artifact_dir_skips_hidden_files() {
        let tmp = make_temp_project();
        let dir = tmp.path().join(".orqa").join("epics");
        fs::create_dir_all(&dir).expect("create dir");
        fs::write(dir.join("EPIC-001.md"), "---\ntitle: Real\n---\nContent.").expect("write");
        fs::write(dir.join(".hidden.md"), "---\ntitle: Hidden\n---\nContent.").expect("write hidden");

        let summaries = scan_orqa_artifact_dir(&dir, ".orqa/epics").expect("scan");
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].name, "Real");
    }

    #[test]
    fn scan_orqa_artifact_dir_sorted_by_numeric_id() {
        let tmp = make_temp_project();
        let dir = tmp.path().join(".orqa").join("epics");
        fs::create_dir_all(&dir).expect("create dir");
        fs::write(dir.join("EPIC-010.md"), "---\ntitle: Ten\n---\n").expect("write");
        fs::write(dir.join("EPIC-002.md"), "---\ntitle: Two\n---\n").expect("write");
        fs::write(dir.join("EPIC-001.md"), "---\ntitle: One\n---\n").expect("write");

        let summaries = scan_orqa_artifact_dir(&dir, ".orqa/epics").expect("scan");
        assert_eq!(summaries.len(), 3);
        assert_eq!(summaries[0].name, "One");
        assert_eq!(summaries[1].name, "Two");
        assert_eq!(summaries[2].name, "Ten");
    }

    #[test]
    fn read_orqa_artifact_returns_content_without_frontmatter() {
        let tmp = make_temp_project();
        let dir = tmp.path().join(".orqa").join("milestones");
        fs::create_dir_all(&dir).expect("create dir");
        fs::write(
            dir.join("MS-001.md"),
            "---\nid: MS-001\ntitle: Dogfooding\nstatus: active\n---\n# Milestone 1\n\nBody here.",
        )
        .expect("write file");

        let artifact = read_orqa_artifact(&dir, ".orqa/milestones", "MS-001").expect("read");
        assert_eq!(artifact.name, "Dogfooding");
        assert_eq!(artifact.rel_path, ".orqa/milestones/MS-001.md");
        assert!(artifact.content.contains("# Milestone 1"));
        assert!(artifact.content.contains("Body here."));
        // Frontmatter must not appear in content
        assert!(!artifact.content.contains("---"));
    }

    #[test]
    fn read_orqa_artifact_not_found() {
        let tmp = make_temp_project();
        let dir = tmp.path().join(".orqa").join("epics");
        fs::create_dir_all(&dir).expect("create dir");

        let result = read_orqa_artifact(&dir, ".orqa/epics", "EPIC-999");
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn scan_milestones_returns_summaries() {
        let tmp = make_temp_project();
        let dir = tmp.path().join(paths::MILESTONES_DIR);
        fs::create_dir_all(&dir).expect("create dir");
        fs::write(dir.join("MS-001.md"), "---\ntitle: First Milestone\n---\n").expect("write");

        let summaries = scan_milestones(tmp.path()).expect("scan");
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].name, "First Milestone");
    }

    #[test]
    fn scan_epics_returns_summaries() {
        let tmp = make_temp_project();
        let dir = tmp.path().join(paths::EPICS_DIR);
        fs::create_dir_all(&dir).expect("create dir");
        fs::write(dir.join("EPIC-001.md"), "---\ntitle: First Epic\n---\n").expect("write");

        let summaries = scan_epics(tmp.path()).expect("scan");
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].name, "First Epic");
    }

    #[test]
    fn scan_tasks_returns_summaries() {
        let tmp = make_temp_project();
        let dir = tmp.path().join(paths::TASKS_DIR);
        fs::create_dir_all(&dir).expect("create dir");
        fs::write(dir.join("TASK-001.md"), "---\ntitle: First Task\n---\n").expect("write");

        let summaries = scan_tasks(tmp.path()).expect("scan");
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].name, "First Task");
    }

    #[test]
    fn scan_ideas_returns_summaries() {
        let tmp = make_temp_project();
        let dir = tmp.path().join(paths::IDEAS_DIR);
        fs::create_dir_all(&dir).expect("create dir");
        fs::write(dir.join("IDEA-001.md"), "---\ntitle: First Idea\n---\n").expect("write");

        let summaries = scan_ideas(tmp.path()).expect("scan");
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].name, "First Idea");
    }

    #[test]
    fn scan_decisions_returns_summaries() {
        let tmp = make_temp_project();
        let dir = tmp.path().join(paths::DECISIONS_DIR);
        fs::create_dir_all(&dir).expect("create dir");
        fs::write(dir.join("AD-001.md"), "---\ntitle: Thick Backend\n---\n").expect("write");

        let summaries = scan_decisions(tmp.path()).expect("scan");
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].name, "Thick Backend");
    }

    #[test]
    fn scan_lessons_dir_returns_summaries() {
        let tmp = make_temp_project();
        let dir = tmp.path().join(paths::LESSONS_DIR);
        fs::create_dir_all(&dir).expect("create dir");
        fs::write(
            dir.join("IMPL-001.md"),
            "---\ntitle: Run vite optimize\n---\n",
        )
        .expect("write");

        let summaries = scan_lessons_dir(tmp.path()).expect("scan");
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].name, "Run vite optimize");
    }

    #[test]
    fn status_compliant_mapping() {
        assert_eq!(status_to_compliance("done"), ComplianceStatus::Compliant);
        assert_eq!(status_to_compliance("accepted"), ComplianceStatus::Compliant);
        assert_eq!(status_to_compliance("active"), ComplianceStatus::Compliant);
        assert_eq!(
            status_to_compliance("non_compliant"),
            ComplianceStatus::NonCompliant
        );
        assert_eq!(
            status_to_compliance("in-progress"),
            ComplianceStatus::Unknown
        );
    }

    #[test]
    fn numeric_id_from_path_extracts_correctly() {
        assert_eq!(numeric_id_from_path(".orqa/epics/EPIC-005.md"), 5);
        assert_eq!(numeric_id_from_path(".orqa/milestones/MS-001.md"), 1);
        assert_eq!(numeric_id_from_path(".orqa/decisions/AD-012.md"), 12);
        assert_eq!(numeric_id_from_path("no-number.md"), u64::MAX);
    }
}
