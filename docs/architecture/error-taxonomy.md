---
title: "Error Taxonomy"
description: "Complete error type hierarchy using thiserror, covering all typed Rust error enums across the application."
category: architecture
tags: []
created: 2026-03-02
updated: 2026-03-04
---

**Date:** 2026-03-02 | **Status:** Phase 0e specification

Complete error type hierarchy for OrqaStudio™. Every error path in the application is represented by a typed Rust enum using `thiserror`. No `unwrap()`, `expect()`, or `panic!()` in production code (AD-003).

**Architecture References:** AD-003 (error propagation via Result types), AD-002 (IPC boundary design), AD-009 (streaming pipeline), AD-011 (security model)

---

## 1. Design Principles

1. **Every function returns `Result<T, E>`.** The application must never crash. Every error is caught, categorized, and surfaced to the user through an appropriate UI mechanism.

2. **Errors are typed, not stringly-typed.** Each error variant carries structured data (error codes, context, retry hints) that the frontend can use for display and recovery logic.

3. **Errors cross the IPC boundary as serialized JSON.** Tauri commands return `Result<T, IpcError>` where `IpcError` serializes to a JSON object with a stable schema.

4. **Error context is preserved through the call chain.** `#[from]` conversions on `thiserror` enums propagate lower-level errors upward while adding context at each layer.

5. **User-facing messages are separate from developer-facing messages.** Each error variant has a `Display` impl for logging and a `user_message()` method for the UI.

---

## 2. Error Type Hierarchy

```
OrqaError (top-level, all commands return this)
  |
  +-- DatabaseError
  |     +-- ConnectionFailed
  |     +-- MigrationFailed
  |     +-- QueryFailed
  |     +-- TransactionFailed
  |     +-- SchemaVersionMismatch
  |
  +-- IpcError
  |     +-- SerializationFailed
  |     +-- DeserializationFailed
  |     +-- ChannelClosed
  |     +-- CommandNotFound
  |     +-- PayloadTooLarge
  |
  +-- SidecarError
  |     +-- SpawnFailed
  |     +-- NotInstalled
  |     +-- CommunicationFailed
  |     +-- ProcessCrashed
  |     +-- ProtocolViolation
  |     +-- ShutdownFailed
  |     +-- HealthCheckFailed
  |
  +-- ProviderError
  |     +-- AuthenticationFailed
  |     +-- RateLimited
  |     +-- NetworkError
  |     +-- InvalidResponse
  |     +-- ModelNotAvailable
  |     +-- SubscriptionExpired
  |     +-- ContextWindowExceeded
  |
  +-- ToolError
  |     +-- FileNotFound
  |     +-- PermissionDenied
  |     +-- ExecutionFailed
  |     +-- Timeout
  |     +-- InvalidInput
  |     +-- SecurityViolation
  |     +-- OutputTooLarge
  |
  +-- ArtifactError
  |     +-- ParseFailed
  |     +-- ValidationFailed
  |     +-- FileSystemError
  |     +-- FrontmatterInvalid
  |     +-- WatcherFailed
  |     +-- IndexOutOfSync
  |
  +-- ThemeError
  |     +-- ExtractionFailed
  |     +-- ConversionFailed
  |     +-- InvalidTokenFormat
  |     +-- FallbackApplied
  |
  +-- McpError
        +-- ServerSpawnFailed
        +-- ConnectionFailed
        +-- ProtocolError
        +-- ToolNotFound
        +-- ServerCrashed
        +-- ConfigurationInvalid
        +-- TrustDenied
```

---

## 3. Rust Definitions

### 3.1 OrqaError (Top-Level)

```rust
use thiserror::Error;

/// Top-level error type for all OrqaStudio operations.
/// All Tauri commands return Result<T, OrqaError>.
#[derive(Error, Debug)]
pub enum OrqaError {
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),

    #[error("IPC error: {0}")]
    Ipc(#[from] IpcError),

    #[error("Sidecar error: {0}")]
    Sidecar(#[from] SidecarError),

    #[error("Provider error: {0}")]
    Provider(#[from] ProviderError),

    #[error("Tool error: {0}")]
    Tool(#[from] ToolError),

    #[error("Artifact error: {0}")]
    Artifact(#[from] ArtifactError),

    #[error("Theme error: {0}")]
    Theme(#[from] ThemeError),

    #[error("MCP error: {0}")]
    Mcp(#[from] McpError),
}
```

