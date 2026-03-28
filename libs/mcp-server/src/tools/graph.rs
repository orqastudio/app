//! Graph tool implementations: query, resolve, relationships, stats, validate, health, read, refresh, traceability.
//!
//! All graph operations that previously operated on a local `ArtifactGraph` now
//! proxy through the validation daemon HTTP API. `graph_read` and
//! `graph_traceability` are the only exceptions: `graph_read` reads a file
//! directly from disk (no graph needed), and `graph_traceability` still uses
//! the local `orqa_validation` library because the daemon does not expose a
//! traceability endpoint.

use serde_json::{json, Value};

use crate::daemon::DaemonClient;
use crate::types::McpToolDefinition;

// ---------------------------------------------------------------------------
// Tool definitions
// ---------------------------------------------------------------------------

/// Return the MCP tool definition list for all graph tools.
#[allow(clippy::too_many_lines)]
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
                        "description": "Relative path to the artifact (e.g. .orqa/implementation/epics/EPIC-094.md)"
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
        McpToolDefinition {
            name: "graph_traceability".into(),
            description: "Get full traceability for an artifact: ancestry chains to pillars, descendants, siblings, and impact radius".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "artifact_id": {
                        "type": "string",
                        "description": "Artifact ID to trace (e.g. EPIC-094, TASK-580)"
                    }
                },
                "required": ["artifact_id"]
            }),
        },
    ]
}

// ---------------------------------------------------------------------------
// Tool implementations — proxy to daemon
// ---------------------------------------------------------------------------

/// `graph_query` — delegates to `POST /query` on the daemon.
///
/// Supports `type`, `status`, and `search` filters. All filtering is
/// performed server-side by the daemon.
pub fn tool_query(daemon: &DaemonClient, args: &Value) -> Result<String, String> {
    let type_filter = args.get("type").and_then(|v| v.as_str());
    let status_filter = args.get("status").and_then(|v| v.as_str());
    let search_filter = args.get("search").and_then(|v| v.as_str());

    let mut query_params = serde_json::Map::new();
    if let Some(t) = type_filter {
        query_params.insert("type".into(), json!(t));
    }
    if let Some(s) = status_filter {
        query_params.insert("status".into(), json!(s));
    }
    if let Some(q) = search_filter {
        query_params.insert("search".into(), json!(q));
    }

    let result = daemon
        .query(&Value::Object(query_params))
        .map_err(|e| e.to_string())?;

    let items = result.as_array().ok_or("daemon returned non-array")?;

    // Return summary fields consistent with the old local implementation.
    let summary: Vec<Value> = items
        .iter()
        .map(|item| {
            json!({
                "id": item.get("id"),
                "type": item.get("type").or_else(|| item.get("artifact_type")),
                "title": item.get("title"),
                "status": item.get("status"),
                "path": item.get("path")
            })
        })
        .collect();

    serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())
}

/// `graph_resolve` — uses `POST /query` filtered by id to get the artifact.
pub fn tool_resolve(daemon: &DaemonClient, args: &Value) -> Result<String, String> {
    let id = args
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or("missing 'id'")?;

    let result = daemon
        .query(&json!({ "id": id }))
        .map_err(|e| e.to_string())?;

    let items = result.as_array().ok_or("daemon returned non-array")?;
    let item = items
        .first()
        .ok_or_else(|| format!("artifact not found: {id}"))?;

    serde_json::to_string_pretty(item).map_err(|e| e.to_string())
}

