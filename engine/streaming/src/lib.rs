// orqa-streaming: Sidecar streaming module for the OrqaStudio engine.
//
// Contains the sidecar protocol types (SidecarRequest, SidecarResponse) and
// the pure streaming logic that can be tested without a Tauri context:
// response translation, terminal detection, accumulation, and tool handlers
// that only use std operations.
//
// The Tauri-specific stream loop driver (holding AppState and Channel<T>)
// remains in the app layer and calls into this module for all business logic.

pub mod protocol;
pub mod stream_loop;
pub mod tools;

pub use protocol::{MessageSummary, SidecarRequest, SidecarResponse};
pub use stream_loop::{
    accumulate_response, friendly_context_overflow_message, is_terminal, translate_response,
    StreamAccumulator,
};
pub use tools::{
    format_search_results, resolve_path, resolve_write_path, strip_frontmatter,
    tool_bash, tool_edit_file, tool_glob, tool_grep, tool_load_knowledge, tool_read_file,
    tool_write_file, truncate_tool_output, DEFAULT_READ_FILE_MAX_LINES, MAX_TOOL_OUTPUT_CHARS,
    READ_ONLY_TOOLS,
};
