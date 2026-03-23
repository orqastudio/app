//! Local IPC socket server for CLI ↔ App communication.
//!
//! Listens on a TCP port on localhost. The port is written to a well-known
//! lock file so the CLI can discover it. Handles MCP and LSP protocol
//! messages from CLI proxy clients.
//!
//! The CLI runs `orqa mcp` or `orqa lsp` which connects to this socket
//! and bridges stdin/stdout ↔ TCP.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::thread;

/// Well-known file where the IPC port is stored.
/// CLI reads this to discover the running app instance.
const IPC_PORT_FILENAME: &str = "ipc.port";

/// Get the path to the IPC port file.
fn port_file_path() -> PathBuf {
    dirs_next::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("com.orqastudio.app")
        .join(IPC_PORT_FILENAME)
}

/// Start the IPC socket server in a background thread.
///
/// Binds to a random available port on localhost, writes the port
/// to the well-known port file, and spawns a thread to accept
/// connections.
///
/// Each connection is handled in its own thread. The first line
/// from the client determines the protocol:
/// - `MCP <project-path>` → run MCP server on this connection
/// - `LSP <project-path>` → run LSP server on this connection
pub fn start(project_root: Option<PathBuf>) {
    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(l) => l,
        Err(e) => {
            tracing::warn!("[ipc] failed to bind IPC socket: {e}");
            return;
        }
    };

    let port = listener.local_addr().map(|a| a.port()).unwrap_or(0);
    if port == 0 {
        tracing::warn!("[ipc] failed to get IPC port");
        return;
    }

    // Write port to the well-known file
    let port_file = port_file_path();
    if let Some(parent) = port_file.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Err(e) = std::fs::write(&port_file, port.to_string()) {
        tracing::warn!("[ipc] failed to write port file: {e}");
        return;
    }

    tracing::info!("[ipc] listening on 127.0.0.1:{port}");

    let default_root = project_root
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let root = default_root.clone();
                    thread::spawn(move || {
                        if let Err(e) = handle_connection(stream, &root) {
                            tracing::warn!("[ipc] connection error: {e}");
                        }
                    });
                }
                Err(e) => {
                    tracing::warn!("[ipc] accept error: {e}");
                }
            }
        }

        // Cleanup port file on exit
        let _ = std::fs::remove_file(port_file_path());
    });
}

/// Handle a single IPC connection.
///
/// Reads the first line to determine the protocol, then dispatches.
fn handle_connection(
    stream: TcpStream,
    default_root: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream;

    // Read protocol header: "MCP [path]" or "LSP [path]"
    let mut header = String::new();
    reader.read_line(&mut header)?;
    let header = header.trim();

    let parts: Vec<&str> = header.splitn(2, ' ').collect();
    let protocol = parts.first().copied().unwrap_or("");
    let project_root = parts
        .get(1)
        .map_or_else(|| default_root.to_path_buf(), |p| PathBuf::from(*p));

    match protocol {
        "MCP" => {
            tracing::info!("[ipc] MCP session for {}", project_root.display());
            handle_mcp_session(&mut reader, &mut writer, &project_root)?;
        }
        "LSP" => {
            tracing::info!("[ipc] LSP session for {}", project_root.display());
            // LSP over TCP would need the full tower-lsp async runtime.
            // For now, return an error suggesting the CLI use --lsp directly.
            writeln!(
                writer,
                "LSP over IPC not yet implemented. Use orqa-studio --lsp directly."
            )?;
        }
        _ => {
            writeln!(writer, "Unknown protocol: {protocol}. Expected MCP or LSP.")?;
        }
    }

    Ok(())
}

/// Run an MCP session over the TCP connection.
///
/// Reads JSON-RPC lines from the client, dispatches to the MCP server,
/// and writes responses back.
fn handle_mcp_session(
    reader: &mut BufReader<TcpStream>,
    writer: &mut TcpStream,
    project_root: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Reuse the existing MCP server logic
    // We can't call mcp::run() directly since it reads from stdin.
    // Instead, we use the same McpServer struct but feed it lines from the TCP stream.

    // Build a minimal MCP server by creating the same structs used in mcp::run()
    let mut line = String::new();
    let mut server_state = McpServerState::new(project_root.to_path_buf());

    loop {
        line.clear();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            break; // Connection closed
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Parse and handle the JSON-RPC request
        let response = server_state.handle_line(trimmed);
        if let Some(resp) = response {
            writeln!(writer, "{resp}")?;
            writer.flush()?;
        }
    }

    Ok(())
}

/// Minimal MCP server state for IPC sessions.
/// Wraps the same logic as mcp::run() but operates on strings instead of stdin/stdout.
struct McpServerState {
    project_root: PathBuf,
    graph: Option<crate::domain::artifact_graph::ArtifactGraph>,
    _search: Option<crate::search::SearchEngine>,
}

impl McpServerState {
    fn new(project_root: PathBuf) -> Self {
        Self {
            project_root,
            graph: None,
            _search: None,
        }
    }