### 3.2 DatabaseError

```rust
/// Errors from SQLite operations via rusqlite and tauri-plugin-sql.
#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to connect to database at {path}: {source}")]
    ConnectionFailed {
        path: String,
        source: rusqlite::Error,
    },

    #[error("Migration {version} failed: {reason}")]
    MigrationFailed {
        version: u32,
        reason: String,
    },

    #[error("Query failed: {query}: {source}")]
    QueryFailed {
        query: String,
        source: rusqlite::Error,
    },

    #[error("Transaction failed: {operation}: {source}")]
    TransactionFailed {
        operation: String,
        source: rusqlite::Error,
    },

    #[error("Schema version mismatch: expected {expected}, found {found}")]
    SchemaVersionMismatch {
        expected: u32,
        found: u32,
    },
}
```

### 3.3 IpcError

```rust
/// Errors in the Tauri IPC boundary (Rust <-> TypeScript).
#[derive(Error, Debug)]
pub enum IpcError {
    #[error("Failed to serialize response: {0}")]
    SerializationFailed(#[from] serde_json::Error),

    #[error("Failed to deserialize request payload: {reason}")]
    DeserializationFailed {
        reason: String,
    },

    #[error("IPC channel closed unexpectedly")]
    ChannelClosed,

    #[error("Command not found: {name}")]
    CommandNotFound {
        name: String,
    },

    #[error("Payload exceeds maximum size: {size_bytes} bytes (max {max_bytes})")]
    PayloadTooLarge {
        size_bytes: usize,
        max_bytes: usize,
    },
}
```

### 3.4 SidecarError

```rust
/// Errors related to the Agent SDK sidecar process.
#[derive(Error, Debug)]
pub enum SidecarError {
    #[error("Failed to spawn sidecar: {reason}")]
    SpawnFailed {
        reason: String,
    },

    #[error("Claude Code CLI not found. Install it with: npm install -g @anthropic-ai/claude-code")]
    NotInstalled,

    #[error("Sidecar communication failed: {reason}")]
    CommunicationFailed {
        reason: String,
    },

    #[error("Sidecar process crashed with exit code {exit_code:?}")]
    ProcessCrashed {
        exit_code: Option<i32>,
        stderr: String,
    },

    #[error("Sidecar protocol violation: {message}")]
    ProtocolViolation {
        message: String,
        raw_line: Option<String>,
    },

    #[error("Failed to shut down sidecar cleanly: {reason}")]
    ShutdownFailed {
        reason: String,
    },

    #[error("Sidecar health check failed after {attempts} attempts")]
    HealthCheckFailed {
        attempts: u32,
    },
}
```

### 3.5 ProviderError

```rust
/// Errors from the AI provider (Claude API via Agent SDK).
#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Authentication failed: {reason}")]
    AuthenticationFailed {
        reason: String,
    },

    #[error("Rate limited. Retry after {retry_after_seconds}s")]
    RateLimited {
        retry_after_seconds: u64,
        is_daily_limit: bool,
    },

    #[error("Network error communicating with provider: {reason}")]
    NetworkError {
        reason: String,
        is_transient: bool,
    },

    #[error("Invalid response from provider: {reason}")]
    InvalidResponse {
        reason: String,
    },

    #[error("Model '{model}' is not available")]
    ModelNotAvailable {
        model: String,
    },

    #[error("Subscription expired or inactive")]
    SubscriptionExpired,

    #[error("Context window exceeded: {token_count} tokens (max {max_tokens})")]
    ContextWindowExceeded {
        token_count: u64,
        max_tokens: u64,
    },
}
```

### 3.6 ToolError

