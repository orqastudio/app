//! Search tool implementations: regex, semantic, research, status.
//!
//! All search operations proxy to the daemon's HTTP search endpoints rather
//! than embedding a local SearchEngine. This keeps a single DuckDB index
//! (in .state/search.duckdb) owned by the daemon, and makes the MCP server
//! stateless with respect to search.

use serde_json::{json, Value};

use crate::daemon::DaemonClient;
use crate::types::McpToolDefinition;

// ---------------------------------------------------------------------------
// Tool definitions
// ---------------------------------------------------------------------------

/// Return the MCP tool definition list for all search tools.
#[allow(clippy::too_many_lines)]
pub fn tool_definitions() -> Vec<McpToolDefinition> {
    vec![
        McpToolDefinition {
            name: "search_regex".into(),
            description: "Search indexed content with a regex pattern".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "pattern": { "type": "string", "description": "Regex pattern to search for" },
                    "path_filter": {
                        "type": "string",
                        "description": "Optional: filter results to files matching this path prefix"
                    },
                    "scope": {
                        "type": "string",
                        "enum": ["artifacts", "codebase", "all"],
                        "description": "Search scope: 'artifacts' (.orqa/ only), 'codebase' (non-.orqa/), 'all' (default)"
                    },
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
                    "scope": {
                        "type": "string",
                        "enum": ["artifacts", "codebase", "all"],
                        "description": "Search scope: 'artifacts' (.orqa/ only), 'codebase' (non-.orqa/), 'all' (default)"
                    },
                    "limit": { "type": "integer", "description": "Max results (default: 10)" }
                },
                "required": ["query"]
            }),
        },
        McpToolDefinition {
            name: "search_research".into(),
            description: "Compound research query: semantic search \u{2192} extract symbols \u{2192} regex follow-up \u{2192} assembled context. Use for 'how does X work?' questions.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "question": {
                        "type": "string",
                        "description": "Natural language question about the codebase"
                    },
                    "scope": {
                        "type": "string",
                        "enum": ["artifacts", "codebase", "all"],
                        "description": "Search scope: 'artifacts' (.orqa/ only), 'codebase' (non-.orqa/), 'all' (default)"
                    },
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
    ]
}

// ---------------------------------------------------------------------------
// Scope filtering
// ---------------------------------------------------------------------------

/// Filter a JSON array of search results by scope.
///
/// Applies after results come back from the daemon, because scope is an MCP
/// concept (artifact vs. codebase distinction) that the daemon does not know
/// about. Operates on the `file_path` field of each result object.
fn filter_by_scope(results: Vec<Value>, scope: &str) -> Vec<Value> {
    match scope {
        "artifacts" => results
            .into_iter()
            .filter(|r| {
                r.get("file_path")
                    .and_then(Value::as_str)
                    .is_some_and(|p| p.contains(".orqa/") || p.contains(".orqa\\"))
            })
            .collect(),
        "codebase" => results
            .into_iter()
            .filter(|r| {
                r.get("file_path")
                    .and_then(Value::as_str)
                    .is_some_and(|p| !p.contains(".orqa/") && !p.contains(".orqa\\"))
            })
            .collect(),
        _ => results, // "all" or unrecognised — pass through
    }
}

/// Convert a daemon search result JSON object to a display-friendly summary.
///
/// Keeps only `file`, `line`, `content`, and `score` fields. Optionally tags
/// the result with a `source` label (e.g. "semantic", "regex_fallback").
fn result_to_json(r: &Value, source: Option<&str>) -> Value {
    let mut v = json!({
        "file": r.get("file_path"),
        "line": r.get("start_line"),
        "content": r.get("content"),
        "score": r.get("score")
    });
    if let Some(s) = source {
        v["source"] = json!(s);
    }
    v
}

/// Parse a daemon response as a JSON array of search results.
///
/// Returns an error if the daemon returned an error object or a non-array value.
fn parse_results(response: Value) -> Result<Vec<Value>, String> {
    if let Some(err) = response.get("error") {
        return Err(format!("daemon search error: {err}"));
    }
    response
        .as_array()
        .cloned()
        .ok_or_else(|| "daemon returned non-array search response".to_owned())
}

// ---------------------------------------------------------------------------
// Tool implementations
// ---------------------------------------------------------------------------

/// Execute a regex search via the daemon and return JSON-formatted results.
///
/// Reads `pattern`, `path_filter`, `scope`, and `limit` from `args`. Proxies
/// to POST /search/regex on the daemon. Defaults: limit=20, scope="all".
pub fn tool_search_regex(daemon: &DaemonClient, args: &Value) -> Result<String, String> {
    let pattern = args
        .get("pattern")
        .and_then(|v| v.as_str())
        .ok_or("missing 'pattern'")?;
    let path_filter = args.get("path_filter").and_then(|v| v.as_str());
    let scope = args.get("scope").and_then(|v| v.as_str()).unwrap_or("all");
    let limit = args.get("limit").and_then(Value::as_u64).unwrap_or(20) as u32;

    let response = daemon
        .search_regex(pattern, path_filter, limit)
        .map_err(|e| e.to_string())?;

    let results = parse_results(response)?;
    let filtered = filter_by_scope(results, scope);
    let summary: Vec<Value> = filtered.iter().map(|r| result_to_json(r, None)).collect();
    serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())
}

