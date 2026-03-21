//! Standalone search server binary.
//!
//! Accepts a project path argument, initializes the search engine, and listens
//! on stdin/stdout for JSON-RPC 2.0 requests (same framing as MCP).
//!
//! # Protocol
//!
//! Each request is a newline-delimited JSON object on stdin:
//! ```json
//! {"jsonrpc":"2.0","id":1,"method":"index","params":{"root":"/path/to/project","excluded":[]}}
//! {"jsonrpc":"2.0","id":2,"method":"search_regex","params":{"pattern":"fn main","max_results":10}}
//! {"jsonrpc":"2.0","id":3,"method":"search_semantic","params":{"query":"error handling","max_results":10}}
//! {"jsonrpc":"2.0","id":4,"method":"get_status","params":{}}
//! ```
//!
//! Each response is a newline-delimited JSON object on stdout:
//! ```json
//! {"jsonrpc":"2.0","id":1,"result":{...}}
//! {"jsonrpc":"2.0","id":1,"error":{"code":-32603,"message":"..."}}
//! ```
//!
//! # Logging
//!
//! All log output goes to stderr with structured JSON formatting.
//! In development, set `RUST_LOG=debug` for verbose output.
//!
//! # Dev Controller Integration
//!
//! When `ORQA_DEV_LOG_PORT` environment variable is set (e.g. `3001`), the server
//! also POSTs structured log events to `http://localhost:<port>/log`.

use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use orqa_search::SearchEngine;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, error, info, warn};

// ---------------------------------------------------------------------------
// JSON-RPC types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}

impl JsonRpcResponse {
    fn ok(id: Option<Value>, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    fn err(id: Option<Value>, code: i32, message: String) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError { code, message }),
        }
    }
}

// ---------------------------------------------------------------------------
// Request param shapes
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct IndexParams {
    root: String,
    #[serde(default)]
    excluded: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SearchRegexParams {
    pattern: String,
    #[serde(default)]
    path_filter: Option<String>,
    #[serde(default = "default_max_results")]
    max_results: u32,
}

#[derive(Debug, Deserialize)]
struct SearchSemanticParams {
    query: String,
    #[serde(default = "default_max_results")]
    max_results: u32,
}

#[derive(Debug, Deserialize)]
struct InitEmbedderParams {
    model_dir: String,
}

fn default_max_results() -> u32 {
    20
}

// ---------------------------------------------------------------------------
// CLI argument parsing
// ---------------------------------------------------------------------------

/// Parse CLI arguments and return the database path.
///
/// Usage: `orqa-search-server [--db <path>]`
///
/// If `--db` is not provided, uses `<temp_dir>/orqa-search-server.duckdb`.
fn parse_db_path(args: &[String]) -> PathBuf {
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--db" && i + 1 < args.len() {
            return PathBuf::from(&args[i + 1]);
        }
        i += 1;
    }
    std::env::temp_dir().join("orqa-search-server.duckdb")
}

// ---------------------------------------------------------------------------
// Request dispatch
// ---------------------------------------------------------------------------

fn handle_request(engine: &mut SearchEngine, request: &JsonRpcRequest) -> Value {
    match request.method.as_str() {
        "index" => handle_index(engine, &request.params),
        "search_regex" => handle_search_regex(engine, &request.params),
        "search_semantic" => handle_search_semantic(engine, &request.params),
        "get_status" => handle_get_status(engine),
        "init_embedder_sync" => handle_init_embedder_sync(engine, &request.params),
        unknown => {
            warn!("unknown method: {unknown}");
            Value::Null
        }
    }
}

fn handle_index(engine: &mut SearchEngine, params: &Value) -> Value {
    let p: IndexParams = match serde_json::from_value(params.clone()) {
        Ok(v) => v,
        Err(e) => return error_value(format!("invalid params: {e}")),
    };
    let root = PathBuf::from(&p.root);
    match engine.index(&root, &p.excluded) {
        Ok(status) => serde_json::to_value(status).unwrap_or(Value::Null),
        Err(e) => error_value(e.to_string()),
    }
}

fn handle_search_regex(engine: &mut SearchEngine, params: &Value) -> Value {
    let p: SearchRegexParams = match serde_json::from_value(params.clone()) {
        Ok(v) => v,
        Err(e) => return error_value(format!("invalid params: {e}")),
    };
    match engine.search_regex(&p.pattern, p.path_filter.as_deref(), p.max_results) {
        Ok(results) => serde_json::to_value(results).unwrap_or(Value::Null),
        Err(e) => error_value(e.to_string()),
    }
}