```rust
/// Errors from tool execution (Read, Write, Edit, Bash, Glob, Grep).
#[derive(Error, Debug)]
pub enum ToolError {
    #[error("File not found: {path}")]
    FileNotFound {
        path: String,
    },

    #[error("Permission denied: {path}: {reason}")]
    PermissionDenied {
        path: String,
        reason: String,
    },

    #[error("Tool execution failed: {tool}: {reason}")]
    ExecutionFailed {
        tool: String,
        reason: String,
    },

    #[error("Tool execution timed out after {timeout_seconds}s: {tool}")]
    Timeout {
        tool: String,
        timeout_seconds: u64,
    },

    #[error("Invalid tool input: {tool}: {reason}")]
    InvalidInput {
        tool: String,
        reason: String,
    },

    #[error("Security violation: {tool} attempted to access {path} outside allowed scope")]
    SecurityViolation {
        tool: String,
        path: String,
    },

    #[error("Tool output exceeds maximum size: {tool}: {size_bytes} bytes")]
    OutputTooLarge {
        tool: String,
        size_bytes: usize,
    },
}
```

### 3.7 ArtifactError

```rust
/// Errors related to governance artifact operations (.claude/ files).
#[derive(Error, Debug)]
pub enum ArtifactError {
    #[error("Failed to parse artifact {path}: {reason}")]
    ParseFailed {
        path: String,
        reason: String,
    },

    #[error("Artifact validation failed for {path}: {violations:?}")]
    ValidationFailed {
        path: String,
        violations: Vec<String>,
    },

    #[error("File system error for artifact {path}: {source}")]
    FileSystemError {
        path: String,
        source: std::io::Error,
    },

    #[error("Invalid YAML frontmatter in {path}: {reason}")]
    FrontmatterInvalid {
        path: String,
        reason: String,
    },

    #[error("File watcher failed for directory {path}: {reason}")]
    WatcherFailed {
        path: String,
        reason: String,
    },

    #[error("Artifact index out of sync for {path}: disk hash {disk_hash}, index hash {index_hash}")]
    IndexOutOfSync {
        path: String,
        disk_hash: String,
        index_hash: String,
    },
}
```

### 3.8 ThemeError

```rust
/// Errors related to per-project theming (design token extraction).
#[derive(Error, Debug)]
pub enum ThemeError {
    #[error("Failed to extract design tokens from {source_file}: {reason}")]
    ExtractionFailed {
        source_file: String,
        reason: String,
    },

    #[error("Failed to convert token '{token}' to CSS: {reason}")]
    ConversionFailed {
        token: String,
        reason: String,
    },

    #[error("Invalid token format in {source_file}: '{raw_value}' is not a valid {expected_type}")]
    InvalidTokenFormat {
        source_file: String,
        raw_value: String,
        expected_type: String,
    },

    #[error("Theme extraction incomplete, applying defaults for: {missing_tokens:?}")]
    FallbackApplied {
        missing_tokens: Vec<String>,
    },
}
```

### 3.9 McpError

```rust
/// Errors related to MCP server management (host role).
#[derive(Error, Debug)]
pub enum McpError {
    #[error("Failed to spawn MCP server '{server_name}': {reason}")]
    ServerSpawnFailed {
        server_name: String,
        reason: String,
    },

    #[error("Failed to connect to MCP server '{server_name}': {reason}")]
    ConnectionFailed {
        server_name: String,
        reason: String,
    },

    #[error("MCP protocol error from '{server_name}': {message}")]
    ProtocolError {
        server_name: String,
        message: String,
    },

    #[error("Tool '{tool_name}' not found on MCP server '{server_name}'")]
    ToolNotFound {
        server_name: String,
        tool_name: String,
    },

    #[error("MCP server '{server_name}' crashed with exit code {exit_code:?}")]
    ServerCrashed {
        server_name: String,
        exit_code: Option<i32>,
    },

    #[error("Invalid MCP configuration for '{server_name}': {reason}")]
    ConfigurationInvalid {
        server_name: String,
        reason: String,
    },

    #[error("MCP server '{server_name}' was denied by user (trust level: {trust_level})")]
    TrustDenied {
        server_name: String,
        trust_level: String,
    },
}
```

---

## 4. IPC Serialization (Rust to TypeScript)

### Serialization Format