    fn handle_line(&mut self, line: &str) -> Option<String> {
        // Delegate to the MCP module's request handling
        // For now, use the standalone mcp server's JSON-RPC parsing
        let req: serde_json::Value = serde_json::from_str(line).ok()?;

        let method = req.get("method")?.as_str()?;
        let id = req.get("id").cloned();
        let params = req.get("params").cloned().unwrap_or(serde_json::json!({}));

        let result = match method {
            "initialize" => Some(serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {},
                    "resources": { "subscribe": false, "listChanged": false }
                },
                "serverInfo": {
                    "name": "orqastudio",
                    "version": env!("CARGO_PKG_VERSION")
                }
            })),
            "initialized" => return None, // notification
            "tools/list" | "tools/call" | "resources/list" | "resources/read" => {
                // For full tool support, we'd need to refactor mcp.rs to expose
                // the handler methods. For now, delegate to a fresh mcp::run() call
                // is not possible (it owns stdin). We'll implement the core tools inline.
                self.handle_mcp_method(method, &params)
            }
            _ => None,
        };

        let id_val = id.unwrap_or(serde_json::Value::Null);

        match result {
            Some(r) => {
                let resp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id_val,
                    "result": r
                });
                Some(serde_json::to_string(&resp).unwrap_or_default())
            }
            None => {
                if id_val.is_null() {
                    None
                } else {
                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id_val,
                        "error": { "code": -32601, "message": format!("method not found: {method}") }
                    });
                    Some(serde_json::to_string(&resp).unwrap_or_default())
                }
            }
        }
    }

    fn handle_mcp_method(
        &mut self,
        method: &str,
        params: &serde_json::Value,
    ) -> Option<serde_json::Value> {
        match method {
            "tools/list" => {
                // Return the tool list (same as mcp.rs)
                Some(serde_json::json!({
                    "tools": [
                        { "name": "graph_query", "description": "Query artifacts by type, status, or search text", "inputSchema": { "type": "object", "properties": {} } },
                        { "name": "graph_resolve", "description": "Get a single artifact by ID", "inputSchema": { "type": "object", "properties": { "id": { "type": "string" } }, "required": ["id"] } },
                        { "name": "graph_stats", "description": "Get graph statistics", "inputSchema": { "type": "object", "properties": {} } },
                        { "name": "graph_read", "description": "Read artifact content", "inputSchema": { "type": "object", "properties": { "path": { "type": "string" } }, "required": ["path"] } },
                        { "name": "graph_validate", "description": "Run integrity check", "inputSchema": { "type": "object", "properties": {} } },
                        { "name": "search_regex", "description": "Regex search with scope", "inputSchema": { "type": "object", "properties": { "pattern": { "type": "string" }, "scope": { "type": "string" } }, "required": ["pattern"] } },
                        { "name": "search_semantic", "description": "Semantic search with scope", "inputSchema": { "type": "object", "properties": { "query": { "type": "string" }, "scope": { "type": "string" } }, "required": ["query"] } }
                    ]
                }))
            }
            "tools/call" => {
                let tool_name = params.get("name")?.as_str()?;
                let args = params
                    .get("arguments")
                    .cloned()
                    .unwrap_or(serde_json::json!({}));

                let result = match tool_name {
                    "graph_stats" => self.graph_stats(),
                    "graph_read" => self.graph_read(&args),
                    "graph_query" => self.graph_query(&args),
                    _ => Err(format!("tool not implemented in IPC mode: {tool_name}")),
                };

                match result {
                    Ok(text) => {
                        Some(serde_json::json!({ "content": [{ "type": "text", "text": text }] }))
                    }
                    Err(e) => Some(
                        serde_json::json!({ "content": [{ "type": "text", "text": e }], "isError": true }),
                    ),
                }
            }
            _ => None,
        }
    }

    fn get_graph(&mut self) -> Result<&crate::domain::artifact_graph::ArtifactGraph, String> {
        if self.graph.is_none() {
            let graph = crate::domain::artifact_graph::build_artifact_graph(&self.project_root)
                .map_err(|e| format!("failed to build graph: {e}"))?;
            self.graph = Some(graph);
        }
        Ok(self.graph.as_ref().unwrap())
    }

    fn graph_stats(&mut self) -> Result<String, String> {
        let graph = self.get_graph()?;
        let stats = crate::domain::artifact_graph::graph_stats(graph);
        serde_json::to_string_pretty(&stats).map_err(|e| e.to_string())
    }

    fn graph_read(&self, args: &serde_json::Value) -> Result<String, String> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or("missing 'path'")?;
        if path.contains("..") {
            return Err("path traversal not allowed".into());
        }
        std::fs::read_to_string(self.project_root.join(path))
            .map_err(|e| format!("failed to read: {e}"))
    }

    fn graph_query(&mut self, args: &serde_json::Value) -> Result<String, String> {
        let graph = self.get_graph()?;
        let type_filter = args.get("type").and_then(|v| v.as_str());
        let status_filter = args.get("status").and_then(|v| v.as_str());

        let nodes: Vec<serde_json::Value> = graph
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
                true
            })
            .map(|n| {
                serde_json::json!({
                    "id": n.id, "type": n.artifact_type, "title": n.title,
                    "status": n.status, "path": n.path
                })
            })
            .collect();

        serde_json::to_string_pretty(&nodes).map_err(|e| e.to_string())
    }
}

/// Read the IPC port from the well-known port file.
/// Returns None if the app is not running.
pub fn read_port() -> Option<u16> {
    let port_file = port_file_path();
    let content = std::fs::read_to_string(&port_file).ok()?;
    content.trim().parse().ok()
}

/// Clean up the port file (called on app shutdown).
pub fn cleanup() {
    let _ = std::fs::remove_file(port_file_path());
}
