//! Structured event types for the OrqaStudio event bus.
//!
//! Defines the core event schema used by all components to emit log events
//! through the daemon's central event bus. All events are serializable for
//! storage, transport, and frontend display.

use serde::{Deserialize, Serialize};
use std::fmt;

/// The severity level of a log event.
///
/// Ordered from least to most severe. `Perf` is a separate axis used for
/// timing and performance measurements rather than error severity.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum EventLevel {
    /// Verbose diagnostic information, only useful during development.
    Debug,
    /// General informational messages about system operation.
    Info,
    /// Non-fatal warnings that indicate potential issues.
    Warn,
    /// Error conditions that require attention.
    Error,
    /// Performance timing measurements.
    Perf,
}

impl fmt::Display for EventLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Debug => write!(f, "DEBUG"),
            Self::Info => write!(f, "INFO"),
            Self::Warn => write!(f, "WARN"),
            Self::Error => write!(f, "ERROR"),
            Self::Perf => write!(f, "PERF"),
        }
    }
}

/// The originating component that emitted a log event.
///
/// Used to route, filter, and display events by their source subsystem.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum EventSource {
    /// The main daemon process.
    Daemon,
    /// The Tauri application backend.
    App,
    /// The Svelte frontend.
    Frontend,
    /// The dev controller coordinating the development environment.
    DevController,
    /// The MCP (Model Context Protocol) server.
    MCP,
    /// The LSP (Language Server Protocol) server.
    LSP,
    /// The semantic search service.
    Search,
    /// A background worker task.
    Worker,
}

impl fmt::Display for EventSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Daemon => write!(f, "daemon"),
            Self::App => write!(f, "app"),
            Self::Frontend => write!(f, "frontend"),
            Self::DevController => write!(f, "dev-controller"),
            Self::MCP => write!(f, "mcp"),
            Self::LSP => write!(f, "lsp"),
            Self::Search => write!(f, "search"),
            Self::Worker => write!(f, "worker"),
        }
    }
}

/// A structured log event emitted by any OrqaStudio component.
///
/// All events flowing through the daemon's event bus use this type.
/// The `metadata` field carries structured context that varies by event category.
/// The `session_id` ties events to a specific agent session when applicable.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogEvent {
    /// Monotonically increasing event identifier within a daemon session.
    pub id: u64,
    /// Unix timestamp in milliseconds when the event was emitted.
    pub timestamp: i64,
    /// Severity level of this event.
    pub level: EventLevel,
    /// The source component that emitted this event.
    pub source: EventSource,
    /// Logical grouping for filtering (e.g. "file-watcher", "agent", "build").
    pub category: String,
    /// Human-readable description of what occurred.
    pub message: String,
    /// Arbitrary structured context for this event, schema varies by category.
    pub metadata: serde_json::Value,
    /// Agent session this event belongs to, if applicable.
    pub session_id: Option<String>,
}
