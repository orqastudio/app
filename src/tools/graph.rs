//! Graph tool implementations: query, resolve, relationships, stats, validate, health, read, refresh.

use serde_json::{json, Value};

use crate::graph::{
    build_artifact_graph, check_integrity_headless, compute_health, graph_stats, ArtifactGraph,
    ArtifactNode,
};
use crate::types::McpToolDefinition;

// ---------------------------------------------------------------------------
// Tool definitions
// ---------------------------------------------------------------------------

/// Return the MCP tool definition list for all graph tools.
pub fn tool_definitions() -> Vec<McpToolDefinition> {
    vec![
        McpToolDefinition {
            name: "graph_query".into(),
            description: "Query artifacts by type, status, or search text".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "type": { "type": "string", "description": "Artifact type (epic, task, decision, rule, etc.)" },
                    "status": { "type": "string", "description": "Filter by status" },
                    "search": { "type": "string", "description": "Search in title and description" }
                }
            }),
        },
        McpToolDefinition {
            name: "graph_resolve".into(),
            description: "Get a single artifact by ID with all frontmatter".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Artifact ID (e.g. EPIC-094, TASK-580)" }
                },
                "required": ["id"]
            }),
        },
        McpToolDefinition {
            name: "graph_relationships".into(),
            description: "Get all relationships for an artifact".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Artifact ID" },
                    "direction": {
                        "type": "string",
                        "enum": ["out", "in", "both"],
                        "description": "Relationship direction (default: both)"
                    }
                },
                "required": ["id"]
            }),
        },
        McpToolDefinition {
            name: "graph_stats".into(),
            description: "Get artifact graph statistics (node counts, edge counts, health)".into(),
            input_schema: json!({ "type": "object", "properties": {} }),
        },
        McpToolDefinition {
            name: "graph_health".into(),
            description: "Get graph-theoretic health metrics: connected components, orphan rate, average degree, graph density, largest component ratio, pillar traceability, and bidirectionality ratio".into(),
            input_schema: json!({ "type": "object", "properties": {} }),
        },
        McpToolDefinition {
            name: "graph_validate".into(),
            description: "Run integrity check and return all violations".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Optional: validate only artifacts under this path"
                    }
                }
            }),
        },
        McpToolDefinition {
            name: "graph_read".into(),
            description: "Read the full content of an artifact file".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Relative path to the artifact (e.g. .orqa/delivery/epics/EPIC-094.md)"
                    }
                },
                "required": ["path"]
            }),
        },
        McpToolDefinition {
            name: "graph_refresh".into(),
            description: "Rebuild the artifact graph from disk".into(),
            input_schema: json!({ "type": "object", "properties": {} }),
        },
    ]
}

// ---------------------------------------------------------------------------
// Tool implementations (stateless — take graph/path as parameters)
// ---------------------------------------------------------------------------

pub fn tool_query(graph: &ArtifactGraph, args: &Value) -> Result<String, String> {
    let type_filter = args.get("type").and_then(|v| v.as_str());
    let status_filter = args.get("status").and_then(|v| v.as_str());
    let search_filter = args.get("search").and_then(|v| v.as_str());

    let nodes: Vec<&ArtifactNode> = graph
        .nodes
        .values()
        .filter(|n| {
            if let Some(t) = type_filter {
                if n.artifact_type != t {
                    return false;
                }
            }
            if let Some(s) = status_filter {
                if n.status.as_deref() != Some(s) {
                    return false;
                }
            }
            if let Some(q) = search_filter {
                let q_lower = q.to_lowercase();
                let title_match = n.title.to_lowercase().contains(&q_lower);
                let desc_match = n
                    .description
                    .as_ref()
                    .is_some_and(|d| d.to_lowercase().contains(&q_lower));
                if !title_match && !desc_match {
                    return false;
                }
            }
            true
        })
        .collect();

    let summary: Vec<Value> = nodes
        .iter()
        .map(|n| {
            json!({
                "id": n.id,
                "type": n.artifact_type,
                "title": n.title,
                "status": n.status,
                "path": n.path
            })
        })
        .collect();

    serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())
}

pub fn tool_resolve(graph: &ArtifactGraph, args: &Value) -> Result<String, String> {
    let id = args
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or("missing 'id'")?;
    let node = graph
        .nodes
        .get(id)
        .ok_or_else(|| format!("artifact not found: {id}"))?;
    serde_json::to_string_pretty(node).map_err(|e| e.to_string())
}

pub fn tool_relationships(graph: &ArtifactGraph, args: &Value) -> Result<String, String> {
    let id = args
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or("missing 'id'")?;
    let direction = args
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("both");
    let node = graph
        .nodes
        .get(id)
        .ok_or_else(|| format!("artifact not found: {id}"))?;

    let mut result = json!({});
    if direction == "out" || direction == "both" {
        let out: Vec<Value> = node
            .references_out
            .iter()
            .map(|r| {
                json!({
                    "target": r.target_id,
                    "type": r.relationship_type,
                    "field": r.field
                })
            })
            .collect();
        result["outgoing"] = json!(out);
    }
    if direction == "in" || direction == "both" {
        let incoming: Vec<Value> = node
            .references_in
            .iter()
            .map(|r| {
                json!({
                    "source": r.source_id,
                    "type": r.relationship_type,
                    "field": r.field
                })
            })
            .collect();
        result["incoming"] = json!(incoming);
    }

    serde_json::to_string_pretty(&result).map_err(|e| e.to_string())
}

pub fn tool_stats(graph: &ArtifactGraph) -> Result<String, String> {
    let stats = graph_stats(graph);
    serde_json::to_string_pretty(&stats).map_err(|e| e.to_string())
}

pub fn tool_health(graph: &ArtifactGraph) -> Result<String, String> {
    let health = compute_health(graph);
    serde_json::to_string_pretty(&health).map_err(|e| e.to_string())
}

pub fn tool_validate(graph: &ArtifactGraph, args: &Value) -> Result<String, String> {
    let checks = check_integrity_headless(graph);
    let path_filter = args.get("path").and_then(|v| v.as_str());

    let filtered: Vec<&_> = if let Some(prefix) = path_filter {
        checks
            .iter()
            .filter(|c| {
                graph
                    .nodes
                    .get(&c.artifact_id)
                    .is_some_and(|n| n.path.starts_with(prefix))
            })
            .collect()
    } else {
        checks.iter().collect()
    };

    let summary: Vec<Value> = filtered
        .iter()
        .map(|c| {
            json!({
                "severity": format!("{:?}", c.severity),
                "category": format!("{:?}", c.category),
                "message": c.message,
                "artifact_id": c.artifact_id,
                "auto_fixable": c.auto_fixable
            })
        })
        .collect();

    serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())
}

pub fn tool_read(project_root: &std::path::Path, args: &Value) -> Result<String, String> {
    let path = args
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or("missing 'path'")?;
    if path.contains("..") {
        return Err("path traversal not allowed".into());
    }
    let full_path = project_root.join(path);
    std::fs::read_to_string(&full_path).map_err(|e| format!("failed to read: {e}"))
}

pub fn tool_refresh(project_root: &std::path::Path) -> Result<(ArtifactGraph, String), String> {
    let graph =
        build_artifact_graph(project_root).map_err(|e| format!("failed to build graph: {e}"))?;
    let stats = graph_stats(&graph);
    let msg = format!(
        "Graph refreshed: {} nodes, {} edges, {} orphans, {} broken refs",
        stats.node_count, stats.edge_count, stats.orphan_count, stats.broken_ref_count
    );
    Ok((graph, msg))
}
