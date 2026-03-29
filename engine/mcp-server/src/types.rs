//! MCP protocol types and JSON-RPC envelope types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// JSON-RPC envelope types
// ---------------------------------------------------------------------------

/// An incoming JSON-RPC request (one line from stdin).
#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC protocol version (always `"2.0"`).
    #[allow(dead_code)]
    pub jsonrpc: String,
    /// Request identifier, echoed back in the response.
    pub id: Option<Value>,
    /// The method name being called (e.g. `"tools/list"`, `"tools/call"`).
    pub method: String,
    /// Method parameters, if any.
    #[serde(default)]
    pub params: Value,
}

/// An outgoing JSON-RPC response (one line to stdout).
#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC protocol version (always `"2.0"`).
    pub jsonrpc: String,
    /// Request identifier echoed from the incoming request.
    pub id: Value,
    /// Successful result payload; mutually exclusive with `error`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error payload; mutually exclusive with `result`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// A JSON-RPC error payload.
#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcError {
    /// Numeric error code per the JSON-RPC 2.0 specification.
    pub code: i64,
    /// Human-readable error message.
    pub message: String,
    /// Optional additional error data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

// ---------------------------------------------------------------------------
// MCP protocol types
// ---------------------------------------------------------------------------

/// A tool definition returned in `tools/list`.
#[derive(Debug, Clone, Serialize)]
pub struct McpToolDefinition {
    /// Unique name for this tool (used in `tools/call` requests).
    pub name: String,
    /// Human-readable description of what this tool does.
    pub description: String,
    /// JSON Schema describing the tool's input parameters.
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

/// A resource definition returned in `resources/list`.
#[derive(Debug, Clone, Serialize)]
pub struct McpResource {
    /// URI identifying this resource.
    pub uri: String,
    /// Human-readable name for this resource.
    pub name: String,
    /// Human-readable description of this resource's content.
    pub description: String,
    /// MIME type of the resource content.
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}
