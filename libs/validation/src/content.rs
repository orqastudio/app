//! Content loading -- retrieve agent preambles, knowledge content, and
//! behavioral rule messages without consumers parsing markdown themselves.

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::error::ValidationError;
use crate::graph::ArtifactGraph;
use crate::parse::parse_artifact;

// ---------------------------------------------------------------------------
// Public output types
// ---------------------------------------------------------------------------

/// Agent preamble and metadata returned by [`find_agent`].
#[derive(Debug, Serialize)]
pub struct AgentContent {
    pub id: String,
    pub title: String,
    /// The `preamble` frontmatter field; falls back to `description` if absent.
    pub preamble: String,
    pub frontmatter: serde_json::Value,
    /// Full markdown body (everything after the frontmatter block).
    pub content: String,
}

/// Knowledge artifact content returned by [`find_knowledge`].
#[derive(Debug, Serialize)]
pub struct KnowledgeContent {
    pub id: String,
    pub title: String,
    pub content: String,
    pub frontmatter: serde_json::Value,
}

/// Behavioral enforcement messages extracted from active rules.
#[derive(Debug, Serialize)]
pub struct BehavioralMessages {
    pub messages: Vec<String>,
    /// Total number of rule artifacts inspected.
    pub rule_count: usize,
    /// Number of enforcement entries with `mechanism: behavioral`.
    pub behavioral_count: usize,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Find an agent artifact whose title matches `agent_type` (case-insensitive).
///
/// Scans all agent directories under `project_root` (core + plugin + connector).
/// Returns `None` if no match is found.
pub fn find_agent(
    project_root: &Path,
    agent_type: &str,
) -> Result<Option<AgentContent>, ValidationError> {
    let needle = agent_type.to_lowercase();

    for dir in agent_directories(project_root) {
        if !dir.exists() {
            continue;
        }

        let entries = std::fs::read_dir(&dir)
            .map_err(|e| ValidationError::FileSystem(e.to_string()))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if !is_md_file(&path) {
                continue;
            }

            if let Ok(parsed) = parse_artifact(&path, project_root) {
                if parsed.title.to_lowercase() == needle {
                    return Ok(Some(agent_content_from_parsed(parsed)));
                }
            }
        }
    }

    Ok(None)
}

/// Find an agent artifact in a pre-built graph whose title matches `agent_type`
/// (case-insensitive).
///
/// Prefer this over [`find_agent`] when the caller already has a graph, as it
/// avoids an extra directory scan.
pub fn find_agent_in_graph(
    graph: &ArtifactGraph,
    project_root: &Path,
    agent_type: &str,
) -> Result<Option<AgentContent>, ValidationError> {
    let needle = agent_type.to_lowercase();

    for (key, node) in &graph.nodes {
        // Skip bare-ID alias nodes in organisation mode.
        if key.as_str() == node.id && node.project.is_some() {
            continue;
        }

        if node.artifact_type != "agent" {
            continue;
        }

        if node.title.to_lowercase() != needle {
            continue;
        }

        let file_path = project_root.join(&node.path);
        let parsed = parse_artifact(&file_path, project_root)?;
        return Ok(Some(agent_content_from_parsed(parsed)));
    }

    Ok(None)
}

/// Find a knowledge artifact by its directory key.
///
/// For each knowledge directory the function tries:
/// 1. `<dir>/<key>/KNOW.md`
/// 2. `<dir>/<key>.md`
///
/// Returns `None` if no matching file is found.
pub fn find_knowledge(
    project_root: &Path,
    key: &str,
) -> Result<Option<KnowledgeContent>, ValidationError> {
    for dir in knowledge_directories(project_root) {
        if !dir.exists() {
            continue;
        }

        // Try directory form first: <dir>/<key>/KNOW.md
        let dir_form = dir.join(key).join("KNOW.md");
        if dir_form.exists() {
            let parsed = parse_artifact(&dir_form, project_root)?;
            return Ok(Some(knowledge_content_from_parsed(parsed)));
        }

        // Try flat file: <dir>/<key>.md
        let flat_form = dir.join(format!("{key}.md"));
        if flat_form.exists() {
            let parsed = parse_artifact(&flat_form, project_root)?;
            return Ok(Some(knowledge_content_from_parsed(parsed)));
        }
    }

    Ok(None)
}