All errors crossing the Tauri IPC boundary are serialized as JSON objects with a stable schema. The `OrqaError` enum implements `serde::Serialize` and Tauri's `IntoResponse` trait.

```rust
impl serde::Serialize for OrqaError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let ipc_error = IpcErrorPayload::from(self);
        ipc_error.serialize(serializer)
    }
}

/// Stable JSON shape for all errors sent to the frontend.
#[derive(serde::Serialize)]
pub struct IpcErrorPayload {
    /// Error category for programmatic handling.
    /// One of: "database", "ipc", "sidecar", "provider", "tool",
    ///         "artifact", "theme", "mcp"
    pub category: String,

    /// Specific error variant within the category.
    /// e.g., "rate_limited", "file_not_found", "spawn_failed"
    pub code: String,

    /// Human-readable message for display in the UI.
    pub message: String,

    /// Developer-facing details for logging.
    pub details: Option<String>,

    /// Whether the operation can be retried.
    pub recoverable: bool,

    /// Suggested retry delay in seconds (if recoverable).
    pub retry_after_seconds: Option<u64>,

    /// Additional structured context (varies by error type).
    pub context: Option<serde_json::Value>,
}
```

### TypeScript Type

```typescript
/** Error shape returned by all Tauri invoke() calls on failure. */
interface OrqaError {
  /** Error category: "database" | "ipc" | "sidecar" | "provider" | "tool" | "artifact" | "theme" | "mcp" */
  category: string;
  /** Specific error code within the category, e.g. "rate_limited" */
  code: string;
  /** Human-readable message suitable for display */
  message: string;
  /** Developer-facing details (not shown to user) */
  details?: string;
  /** Whether this error is recoverable (retry makes sense) */
  recoverable: boolean;
  /** Suggested retry delay in seconds */
  retry_after_seconds?: number;
  /** Additional structured context */
  context?: Record<string, unknown>;
}
```

### Conversion Example

A `ProviderError::RateLimited` in Rust:

```rust
ProviderError::RateLimited {
    retry_after_seconds: 30,
    is_daily_limit: false,
}
```

Serializes to this JSON on the IPC boundary:

```json
{
  "category": "provider",
  "code": "rate_limited",
  "message": "Rate limited. Retry after 30s",
  "details": null,
  "recoverable": true,
  "retry_after_seconds": 30,
  "context": {
    "is_daily_limit": false
  }
}
```

---

## 5. UI Surface Mapping

Each error category maps to one or more UI presentation mechanisms. The frontend uses the `category` and `code` fields to decide how to surface the error.

### Surface Types

| Surface | Description | Duration | User Action Required |
|---------|-------------|----------|---------------------|
| **Toast** | Non-blocking notification in bottom-right corner. | 5s auto-dismiss (error toasts: 8s). | Optional (click to dismiss or act). |
| **Inline error** | Error block rendered inside the conversation stream. | Persistent (part of message history). | Retry button or link to settings. |
| **Status bar** | Persistent indicator in the bottom status bar. | Until resolved. | Click to see details or navigate to settings. |
| **Dialog** | Modal dialog requiring user action before continuing. | Until dismissed. | Must acknowledge, choose action, or configure. |
| **Banner** | Non-modal banner above content area. | Until dismissed or resolved. | Dismiss or take action. |

### Error-to-Surface Mapping

