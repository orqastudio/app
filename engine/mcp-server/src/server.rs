//! MCP server state and JSON-RPC dispatch loop.
//!
//! Supports two transports:
//! - **stdio** (default): reads JSON-RPC from stdin, writes to stdout — used by
//!   LLM clients that spawn `orqa-mcp-server` directly.
//! - **TCP**: listens on `127.0.0.1:<port>` and handles one client connection —
//!   used when the daemon manages the MCP server as a persistent process.
//!
//! Search operations are fully delegated to the daemon's HTTP search endpoints.
//! The MCP server holds no local SearchEngine instance — the daemon owns the
//! single DuckDB index at .state/search.duckdb.

use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use serde_json::{json, Value};
use tracing::{debug, warn};

use crate::daemon::{default_daemon_port, DaemonClient};
use crate::error::McpError;
use crate::tools::{graph as graph_tools, search as search_tools};
use crate::types::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, McpResource};

// ---------------------------------------------------------------------------
// Server state
// ---------------------------------------------------------------------------

/// The MCP server. Holds a daemon client for all graph and search operations.
///
/// Search is fully delegated to the daemon — no local SearchEngine instance is
/// held here. All tool calls go through the daemon HTTP API.
pub struct McpServer {
    project_root: PathBuf,
    daemon: DaemonClient,
}

impl McpServer {
    /// Create a new server for the given project root.
    pub fn new(project_root: PathBuf) -> Self {
        Self::with_daemon_port(project_root, default_daemon_port())
    }

    /// Create a new server connecting to the daemon on `daemon_port`.
    pub fn with_daemon_port(project_root: PathBuf, daemon_port: u16) -> Self {
        Self {
            project_root,
            daemon: DaemonClient::new(daemon_port),
        }
    }

    // -----------------------------------------------------------------------
    // Method handlers
    // -----------------------------------------------------------------------

    fn handle_initialize(_params: &Value) -> Value {
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

    fn handle_tools_list() -> Value {
        let mut tools = graph_tools::tool_definitions();
        tools.extend(search_tools::tool_definitions());
        json!({ "tools": tools })
    }

    fn handle_resources_list() -> Value {
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
            "graph_query" => graph_tools::tool_query(&self.daemon, &arguments),
            "graph_resolve" => graph_tools::tool_resolve(&self.daemon, &arguments),
            "graph_relationships" => graph_tools::tool_relationships(&self.daemon, &arguments),
            "graph_stats" => graph_tools::tool_stats(&self.daemon),
            "graph_health" => graph_tools::tool_health(&self.daemon),
            "graph_validate" => graph_tools::tool_validate(&self.daemon, &arguments),
            "graph_read" => graph_tools::tool_read(&self.project_root, &arguments),
            "graph_refresh" => graph_tools::tool_refresh(&self.daemon),
            "graph_traceability" => graph_tools::tool_traceability(&self.daemon, &arguments),
            "search_regex" => search_tools::tool_search_regex(&self.daemon, &arguments),
            "search_semantic" => search_tools::tool_search_semantic(&self.daemon, &arguments),
            "search_research" => search_tools::tool_search_research(&self.daemon, &arguments),
            "search_status" => search_tools::tool_search_status(&self.daemon),
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
            "initialize" => Some(Self::handle_initialize(&req.params)),
            "initialized" => return None, // notification — no response
            "tools/list" => Some(Self::handle_tools_list()),
            "tools/call" => Some(self.handle_tool_call(&req.params)),
            "resources/list" => Some(Self::handle_resources_list()),
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
// Public entry points
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
    run_with_daemon_port(project_root, default_daemon_port())
}

/// Run the MCP server over stdio, connecting to the daemon on `daemon_port`.
///
/// # Errors
///
/// Returns `McpError::Io` if reading from stdin or writing to stdout fails.
pub fn run_with_daemon_port(
    project_root: &std::path::Path,
    daemon_port: u16,
) -> Result<(), McpError> {
    let mut server = McpServer::with_daemon_port(project_root.to_path_buf(), daemon_port);
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

/// Run the MCP server over a TCP connection, connecting to the daemon on `daemon_port`.
///
/// Binds to `127.0.0.1:<tcp_port>` and waits for a single client connection.
/// Once connected, handles the MCP JSON-RPC protocol over the TCP stream until
/// the client disconnects. The MCP protocol is unchanged — only the transport
/// layer differs from stdio mode.
///
/// Used by the daemon to run a persistent MCP server that LLM clients can
/// connect to over TCP. Multiple sequential clients are served (each client
/// causes the server to loop back and wait for the next connection).
///
/// # Errors
///
/// Returns `McpError::Io` if the TCP listener cannot be bound or a stream
/// read/write fails.
pub fn run_tcp(
    project_root: &std::path::Path,
    tcp_port: u16,
    daemon_port: u16,
) -> Result<(), McpError> {
    use socket2::{Domain, Protocol, Socket, Type};
    use std::net::SocketAddr;

    // Use socket2 to set SO_REUSEADDR before binding, preventing OS error 10048
    // (EADDRINUSE) on rapid restart when the previous socket is still in TIME_WAIT.
    // In container mode (ORQA_HEADLESS), bind to 0.0.0.0 so Docker port
    // publishing works. Natively, bind to 127.0.0.1 for security.
    let host = if std::env::var("ORQA_HEADLESS").is_ok_and(|v| v == "1" || v == "true") {
        "0.0.0.0"
    } else {
        "127.0.0.1"
    };
    let addr: SocketAddr = format!("{host}:{tcp_port}")
        .parse()
        .map_err(|e| McpError::Io(io::Error::new(io::ErrorKind::InvalidInput, e)))?;
    let socket =
        Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).map_err(McpError::Io)?;
    socket.set_reuse_address(true).map_err(McpError::Io)?;
    socket.bind(&addr.into()).map_err(McpError::Io)?;
    socket.listen(1).map_err(McpError::Io)?;
    let listener: std::net::TcpListener = socket.into();
    tracing::info!(addr = %addr, "MCP server listening on TCP");

    loop {
        let (stream, peer_addr) = listener.accept().map_err(McpError::Io)?;
        tracing::info!(peer = %peer_addr, "MCP client connected");
        serve_tcp_client(project_root, daemon_port, stream)?;
        tracing::info!(peer = %peer_addr, "MCP client disconnected");
    }
}

fn serve_tcp_client(
    project_root: &std::path::Path,
    daemon_port: u16,
    stream: std::net::TcpStream,
) -> Result<(), McpError> {
    use std::io::BufReader;

    let mut server = McpServer::with_daemon_port(project_root.to_path_buf(), daemon_port);
    let writer_stream = stream.try_clone().map_err(McpError::Io)?;
    let reader = BufReader::new(&stream);
    let mut writer = writer_stream;

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                tracing::debug!(error = %e, "TCP read error — client disconnected");
                break;
            }
        };

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
                writeln!(writer, "{out}").map_err(McpError::Io)?;
                writer.flush().map_err(McpError::Io)?;
                continue;
            }
        };

        if let Some(resp) = server.handle_request(&req) {
            let out = serde_json::to_string(&resp).map_err(|e| McpError::Json(e.to_string()))?;
            writeln!(writer, "{out}").map_err(McpError::Io)?;
            writer.flush().map_err(McpError::Io)?;
        }
    }

    Ok(())
}