/// Extract all `mechanism: behavioral` enforcement messages from active rules.
///
/// Only rules with `status: active` are inspected. Messages are returned
/// in sorted order for deterministic output.
pub fn extract_behavioral_messages(
    graph: &ArtifactGraph,
    _project_root: &Path,
) -> Result<BehavioralMessages, ValidationError> {
    let mut messages: Vec<String> = Vec::new();
    let mut rule_count: usize = 0;
    let mut behavioral_count: usize = 0;

    for (key, node) in &graph.nodes {
        // Skip bare-ID alias nodes in organisation mode.
        if key.as_str() == node.id && node.project.is_some() {
            continue;
        }

        if node.artifact_type != "rule" {
            continue;
        }

        // Only active rules are enforced.
        if node.status.as_deref() != Some("active") {
            continue;
        }

        rule_count += 1;

        let Some(enforcement) = node.frontmatter.get("enforcement") else {
            continue;
        };

        let Some(entries) = enforcement.as_array() else {
            continue;
        };

        for entry in entries {
            let Some(obj) = entry.as_object() else {
                continue;
            };

            if obj
                .get("mechanism")
                .and_then(serde_json::Value::as_str)
                != Some("behavioral")
            {
                continue;
            }

            behavioral_count += 1;

            if let Some(msg) = obj.get("message").and_then(serde_json::Value::as_str) {
                messages.push(msg.to_owned());
            }
        }
    }

    messages.sort();

    Ok(BehavioralMessages {
        messages,
        rule_count,
        behavioral_count,
    })
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

fn is_md_file(path: &Path) -> bool {
    path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("md")
}

/// Directories that may contain agent artifacts, in priority order.
fn agent_directories(root: &Path) -> Vec<PathBuf> {
    let mut dirs = vec![root.join(".orqa/process/agents")];

    // plugins/<name>/agents/
    dirs.extend(subdirectory_named(root, "plugins", "agents"));

    // connectors/<name>/agents/
    dirs.extend(subdirectory_named(root, "connectors", "agents"));

    dirs
}

/// Directories that may contain knowledge artifacts, in priority order.
fn knowledge_directories(root: &Path) -> Vec<PathBuf> {
    let mut dirs = vec![root.join(".orqa/process/knowledge")];

    // plugins/<name>/knowledge/
    dirs.extend(subdirectory_named(root, "plugins", "knowledge"));

    dirs
}

/// For each immediate subdirectory of `<root>/<container>/`, return the path
/// `<root>/<container>/<subdir_name>/<subdir>` if it exists.
fn subdirectory_named(root: &Path, container: &str, subdir: &str) -> Vec<PathBuf> {
    let container_dir = root.join(container);
    if !container_dir.exists() {
        return Vec::new();
    }

    let Ok(entries) = std::fs::read_dir(&container_dir) else {
        return Vec::new();
    };

    entries
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| e.path().join(subdir))
        .filter(|p| p.exists())
        .collect()
}

fn agent_content_from_parsed(parsed: crate::types::ParsedArtifact) -> AgentContent {
    let preamble = parsed
        .frontmatter
        .get("preamble")
        .and_then(serde_json::Value::as_str)
        .or_else(|| {
            parsed
                .frontmatter
                .get("description")
                .and_then(serde_json::Value::as_str)
        })
        .unwrap_or("")
        .to_owned();

    AgentContent {
        id: parsed.id,
        title: parsed.title,
        preamble,
        frontmatter: parsed.frontmatter,
        content: parsed.content,
    }
}

