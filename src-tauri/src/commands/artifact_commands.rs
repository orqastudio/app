use std::path::{Path, PathBuf};

use tauri::State;

use crate::domain::artifact::{
    parse_doc_frontmatter, parse_plan_frontmatter, parse_research_frontmatter, Artifact,
    ArtifactSummary, ArtifactType, DocFrontmatter, DocNode,
};
use crate::error::OrqaError;
use crate::repo::{artifact_repo, project_repo};
use crate::state::AppState;

/// List artifacts for a project, optionally filtered by type.
#[tauri::command]
pub fn artifact_list(
    project_id: i64,
    artifact_type: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<ArtifactSummary>, OrqaError> {
    let type_filter = artifact_type
        .as_deref()
        .map(parse_artifact_type)
        .transpose()?;

    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    artifact_repo::list(&conn, project_id, type_filter.as_ref())
}

/// Get an artifact by its ID.
#[tauri::command]
pub fn artifact_get(artifact_id: i64, state: State<'_, AppState>) -> Result<Artifact, OrqaError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let mut artifact = artifact_repo::get(&conn, artifact_id)?;

    // Load content from disk if the project path is available
    if let Ok(project) = project_repo::get(&conn, artifact.project_id) {
        let full_path = Path::new(&project.path).join(&artifact.rel_path);
        if full_path.exists() {
            artifact.content = std::fs::read_to_string(&full_path).unwrap_or_default();
        }
    }

    Ok(artifact)
}

/// Get an artifact by project ID and relative path.
#[tauri::command]
pub fn artifact_get_by_path(
    project_id: i64,
    rel_path: String,
    state: State<'_, AppState>,
) -> Result<Artifact, OrqaError> {
    if rel_path.trim().is_empty() {
        return Err(OrqaError::Validation(
            "relative path cannot be empty".to_string(),
        ));
    }

    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let mut artifact = artifact_repo::get_by_path(&conn, project_id, &rel_path)?;

    // Load content from disk
    if let Ok(project) = project_repo::get(&conn, artifact.project_id) {
        let full_path = Path::new(&project.path).join(&artifact.rel_path);
        if full_path.exists() {
            artifact.content = std::fs::read_to_string(&full_path).unwrap_or_default();
        }
    }

    Ok(artifact)
}

/// Write artifact content to disk, creating parent directories as needed.
fn write_artifact_file(full_path: &Path, content: &str) -> Result<(), OrqaError> {
    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(full_path, content)?;
    Ok(())
}

/// Index a new artifact in the database and update its file metadata.
fn index_artifact(
    conn: &rusqlite::Connection,
    project_id: i64,
    parsed_type: &crate::domain::artifact::ArtifactType,
    rel_path: &str,
    name: &str,
    content: &str,
) -> Result<Artifact, OrqaError> {
    let file_size = content.len() as i64;
    let file_hash = format!("sha256:{:x}", simple_hash(content));

    let mut artifact =
        artifact_repo::create(conn, project_id, parsed_type, rel_path, name, content, None)?;

    artifact_repo::update(
        conn,
        artifact.id,
        &file_hash,
        file_size,
        &artifact.created_at,
    )?;

    artifact = artifact_repo::get(conn, artifact.id)?;
    artifact.content = content.to_string();
    Ok(artifact)
}

/// Create a new artifact, writing the file to disk and indexing in the database.
#[tauri::command]
pub fn artifact_create(
    project_id: i64,
    artifact_type: String,
    name: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<Artifact, OrqaError> {
    if name.trim().is_empty() {
        return Err(OrqaError::Validation(
            "artifact name cannot be empty".to_string(),
        ));
    }

    let parsed_type = parse_artifact_type(&artifact_type)?;
    let rel_path = derive_rel_path(&parsed_type, name.trim());

    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let project = project_repo::get(&conn, project_id)?;
    let full_path = Path::new(&project.path).join(&rel_path);

    write_artifact_file(&full_path, &content)?;
    index_artifact(
        &conn,
        project_id,
        &parsed_type,
        &rel_path,
        name.trim(),
        &content,
    )
}

/// Update an artifact's content, writing to disk and updating the database.
#[tauri::command]
pub fn artifact_update(
    artifact_id: i64,
    content: String,
    state: State<'_, AppState>,
) -> Result<Artifact, OrqaError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    // Get the artifact to find its path
    let artifact = artifact_repo::get(&conn, artifact_id)?;
    let project = project_repo::get(&conn, artifact.project_id)?;
    let full_path = Path::new(&project.path).join(&artifact.rel_path);

    // Write updated content to disk
    std::fs::write(&full_path, &content)?;

    // Update file metadata in DB
    let file_size = content.len() as i64;
    let file_hash = format!("sha256:{:x}", simple_hash(&content));
    let now = chrono_now_iso();

    artifact_repo::update(&conn, artifact_id, &file_hash, file_size, &now)?;

    // Update FTS index content
    conn.execute(
        "UPDATE artifacts_fts SET content = ?1 WHERE rowid = ?2",
        rusqlite::params![content, artifact_id],
    )?;

    // Re-fetch and return with content
    let mut updated = artifact_repo::get(&conn, artifact_id)?;
    updated.content = content;

    Ok(updated)
}

/// Delete an artifact, removing the file from disk and the database record.
#[tauri::command]
pub fn artifact_delete(artifact_id: i64, state: State<'_, AppState>) -> Result<(), OrqaError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    // Get the artifact to find its path
    let artifact = artifact_repo::get(&conn, artifact_id)?;
    let project = project_repo::get(&conn, artifact.project_id)?;
    let full_path = Path::new(&project.path).join(&artifact.rel_path);

    // Delete file from disk (ignore if already gone)
    if full_path.exists() {
        std::fs::remove_file(&full_path)?;
    }

    // Delete from database (handles FTS cleanup)
    artifact_repo::delete(&conn, artifact_id)
}

/// Look up the active project's filesystem path from the database.
fn active_project_path(state: &State<'_, AppState>) -> Result<String, OrqaError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let project = project_repo::get_active(&conn)?.ok_or_else(|| {
        OrqaError::NotFound("no active project — open a project first".to_string())
    })?;

    Ok(project.path)
}