/// `graph_relationships` — uses `POST /query` to find the artifact, then
/// extracts its relationship fields.
#[allow(clippy::too_many_lines)]
pub fn tool_relationships(daemon: &DaemonClient, args: &Value) -> Result<String, String> {
    let id = args
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or("missing 'id'")?;
    let direction = args
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("both");

    let result = daemon
        .query(&json!({ "id": id }))
        .map_err(|e| e.to_string())?;

    let items = result.as_array().ok_or("daemon returned non-array")?;
    let item = items
        .first()
        .ok_or_else(|| format!("artifact not found: {id}"))?;

    let mut out = json!({});
    if direction == "out" || direction == "both" {
        let refs_out = item.get("references_out").cloned().unwrap_or(json!([]));
        // Normalise field names to match original tool output.
        let formatted: Vec<Value> = refs_out
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|r| {
                        json!({
                            "target": r.get("target_id").or_else(|| r.get("target")),
                            "type": r.get("relationship_type").or_else(|| r.get("type")),
                            "field": r.get("field")
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();
        out["outgoing"] = json!(formatted);
    }
    if direction == "in" || direction == "both" {
        let refs_in = item.get("references_in").cloned().unwrap_or(json!([]));
        let formatted: Vec<Value> = refs_in
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|r| {
                        json!({
                            "source": r.get("source_id").or_else(|| r.get("source")),
                            "type": r.get("relationship_type").or_else(|| r.get("type")),
                            "field": r.get("field")
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();
        out["incoming"] = json!(formatted);
    }

    serde_json::to_string_pretty(&out).map_err(|e| e.to_string())
}

/// `graph_stats` — reads artifact and rule counts from `GET /health`.
pub fn tool_stats(daemon: &DaemonClient) -> Result<String, String> {
    let health = daemon.health().map_err(|e| e.to_string())?;
    let artifact_count = health.get("artifacts").and_then(Value::as_u64).unwrap_or(0);
    let rule_count = health.get("rules").and_then(Value::as_u64).unwrap_or(0);

    let stats = json!({
        "node_count": artifact_count,
        "rule_count": rule_count
    });
    serde_json::to_string_pretty(&stats).map_err(|e| e.to_string())
}

/// `graph_health` — reads from `GET /health`.
pub fn tool_health(daemon: &DaemonClient) -> Result<String, String> {
    let health = daemon.health().map_err(|e| e.to_string())?;
    serde_json::to_string_pretty(&health).map_err(|e| e.to_string())
}

/// `graph_validate` — calls `POST /validate` and optionally filters by path.
pub fn tool_validate(daemon: &DaemonClient, args: &Value) -> Result<String, String> {
    let report = daemon.validate().map_err(|e| e.to_string())?;
    let path_filter = args.get("path").and_then(|v| v.as_str());

    let checks = report
        .get("checks")
        .and_then(|v| v.as_array())
        .ok_or("daemon returned no 'checks' field")?;

    let filtered: Vec<&Value> = if let Some(prefix) = path_filter {
        checks
            .iter()
            .filter(|c| {
                c.get("path")
                    .and_then(|v| v.as_str())
                    .is_some_and(|p| p.starts_with(prefix))
            })
            .collect()
    } else {
        checks.iter().collect()
    };

    let summary: Vec<Value> = filtered
        .iter()
        .map(|c| {
            json!({
                "severity": c.get("severity"),
                "category": c.get("category"),
                "message": c.get("message"),
                "artifact_id": c.get("artifact_id"),
                "auto_fixable": c.get("auto_fixable")
            })
        })
        .collect();

    serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())
}

/// `graph_read` — reads an artifact file directly from disk.
///
/// This tool does not go through the daemon because reading raw file content
/// is a local filesystem operation that doesn't require graph state.
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

/// `graph_refresh` — calls `POST /reload` on the daemon.
pub fn tool_refresh(daemon: &DaemonClient) -> Result<String, String> {
    let result = daemon.reload().map_err(|e| e.to_string())?;
    let artifact_count = result.get("artifacts").and_then(Value::as_u64).unwrap_or(0);
    Ok(format!("Graph refreshed: {artifact_count} artifacts"))
}

/// `graph_traceability` — delegates to `POST /traceability` on the daemon.
///
/// Uses the daemon's cached graph instead of rebuilding from disk on every call.
pub fn tool_traceability(daemon: &DaemonClient, args: &Value) -> Result<String, String> {
    let artifact_id = args
        .get("artifact_id")
        .and_then(|v| v.as_str())
        .ok_or("missing 'artifact_id'")?;
    if artifact_id.trim().is_empty() {
        return Err("artifact_id cannot be empty".into());
    }

    let result = daemon
        .traceability(artifact_id)
        .map_err(|e| e.to_string())?;
    serde_json::to_string_pretty(&result).map_err(|e| e.to_string())
}