| Error | Code | Primary Surface | Secondary Surface | Behavior |
|-------|------|----------------|-------------------|----------|
| `DatabaseError::ConnectionFailed` | `connection_failed` | Dialog | Status bar | App cannot function without DB. Dialog blocks until resolved. Status bar shows persistent red indicator. |
| `DatabaseError::MigrationFailed` | `migration_failed` | Dialog | -- | Fatal on startup. Dialog with "Reset Database" or "Contact Support" options. |
| `DatabaseError::QueryFailed` | `query_failed` | Toast | -- | "Failed to load {resource}. Retrying..." Auto-retry once. |
| `DatabaseError::TransactionFailed` | `transaction_failed` | Toast | -- | "Failed to save changes. Retrying..." |
| `DatabaseError::SchemaVersionMismatch` | `schema_version_mismatch` | Dialog | -- | "Database was created by a newer version of OrqaStudio. Please update." |
| `IpcError::SerializationFailed` | `serialization_failed` | Toast | -- | "Internal error. Please try again." Log full details. |
| `IpcError::DeserializationFailed` | `deserialization_failed` | Toast | -- | "Internal error. Please try again." |
| `IpcError::ChannelClosed` | `channel_closed` | Toast | Status bar | "Connection to backend lost. Restarting..." |
| `IpcError::CommandNotFound` | `command_not_found` | Toast | -- | "Feature unavailable. Please update OrqaStudio." |
| `IpcError::PayloadTooLarge` | `payload_too_large` | Toast | -- | "Response too large to display." |
| `SidecarError::SpawnFailed` | `spawn_failed` | Inline error | Status bar | Conversation: "Failed to start AI connection." Status bar: red dot. |
| `SidecarError::NotInstalled` | `not_installed` | Dialog | Status bar | "Claude Code CLI not found." With installation instructions and link to settings. |
| `SidecarError::CommunicationFailed` | `communication_failed` | Inline error | Status bar | "Connection to AI interrupted." Auto-restart attempt. |
| `SidecarError::ProcessCrashed` | `process_crashed` | Toast | Status bar | "AI connection lost. Reconnecting..." Auto-restart. |
| `SidecarError::ProtocolViolation` | `protocol_violation` | Toast | -- | "Unexpected response from AI. Please try again." |
| `SidecarError::ShutdownFailed` | `shutdown_failed` | -- | -- | Silent. Logged only. Force-kill proceeds. |
| `SidecarError::HealthCheckFailed` | `health_check_failed` | Status bar | -- | Status bar turns yellow. "AI connection degraded." |
| `ProviderError::AuthenticationFailed` | `auth_failed` | Inline error | Status bar | Conversation: "Authentication failed." Link to settings. Status bar: red dot. |
| `ProviderError::RateLimited` | `rate_limited` | Inline error | Status bar | Conversation: "Rate limited. Retry in {N}s." Countdown timer. Auto-retry. Status bar: yellow dot. |
| `ProviderError::NetworkError` | `network_error` | Inline error | Status bar | Conversation: "Network error." Retry button. Status bar: yellow/red dot. |
| `ProviderError::InvalidResponse` | `invalid_response` | Inline error | -- | Conversation: "Received invalid response. Please try again." |
| `ProviderError::ModelNotAvailable` | `model_not_available` | Inline error | -- | Conversation: "Model unavailable. Try a different model." |
| `ProviderError::SubscriptionExpired` | `subscription_expired` | Dialog | Status bar | "Your Claude subscription has expired." Link to subscription management. |
| `ProviderError::ContextWindowExceeded` | `context_exceeded` | Inline error | -- | "Conversation too long. Start a new session to continue." |
| `ToolError::FileNotFound` | `file_not_found` | Inline error | -- | Shown in tool call card: "File not found: {path}." |
| `ToolError::PermissionDenied` | `permission_denied` | Inline error | -- | Shown in tool call card: "Permission denied: {path}." |
| `ToolError::ExecutionFailed` | `execution_failed` | Inline error | -- | Shown in tool call card with error output. |
| `ToolError::Timeout` | `timeout` | Inline error | -- | Shown in tool call card: "Timed out after {N}s." |
| `ToolError::InvalidInput` | `invalid_input` | Inline error | -- | Shown in tool call card: "Invalid input: {reason}." |
| `ToolError::SecurityViolation` | `security_violation` | Inline error | Toast | Tool call card: red badge. Toast: "Blocked: access outside project scope." |
| `ToolError::OutputTooLarge` | `output_too_large` | Inline error | -- | Tool call card: "Output truncated ({size})." |
| `ArtifactError::ParseFailed` | `parse_failed` | Banner | -- | Above artifact viewer: "Failed to parse this file." Raw content shown instead. |
| `ArtifactError::ValidationFailed` | `validation_failed` | Banner | -- | Above artifact editor: "Validation errors: {list}." |
| `ArtifactError::FileSystemError` | `filesystem_error` | Toast | -- | "Failed to read/write {filename}." |
| `ArtifactError::FrontmatterInvalid` | `frontmatter_invalid` | Banner | -- | Above artifact viewer: "Invalid YAML frontmatter." |
| `ArtifactError::WatcherFailed` | `watcher_failed` | Toast | Status bar | "File watching interrupted." Status bar: yellow indicator. |
| `ArtifactError::IndexOutOfSync` | `index_out_of_sync` | -- | -- | Silent. Triggers automatic re-index. Logged. |
| `ThemeError::ExtractionFailed` | `extraction_failed` | Toast | -- | "Could not extract project theme. Using defaults." |
| `ThemeError::ConversionFailed` | `conversion_failed` | -- | -- | Silent. Falls back to default for that token. Logged. |
| `ThemeError::InvalidTokenFormat` | `invalid_token_format` | -- | -- | Silent. Falls back to default. Logged. |
| `ThemeError::FallbackApplied` | `fallback_applied` | -- | -- | Silent. Informational log only. |
| `McpError::ServerSpawnFailed` | `server_spawn_failed` | Toast | -- | "Failed to start MCP server '{name}'." |
| `McpError::ConnectionFailed` | `mcp_connection_failed` | Toast | -- | "Cannot connect to MCP server '{name}'." |
| `McpError::ProtocolError` | `protocol_error` | Toast | -- | "MCP server '{name}' sent an invalid response." |
| `McpError::ToolNotFound` | `tool_not_found` | Inline error | -- | Tool call card: "Tool not available on server." |
| `McpError::ServerCrashed` | `server_crashed` | Toast | -- | "MCP server '{name}' stopped unexpectedly." Auto-restart attempted. |
| `McpError::ConfigurationInvalid` | `config_invalid` | Toast | -- | "Invalid MCP configuration for '{name}'. Check settings." |
| `McpError::TrustDenied` | `trust_denied` | Toast | -- | "MCP server '{name}' was blocked." |

