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
//! When `ORQA_DEV_LOG_PORT` environment variable is set (e.g. `10401`), the server
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

#[derive(Debug, Deserialize)]
struct DownloadModelParams {
    model_dir: String,
}

fn default_max_results() -> u32 {
    20
}

// ---------------------------------------------------------------------------
// CLI argument parsing
// ---------------------------------------------------------------------------

/// Parsed CLI options.
struct CliOptions {
    db_path: PathBuf,
    model_dir: Option<PathBuf>,
}

/// Parse CLI arguments.
///
/// Usage: `orqa-search-server [--db <path>] [--model-dir <path>]`
///
/// If `--db` is not provided, uses `<temp_dir>/orqa-search-server.duckdb`.
/// If `--model-dir` is not provided, falls back to `ORQA_MODEL_DIR` env var,
/// then `models/all-MiniLM-L6-v2/` relative to the current directory.
fn parse_cli_options(args: &[String]) -> CliOptions {
    let mut db_path: Option<PathBuf> = None;
    let mut model_dir: Option<PathBuf> = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--db" && i + 1 < args.len() {
            db_path = Some(PathBuf::from(&args[i + 1]));
            i += 2;
        } else if args[i] == "--model-dir" && i + 1 < args.len() {
            model_dir = Some(PathBuf::from(&args[i + 1]));
            i += 2;
        } else {
            i += 1;
        }
    }
    CliOptions {
        db_path: db_path.unwrap_or_else(|| std::env::temp_dir().join("orqa-search-server.duckdb")),
        model_dir,
    }
}

/// Resolve the model directory from CLI arg, env var, or default.
fn resolve_model_dir(cli_model_dir: Option<PathBuf>) -> PathBuf {
    if let Some(dir) = cli_model_dir {
        return dir;
    }
    if let Ok(dir) = std::env::var("ORQA_MODEL_DIR") {
        return PathBuf::from(dir);
    }
    PathBuf::from("models").join("all-MiniLM-L6-v2")
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
        "embed_chunks" => handle_embed_chunks(engine),
        "download_model" => handle_download_model(&request.params),
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

fn handle_embed_chunks(engine: &mut SearchEngine) -> Value {
    match engine.embed_chunks() {
        Ok(count) => serde_json::json!({ "embedded": count }),
        Err(e) => error_value(e.to_string()),
    }
}

fn handle_download_model(params: &Value) -> Value {
    let p: DownloadModelParams = match serde_json::from_value(params.clone()) {
        Ok(v) => v,
        Err(e) => return error_value(format!("invalid params: {e}")),
    };
    let model_dir = PathBuf::from(&p.model_dir);
    info!("downloading model to {}", model_dir.display());

    // Run the async download in a blocking context.
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(e) => return error_value(format!("failed to create tokio runtime: {e}")),
    };
    match rt.block_on(orqa_search::embedder::ensure_model_exists(
        &model_dir,
        |file, downloaded, total| {
            if let Some(total) = total {
                info!("downloading {file}: {downloaded}/{total} bytes");
            } else {
                info!("downloading {file}: {downloaded} bytes");
            }
        },
    )) {
        Ok(()) => {
            info!("model download complete: {}", model_dir.display());
            serde_json::json!({ "downloaded": true, "model_dir": p.model_dir })
        }
        Err(e) => error_value(format!("model download failed: {e}")),
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
    let options = parse_cli_options(&args);
    let model_dir = resolve_model_dir(options.model_dir);

    info!(
        "orqa-search-server starting, db={}",
        options.db_path.display()
    );

    let mut engine = match SearchEngine::new(&options.db_path) {
        Ok(e) => e,
        Err(err) => {
            error!("failed to initialize search engine: {err}");
            std::process::exit(1);
        }
    };

    // Auto-init embedder if model files exist on disk.
    if model_dir.join("model.onnx").exists() && model_dir.join("tokenizer.json").exists() {
        info!(
            "model files found at {}, initializing embedder",
            model_dir.display()
        );
        match engine.init_embedder_sync(&model_dir) {
            Ok(()) => {
                info!("embedder initialized, embedding any unembedded chunks");
                match engine.embed_chunks() {
                    Ok(count) => info!("embedded {count} chunks"),
                    Err(e) => warn!("failed to embed chunks: {e}"),
                }
            }
            Err(e) => warn!("failed to initialize embedder: {e}"),
        }
    } else {
        warn!(
            "model files not found at {}, semantic search unavailable (regex search still works)",
            model_dir.display()
        );
    }

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
                if request.jsonrpc == "2.0" {
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
                } else {
                    JsonRpcResponse::err(request.id, -32600, "invalid JSON-RPC version".to_string())
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
    fn parse_cli_options_default() {
        let options = parse_cli_options(&[]);
        assert!(options
            .db_path
            .to_string_lossy()
            .contains("orqa-search-server.duckdb"));
        assert!(options.model_dir.is_none());
    }

    #[test]
    fn parse_cli_options_explicit() {
        let args = vec![
            "--db".to_string(),
            "/tmp/custom.duckdb".to_string(),
            "--model-dir".to_string(),
            "/tmp/models".to_string(),
        ];
        let options = parse_cli_options(&args);
        assert_eq!(options.db_path, PathBuf::from("/tmp/custom.duckdb"));
        assert_eq!(options.model_dir, Some(PathBuf::from("/tmp/models")));
    }

    #[test]
    fn resolve_model_dir_uses_cli_first() {
        let dir = resolve_model_dir(Some(PathBuf::from("/custom/models")));
        assert_eq!(dir, PathBuf::from("/custom/models"));
    }

    #[test]
    fn resolve_model_dir_default_fallback() {
        // When no CLI arg and no env var, falls back to models/all-MiniLM-L6-v2
        let dir = resolve_model_dir(None);
        assert!(dir.to_string_lossy().contains("all-MiniLM-L6-v2"));
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
