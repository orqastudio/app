use std::collections::HashMap;
use std::path::Path;

use crate::domain::artifact::{
    DocNode, FilterableField, NavGroup, NavTree, NavType, NavigationConfig, SortableField,
};
use crate::domain::project_settings::{ArtifactEntry, ArtifactTypeConfig};
use crate::error::OrqaError;

// ---------------------------------------------------------------------------
// Navigation tree scanner
// ---------------------------------------------------------------------------

/// Build the unified navigation tree from the project's artifacts config.
///
/// Each `ArtifactEntry::Type` in the config produces a `NavGroup` wrapping a
/// single `NavType`. Each `ArtifactEntry::Group` produces a `NavGroup` with one
/// `NavType` per child. The `NavTree` structure (groups → types → nodes) is
/// unchanged — only the source of truth changes from folder-guessing to config.
///
/// Returns an empty `NavTree` when `entries` is empty or the project has no
/// matching directories.
pub fn artifact_scan_tree(
    project_path: &Path,
    entries: &[ArtifactEntry],
) -> Result<NavTree, OrqaError> {
    if entries.is_empty() {
        return Ok(NavTree { groups: vec![] });
    }

    let mut groups: Vec<NavGroup> = Vec::new();

    for entry in entries {
        match entry {
            ArtifactEntry::Group {
                key,
                label,
                icon,
                children,
            } => {
                let group = scan_group_from_config(
                    project_path,
                    key,
                    label.as_deref(),
                    icon.as_deref(),
                    children,
                )?;
                if !group.types.is_empty() {
                    groups.push(group);
                }
            }
            ArtifactEntry::Type(type_cfg) => {
                // Wrap the direct type in a synthetic group so the NavTree
                // structure stays uniform (NavGroup → NavType → DocNode).
                let nav_type = scan_type_from_config(project_path, type_cfg)?;
                if !nav_type.nodes.is_empty() {
                    let group = NavGroup {
                        label: nav_type.label.clone(),
                        description: String::new(),
                        icon: nav_type.icon.clone(),
                        sort: i64::MAX,
                        path: type_cfg.path.clone(),
                        readme_content: read_readme_content(&project_path.join(&type_cfg.path)),
                        types: vec![nav_type],
                    };
                    groups.push(group);
                }
            }
        }
    }

    Ok(NavTree { groups })
}

// ---------------------------------------------------------------------------
// Config-driven group and type scanning
// ---------------------------------------------------------------------------

/// Build a `NavGroup` from a config group entry.
///
/// Scans each child's path for markdown files to produce `NavType` entries.
/// Label and icon are resolved in priority order:
/// 1. Config value (explicit override)
/// 2. README.md frontmatter in the group directory
/// 3. Humanized key name / "folder" icon fallback
fn scan_group_from_config(
    project_path: &Path,
    key: &str,
    config_label: Option<&str>,
    config_icon: Option<&str>,
    children: &[ArtifactTypeConfig],
) -> Result<NavGroup, OrqaError> {
    let mut types: Vec<NavType> = Vec::new();

    for child in children {
        let nav_type = scan_type_from_config(project_path, child)?;
        // Include the type even if empty — callers decide whether to drop empty groups.
        types.push(nav_type);
    }

    // Drop types with no nodes so the sidebar doesn't show empty sections.
    types.retain(|t| !t.nodes.is_empty());

    let group_dir = project_path.join(
        children
            .first()
            .map_or(key, |c| c.path.as_str())
            .split('/')
            .take(2)
            .collect::<Vec<_>>()
            .join("/"),
    );

    let readme_fm = read_readme_frontmatter(&group_dir);

    // Resolve label: config override → README frontmatter → humanized key
    let label = config_label
        .map(str::to_owned)
        .or_else(|| readme_fm.as_ref().and_then(|fm| fm.label.clone()))
        .unwrap_or_else(|| humanize_name(key));

    // Resolve icon: config override → README frontmatter → "folder"
    let icon = config_icon
        .map(str::to_owned)
        .or_else(|| readme_fm.as_ref().and_then(|fm| fm.icon.clone()))
        .unwrap_or_else(|| "folder".to_string());

    Ok(NavGroup {
        label,
        description: String::new(),
        icon,
        sort: i64::MAX,
        path: format!(".orqa/{key}"),
        readme_content: read_readme_content(&group_dir),
        types,
    })
}

