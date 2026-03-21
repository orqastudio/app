//! MCP server state and JSON-RPC dispatch loop.

use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use orqa_search::SearchEngine;
use serde_json::{json, Value};
use tracing::{debug, warn};

use crate::error::McpError;
use crate::graph::{build_artifact_graph, ArtifactGraph};
use crate::tools::{graph as graph_tools, search as search_tools};
use crate::types::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, McpResource};

// ---------------------------------------------------------------------------
// Server state
// ---------------------------------------------------------------------------

/// The MCP server.  Holds lazy-initialised graph and search engine state.
pub struct McpServer {
    project_root: PathBuf,
    graph: Option<ArtifactGraph>,
    search: Option<SearchEngine>,
}

impl McpServer {
    /// Create a new server for the given project root.
    pub fn new(project_root: PathBuf) -> Self {
        Self {
            project_root,
            graph: None,
            search: None,
        }
    }

    // -----------------------------------------------------------------------
    // Lazy accessors
    // -----------------------------------------------------------------------

    /// Get or initialise the search engine (lazy init).
    fn get_search(&mut self) -> Result<&mut SearchEngine, String> {
        if self.search.is_none() {
            let db_path = self.project_root.join(".orqa").join("search.duckdb");
            let mut engine = SearchEngine::new(&db_path)
                .map_err(|e| format!("failed to init search engine: {e}"))?;

            engine
                .index(
                    &self.project_root,
                    &[
                        "node_modules".into(),
                        "target".into(),
                        ".git".into(),
                        "dist".into(),
                    ],
                )
                .map_err(|e| format!("failed to index project: {e}"))?;

            // Try to init embedder from known model locations.
            let model_dirs = [
                dirs_next::data_dir().map(|d| {
                    d.join("com.orqastudio.app")
                        .join("models")
                        .join("bge-small-en-v1.5")
                }),
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
        self.search
            .as_mut()
            .ok_or_else(|| "search engine not available".into())
    }

    /// Get or build the artifact graph (lazy init).
    fn get_graph(&mut self) -> Result<&ArtifactGraph, String> {
        if self.graph.is_none() {
            let graph = build_artifact_graph(&self.project_root)
                .map_err(|e| format!("failed to build graph: {e}"))?;
            self.graph = Some(graph);
        }
        self.graph
            .as_ref()
            .ok_or_else(|| "graph not available".into())
    }

    // -----------------------------------------------------------------------
    // Method handlers
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
        let mut tools = graph_tools::tool_definitions();
        tools.extend(search_tools::tool_definitions());
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
                let candidates = [
                    self.project_root.join("libs/types/src/platform/core.json"),
                    self.project_root.join("app/.orqa/platform/core.json"),
                ];
                candidates.into_iter().find(|p| p.exists())
            }
            "orqa://schema/project.json" => {
                let p = self.project_root.join(".orqa/project.json");
                if p.exists() {
                    Some(p)
                } else {
                    None
                }
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

        debug!(tool = tool_name, "tool call");

        let result: Result<String, String> = match tool_name {
            "graph_query" => self
                .get_graph()
                .and_then(|g| graph_tools::tool_query(g, &arguments)),
            "graph_resolve" => self
                .get_graph()
                .and_then(|g| graph_tools::tool_resolve(g, &arguments)),
            "graph_relationships" => self
                .get_graph()
                .and_then(|g| graph_tools::tool_relationships(g, &arguments)),
            "graph_stats" => self.get_graph().and_then(|g| graph_tools::tool_stats(g)),
            "graph_validate" => self
                .get_graph()
                .and_then(|g| graph_tools::tool_validate(g, &arguments)),
            "graph_read" => graph_tools::tool_read(&self.project_root, &arguments),
            "graph_refresh" => {
                let root = self.project_root.clone();
                graph_tools::tool_refresh(&root).map(|(new_graph, msg)| {
                    self.graph = Some(new_graph);
                    msg
                })
            }
            "search_regex" => self
                .get_search()
                .and_then(|e| search_tools::tool_search_regex(e, &arguments)),
            "search_semantic" => self
                .get_search()
                .and_then(|e| search_tools::tool_search_semantic(e, &arguments)),
            "search_research" => self
                .get_search()
                .and_then(|e| search_tools::tool_search_research(e, &arguments)),
            "search_status" => self
                .get_search()
                .and_then(|e| search_tools::tool_search_status(e)),
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
    // Request dispatch
    // -----------------------------------------------------------------------

    fn handle_request(&mut self, req: &JsonRpcRequest) -> Option<JsonRpcResponse> {
        let result = match req.method.as_str() {
            "initialize" => Some(self.handle_initialize(&req.params)),
            "initialized" => return None, // notification — no response
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
                    warn!(method = req.method.as_str(), "method not found");
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
///
/// # Errors
///
/// Returns `McpError::Io` if reading from stdin or writing to stdout fails.
pub fn run(project_root: &std::path::Path) -> Result<(), McpError> {
    let mut server = McpServer::new(project_root.to_path_buf());
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line.map_err(McpError::Io)?;
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
                let out = serde_json::to_string(&error_resp)
                    .map_err(|e| McpError::Json(e.to_string()))?;
                writeln!(stdout, "{out}").map_err(McpError::Io)?;
                stdout.flush().map_err(McpError::Io)?;
                continue;
            }
        };

        if let Some(resp) = server.handle_request(&req) {
            let out = serde_json::to_string(&resp).map_err(|e| McpError::Json(e.to_string()))?;
            writeln!(stdout, "{out}").map_err(McpError::Io)?;
            stdout.flush().map_err(McpError::Io)?;
        }
    }

    Ok(())
}