---

## 6. Error Recovery Strategies

### 6.1 Automatic Recovery

These errors are retried automatically without user intervention:

| Error | Strategy | Max Retries | Backoff |
|-------|----------|-------------|---------|
| `DatabaseError::QueryFailed` | Retry the query | 2 | 100ms, 500ms |
| `DatabaseError::TransactionFailed` | Retry the transaction | 2 | 100ms, 500ms |
| `SidecarError::ProcessCrashed` | Respawn sidecar | 1 | Immediate |
| `SidecarError::CommunicationFailed` | Restart communication channel | 1 | 500ms |
| `ProviderError::RateLimited` | Wait and retry | 1 | `retry_after_seconds` from response |
| `ProviderError::NetworkError` (transient) | Retry request | 3 | 1s, 2s, 4s (exponential) |
| `ArtifactError::IndexOutOfSync` | Re-index the file | 1 | Immediate |
| `ArtifactError::WatcherFailed` | Restart file watcher | 1 | 1s |
| `McpError::ServerCrashed` | Respawn server | 1 | 1s |
| `ThemeError::*` (all variants) | Apply default theme tokens | -- | Immediate |

### 6.2 User-Initiated Recovery

These errors require user action. The UI provides a clear path to recovery:

| Error | Recovery Path |
|-------|--------------|
| `DatabaseError::ConnectionFailed` | Dialog: "Reset Database" creates a fresh DB. "Choose Location" lets user pick a new path. |
| `DatabaseError::MigrationFailed` | Dialog: "Reset Database" (loses data) or "Update OrqaStudio" (if schema is from a newer version). |
| `DatabaseError::SchemaVersionMismatch` | Dialog: "Update OrqaStudio" link to download page. |
| `SidecarError::NotInstalled` | Dialog: step-by-step installation instructions for Claude Code CLI. "Check Again" button re-runs detection. |
| `SidecarError::SpawnFailed` | Inline error with "Retry" button. Link to settings for CLI path configuration. |
| `ProviderError::AuthenticationFailed` | Inline error: "Open Settings" link. Settings view shows re-authentication flow (`claude login`). |
| `ProviderError::SubscriptionExpired` | Dialog: link to claude.ai subscription management. |
| `ProviderError::ContextWindowExceeded` | Inline error: "New Session" button. Handoff summary preserves context. |
| `ProviderError::NetworkError` (persistent) | Status bar click opens network diagnostics. "Retry" button in conversation. |
| `ToolError::PermissionDenied` | Tool call card: "Grant Access" button expands Tauri file scope for the requested path. |
| `ToolError::SecurityViolation` | Tool call card: explains why the path is blocked. No override (deny always wins per AD-011). |
| `ArtifactError::ParseFailed` | Banner: "Edit Source" button opens raw file in CodeMirror to fix syntax. |
| `ArtifactError::ValidationFailed` | Banner: lists violations with line numbers. Clicking a violation jumps to the line in the editor. |
| `McpError::ConfigurationInvalid` | Toast links to Settings > MCP Servers. Highlights the misconfigured server. |
| `McpError::TrustDenied` | Toast: "You can approve this server in Settings > MCP Servers." |