/// Build a `NavType` from a single artifact type config entry.
///
/// Walks the configured path for `.md` files and populates `DocNode` entries.
/// Label, icon, and description are resolved in priority order:
/// 1. Config value (explicit override)
/// 2. README.md frontmatter in the type directory
/// 3. Humanized key name / "file" icon / empty description fallback
fn scan_type_from_config(
    project_path: &Path,
    cfg: &ArtifactTypeConfig,
) -> Result<NavType, OrqaError> {
    let type_dir = project_path.join(&cfg.path);
    let readme_content = read_readme_content(&type_dir);
    let readme_fm = read_readme_frontmatter(&type_dir);
    let nodes = scan_type_nodes(&type_dir, &cfg.key, &cfg.path)?;

    // Resolve label: config override → README frontmatter → humanized key
    let label = cfg
        .label
        .clone()
        .or_else(|| readme_fm.as_ref().and_then(|fm| fm.label.clone()))
        .unwrap_or_else(|| humanize_name(&cfg.key));

    // Resolve icon: config override → README frontmatter → "file"
    let icon = cfg
        .icon
        .clone()
        .or_else(|| readme_fm.as_ref().and_then(|fm| fm.icon.clone()))
        .unwrap_or_else(|| "file".to_string());

    // README description is the fallback when config provides none.
    let description = readme_fm
        .as_ref()
        .and_then(|fm| fm.description.clone())
        .unwrap_or_default();

    let (filterable_fields, sortable_fields) = read_schema_fields(&type_dir);
    let navigation_config = read_navigation_config(&type_dir);

    Ok(NavType {
        label,
        description,
        icon,
        sort: i64::MAX,
        path: cfg.path.clone(),
        readme_content,
        nodes,
        filterable_fields,
        sortable_fields,
        navigation_config,
    })
}

// ---------------------------------------------------------------------------
// File scanning
// ---------------------------------------------------------------------------

/// Scan artifact files within a type directory and return sorted `DocNode` entries.
///
/// Skills are special: they are subdirectories containing `SKILL.md`.
/// Hooks are special: they may contain `.sh` files in addition to `.md` files.
/// All other types recursively walk subdirectories, building a tree of `DocNode`
/// entries. Directory nodes have `children` set; file nodes have `path` set.
/// Hidden files (starting with `.` or `_`) are skipped at every level.
fn scan_type_nodes(
    type_dir: &Path,
    type_key: &str,
    type_path: &str,
) -> Result<Vec<DocNode>, OrqaError> {
    if type_key == "skills" {
        scan_skills_nodes(type_dir, type_path)
    } else if type_key == "hooks" {
        scan_hooks_nodes(type_dir, type_path)
    } else {
        scan_recursive_nodes(type_dir, type_path)
    }
}

/// Scan a skills directory: each subdirectory that contains `SKILL.md` becomes a node.
fn scan_skills_nodes(type_dir: &Path, type_path: &str) -> Result<Vec<DocNode>, OrqaError> {
    let mut nodes: Vec<DocNode> = Vec::new();

    let Ok(entries) = std::fs::read_dir(type_dir) else {
        return Ok(nodes);
    };

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if name.starts_with('.') || name.starts_with('_') {
            continue;
        }

        // Use path().is_dir() instead of file_type().is_dir() to follow symlinks
        if !entry.path().is_dir() {
            continue;
        }

        let skill_file = entry.path().join("SKILL.md");
        if !skill_file.exists() {
            continue;
        }

        let content = std::fs::read_to_string(&skill_file).unwrap_or_default();
        let (_, title, status, description) = extract_basic_frontmatter(&content);
        let label = title.unwrap_or_else(|| humanize_name(&name));
        let frontmatter = extract_full_frontmatter(&content);
        nodes.push(DocNode {
            label,
            path: Some(format!("{type_path}/{name}/SKILL.md")),
            children: None,
            frontmatter,
            status,
            description,
            icon: None,
        });
    }

    nodes.sort_by(|a, b| a.label.cmp(&b.label));
    Ok(nodes)
}

