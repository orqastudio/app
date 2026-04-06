//! Search tool implementations: regex, semantic, research, status.

use orqa_engine::search::{SearchEngine, SearchResult};
use serde_json::{json, Value};

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

fn filter_by_scope(results: Vec<SearchResult>, scope: &str) -> Vec<SearchResult> {
    match scope {
        "artifacts" => results
            .into_iter()
            .filter(|r| r.file_path.contains(".orqa/") || r.file_path.contains(".orqa\\"))
            .collect(),
        "codebase" => results
            .into_iter()
            .filter(|r| !r.file_path.contains(".orqa/") && !r.file_path.contains(".orqa\\"))
            .collect(),
        _ => results, // "all" or unrecognised
    }
}

fn result_to_json(r: &SearchResult, source: Option<&str>) -> Value {
    let mut v = json!({
        "file": r.file_path,
        "line": r.start_line,
        "content": r.content,
        "score": r.score
    });
    if let Some(s) = source {
        v["source"] = json!(s);
    }
    v
}

// ---------------------------------------------------------------------------
// Tool implementations
// ---------------------------------------------------------------------------

/// Execute a regex search over the indexed codebase and return JSON-formatted results.
///
/// Reads `pattern`, `path_filter`, `scope`, and `limit` from `args`. Returns a pretty-printed
/// JSON array of matching chunks. Defaults: limit=20, scope="all".
pub fn tool_search_regex(engine: &mut SearchEngine, args: &Value) -> Result<String, String> {
    let pattern = args
        .get("pattern")
        .and_then(|v| v.as_str())
        .ok_or("missing 'pattern'")?;
    let path_filter = args.get("path_filter").and_then(|v| v.as_str());
    let scope = args.get("scope").and_then(|v| v.as_str()).unwrap_or("all");
    let limit = args.get("limit").and_then(Value::as_u64).unwrap_or(20) as u32;

    let results = engine
        .search_regex(pattern, path_filter, limit)
        .map_err(|e| format!("search error: {e}"))?;

    let filtered = filter_by_scope(results, scope);
    let summary: Vec<Value> = filtered.iter().map(|r| result_to_json(r, None)).collect();
    serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())
}

/// Execute a semantic (embedding-based) search and return JSON-formatted results.
///
/// Reads `query`, `scope`, and `limit` from `args`. Returns a pretty-printed JSON array of
/// matching chunks sorted by semantic similarity. Defaults: limit=10, scope="all".
pub fn tool_search_semantic(engine: &mut SearchEngine, args: &Value) -> Result<String, String> {
    let query = args
        .get("query")
        .and_then(|v| v.as_str())
        .ok_or("missing 'query'")?;
    let scope = args.get("scope").and_then(|v| v.as_str()).unwrap_or("all");
    let limit = args.get("limit").and_then(Value::as_u64).unwrap_or(10) as u32;

    let results = engine
        .search_semantic(query, limit)
        .map_err(|e| format!("semantic search error: {e}"))?;

    let filtered = filter_by_scope(results, scope);
    let summary: Vec<Value> = filtered.iter().map(|r| result_to_json(r, None)).collect();
    serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())
}

/// Execute a multi-step research query: semantic search followed by symbol-level follow-up.
///
/// Reads `question`, `scope`, and `limit` from `args`. Performs semantic search first; if
/// no results are found, falls back to a regex keyword search. Returns a structured JSON object
/// with primary and related results. Defaults: limit=5, scope="all".
#[allow(clippy::too_many_lines)]
pub fn tool_search_research(engine: &mut SearchEngine, args: &Value) -> Result<String, String> {
    let question = args
        .get("question")
        .and_then(|v| v.as_str())
        .ok_or("missing 'question'")?;
    let scope = args.get("scope").and_then(|v| v.as_str()).unwrap_or("all");
    let limit = args.get("limit").and_then(Value::as_u64).unwrap_or(5) as u32;

    // Step 1: Semantic search for conceptually relevant chunks
    let raw_results = engine
        .search_semantic(question, limit)
        .map_err(|e| format!("semantic search error: {e}"))?;
    let semantic_results = filter_by_scope(raw_results, scope);

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
        let fallback = engine
            .search_regex(&pattern, None, limit)
            .map_err(|e| format!("regex fallback error: {e}"))?;

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

    // Step 2: Extract symbols from semantic results
    let symbol_pattern = regex::Regex::new(
        r"(?:fn|pub fn|struct|enum|trait|type|const|interface|class|function|export)\s+(\w+)",
    )
    .map_err(|e| format!("regex compile error: {e}"))?;

    let symbols: Vec<String> = semantic_results
        .iter()
        .flat_map(|result| {
            symbol_pattern
                .captures_iter(&result.content)
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

    // Step 3: Regex follow-up for extracted symbols
    let mut follow_up_results = Vec::new();
    if !symbols.is_empty() {
        let follow_pattern = symbols
            .iter()
            .take(5)
            .cloned()
            .collect::<Vec<_>>()
            .join("|");
        if let Ok(results) = engine.search_regex(&follow_pattern, None, 10) {
            for r in results {
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

/// Return the current search index status as a JSON object.
///
/// Returns `is_indexed`, `chunk_count`, and `has_embeddings` fields reflecting
/// the current state of the ONNX search engine.
pub fn tool_search_status(engine: &mut SearchEngine) -> Result<String, String> {
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