/// Read a documentation file directly from the active project's docs/ directory on disk.
#[tauri::command]
pub fn doc_read(rel_path: String, state: State<'_, AppState>) -> Result<Artifact, OrqaError> {
    use crate::domain::artifact::ComplianceStatus;

    // Security: prevent path traversal
    if rel_path.contains("..") {
        return Err(OrqaError::Validation(
            "path traversal not allowed".to_string(),
        ));
    }

    let project_path = active_project_path(&state)?;
    let docs_path = Path::new(&project_path)
        .join("docs")
        .join(format!("{}.md", rel_path));

    if !docs_path.exists() {
        return Err(OrqaError::NotFound(format!("doc not found: {}", rel_path)));
    }

    let raw_content = std::fs::read_to_string(&docs_path)?;
    let (frontmatter, body) = parse_doc_frontmatter(&raw_content);

    // Use frontmatter title if available, otherwise derive from path
    let name = frontmatter.title.clone().unwrap_or_else(|| {
        rel_path
            .split('/')
            .next_back()
            .unwrap_or(&rel_path)
            .replace('-', " ")
    });

    let fs_metadata = std::fs::metadata(&docs_path).ok();
    let file_size = fs_metadata.as_ref().map(|m| m.len() as i64);
    let fm_json = serde_json::to_value(&frontmatter).ok();

    Ok(Artifact {
        id: 0,
        project_id: 0,
        artifact_type: ArtifactType::Doc,
        rel_path: format!("docs/{}.md", rel_path),
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

/// Read a single research document from `.orqa/research/`.
#[tauri::command]
pub fn research_read(rel_path: String, state: State<'_, AppState>) -> Result<Artifact, OrqaError> {
    use crate::domain::artifact::ComplianceStatus;

    if rel_path.contains("..") {
        return Err(OrqaError::Validation(
            "path traversal not allowed".to_string(),
        ));
    }

    let project_path = active_project_path(&state)?;
    let file_path = Path::new(&project_path)
        .join(".orqa")
        .join("research")
        .join(format!("{}.md", rel_path));

    if !file_path.exists() {
        return Err(OrqaError::NotFound(format!(
            "research doc not found: {}",
            rel_path
        )));
    }

    let raw_content = std::fs::read_to_string(&file_path)?;
    let (frontmatter, body) = parse_research_frontmatter(&raw_content);

    // Use category as a title fallback (e.g. "persistence" -> "Persistence Research")
    let name = frontmatter
        .category
        .as_deref()
        .map(|c| {
            let titled = c
                .split('-')
                .map(|w| {
                    let mut chars = w.chars();
                    match chars.next() {
                        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
                        None => String::new(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            format!("{titled} Research")
        })
        .unwrap_or_else(|| {
            rel_path
                .split('/')
                .next_back()
                .unwrap_or(&rel_path)
                .replace('-', " ")
        });

    let description = frontmatter.description.clone();
    let fs_metadata = std::fs::metadata(&file_path).ok();
    let file_size = fs_metadata.as_ref().map(|m| m.len() as i64);
    let fm_json = serde_json::to_value(&frontmatter).ok();

    Ok(Artifact {
        id: 0,
        project_id: 0,
        artifact_type: ArtifactType::Doc,
        rel_path: format!(".orqa/research/{}.md", rel_path),
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

/// Scan the active project's `docs/` directory tree and return a hierarchical structure.
///
/// Returns an empty vec if `docs/` does not exist (no error).
#[tauri::command]
pub fn doc_tree_scan(state: State<'_, AppState>) -> Result<Vec<DocNode>, OrqaError> {
    let project_path = active_project_path(&state)?;
    let docs_dir = Path::new(&project_path).join("docs");
    if !docs_dir.is_dir() {
        return Ok(Vec::new());
    }

    scan_directory(&docs_dir, &docs_dir)
}

/// Scan the active project's `.orqa/research/` directory tree and return a hierarchical structure.
///
/// Uses `ResearchFrontmatter` to extract labels from the research-specific YAML schema for leaf
/// nodes. Subdirectories produce `DocNode` entries with `children` (label = humanized directory
/// name, path = None). This mirrors the behaviour of `doc_tree_scan` / `scan_directory`.
/// Returns an empty vec if `.orqa/research/` does not exist (no error).
#[tauri::command]
pub fn research_tree_scan(state: State<'_, AppState>) -> Result<Vec<DocNode>, OrqaError> {
    let project_path = active_project_path(&state)?;
    let research_dir = Path::new(&project_path).join(".orqa").join("research");
    if !research_dir.is_dir() {
        return Ok(Vec::new());
    }

    scan_research_directory(&research_dir, &research_dir)
}

/// Recursively scan a research directory and build a sorted list of `DocNode` entries.
///
/// Subdirectories become directory nodes with `children`. `.md` files (except README and hidden
/// entries) become leaf nodes using `ResearchFrontmatter` for the label.
/// Directories come first (alphabetically), then `.md` files (alphabetically).
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
        let rel = relative_research_path(&path, research_root);
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        let (fm, _) = parse_research_frontmatter(&content);

        // Derive label: use category (title-cased) + " Research", or fall back to filename.
        let label = fm
            .category
            .as_deref()
            .map(|c| {
                let titled: String = c
                    .split('-')
                    .map(|w| {
                        let mut chars = w.chars();
                        match chars.next() {
                            Some(first) => first.to_uppercase().to_string() + chars.as_str(),
                            None => String::new(),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("{titled} Research")
            })
            .unwrap_or_else(|| humanize_name(&name));

        // Convert research frontmatter to DocFrontmatter for the tree node.
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

/// Build the relative path from `research_root` without the `.md` extension.
///
/// Example: `.orqa/research/mvp/persistence.md` -> `"mvp/persistence"`.
fn relative_research_path(file: &Path, research_root: &Path) -> String {
    let rel = file
        .strip_prefix(research_root)
        .unwrap_or(file)
        .with_extension("");
    // Normalise to forward slashes (important on Windows)
    rel.to_string_lossy().replace('\\', "/")
}

/// Scan the active project's `.orqa/plans/` directory and return a flat list of plan docs.
///
/// Uses `PlanFrontmatter` to extract labels from the plan-specific YAML schema.
/// Returns an empty vec if `.orqa/plans/` does not exist (no error).
#[tauri::command]
pub fn plan_tree_scan(state: State<'_, AppState>) -> Result<Vec<DocNode>, OrqaError> {
    let project_path = active_project_path(&state)?;
    let plans_dir = Path::new(&project_path).join(".orqa").join("plans");
    if !plans_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut nodes = Vec::new();
    for entry in std::fs::read_dir(&plans_dir)? {
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

        // Derive label: use title field if available, otherwise humanize the filename
        let label = fm.title.clone().unwrap_or_else(|| humanize_name(&name));

        // Convert plan frontmatter to DocFrontmatter for the tree node
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

/// Read a single implementation plan from `.orqa/plans/`.
#[tauri::command]
pub fn plan_read(rel_path: String, state: State<'_, AppState>) -> Result<Artifact, OrqaError> {
    use crate::domain::artifact::ComplianceStatus;

    if rel_path.contains("..") {
        return Err(OrqaError::Validation(
            "path traversal not allowed".to_string(),
        ));
    }

    let project_path = active_project_path(&state)?;
    let file_path = Path::new(&project_path)
        .join(".orqa")
        .join("plans")
        .join(format!("{}.md", rel_path));

    if !file_path.exists() {
        return Err(OrqaError::NotFound(format!("plan not found: {}", rel_path)));
    }

    let raw_content = std::fs::read_to_string(&file_path)?;
    let (frontmatter, body) = parse_plan_frontmatter(&raw_content);

    // Use title field if available, otherwise humanize the filename
    let file_name = file_path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let name = frontmatter
        .title
        .clone()
        .unwrap_or_else(|| humanize_name(&file_name));

    let fs_metadata = std::fs::metadata(&file_path).ok();
    let file_size = fs_metadata.as_ref().map(|m| m.len() as i64);
    let fm_json = serde_json::to_value(&frontmatter).ok();

    Ok(Artifact {
        id: 0,
        project_id: 0,
        artifact_type: ArtifactType::Doc,
        rel_path: format!(".orqa/plans/{}.md", rel_path),
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

/// List governance artifacts (agents, rules, skills, hooks) by scanning disk.
///
/// Returns file-based summaries — the database is not consulted.
#[tauri::command]
pub fn governance_list(
    artifact_type: String,
    state: State<'_, AppState>,
) -> Result<Vec<ArtifactSummary>, OrqaError> {
    let parsed = parse_artifact_type(&artifact_type)?;
    if parsed == ArtifactType::Doc {
        return Err(OrqaError::Validation(
            "use doc_tree_scan for docs".to_string(),
        ));
    }

    let project_path = active_project_path(&state)?;
    let root = Path::new(&project_path);
    list_governance_files(root, &parsed)
}

/// Infer an `ArtifactType` from a `.claude/` relative path prefix.
fn infer_artifact_type_from_path(rel_path: &str) -> ArtifactType {
    if rel_path.starts_with(".claude/agents") {
        ArtifactType::Agent
    } else if rel_path.starts_with(".claude/rules") {
        ArtifactType::Rule
    } else if rel_path.starts_with(".claude/skills") {
        ArtifactType::Skill
    } else if rel_path.starts_with(".claude/hooks") {
        ArtifactType::Hook
    } else {
        ArtifactType::Doc
    }
}

/// Build an in-memory `Artifact` struct from a file on disk (no DB record).
fn artifact_from_file(
    full_path: &Path,
    rel_path: String,
    artifact_type: ArtifactType,
) -> Result<Artifact, OrqaError> {
    use crate::domain::artifact::ComplianceStatus;

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

/// Read a single governance artifact from disk by its relative path.
#[tauri::command]
pub fn governance_read(
    rel_path: String,
    state: State<'_, AppState>,
) -> Result<Artifact, OrqaError> {
    if rel_path.contains("..") {
        return Err(OrqaError::Validation(
            "path traversal not allowed".to_string(),
        ));
    }

    let project_path = active_project_path(&state)?;
    let full_path = Path::new(&project_path).join(&rel_path);

    if !full_path.exists() {
        return Err(OrqaError::NotFound(format!(
            "artifact not found: {}",
            rel_path
        )));
    }

    let artifact_type = infer_artifact_type_from_path(&rel_path);
    artifact_from_file(&full_path, rel_path, artifact_type)
}

/// Map an `ArtifactType` to its `.claude/` subdirectory path.
fn governance_dir(root: &Path, artifact_type: &ArtifactType) -> Option<PathBuf> {
    match artifact_type {
        ArtifactType::Agent => Some(root.join(".claude").join("agents")),
        ArtifactType::Rule => Some(root.join(".claude").join("rules")),
        ArtifactType::Skill => Some(root.join(".claude").join("skills")),
        ArtifactType::Hook => Some(root.join(".claude").join("hooks")),
        ArtifactType::Doc => None,
    }
}

/// Try to build an `ArtifactSummary` from a single directory entry.
///
/// Returns `None` if the entry should be skipped (wrong type, hidden, invalid extension).
fn summary_from_entry(
    entry: &std::fs::DirEntry,
    artifact_type: &ArtifactType,
) -> Result<Option<ArtifactSummary>, OrqaError> {
    use crate::domain::artifact::ComplianceStatus;

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
                    ArtifactType::Agent => format!(".claude/agents/{}", name),
                    ArtifactType::Rule => format!(".claude/rules/{}", name),
                    ArtifactType::Hook => format!(".claude/hooks/{}", name),
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

/// Scan a governance directory and return sorted artifact summaries.
fn list_governance_files(
    root: &Path,
    artifact_type: &ArtifactType,
) -> Result<Vec<ArtifactSummary>, OrqaError> {
    let Some(dir) = governance_dir(root, artifact_type) else {
        return Ok(Vec::new());
    };

    if !dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut summaries = Vec::new();
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        if let Some(summary) = summary_from_entry(&entry, artifact_type)? {
            summaries.push(summary);
        }
    }

    summaries.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(summaries)
}

/// Recursively scan a directory and build a sorted list of `DocNode` entries.
///
/// Hidden entries (starting with `.` or `_`) are skipped.
/// Directories come first (alphabetically), then `.md` files (alphabetically).
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
        let rel = relative_doc_path(&path, docs_root);
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
fn humanize_name(filename: &str) -> String {
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

/// Parse a string into an `ArtifactType`, returning a validation error for unknown types.
fn parse_artifact_type(s: &str) -> Result<ArtifactType, OrqaError> {
    match s {
        "agent" => Ok(ArtifactType::Agent),
        "rule" => Ok(ArtifactType::Rule),
        "skill" => Ok(ArtifactType::Skill),
        "hook" => Ok(ArtifactType::Hook),
        "doc" => Ok(ArtifactType::Doc),
        other => Err(OrqaError::Validation(format!(
            "unknown artifact type: {other} (valid: agent, rule, skill, hook, doc)"
        ))),
    }
}

/// Derive the relative path for an artifact based on its type and name.
fn derive_rel_path(artifact_type: &ArtifactType, name: &str) -> String {
    let sanitized = name.replace(' ', "-").to_lowercase();

    match artifact_type {
        ArtifactType::Agent => format!(".claude/agents/{sanitized}.md"),
        ArtifactType::Rule => format!(".claude/rules/{sanitized}.md"),
        ArtifactType::Skill => format!(".claude/skills/{sanitized}/SKILL.md"),
        ArtifactType::Hook => format!(".claude/hooks/{sanitized}.sh"),
        ArtifactType::Doc => format!("docs/{sanitized}.md"),
    }
}

/// Simple hash for file content (not cryptographic, just for change detection).
fn simple_hash(content: &str) -> u64 {
    let mut hash: u64 = 5381;
    for byte in content.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(u64::from(byte));
    }
    hash
}

/// Generate a basic ISO 8601 timestamp (approximation without chrono crate).
fn chrono_now_iso() -> String {
    let now = std::time::SystemTime::now();
    let duration = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();

    // Basic UTC timestamp calculation
    let days = secs / 86400;
    let remaining = secs % 86400;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;
    let seconds = remaining % 60;

    // Calculate year/month/day from days since epoch (1970-01-01)
    let (year, month, day) = days_to_ymd(days);

    format!("{year:04}-{month:02}-{day:02}T{hours:02}:{minutes:02}:{seconds:02}Z")
}

/// Convert days since Unix epoch to (year, month, day).
fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    // Simplified calendar calculation
    let mut y = 1970;
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

    let mut m = 0;
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
    use crate::db::init_memory_db;
    use crate::repo::project_repo;

    fn setup() -> rusqlite::Connection {
        let conn = init_memory_db().expect("db init");
        project_repo::create(&conn, "test", "/test", None).expect("create project");
        conn
    }

    #[test]
    fn parse_artifact_type_valid() {
        assert!(matches!(
            parse_artifact_type("agent"),
            Ok(ArtifactType::Agent)
        ));
        assert!(matches!(
            parse_artifact_type("rule"),
            Ok(ArtifactType::Rule)
        ));
        assert!(matches!(
            parse_artifact_type("skill"),
            Ok(ArtifactType::Skill)
        ));
        assert!(matches!(
            parse_artifact_type("hook"),
            Ok(ArtifactType::Hook)
        ));
        assert!(matches!(parse_artifact_type("doc"), Ok(ArtifactType::Doc)));
    }

    #[test]
    fn parse_artifact_type_invalid() {
        let result = parse_artifact_type("unknown");
        assert!(matches!(result, Err(OrqaError::Validation(_))));
    }

    #[test]
    fn derive_rel_path_agent() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Agent, "backend-engineer"),
            ".claude/agents/backend-engineer.md"
        );
    }

    #[test]
    fn derive_rel_path_rule() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Rule, "no-stubs"),
            ".claude/rules/no-stubs.md"
        );
    }

    #[test]
    fn derive_rel_path_skill() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Skill, "chunkhound"),
            ".claude/skills/chunkhound/SKILL.md"
        );
    }

    #[test]
    fn derive_rel_path_hook() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Hook, "pre-commit"),
            ".claude/hooks/pre-commit.sh"
        );
    }

    #[test]
    fn derive_rel_path_doc() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Doc, "architecture"),
            "docs/architecture.md"
        );
    }

    #[test]
    fn derive_rel_path_sanitizes_spaces() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Rule, "No Stubs Rule"),
            ".claude/rules/no-stubs-rule.md"
        );
    }

    #[test]
    fn simple_hash_deterministic() {
        let h1 = simple_hash("hello");
        let h2 = simple_hash("hello");
        assert_eq!(h1, h2);
    }

    #[test]
    fn simple_hash_varies_for_different_input() {
        let h1 = simple_hash("hello");
        let h2 = simple_hash("world");
        assert_ne!(h1, h2);
    }

    #[test]
    fn artifact_list_empty() {
        let conn = setup();
        let artifacts = artifact_repo::list(&conn, 1, None).expect("list");
        assert!(artifacts.is_empty());
    }

    #[test]
    fn artifact_list_filtered() {
        let conn = setup();
        artifact_repo::create(
            &conn,
            1,
            &ArtifactType::Rule,
            "r1.md",
            "rule1",
            "content",
            None,
        )
        .expect("create");
        artifact_repo::create(
            &conn,
            1,
            &ArtifactType::Agent,
            "a1.md",
            "agent1",
            "content",
            None,
        )
        .expect("create");

        let all = artifact_repo::list(&conn, 1, None).expect("list all");
        assert_eq!(all.len(), 2);

        let rules = artifact_repo::list(&conn, 1, Some(&ArtifactType::Rule)).expect("list rules");
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn artifact_get_by_id() {
        let conn = setup();
        let created = artifact_repo::create(
            &conn,
            1,
            &ArtifactType::Rule,
            "test.md",
            "test-rule",
            "# Test",
            None,
        )
        .expect("create");

        let fetched = artifact_repo::get(&conn, created.id).expect("get");
        assert_eq!(fetched.name, "test-rule");
    }

    #[test]
    fn artifact_get_nonexistent() {
        let conn = setup();
        let result = artifact_repo::get(&conn, 999);
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn artifact_get_by_path_works() {
        let conn = setup();
        artifact_repo::create(
            &conn,
            1,
            &ArtifactType::Agent,
            ".claude/agents/test.md",
            "test-agent",
            "# Agent",
            None,
        )
        .expect("create");

        let fetched =
            artifact_repo::get_by_path(&conn, 1, ".claude/agents/test.md").expect("get_by_path");
        assert_eq!(fetched.name, "test-agent");
    }

    #[test]
    fn artifact_delete_works() {
        let conn = setup();
        let artifact = artifact_repo::create(
            &conn,
            1,
            &ArtifactType::Rule,
            "delete-me.md",
            "delete-me",
            "content",
            None,
        )
        .expect("create");

        artifact_repo::delete(&conn, artifact.id).expect("delete");
        let result = artifact_repo::get(&conn, artifact.id);
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn empty_name_validation() {
        let name = "   ";
        assert!(name.trim().is_empty());
    }

    #[test]
    fn chrono_now_iso_format() {
        let ts = chrono_now_iso();
        // Should match pattern like "2026-03-03T12:00:00Z"
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
}