fn handle_search_semantic(engine: &mut SearchEngine, params: &Value) -> Value {
    let p: SearchSemanticParams = match serde_json::from_value(params.clone()) {
        Ok(v) => v,
        Err(e) => return error_value(format!("invalid params: {e}")),
    };
    match engine.search_semantic(&p.query, p.max_results) {
        Ok(results) => serde_json::to_value(results).unwrap_or(Value::Null),
        Err(e) => error_value(e.to_string()),
    }
}

fn handle_get_status(engine: &mut SearchEngine) -> Value {
    match engine.get_status() {
        Ok(status) => serde_json::to_value(status).unwrap_or(Value::Null),
        Err(e) => error_value(e.to_string()),
    }
}

fn handle_init_embedder_sync(engine: &mut SearchEngine, params: &Value) -> Value {
    let p: InitEmbedderParams = match serde_json::from_value(params.clone()) {
        Ok(v) => v,
        Err(e) => return error_value(format!("invalid params: {e}")),
    };
    match engine.init_embedder_sync(&PathBuf::from(&p.model_dir)) {
        Ok(()) => Value::Bool(true),
        Err(e) => error_value(e.to_string()),
    }
}

/// Wrap an error message as a JSON object `{"error": "..."}`.
fn error_value(msg: String) -> Value {
    serde_json::json!({ "error": msg })
}

// ---------------------------------------------------------------------------
// Main loop
// ---------------------------------------------------------------------------

fn main() {
    // Initialize structured tracing to stderr.
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .json()
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let db_path = parse_db_path(&args);

    info!("orqa-search-server starting, db={}", db_path.display());

    let mut engine = match SearchEngine::new(&db_path) {
        Ok(e) => e,
        Err(err) => {
            error!("failed to initialize search engine: {err}");
            std::process::exit(1);
        }
    };

    info!("search engine ready, listening on stdin");

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    let reader = BufReader::new(stdin.lock());

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(e) => {
                error!("stdin read error: {e}");
                break;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        debug!("received: {line}");

        let response = match serde_json::from_str::<JsonRpcRequest>(&line) {
            Ok(request) => {
                if request.jsonrpc != "2.0" {
                    JsonRpcResponse::err(
                        request.id,
                        -32600,
                        "invalid JSON-RPC version".to_string(),
                    )
                } else {
                    info!("dispatch method={}", request.method);
                    let result = handle_request(&mut engine, &request);
                    // If the result itself contains an "error" key, surface it as a JSON-RPC error
                    if result.get("error").is_some() {
                        let msg = result["error"]
                            .as_str()
                            .unwrap_or("unknown error")
                            .to_string();
                        JsonRpcResponse::err(request.id, -32603, msg)
                    } else {
                        JsonRpcResponse::ok(request.id, result)
                    }
                }
            }
            Err(e) => JsonRpcResponse::err(None, -32700, format!("parse error: {e}")),
        };

        let serialized = match serde_json::to_string(&response) {
            Ok(s) => s,
            Err(e) => {
                error!("failed to serialize response: {e}");
                continue;
            }
        };

        if let Err(e) = writeln!(stdout, "{serialized}") {
            error!("stdout write error: {e}");
            break;
        }

        if let Err(e) = stdout.flush() {
            error!("stdout flush error: {e}");
            break;
        }
    }

    info!("orqa-search-server shutting down");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_db_path_default() {
        let path = parse_db_path(&[]);
        assert!(path.to_string_lossy().contains("orqa-search-server.duckdb"));
    }

    #[test]
    fn parse_db_path_explicit() {
        let args = vec!["--db".to_string(), "/tmp/custom.duckdb".to_string()];
        let path = parse_db_path(&args);
        assert_eq!(path, PathBuf::from("/tmp/custom.duckdb"));
    }

    #[test]
    fn jsonrpc_response_ok_has_no_error() {
        let resp = JsonRpcResponse::ok(Some(serde_json::json!(1)), serde_json::json!({"foo": 1}));
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["jsonrpc"], "2.0");
        assert!(json["result"].is_object());
        assert!(json["error"].is_null());
    }

    #[test]
    fn jsonrpc_response_err_has_no_result() {
        let resp = JsonRpcResponse::err(Some(serde_json::json!(1)), -32603, "oops".to_string());
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["error"]["code"], -32603);
        assert_eq!(json["error"]["message"], "oops");
        assert!(json["result"].is_null());
    }

    #[test]
    fn error_value_produces_object_with_error_key() {
        let v = error_value("something failed".to_string());
        assert_eq!(v["error"], "something failed");
    }
}