/// Scan a hooks directory: `.sh` and `.md` files (excluding `README.md`) each become a node.
///
/// Shell scripts have no YAML frontmatter, so their label is derived from the filename.
fn scan_hooks_nodes(type_dir: &Path, type_path: &str) -> Result<Vec<DocNode>, OrqaError> {
    let mut nodes: Vec<DocNode> = Vec::new();

    let Ok(entries) = std::fs::read_dir(type_dir) else {
        return Ok(nodes);
    };

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if name.starts_with('.') || name.starts_with('_') {
            continue;
        }

        if name.eq_ignore_ascii_case("README.MD") || name.eq_ignore_ascii_case("README") {
            continue;
        }

        // Use path() methods instead of file_type() to follow symlinks
        if !entry.path().is_file() {
            continue;
        }

        let is_md = name.ends_with(".md");
        let is_sh = name.ends_with(".sh");
        if !is_md && !is_sh {
            continue;
        }

        let (label, status, description, frontmatter) = if is_md {
            let content = std::fs::read_to_string(entry.path()).unwrap_or_default();
            let (_, title, status, description) = extract_basic_frontmatter(&content);
            let stem = name.trim_end_matches(".md");
            let label = title.unwrap_or_else(|| humanize_name(stem));
            let frontmatter = extract_full_frontmatter(&content);
            (label, status, description, frontmatter)
        } else {
            // .sh files: no frontmatter; humanize the filename for the label.
            let label = humanize_name(&name);
            (label, None, None, None)
        };

        nodes.push(DocNode {
            label,
            path: Some(format!("{type_path}/{name}")),
            children: None,
            frontmatter,
            status,
            description,
            icon: None,
        });
    }

    nodes.sort_by(|a, b| a.label.cmp(&b.label));
    Ok(nodes)
}

/// Recursively scan a directory, returning a flat+tree mix of `DocNode` entries.
///
/// - `.md` files become leaf nodes with `path` set relative to `current_path`.
/// - Subdirectories become branch nodes with `children` set.
/// - Empty subdirectories (no `.md` files anywhere inside) are omitted.
/// - `README.md` is skipped at every level.
/// - Hidden entries (starting with `.` or `_`) are skipped.
/// - Directories are sorted alphabetically; files within each level are sorted alphabetically.
fn scan_recursive_nodes(dir: &Path, current_path: &str) -> Result<Vec<DocNode>, OrqaError> {
    let mut file_nodes: Vec<DocNode> = Vec::new();
    let mut dir_nodes: Vec<DocNode> = Vec::new();

    let Ok(entries) = std::fs::read_dir(dir) else {
        return Ok(vec![]);
    };

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if name.starts_with('.') || name.starts_with('_') {
            continue;
        }

        if name.eq_ignore_ascii_case("README.MD") {
            continue;
        }

        // Use path() methods instead of file_type() to follow symlinks
        let entry_path = entry.path();

        if entry_path.is_file() {
            if !name.ends_with(".md") {
                continue;
            }
            let content = std::fs::read_to_string(&entry_path).unwrap_or_default();
            let (_, title, status, description) = extract_basic_frontmatter(&content);
            let stem = name.trim_end_matches(".md");
            let label = title.unwrap_or_else(|| humanize_name(stem));
            let frontmatter = extract_full_frontmatter(&content);
            file_nodes.push(DocNode {
                label,
                path: Some(format!("{current_path}/{stem}.md")),
                children: None,
                frontmatter,
                status,
                description,
                icon: None,
            });
        } else if entry_path.is_dir() {
            let child_path = format!("{current_path}/{name}");
            let children = scan_recursive_nodes(&entry_path, &child_path)?;
            if children.is_empty() {
                // Skip directories with no .md content anywhere inside.
                continue;
            }
            // Enrich the directory node with README frontmatter if present.
            let dir_readme = read_readme_frontmatter(&entry_path);
            let dir_label = dir_readme
                .as_ref()
                .and_then(|fm| fm.label.clone())
                .unwrap_or_else(|| humanize_name(&name));
            let dir_description = dir_readme.as_ref().and_then(|fm| fm.description.clone());
            let dir_icon = dir_readme.as_ref().and_then(|fm| fm.icon.clone());
            dir_nodes.push(DocNode {
                label: dir_label,
                path: None,
                children: Some(children),
                frontmatter: None,
                status: None,
                description: dir_description,
                icon: dir_icon,
            });
        }
    }

    file_nodes.sort_by(|a, b| a.label.cmp(&b.label));
    dir_nodes.sort_by(|a, b| a.label.cmp(&b.label));

    // Directories first, then files — consistent with file explorer conventions.
    let mut nodes = dir_nodes;
    nodes.extend(file_nodes);
    Ok(nodes)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Read the raw content of a `README.md` in `dir`, or return an empty string.