### 6.3 Non-Recoverable Errors

These errors indicate a fundamental problem. The app degrades gracefully:

| Error | Degradation |
|-------|-------------|
| `DatabaseError::ConnectionFailed` (after reset) | App runs in read-only mode. No session persistence. Clear warning banner. |
| `SidecarError::SpawnFailed` (after retry) | Conversation disabled. Artifact browsing and editing still work. |
| `IpcError::ChannelClosed` | Full app restart required. Dialog: "OrqaStudio encountered an error. Restart?" |

---

## 7. Logging

All errors are logged before being surfaced to the UI. Log levels follow this mapping:

| Category | Log Level | Notes |
|----------|-----------|-------|
| `DatabaseError` | `error` | Always logged with full query context. |
| `IpcError` | `error` | Logged with serialized payload (redacted if sensitive). |
| `SidecarError` | `error` (crash, spawn) / `warn` (health check, shutdown) | stderr captured for crash diagnostics. |
| `ProviderError` | `warn` (rate limit, transient network) / `error` (auth, subscription) | Rate limit logs include window timing. |
| `ToolError` | `warn` (not found, timeout) / `error` (security violation) | Security violations are always `error`. |
| `ArtifactError` | `warn` (parse, frontmatter) / `error` (watcher) | Parse failures include the problematic content. |
| `ThemeError` | `info` (fallback) / `warn` (extraction, conversion) | Theme errors are never `error` level. |
| `McpError` | `warn` (connection, tool not found) / `error` (crash, spawn) | Server name always included. |

Logs are written via the `tracing` crate with structured fields. Sensitive data (API keys, file contents, user messages) is never included in log output.

---

## 8. Testing Strategy

### Unit Tests

Each error type has tests verifying:

- `Display` output matches expected format
- `From` conversions work correctly (e.g., `rusqlite::Error` -> `DatabaseError` -> `OrqaError`)
- `IpcErrorPayload` serialization produces the expected JSON shape
- `recoverable` flag is correct for each variant
- `user_message()` returns a user-friendly string distinct from the developer `Display`

### Integration Tests

- Simulate sidecar crash, verify auto-restart and UI notification
- Simulate rate limiting, verify countdown timer and auto-retry
- Simulate network loss, verify offline indicator and reconnect
- Simulate file permission errors, verify tool call card error display
- Verify that `SecurityViolation` errors cannot be overridden

### Property Tests

- Any `OrqaError` variant can be serialized to `IpcErrorPayload` and deserialized on the TypeScript side without loss of `category` or `code`
- No `OrqaError` variant produces an empty `message`
- All `recoverable: true` errors have either `retry_after_seconds` or a UI recovery path

---

## Related Documents

- [Architecture Decisions](./decisions.md) -- AD-003 (Result types), AD-010 (MCP tools), AD-011 (security)
- [MCP Host Interface](./mcp-host.md) -- McpError context
- [Interaction Patterns](/ui/interaction-patterns) -- Error states UI patterns
- [MVP Specification](/product/mvp-specification) -- Feature acceptance criteria referencing error handling