/// Execute a semantic search via the daemon and return JSON-formatted results.
///
/// Reads `query`, `scope`, and `limit` from `args`. Proxies to POST
/// /search/semantic on the daemon. Defaults: limit=10, scope="all".
pub fn tool_search_semantic(daemon: &DaemonClient, args: &Value) -> Result<String, String> {
    let query = args
        .get("query")
        .and_then(|v| v.as_str())
        .ok_or("missing 'query'")?;
    let scope = args.get("scope").and_then(|v| v.as_str()).unwrap_or("all");
    let limit = args.get("limit").and_then(Value::as_u64).unwrap_or(10) as u32;

    let response = daemon
        .search_semantic(query, limit)
        .map_err(|e| e.to_string())?;

    let results = parse_results(response)?;
    let filtered = filter_by_scope(results, scope);
    let summary: Vec<Value> = filtered.iter().map(|r| result_to_json(r, None)).collect();
    serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())
}

/// Execute a multi-step research query via the daemon.
///
/// Reads `question`, `scope`, and `limit` from `args`. Performs semantic search
/// first; extracts symbols from results; runs a regex follow-up for those
/// symbols. Falls back to keyword regex if semantic search returns nothing.
/// Defaults: limit=5, scope="all".
#[allow(clippy::too_many_lines)]
pub fn tool_search_research(daemon: &DaemonClient, args: &Value) -> Result<String, String> {
    let question = args
        .get("question")
        .and_then(|v| v.as_str())
        .ok_or("missing 'question'")?;
    let scope = args.get("scope").and_then(|v| v.as_str()).unwrap_or("all");
    let limit = args.get("limit").and_then(Value::as_u64).unwrap_or(5) as u32;

    // Step 1: Semantic search for conceptually relevant chunks.
    let raw_response = daemon
        .search_semantic(question, limit)
        .map_err(|e| format!("semantic search error: {e}"))?;

    let raw_results = parse_results(raw_response)?;
    let semantic_results = filter_by_scope(raw_results, scope);

    if semantic_results.is_empty() {
        // Fall back to regex with keywords from the question.
        let keywords: Vec<&str> = question
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .take(3)
            .collect();

        if keywords.is_empty() {
            return Ok("No results found.".into());
        }

        let pattern = keywords.join("|");
        let fallback_response = daemon
            .search_regex(&pattern, None, limit)
            .map_err(|e| format!("regex fallback error: {e}"))?;
        let fallback = parse_results(fallback_response)?;

        let summary: Vec<Value> = fallback
            .iter()
            .map(|r| result_to_json(r, Some("regex_fallback")))
            .collect();

        return serde_json::to_string_pretty(&json!({
            "question": question,
            "method": "regex_fallback",
            "results": summary
        }))
        .map_err(|e| e.to_string());
    }

    // Step 2: Extract symbols from semantic result content.
    let symbol_pattern = regex::Regex::new(
        r"(?:fn|pub fn|struct|enum|trait|type|const|interface|class|function|export)\s+(\w+)",
    )
    .map_err(|e| format!("regex compile error: {e}"))?;

    let symbols: Vec<String> = semantic_results
        .iter()
        .flat_map(|result| {
            let content = result
                .get("content")
                .and_then(Value::as_str)
                .unwrap_or_default();
            symbol_pattern
                .captures_iter(content)
                .map(|cap| cap[1].to_string())
                .collect::<Vec<_>>()
        })
        .filter(|sym| sym.len() > 2)
        .fold(Vec::new(), |mut acc, sym| {
            if !acc.contains(&sym) {
                acc.push(sym);
            }
            acc
        });

    // Step 3: Regex follow-up for extracted symbols.
    let mut follow_up_results = Vec::new();
    if !symbols.is_empty() {
        let follow_pattern = symbols
            .iter()
            .take(5)
            .cloned()
            .collect::<Vec<_>>()
            .join("|");
        if let Ok(resp) = daemon.search_regex(&follow_pattern, None, 10) {
            if let Ok(results) = parse_results(resp) {
                for r in results {
                    // Skip results already in the semantic set (same file + line).
                    let already_found = semantic_results.iter().any(|s| {
                        s.get("file_path") == r.get("file_path")
                            && s.get("start_line") == r.get("start_line")
                    });
                    if !already_found {
                        follow_up_results.push(r);
                    }
                }
            }
        }
    }

    // Step 4: Assemble response.
    let primary: Vec<Value> = semantic_results
        .iter()
        .map(|r| result_to_json(r, Some("semantic")))
        .collect();

    let related: Vec<Value> = follow_up_results
        .iter()
        .map(|r| result_to_json(r, Some("symbol_follow_up")))
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

/// Return the current search index status from the daemon as a JSON object.
///
/// Proxies to GET /search/status on the daemon and forwards the response fields.
pub fn tool_search_status(daemon: &DaemonClient) -> Result<String, String> {
    let status = daemon.search_status().map_err(|e| e.to_string())?;

    if let Some(err) = status.get("error") {
        return Err(format!("daemon status error: {err}"));
    }

    serde_json::to_string_pretty(&json!({
        "is_indexed": status.get("is_indexed"),
        "chunk_count": status.get("chunk_count"),
        "has_embeddings": status.get("has_embeddings"),
    }))
    .map_err(|e| e.to_string())
}
