//! MCP (Model Context Protocol) server for Claude Code integration.
//!
//! Exposes the OrqaStudio artifact graph as MCP tools over stdio.
//! Launched via `orqa-studio --mcp <project-path>`.

use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::domain::artifact_graph::{
    build_artifact_graph, check_integrity, graph_stats, ArtifactGraph, ArtifactNode,
};

// ---------------------------------------------------------------------------
// JSON-RPC types
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Serialize)]
struct JsonRpcError {
    code: i64,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

// ---------------------------------------------------------------------------
// MCP types
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct McpToolDefinition {
    name: String,
    description: String,
    #[serde(rename = "inputSchema")]
    input_schema: Value,
}

#[derive(Serialize)]
struct McpResource {
    uri: String,
    name: String,
    description: String,
    #[serde(rename = "mimeType")]
    mime_type: String,
}

// ---------------------------------------------------------------------------
// Server state
// ---------------------------------------------------------------------------

struct McpServer {
    project_root: PathBuf,
    graph: Option<ArtifactGraph>,
}

impl McpServer {
    fn new(project_root: PathBuf) -> Self {
        Self {
            project_root,
            graph: None,
        }
    }

    /// Get or build the artifact graph.
    fn get_graph(&mut self) -> Result<&ArtifactGraph, String> {
        if self.graph.is_none() {
            let graph = build_artifact_graph(&self.project_root)
                .map_err(|e| format!("failed to build graph: {e}"))?;
            self.graph = Some(graph);
        }
        Ok(self.graph.as_ref().unwrap())
    }

    /// Rebuild the graph from disk.
    fn refresh_graph(&mut self) -> Result<&ArtifactGraph, String> {
        let graph = build_artifact_graph(&self.project_root)
            .map_err(|e| format!("failed to build graph: {e}"))?;
        self.graph = Some(graph);
        Ok(self.graph.as_ref().unwrap())
    }

    // -----------------------------------------------------------------------
    // MCP method handlers
    // -----------------------------------------------------------------------