fn knowledge_content_from_parsed(parsed: crate::types::ParsedArtifact) -> KnowledgeContent {
    KnowledgeContent {
        id: parsed.id,
        title: parsed.title,
        content: parsed.content,
        frontmatter: parsed.frontmatter,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_project() -> TempDir {
        tempfile::tempdir().expect("tempdir")
    }

    fn write_file(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create dir");
        }
        fs::write(path, content).expect("write file");
    }

    // -------------------------------------------------------------------------
    // find_agent tests
    // -------------------------------------------------------------------------

    #[test]
    fn find_agent_returns_none_when_no_agents_dir() {
        let tmp = make_project();
        let result = find_agent(tmp.path(), "Planner").expect("ok");
        assert!(result.is_none());
    }

    #[test]
    fn find_agent_returns_none_for_unknown_agent() {
        let tmp = make_project();
        let agents_dir = tmp.path().join(".orqa/process/agents");
        write_file(
            &agents_dir.join("planner.md"),
            "---\nid: AGENT-a1b2c3d4\ntitle: Planner\nstatus: active\n---\nPlan things.\n",
        );
        let result = find_agent(tmp.path(), "Reviewer").expect("ok");
        assert!(result.is_none());
    }

    #[test]
    fn find_agent_matches_case_insensitively() {
        let tmp = make_project();
        let agents_dir = tmp.path().join(".orqa/process/agents");
        write_file(
            &agents_dir.join("planner.md"),
            "---\nid: AGENT-a1b2c3d4\ntitle: Planner\nstatus: active\npreamble: You plan things.\n---\nBody.\n",
        );

        let result = find_agent(tmp.path(), "planner").expect("ok");
        assert!(result.is_some());
        let agent = result.unwrap();
        assert_eq!(agent.id, "AGENT-a1b2c3d4");
        assert_eq!(agent.title, "Planner");
        assert_eq!(agent.preamble, "You plan things.");
    }

    #[test]
    fn find_agent_falls_back_to_description_preamble() {
        let tmp = make_project();
        let agents_dir = tmp.path().join(".orqa/process/agents");
        write_file(
            &agents_dir.join("reviewer.md"),
            "---\nid: AGENT-b2c3d4e5\ntitle: Reviewer\nstatus: active\ndescription: Reviews code.\n---\nBody.\n",
        );

        let agent = find_agent(tmp.path(), "Reviewer").expect("ok").unwrap();
        assert_eq!(agent.preamble, "Reviews code.");
    }

    #[test]
    fn find_agent_preamble_empty_when_neither_field_present() {
        let tmp = make_project();
        let agents_dir = tmp.path().join(".orqa/process/agents");
        write_file(
            &agents_dir.join("writer.md"),
            "---\nid: AGENT-c3d4e5f6\ntitle: Writer\nstatus: active\n---\nBody.\n",
        );

        let agent = find_agent(tmp.path(), "Writer").expect("ok").unwrap();
        assert_eq!(agent.preamble, "");
    }

    #[test]
    fn find_agent_scans_plugin_directories() {
        let tmp = make_project();
        let plugin_agents_dir = tmp.path().join("plugins/software/agents");
        write_file(
            &plugin_agents_dir.join("specialist.md"),
            "---\nid: AGENT-d4e5f6a7\ntitle: Specialist\nstatus: active\npreamble: Domain expert.\n---\nBody.\n",
        );

        let result = find_agent(tmp.path(), "Specialist").expect("ok");
        assert!(result.is_some());
        assert_eq!(result.unwrap().id, "AGENT-d4e5f6a7");
    }

    // -------------------------------------------------------------------------
    // find_knowledge tests
    // -------------------------------------------------------------------------

    #[test]
    fn find_knowledge_returns_none_when_no_knowledge_dir() {
        let tmp = make_project();
        let result = find_knowledge(tmp.path(), "coding").expect("ok");
        assert!(result.is_none());
    }

    #[test]
    fn find_knowledge_flat_file_form() {
        let tmp = make_project();
        let know_dir = tmp.path().join(".orqa/process/knowledge");
        write_file(
            &know_dir.join("coding.md"),
            "---\nid: KNOW-a1b2c3d4\ntitle: Coding Standards\nstatus: active\n---\nContent.\n",
        );

        let result = find_knowledge(tmp.path(), "coding").expect("ok");
        assert!(result.is_some());
        let k = result.unwrap();
        assert_eq!(k.id, "KNOW-a1b2c3d4");
        assert_eq!(k.title, "Coding Standards");
        assert!(k.content.contains("Content."));
    }

    #[test]
    fn find_knowledge_directory_form_preferred() {
        let tmp = make_project();
        let know_dir = tmp.path().join(".orqa/process/knowledge");

        // Both forms exist -- directory form wins.
        write_file(
            &know_dir.join("coding/KNOW.md"),
            "---\nid: KNOW-dir\ntitle: Coding Dir\nstatus: active\n---\nDir form.\n",
        );
        write_file(
            &know_dir.join("coding.md"),
            "---\nid: KNOW-flat\ntitle: Coding Flat\nstatus: active\n---\nFlat form.\n",
        );

        let result = find_knowledge(tmp.path(), "coding").expect("ok");
        let k = result.unwrap();
        assert_eq!(k.id, "KNOW-dir");
    }

    #[test]
    fn find_knowledge_returns_none_for_unknown_key() {
        let tmp = make_project();
        let know_dir = tmp.path().join(".orqa/process/knowledge");
        write_file(
            &know_dir.join("coding.md"),
            "---\nid: KNOW-a1b2c3d4\ntitle: Coding\nstatus: active\n---\nContent.\n",
        );

        let result = find_knowledge(tmp.path(), "unknown-key").expect("ok");
        assert!(result.is_none());
    }

    // -------------------------------------------------------------------------
    // extract_behavioral_messages tests
    // -------------------------------------------------------------------------

    #[test]
    fn extract_behavioral_returns_empty_for_empty_graph() {
        use crate::graph::ArtifactGraph;
        use std::collections::HashMap;

        let graph = ArtifactGraph {
            nodes: HashMap::new(),
            path_index: HashMap::new(),
        };
        let tmp = make_project();
        let result = extract_behavioral_messages(&graph, tmp.path()).expect("ok");
        assert!(result.messages.is_empty());
        assert_eq!(result.rule_count, 0);
        assert_eq!(result.behavioral_count, 0);
    }

    #[test]
    fn extract_behavioral_collects_messages_from_active_rules() {
        use crate::graph::{ArtifactGraph, ArtifactNode};
        use std::collections::HashMap;

        let node = ArtifactNode {
            id: "RULE-a1b2c3d4".to_owned(),
            project: None,
            artifact_type: "rule".to_owned(),
            path: ".orqa/process/rules/RULE-a1b2c3d4.md".to_owned(),
            title: "Test Rule".to_owned(),
            description: None,
            status: Some("active".to_owned()),
            priority: None,
            frontmatter: serde_json::json!({
                "id": "RULE-a1b2c3d4",
                "status": "active",
                "enforcement": [
                    { "mechanism": "behavioral", "message": "Always do X" },
                    { "mechanism": "behavioral", "message": "Never do Y" },
                    { "mechanism": "hook", "type": "PreAction", "action": "block" }
                ]
            }),
            references_out: vec![],
            references_in: vec![],
        };

        let mut graph = ArtifactGraph {
            nodes: HashMap::new(),
            path_index: HashMap::new(),
        };
        graph.nodes.insert(node.id.clone(), node);

        let tmp = make_project();
        let result = extract_behavioral_messages(&graph, tmp.path()).expect("ok");
        assert_eq!(result.rule_count, 1);
        assert_eq!(result.behavioral_count, 2);
        assert_eq!(result.messages.len(), 2);
        // Messages are sorted.
        assert_eq!(result.messages[0], "Always do X");
        assert_eq!(result.messages[1], "Never do Y");
    }

    #[test]
    fn extract_behavioral_skips_inactive_rules() {
        use crate::graph::{ArtifactGraph, ArtifactNode};
        use std::collections::HashMap;

        let node = ArtifactNode {
            id: "RULE-b2c3d4e5".to_owned(),
            project: None,
            artifact_type: "rule".to_owned(),
            path: ".orqa/process/rules/RULE-b2c3d4e5.md".to_owned(),
            title: "Inactive Rule".to_owned(),
            description: None,
            status: Some("inactive".to_owned()),
            priority: None,
            frontmatter: serde_json::json!({
                "id": "RULE-b2c3d4e5",
                "status": "inactive",
                "enforcement": [
                    { "mechanism": "behavioral", "message": "Ignored message" }
                ]
            }),
            references_out: vec![],
            references_in: vec![],
        };

        let mut graph = ArtifactGraph {
            nodes: HashMap::new(),
            path_index: HashMap::new(),
        };
        graph.nodes.insert(node.id.clone(), node);

        let tmp = make_project();
        let result = extract_behavioral_messages(&graph, tmp.path()).expect("ok");
        assert!(result.messages.is_empty());
        assert_eq!(result.rule_count, 0);
    }

    #[test]
    fn extract_behavioral_skips_non_rule_artifacts() {
        use crate::graph::{ArtifactGraph, ArtifactNode};
        use std::collections::HashMap;

        let node = ArtifactNode {
            id: "EPIC-a1b2c3d4".to_owned(),
            project: None,
            artifact_type: "epic".to_owned(),
            path: ".orqa/delivery/epics/EPIC-a1b2c3d4.md".to_owned(),
            title: "An Epic".to_owned(),
            description: None,
            status: Some("active".to_owned()),
            priority: None,
            frontmatter: serde_json::json!({
                "id": "EPIC-a1b2c3d4",
                "status": "active",
                "enforcement": [
                    { "mechanism": "behavioral", "message": "Not from an epic" }
                ]
            }),
            references_out: vec![],
            references_in: vec![],
        };

        let mut graph = ArtifactGraph {
            nodes: HashMap::new(),
            path_index: HashMap::new(),
        };
        graph.nodes.insert(node.id.clone(), node);

        let tmp = make_project();
        let result = extract_behavioral_messages(&graph, tmp.path()).expect("ok");
        assert!(result.messages.is_empty());
    }

    #[test]
    fn extract_behavioral_messages_are_sorted() {
        use crate::graph::{ArtifactGraph, ArtifactNode};
        use std::collections::HashMap;

        let node = ArtifactNode {
            id: "RULE-c3d4e5f6".to_owned(),
            project: None,
            artifact_type: "rule".to_owned(),
            path: ".orqa/process/rules/RULE-c3d4e5f6.md".to_owned(),
            title: "Multi Rule".to_owned(),
            description: None,
            status: Some("active".to_owned()),
            priority: None,
            frontmatter: serde_json::json!({
                "id": "RULE-c3d4e5f6",
                "status": "active",
                "enforcement": [
                    { "mechanism": "behavioral", "message": "Zebra rule" },
                    { "mechanism": "behavioral", "message": "Apple rule" },
                    { "mechanism": "behavioral", "message": "Mango rule" }
                ]
            }),
            references_out: vec![],
            references_in: vec![],
        };

        let mut graph = ArtifactGraph {
            nodes: HashMap::new(),
            path_index: HashMap::new(),
        };
        graph.nodes.insert(node.id.clone(), node);

        let tmp = make_project();
        let result = extract_behavioral_messages(&graph, tmp.path()).expect("ok");
        assert_eq!(result.messages, vec!["Apple rule", "Mango rule", "Zebra rule"]);
    }
}
