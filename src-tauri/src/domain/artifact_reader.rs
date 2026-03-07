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
                    rel_path: format!(".claude/skills/{}/SKILL.md", name),
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
                    ArtifactType::Agent => format!(".claude/agents/{name}"),
                    ArtifactType::Rule => format!(".claude/rules/{name}"),
                    ArtifactType::Hook => format!(".claude/hooks/{name}"),
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
// Private helpers
// ---------------------------------------------------------------------------

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

/// Title-case a hyphenated string (e.g. `"my-category"` -> `"My Category"`).
fn title_case_hyphenated(s: &str) -> String {
    s.split('-').map(title_case_word).collect::<Vec<_>>().join(" ")
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
}
