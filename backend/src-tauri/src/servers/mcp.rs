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
use crate::search::SearchEngine;

// ---------------------------------------------------------------------------
// JSON-RPC types
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct JsonRpcRequest {
    #[allow(dead_code)]
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
    search: Option<SearchEngine>,
}

impl McpServer {
    fn new(project_root: PathBuf) -> Self {
        Self {
            project_root,
            graph: None,
            search: None,
        }
    }

    /// Get or initialise the search engine (lazy init).
    fn get_search(&mut self) -> Result<&mut SearchEngine, String> {
        if self.search.is_none() {
            let db_path = self.project_root.join(".orqa").join("search.duckdb");
            let mut engine = SearchEngine::new(&db_path)
                .map_err(|e| format!("failed to init search engine: {e}"))?;

            // Index the project
            engine
                .index(&self.project_root, &["node_modules".into(), "target".into(), ".git".into(), "dist".into()])
                .map_err(|e| format!("failed to index project: {e}"))?;

            // Try to init embedder from known model locations
            let model_dirs = [
                dirs_next::data_dir()
                    .map(|d| d.join("com.orqastudio.app").join("models").join("bge-small-en-v1.5")),
                dirs_next::home_dir().map(|d| d.join("Downloads")),
            ];
            for dir in model_dirs.into_iter().flatten() {
                if dir.join("model.onnx").exists() && dir.join("tokenizer.json").exists() {
                    if engine.init_embedder_sync(&dir).is_ok() {
                        let _ = engine.embed_chunks();
                        break;
                    }
                }
            }

            self.search = Some(engine);
        }
        self.search.as_mut().ok_or_else(|| "search engine not available".into())
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
            McpToolDefinition {
                name: "search_regex".into(),
                description: "Search indexed content with a regex pattern".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "pattern": { "type": "string", "description": "Regex pattern to search for" },
                        "path_filter": { "type": "string", "description": "Optional: filter results to files matching this path prefix" },
                        "scope": { "type": "string", "enum": ["artifacts", "codebase", "all"], "description": "Search scope: 'artifacts' (.orqa/ only), 'codebase' (non-.orqa/), 'all' (default)" },
                        "limit": { "type": "integer", "description": "Max results (default: 20)" }
                    },
                    "required": ["pattern"]
                }),
            },
            McpToolDefinition {
                name: "search_semantic".into(),
                description: "Semantic search over indexed content using natural language".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "Natural language search query" },
                        "scope": { "type": "string", "enum": ["artifacts", "codebase", "all"], "description": "Search scope: 'artifacts' (.orqa/ only), 'codebase' (non-.orqa/), 'all' (default)" },
                        "limit": { "type": "integer", "description": "Max results (default: 10)" }
                    },
                    "required": ["query"]
                }),
            },
            McpToolDefinition {
                name: "search_research".into(),
                description: "Compound research query: semantic search → extract symbols → regex follow-up → assembled context. Use for 'how does X work?' questions.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "question": { "type": "string", "description": "Natural language question about the codebase" },
                        "scope": { "type": "string", "enum": ["artifacts", "codebase", "all"], "description": "Search scope: 'artifacts' (.orqa/ only), 'codebase' (non-.orqa/), 'all' (default)" },
                        "limit": { "type": "integer", "description": "Max initial semantic results (default: 5)" }
                    },
                    "required": ["question"]
                }),
            },
            McpToolDefinition {
                name: "search_status".into(),
                description: "Get search index status (chunk count, embedding status)".into(),
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
            "search_regex" => self.tool_search_regex(&arguments),
            "search_semantic" => self.tool_search_semantic(&arguments),
            "search_research" => self.tool_search_research(&arguments),
            "search_status" => self.tool_search_status(),
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

    /// Filter search results by scope (artifacts vs codebase).
    fn filter_by_scope(results: Vec<crate::search::types::SearchResult>, scope: &str) -> Vec<crate::search::types::SearchResult> {
        match scope {
            "artifacts" => results.into_iter().filter(|r| r.file_path.contains(".orqa/") || r.file_path.contains(".orqa\\")).collect(),
            "codebase" => results.into_iter().filter(|r| !r.file_path.contains(".orqa/") && !r.file_path.contains(".orqa\\")).collect(),
            _ => results, // "all" or unrecognised
        }
    }

    fn tool_search_regex(&mut self, args: &Value) -> Result<String, String> {
        let pattern = args.get("pattern").and_then(|v| v.as_str()).ok_or("missing 'pattern'")?;
        let path_filter = args.get("path_filter").and_then(|v| v.as_str());
        let scope = args.get("scope").and_then(|v| v.as_str()).unwrap_or("all");
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(20) as u32;

        let engine = self.get_search()?;
        let results = engine
            .search_regex(pattern, path_filter, limit)
            .map_err(|e| format!("search error: {e}"))?;

        let filtered = Self::filter_by_scope(results, scope);

        let summary: Vec<Value> = filtered
            .iter()
            .map(|r| json!({
                "file": r.file_path,
                "line": r.start_line,
                "content": r.content,
                "score": r.score
            }))
            .collect();

        serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())
    }

    fn tool_search_semantic(&mut self, args: &Value) -> Result<String, String> {
        let query = args.get("query").and_then(|v| v.as_str()).ok_or("missing 'query'")?;
        let scope = args.get("scope").and_then(|v| v.as_str()).unwrap_or("all");
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as u32;

        let engine = self.get_search()?;
        let results = engine
            .search_semantic(query, limit)
            .map_err(|e| format!("semantic search error: {e}"))?;

        let filtered = Self::filter_by_scope(results, scope);

        let summary: Vec<Value> = filtered
            .iter()
            .map(|r| json!({
                "file": r.file_path,
                "line": r.start_line,
                "content": r.content,
                "score": r.score
            }))
            .collect();

        serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())
    }

    fn tool_search_research(&mut self, args: &Value) -> Result<String, String> {
        let question = args.get("question").and_then(|v| v.as_str()).ok_or("missing 'question'")?;
        let scope = args.get("scope").and_then(|v| v.as_str()).unwrap_or("all");
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(5) as u32;

        // Step 1: Semantic search for conceptually relevant chunks
        let engine = self.get_search()?;
        let raw_results = engine
            .search_semantic(question, limit)
            .map_err(|e| format!("semantic search error: {e}"))?;
        let semantic_results = Self::filter_by_scope(raw_results, scope);

        if semantic_results.is_empty() {
            // Fall back to regex with keywords from the question
            let keywords: Vec<&str> = question
                .split_whitespace()
                .filter(|w| w.len() > 3)
                .take(3)
                .collect();

            if keywords.is_empty() {
                return Ok("No results found.".into());
            }

            let pattern = keywords.join("|");
            let engine = self.get_search()?;
            let fallback = engine
                .search_regex(&pattern, None, limit)
                .map_err(|e| format!("regex fallback error: {e}"))?;

            let summary: Vec<Value> = fallback
                .iter()
                .map(|r| json!({
                    "file": r.file_path,
                    "line": r.start_line,
                    "content": r.content,
                    "score": r.score,
                    "source": "regex_fallback"
                }))
                .collect();

            return serde_json::to_string_pretty(&json!({
                "question": question,
                "method": "regex_fallback",
                "results": summary
            }))
            .map_err(|e| e.to_string());
        }

        // Step 2: Extract symbols from semantic results (regex-based)
        let symbol_pattern = regex::Regex::new(
            r"(?:fn|pub fn|struct|enum|trait|type|const|interface|class|function|export)\s+(\w+)"
        ).unwrap();

        let mut symbols: Vec<String> = Vec::new();
        for result in &semantic_results {
            for cap in symbol_pattern.captures_iter(&result.content) {
                let sym = cap[1].to_string();
                if sym.len() > 2 && !symbols.contains(&sym) {
                    symbols.push(sym);
                }
            }
        }

        // Step 3: Regex follow-up for extracted symbols
        let mut follow_up_results = Vec::new();
        if !symbols.is_empty() {
            let symbol_pattern = symbols.iter().take(5).cloned().collect::<Vec<_>>().join("|");
            let engine = self.get_search()?;
            if let Ok(results) = engine.search_regex(&symbol_pattern, None, 10) {
                for r in results {
                    // Deduplicate against semantic results
                    let already_found = semantic_results
                        .iter()
                        .any(|s| s.file_path == r.file_path && s.start_line == r.start_line);
                    if !already_found {
                        follow_up_results.push(r);
                    }
                }
            }
        }

        // Step 4: Assemble response
        let primary: Vec<Value> = semantic_results
            .iter()
            .map(|r| json!({
                "file": r.file_path,
                "line": r.start_line,
                "content": r.content,
                "score": r.score,
                "source": "semantic"
            }))
            .collect();

        let related: Vec<Value> = follow_up_results
            .iter()
            .map(|r| json!({
                "file": r.file_path,
                "line": r.start_line,
                "content": r.content,
                "score": r.score,
                "source": "symbol_follow_up"
            }))
            .collect();

        serde_json::to_string_pretty(&json!({
            "question": question,
            "method": "semantic_with_follow_up",
            "symbols_found": symbols,
            "primary_results": primary,
            "related_results": related
        }))
        .map_err(|e| e.to_string())
    }

    fn tool_search_status(&mut self) -> Result<String, String> {
        let engine = self.get_search()?;
        let status = engine
            .get_status()
            .map_err(|e| format!("status error: {e}"))?;

        serde_json::to_string_pretty(&json!({
            "is_indexed": status.is_indexed,
            "chunk_count": status.chunk_count,
            "has_embeddings": status.has_embeddings,
        }))
        .map_err(|e| e.to_string())
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
