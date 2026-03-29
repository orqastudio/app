//! Domain logic for the OrqaStudio Tauri backend.
//!
//! Each submodule covers a distinct business-logic concern. All modules are
//! free of Tauri-specific types and can be tested in isolation.

/// Artifact ID, type, and metadata helpers.
pub mod artifact;
/// Artifact filesystem operations: reading and writing artifact files.
pub mod artifact_fs;
/// Artifact graph construction from `.orqa/` directory scans.
pub mod artifact_graph;
/// Artifact file reader: parse frontmatter and body from disk.
pub mod artifact_reader;
/// Project configuration loader: resolves `project.json` from a path.
pub mod config_loader;
/// Enforcement rule evaluation: runs rules against project artifacts.
pub mod enforcement;
/// Enforcement engine: compiles and caches rules for repeated evaluation.
pub mod enforcement_engine;
/// Enforcement rule parser: reads and validates `.orqa/rules/*.md` files.
pub mod enforcement_parser;
/// Enforcement violation types and formatting.
pub mod enforcement_violation;
/// Governance artifact read operations: reads raw files from `.orqa/`.
pub mod governance;
/// Governance directory scanner: enumerates all `.orqa/` artifact files.
pub mod governance_scanner;
/// Health snapshot persistence: stores graph health metrics over time.
pub mod health_snapshot;
/// Integrity engine: runs integrity checks against the artifact graph.
pub mod integrity_engine;
/// Knowledge injector: ONNX-based semantic knowledge artifact matching.
pub mod knowledge_injector;
/// Lesson CRUD domain logic.
pub mod lessons;
/// Conversation message types and persistence helpers.
pub mod message;
/// Path resolution: derives artifact and config paths from project root.
pub mod paths;
/// Platform configuration: reads plugin-contributed platform config.
pub mod platform_config;
/// Process gates: checks preconditions before workflow state transitions.
pub mod process_gates;
/// Process state: tracks agent process context and workflow violations.
pub mod process_state;
/// Project metadata: reads and writes project-level settings.
pub mod project;
/// Project scanner: detects stack, counts governance artifacts.
pub mod project_scanner;
/// Project settings: deserializes `project.json` into typed settings structs.
pub mod project_settings;
/// Provider events: Tauri event payloads emitted by streaming and tool execution.
pub mod provider_event;
/// Session CRUD: creates, lists, updates, and deletes conversation sessions.
pub mod session;
/// Session title generation: LLM-driven title inference via the sidecar.
pub mod session_title;
/// Application settings: persists and reads user preferences.
pub mod settings;
/// Setup wizard domain logic: checks Claude CLI auth and embedding model availability.
pub mod setup;
/// Status transition evaluation: validates and applies artifact workflow state changes.
pub mod status_transitions;
/// Streaming loop: translates sidecar NDJSON events into Tauri frontend events.
pub mod stream_loop;
/// System prompt builder: assembles governance artifacts into a structured prompt.
pub mod system_prompt;
/// Time utilities: ISO-8601 timestamp formatting.
pub mod time_utils;
/// Tool executor: dispatches tool calls from the Claude sidecar to domain handlers.
pub mod tool_executor;
/// Workflow tracker: records reads, writes, and tool invocations per session.
pub mod workflow_tracker;
/// Workflow configuration: builds process gate and tracker configs from resolved workflow.
pub mod workflow_config;
/// Workflow loader: reads process gate definitions from resolved workflow YAML files.
pub mod workflow_loader;
