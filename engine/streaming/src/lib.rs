//! orqa-streaming: Sidecar protocol and pure streaming logic for OrqaStudio.
//!
//! Contains the sidecar NDJSON protocol types (SidecarRequest, SidecarResponse)
//! and the pure streaming functions that have no Tauri dependency: response
//! translation, terminal detection, accumulation, and tool handler implementations
//! using only std operations.
//!
//! The daemon imports this crate to run the stream loop in-process and expose
//! SSE endpoints. The Tauri app is a pure HTTP/SSE consumer of those endpoints.

/// Sidecar NDJSON request/response protocol types.
pub mod protocol;
/// Pure stream loop logic: translation, accumulation, and terminal detection.
pub mod stream_loop;
/// Pure tool handler implementations with no Tauri dependency.
pub mod tools;

pub use protocol::{MessageSummary, SidecarRequest, SidecarResponse};
pub use stream_loop::{
    accumulate_response, friendly_context_overflow_message, is_terminal, translate_response,
    StreamAccumulator,
};
pub use tools::{
    format_search_results, resolve_path, resolve_write_path, strip_frontmatter, tool_bash,
    tool_edit_file, tool_glob, tool_grep, tool_load_knowledge, tool_read_file, tool_write_file,
    truncate_tool_output, DEFAULT_READ_FILE_MAX_LINES, MAX_TOOL_OUTPUT_CHARS, READ_ONLY_TOOLS,
};

// Re-export the search engine so callers that need search can source it from orqa-streaming.
pub use orqa_search::{chunker, embedder, store, types, SearchEngine, SearchError};