fn read_readme_content(dir: &Path) -> String {
    let readme = dir.join("README.md");
    if readme.exists() {
        std::fs::read_to_string(&readme).unwrap_or_default()
    } else {
        String::new()
    }
}

/// Parse the `NavReadme` frontmatter from a `README.md` in `dir`.
///
/// Returns `None` when the README does not exist or has no parseable frontmatter.
fn read_readme_frontmatter(dir: &Path) -> Option<crate::domain::artifact::NavReadme> {
    use crate::domain::artifact::parse_frontmatter;

    let readme = dir.join("README.md");
    let content = std::fs::read_to_string(&readme).ok()?;
    let (fm, _): (crate::domain::artifact::NavReadme, _) = parse_frontmatter(&content);
    // Return Some only when at least one meaningful field was extracted.
    if fm.icon.is_some() || fm.description.is_some() || fm.label.is_some() || fm.sort.is_some() {
        Some(fm)
    } else {
        None
    }
}

/// Extract the four most common frontmatter fields without committing to a specific schema.
///
/// Returns `(id, title, status, description)` — all `Option<String>`.
fn extract_basic_frontmatter(
    content: &str,
) -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
) {
    use crate::domain::artifact::extract_frontmatter;

    let (fm_text, _) = extract_frontmatter(content);
    let Some(yaml) = fm_text else {
        return (None, None, None, None);
    };

    // Use the serde_yaml Value type to avoid coupling to any specific struct.
    let value: serde_yaml::Value = serde_yaml::from_str(&yaml).unwrap_or(serde_yaml::Value::Null);

    let get_str = |key: &str| -> Option<String> {
        value.get(key).and_then(|v| v.as_str()).map(str::to_owned)
    };

    (
        get_str("id"),
        get_str("title"),
        get_str("status"),
        get_str("description"),
    )
}

/// Extract all scalar YAML frontmatter values as a JSON-compatible map.
///
/// Arrays, objects, and null values are excluded. Returns `None` when the file
/// has no parseable frontmatter. Used to populate `DocNode::frontmatter`.
fn extract_full_frontmatter(content: &str) -> Option<HashMap<String, serde_json::Value>> {
    use crate::domain::artifact::extract_frontmatter;

    let (fm_text, _) = extract_frontmatter(content);
    let yaml = fm_text?;
    let value: serde_yaml::Value = serde_yaml::from_str(&yaml).ok()?;
    let mapping = value.as_mapping()?;

    let mut map = HashMap::new();
    for (k, v) in mapping {
        let key = k.as_str()?.to_owned();
        let json_val = match v {
            serde_yaml::Value::String(s) => serde_json::Value::String(s.clone()),
            serde_yaml::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    serde_json::Value::Number(i.into())
                } else if let Some(f) = n.as_f64() {
                    serde_json::json!(f)
                } else {
                    continue;
                }
            }
            serde_yaml::Value::Bool(b) => serde_json::Value::Bool(*b),
            // Skip arrays, mappings, and nulls — they are not useful for filtering/sorting.
            _ => continue,
        };
        map.insert(key, json_val);
    }

    if map.is_empty() {
        None
    } else {
        Some(map)
    }
}