    fn handle_initialize(&self, _params: &Value) -> Value {
        json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {},
                "resources": { "subscribe": false, "listChanged": false }
            },
            "serverInfo": {
                "name": "orqastudio",
                "version": env!("CARGO_PKG_VERSION")
            }
        })
    }

    fn handle_tools_list(&self) -> Value {
        let tools = vec![
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
                        "direction": { "type": "string", "enum": ["out", "in", "both"], "description": "Relationship direction (default: both)" }
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
                name: "graph_validate".into(),
                description: "Run integrity check and return all violations".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Optional: validate only artifacts under this path" }
                    }
                }),
            },
            McpToolDefinition {
                name: "graph_read".into(),
                description: "Read the full content of an artifact file".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to the artifact (e.g. .orqa/delivery/epics/EPIC-094.md)" }
                    },
                    "required": ["path"]
                }),
            },
            McpToolDefinition {
                name: "graph_refresh".into(),
                description: "Rebuild the artifact graph from disk".into(),
                input_schema: json!({ "type": "object", "properties": {} }),
            },
        ];

        json!({ "tools": tools })
    }

    fn handle_resources_list(&self) -> Value {
        let resources = vec![
            McpResource {
                uri: "orqa://schema/core.json".into(),
                name: "Core Schema".into(),
                description: "Platform-level artifact types, relationships, and statuses".into(),
                mime_type: "application/json".into(),
            },
            McpResource {
                uri: "orqa://schema/project.json".into(),
                name: "Project Config".into(),
                description: "Project-level artifact configuration and relationships".into(),
                mime_type: "application/json".into(),
            },
        ];

        json!({ "resources": resources })
    }

    fn handle_resources_read(&self, params: &Value) -> Value {
        let uri = params.get("uri").and_then(|v| v.as_str()).unwrap_or("");

        let path = match uri {
            "orqa://schema/core.json" => {
                // core.json is in the types lib relative to the project root
                // In org mode, it's at libs/types/src/platform/core.json
                let candidates = [
                    self.project_root.join("libs/types/src/platform/core.json"),
                    self.project_root.join("app/.orqa/platform/core.json"),
                ];
                candidates.into_iter().find(|p| p.exists())
            }
            "orqa://schema/project.json" => {
                let p = self.project_root.join(".orqa/project.json");
                if p.exists() { Some(p) } else { None }
            }
            _ => None,
        };

        match path {
            Some(p) => match std::fs::read_to_string(&p) {
                Ok(content) => json!({
                    "contents": [{
                        "uri": uri,
                        "mimeType": "application/json",
                        "text": content
                    }]
                }),
                Err(e) => json!({ "error": format!("failed to read: {e}") }),
            },
            None => json!({ "error": format!("resource not found: {uri}") }),
        }
    }

    fn handle_tool_call(&mut self, params: &Value) -> Value {
        let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

        let result = match tool_name {
            "graph_query" => self.tool_query(&arguments),
            "graph_resolve" => self.tool_resolve(&arguments),
            "graph_relationships" => self.tool_relationships(&arguments),
            "graph_stats" => self.tool_stats(),
            "graph_validate" => self.tool_validate(&arguments),
            "graph_read" => self.tool_read(&arguments),
            "graph_refresh" => self.tool_refresh(),
            _ => Err(format!("unknown tool: {tool_name}")),
        };

        match result {
            Ok(text) => json!({
                "content": [{ "type": "text", "text": text }]
            }),
            Err(e) => json!({
                "content": [{ "type": "text", "text": e }],
                "isError": true
            }),
        }
    }

    // -----------------------------------------------------------------------
    // Tool implementations
    // -----------------------------------------------------------------------

    fn tool_query(&mut self, args: &Value) -> Result<String, String> {
        let graph = self.get_graph()?;
        let type_filter = args.get("type").and_then(|v| v.as_str());
        let status_filter = args.get("status").and_then(|v| v.as_str());
        let search_filter = args.get("search").and_then(|v| v.as_str());

        let nodes: Vec<&ArtifactNode> = graph
            .nodes
            .values()
            .filter(|n| {
                if let Some(t) = type_filter {
                    if n.artifact_type != t { return false; }
                }
                if let Some(s) = status_filter {
                    if n.status.as_deref() != Some(s) { return false; }
                }
                if let Some(q) = search_filter {
                    let q_lower = q.to_lowercase();
                    let title_match = n.title.to_lowercase().contains(&q_lower);
                    let desc_match = n.description.as_ref()
                        .map(|d| d.to_lowercase().contains(&q_lower))
                        .unwrap_or(false);
                    if !title_match && !desc_match { return false; }
                }
                true
            })
            .collect();

        let summary: Vec<Value> = nodes
            .iter()
            .map(|n| json!({
                "id": n.id,
                "type": n.artifact_type,
                "title": n.title,
                "status": n.status,
                "path": n.path
            }))
            .collect();

        serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())
    }

    fn tool_resolve(&mut self, args: &Value) -> Result<String, String> {
        let id = args.get("id").and_then(|v| v.as_str()).ok_or("missing 'id'")?;
        let graph = self.get_graph()?;
        let node = graph.nodes.get(id).ok_or(format!("artifact not found: {id}"))?;
        serde_json::to_string_pretty(node).map_err(|e| e.to_string())
    }

    fn tool_relationships(&mut self, args: &Value) -> Result<String, String> {
        let id = args.get("id").and_then(|v| v.as_str()).ok_or("missing 'id'")?;
        let direction = args.get("direction").and_then(|v| v.as_str()).unwrap_or("both");
        let graph = self.get_graph()?;
        let node = graph.nodes.get(id).ok_or(format!("artifact not found: {id}"))?;

        let mut result = json!({});
        if direction == "out" || direction == "both" {
            let out: Vec<Value> = node.references_out.iter().map(|r| json!({
                "target": r.target_id,
                "type": r.relationship_type,
                "field": r.field
            })).collect();
            result["outgoing"] = json!(out);
        }
        if direction == "in" || direction == "both" {
            let incoming: Vec<Value> = node.references_in.iter().map(|r| json!({
                "source": r.source_id,
                "type": r.relationship_type,
                "field": r.field
            })).collect();
            result["incoming"] = json!(incoming);
        }

        serde_json::to_string_pretty(&result).map_err(|e| e.to_string())
    }

    fn tool_stats(&mut self) -> Result<String, String> {
        let graph = self.get_graph()?;
        let stats = graph_stats(graph);
        serde_json::to_string_pretty(&stats).map_err(|e| e.to_string())
    }

    fn tool_validate(&mut self, args: &Value) -> Result<String, String> {
        let graph = self.get_graph()?;
        // Load minimal validation context (no project settings in headless mode)
        let checks = check_integrity(graph, &[], &Default::default(), &[], &[]);

        let path_filter = args.get("path").and_then(|v| v.as_str());

        let filtered: Vec<&_> = if let Some(prefix) = path_filter {
            // Filter by artifact ID prefix or by looking up the node's path
            checks
                .iter()
                .filter(|c| {
                    if let Some(node) = graph.nodes.get(&c.artifact_id) {
                        node.path.starts_with(prefix)
                    } else {
                        false
                    }
                })
                .collect()
        } else {
            checks.iter().collect()
        };

        let summary: Vec<Value> = filtered
            .iter()
            .map(|c| json!({
                "severity": format!("{:?}", c.severity),
                "category": format!("{:?}", c.category),
                "message": c.message,
                "artifact_id": c.artifact_id,
                "auto_fixable": c.auto_fixable
            }))
            .collect();

        serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())
    }

    fn tool_read(&self, args: &Value) -> Result<String, String> {
        let path = args.get("path").and_then(|v| v.as_str()).ok_or("missing 'path'")?;
        if path.contains("..") {
            return Err("path traversal not allowed".into());
        }
        let full_path = self.project_root.join(path);
        std::fs::read_to_string(&full_path).map_err(|e| format!("failed to read: {e}"))
    }

    fn tool_refresh(&mut self) -> Result<String, String> {
        let graph = self.refresh_graph()?;
        let stats = graph_stats(graph);
        Ok(format!(
            "Graph refreshed: {} nodes, {} edges, {} orphans, {} broken refs",
            stats.node_count, stats.edge_count, stats.orphan_count, stats.broken_ref_count
        ))
    }

    // -----------------------------------------------------------------------
    // Request dispatch
    // -----------------------------------------------------------------------

    fn handle_request(&mut self, req: &JsonRpcRequest) -> Option<JsonRpcResponse> {
        let result = match req.method.as_str() {
            "initialize" => Some(self.handle_initialize(&req.params)),
            "initialized" => return None, // notification, no response
            "tools/list" => Some(self.handle_tools_list()),
            "tools/call" => Some(self.handle_tool_call(&req.params)),
            "resources/list" => Some(self.handle_resources_list()),
            "resources/read" => Some(self.handle_resources_read(&req.params)),
            _ => None,
        };

        let id = req.id.clone().unwrap_or(Value::Null);

        match result {
            Some(value) => Some(JsonRpcResponse {
                jsonrpc: "2.0".into(),
                id,
                result: Some(value),
                error: None,
            }),
            None => {
                if req.id.is_some() {
                    Some(JsonRpcResponse {
                        jsonrpc: "2.0".into(),
                        id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32601,
                            message: format!("method not found: {}", req.method),
                            data: None,
                        }),
                    })
                } else {
                    None // notification, no response needed
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Run the MCP server over stdio.
///
/// Reads JSON-RPC messages from stdin (one per line), dispatches them, and
/// writes responses to stdout. Runs until stdin is closed.
pub fn run(project_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new(project_root.to_path_buf());
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let req: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let error_resp = JsonRpcResponse {
                    jsonrpc: "2.0".into(),
                    id: Value::Null,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: format!("parse error: {e}"),
                        data: None,
                    }),
                };
                let out = serde_json::to_string(&error_resp)?;
                writeln!(stdout, "{out}")?;
                stdout.flush()?;
                continue;
            }
        };

        if let Some(resp) = server.handle_request(&req) {
            let out = serde_json::to_string(&resp)?;
            writeln!(stdout, "{out}")?;
            stdout.flush()?;
        }
    }

    Ok(())
}