/// Read `schema.json` from a type directory and extract filterable and sortable fields.
///
/// - Properties with an `enum` array become `FilterableField` entries (array order preserved).
/// - Properties with `type: "string"` and `format: "date"` become `SortableField` with `field_type: "date"`.
/// - The `title` property becomes a `SortableField` with `field_type: "string"`.
/// - The `id` property is skipped.
///
/// Returns empty vecs when `schema.json` does not exist or is not parseable.
fn read_schema_fields(type_dir: &Path) -> (Vec<FilterableField>, Vec<SortableField>) {
    let schema_path = type_dir.join("schema.json");
    let Ok(content) = std::fs::read_to_string(&schema_path) else {
        return (vec![], vec![]);
    };
    let Ok(schema) = serde_json::from_str::<serde_json::Value>(&content) else {
        return (vec![], vec![]);
    };

    let Some(properties) = schema.get("properties").and_then(|p| p.as_object()) else {
        return (vec![], vec![]);
    };

    let mut filterable: Vec<FilterableField> = Vec::new();
    let mut sortable: Vec<SortableField> = Vec::new();

    for (name, prop) in properties {
        if name == "id" {
            continue;
        }
        extract_filterable_field(name, prop, &mut filterable);
        extract_sortable_field(name, prop, &mut sortable);
    }

    (filterable, sortable)
}

/// Extract a `FilterableField` from a schema property if it has an `enum` array.
fn extract_filterable_field(name: &str, prop: &serde_json::Value, out: &mut Vec<FilterableField>) {
    let Some(enum_arr) = prop.get("enum").and_then(|e| e.as_array()) else {
        return;
    };
    let values: Vec<String> = enum_arr
        .iter()
        .filter_map(|v| v.as_str().map(str::to_owned))
        .collect();
    if !values.is_empty() {
        out.push(FilterableField {
            name: name.to_owned(),
            values,
        });
    }
}

/// Extract a `SortableField` from a schema property if it is a date string or the `title` field.
fn extract_sortable_field(name: &str, prop: &serde_json::Value, out: &mut Vec<SortableField>) {
    if name == "title" {
        out.push(SortableField {
            name: name.to_owned(),
            field_type: "string".to_owned(),
        });
        return;
    }

    // Accept {"type": "string", "format": "date"} or {"type": ["string", "null"], "format": "date"}
    let is_string_type = prop
        .get("type")
        .is_some_and(|t| {
            (t.as_str() == Some("string"))
                || t.as_array()
                    .is_some_and(|arr| arr.iter().any(|v| v.as_str() == Some("string")))
        });

    let is_date_format = prop.get("format").and_then(|f| f.as_str()) == Some("date");

    if is_string_type && is_date_format {
        out.push(SortableField {
            name: name.to_owned(),
            field_type: "date".to_owned(),
        });
    }
}

/// Read `_navigation.json` from a type directory as a `NavigationConfig`.
///
/// Returns `None` when the file does not exist or is not parseable.
fn read_navigation_config(type_dir: &Path) -> Option<NavigationConfig> {
    let nav_path = type_dir.join("_navigation.json");
    let content = std::fs::read_to_string(&nav_path).ok()?;
    serde_json::from_str::<NavigationConfig>(&content).ok()
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
    // Preserve all-caps names like README, CHANGELOG, IDEA-001, EPIC-023.
    // A name is "all-caps" when every alphabetic character is uppercase.
    let has_uppercase = stem.chars().any(|c| c.is_ascii_uppercase());
    let all_caps = stem
        .chars()
        .all(|c| c.is_ascii_uppercase() || c == '-' || c == '_' || c.is_ascii_digit());
    if has_uppercase && all_caps {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::project_settings::ArtifactTypeConfig;
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
    fn empty_entries_returns_empty_tree() {
        let tmp = make_temp_project();
        let tree = artifact_scan_tree(tmp.path(), &[]).expect("scan");
        assert!(tree.groups.is_empty());
    }

    #[test]
    fn type_entry_with_no_directory_returns_empty_tree() {
        let tmp = make_temp_project();
        let entries = vec![ArtifactEntry::Type(ArtifactTypeConfig {
            key: "ideas".to_string(),
            label: None,
            icon: None,
            path: ".orqa/planning/ideas".to_string(),
        })];
        let tree = artifact_scan_tree(tmp.path(), &entries).expect("scan");
        // Directory doesn't exist — type has no nodes, so the group is dropped.
        assert!(tree.groups.is_empty());
    }

    #[test]
    fn type_entry_with_md_files_produces_nodes() {
        let tmp = make_temp_project();
        let type_dir = tmp.path().join(".orqa/planning/ideas");
        fs::create_dir_all(&type_dir).expect("create dir");
        fs::write(
            type_dir.join("IDEA-001.md"),
            "---\ntitle: First Idea\n---\n",
        )
        .expect("write");
        fs::write(type_dir.join("IDEA-002.md"), "# Bare\n").expect("write");
        // README with frontmatter label used when config label is absent.
        fs::write(
            type_dir.join("README.md"),
            "---\nlabel: Ideas\nicon: lightbulb\n---\n",
        )
        .expect("write readme");

        let entries = vec![ArtifactEntry::Type(ArtifactTypeConfig {
            key: "ideas".to_string(),
            label: None,
            icon: None,
            path: ".orqa/planning/ideas".to_string(),
        })];

        let tree = artifact_scan_tree(tmp.path(), &entries).expect("scan");
        assert_eq!(tree.groups.len(), 1);
        let group = &tree.groups[0];
        // Label comes from README frontmatter.
        assert_eq!(group.label, "Ideas");
        assert_eq!(group.types.len(), 1);
        let nav_type = &group.types[0];
        assert_eq!(nav_type.nodes.len(), 2, "README should be excluded");

        // Nodes sorted alphabetically by label.
        assert_eq!(nav_type.nodes[0].label, "First Idea");
        assert_eq!(nav_type.nodes[1].label, "IDEA-002");
    }

    #[test]
    fn type_entry_label_falls_back_to_humanized_key() {
        let tmp = make_temp_project();
        let type_dir = tmp.path().join(".orqa/planning/ideas");
        fs::create_dir_all(&type_dir).expect("create dir");
        fs::write(
            type_dir.join("IDEA-001.md"),
            "---\ntitle: First Idea\n---\n",
        )
        .expect("write");

        // No README — label should be humanized from key.
        let entries = vec![ArtifactEntry::Type(ArtifactTypeConfig {
            key: "my-ideas".to_string(),
            label: None,
            icon: None,
            path: ".orqa/planning/ideas".to_string(),
        })];

        let tree = artifact_scan_tree(tmp.path(), &entries).expect("scan");
        assert_eq!(tree.groups.len(), 1);
        assert_eq!(tree.groups[0].label, "My Ideas");
    }

    #[test]
    fn group_entry_produces_multi_type_group() {
        let tmp = make_temp_project();

        let ideas_dir = tmp.path().join(".orqa/planning/ideas");
        let epics_dir = tmp.path().join(".orqa/planning/epics");
        fs::create_dir_all(&ideas_dir).expect("ideas dir");
        fs::create_dir_all(&epics_dir).expect("epics dir");
        // Write README with frontmatter for the group directory.
        let planning_dir = tmp.path().join(".orqa/planning");
        fs::write(
            planning_dir.join("README.md"),
            "---\nlabel: Planning\nicon: target\n---\n",
        )
        .expect("group readme");
        fs::write(ideas_dir.join("IDEA-001.md"), "---\ntitle: My Idea\n---\n").expect("idea");
        fs::write(epics_dir.join("EPIC-001.md"), "---\ntitle: My Epic\n---\n").expect("epic");
        // READMEs for child types.
        fs::write(ideas_dir.join("README.md"), "---\nlabel: Ideas\n---\n").expect("ideas readme");
        fs::write(epics_dir.join("README.md"), "---\nlabel: Epics\n---\n").expect("epics readme");

        let entries = vec![ArtifactEntry::Group {
            key: "planning".to_string(),
            label: None,
            icon: None,
            children: vec![
                ArtifactTypeConfig {
                    key: "ideas".to_string(),
                    label: None,
                    icon: None,
                    path: ".orqa/planning/ideas".to_string(),
                },
                ArtifactTypeConfig {
                    key: "epics".to_string(),
                    label: None,
                    icon: None,
                    path: ".orqa/planning/epics".to_string(),
                },
            ],
        }];

        let tree = artifact_scan_tree(tmp.path(), &entries).expect("scan");
        assert_eq!(tree.groups.len(), 1);
        let group = &tree.groups[0];
        // Label and icon come from group directory README frontmatter.
        assert_eq!(group.label, "Planning");
        assert_eq!(group.icon, "target");
        assert_eq!(group.types.len(), 2);
        assert_eq!(group.types[0].label, "Ideas");
        assert_eq!(group.types[1].label, "Epics");
    }

    #[test]
    fn recursive_scan_builds_tree_for_subdirectories() {
        let tmp = make_temp_project();

        // Simulate a docs type with subdirectories like .orqa/documentation.
        let docs_root = tmp.path().join(".orqa/documentation");
        let arch_dir = docs_root.join("architecture");
        let product_dir = docs_root.join("product");
        let nested_dir = product_dir.join("deep");

        fs::create_dir_all(&arch_dir).expect("arch dir");
        fs::create_dir_all(&product_dir).expect("product dir");
        fs::create_dir_all(&nested_dir).expect("nested dir");

        // README at root level with frontmatter.
        fs::write(
            docs_root.join("README.md"),
            "---\nlabel: Documentation\nicon: file-text\n---\n",
        )
        .expect("readme");

        // Files in subdirectories.
        fs::write(
            arch_dir.join("decisions.md"),
            "---\ntitle: Architecture Decisions\n---\n",
        )
        .expect("decisions");
        fs::write(arch_dir.join("overview.md"), "# Overview\n").expect("overview");
        fs::write(product_dir.join("vision.md"), "---\ntitle: Vision\n---\n").expect("vision");

        // Nested subdir file.
        fs::write(
            nested_dir.join("deep-doc.md"),
            "---\ntitle: Deep Doc\n---\n",
        )
        .expect("deep");

        // Empty subdir should be omitted.
        let empty_dir = docs_root.join("empty");
        fs::create_dir_all(&empty_dir).expect("empty dir");

        let entries = vec![ArtifactEntry::Type(ArtifactTypeConfig {
            key: "docs".to_string(),
            label: None,
            icon: None,
            path: ".orqa/documentation".to_string(),
        })];

        let tree = artifact_scan_tree(tmp.path(), &entries).expect("scan");
        assert_eq!(
            tree.groups.len(),
            1,
            "documentation group should be present"
        );

        let group = &tree.groups[0];
        let nav_type = &group.types[0];

        // Top-level nodes should be directory nodes (architecture, product) — no files at root.
        assert_eq!(
            nav_type.nodes.len(),
            2,
            "architecture and product dirs, empty dir omitted"
        );

        // Directories sorted alphabetically: architecture before product.
        assert_eq!(nav_type.nodes[0].label, "Architecture");
        assert!(
            nav_type.nodes[0].path.is_none(),
            "directory node has no path"
        );
        assert!(nav_type.nodes[0].children.is_some());

        let arch_children = nav_type.nodes[0].children.as_ref().unwrap();
        assert_eq!(arch_children.len(), 2, "decisions and overview");
        // Files sorted alphabetically: Architecture Decisions before Overview.
        assert_eq!(arch_children[0].label, "Architecture Decisions");
        assert_eq!(
            arch_children[0].path,
            Some(".orqa/documentation/architecture/decisions.md".to_string())
        );
        assert_eq!(arch_children[1].label, "Overview");

        assert_eq!(nav_type.nodes[1].label, "Product");
        let product_children = nav_type.nodes[1].children.as_ref().unwrap();
        // product has a nested dir (deep) and a file (vision).
        // Dirs first, then files.
        assert_eq!(product_children.len(), 2);
        assert_eq!(
            product_children[0].label, "Deep",
            "directory node for 'deep' subdir"
        );
        assert!(product_children[0].children.is_some());
        let deep_children = product_children[0].children.as_ref().unwrap();
        assert_eq!(deep_children.len(), 1);
        assert_eq!(deep_children[0].label, "Deep Doc");
        assert_eq!(
            deep_children[0].path,
            Some(".orqa/documentation/product/deep/deep-doc.md".to_string())
        );
        assert_eq!(product_children[1].label, "Vision");
    }
}
