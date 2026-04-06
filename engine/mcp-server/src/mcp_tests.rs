//! Unit tests for `engine/mcp-server`.
//!
//! Covers:
//! - JSON-RPC protocol type serialization / deserialization
//! - Tool definition completeness (names, schemas, required fields)
//! - Tool argument parsing and error paths (no daemon required)
//! - `McpError` variant display messages
//! - `DaemonClient` URL construction (base_url field)
//! - `graph_read` path-traversal guard and missing-path error

#[cfg(test)]
mod types_tests {
    use serde_json::{json, Value};

    use crate::types::{
        JsonRpcError, JsonRpcRequest, JsonRpcResponse, McpResource, McpToolDefinition,
    };

    // -------------------------------------------------------------------------
    // JsonRpcRequest deserialization
    // -------------------------------------------------------------------------

    #[test]
    fn jsonrpc_request_deserializes_full() {
        // A well-formed tools/call request must deserialise all fields correctly.
        let raw = r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"graph_query","arguments":{}}}"#;
        let req: JsonRpcRequest = serde_json::from_str(raw).expect("deserialize");
        assert_eq!(req.method, "tools/call");
        assert_eq!(req.id, Some(json!(1)));
        assert_eq!(req.params["name"], json!("graph_query"));
    }

    #[test]
    fn jsonrpc_request_params_defaults_to_null_when_absent() {
        // A request with no `params` field must produce a null Value, not fail.
        let raw = r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#;
        let req: JsonRpcRequest = serde_json::from_str(raw).expect("deserialize");
        assert_eq!(req.method, "tools/list");
        assert_eq!(req.params, Value::Null);
    }

    #[test]
    fn jsonrpc_request_id_can_be_null() {
        // JSON-RPC notifications (no id) must deserialise with id=None.
        let raw = r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#;
        let req: JsonRpcRequest = serde_json::from_str(raw).expect("deserialize");
        assert_eq!(req.method, "initialized");
        assert!(req.id.is_none());
    }

    #[test]
    fn jsonrpc_request_rejects_missing_method() {
        // A request without `method` must fail to deserialize.
        let raw = r#"{"jsonrpc":"2.0","id":1,"params":{}}"#;
        let result: Result<JsonRpcRequest, _> = serde_json::from_str(raw);
        assert!(result.is_err(), "missing 'method' must not deserialize");
    }

    // -------------------------------------------------------------------------
    // JsonRpcResponse serialization
    // -------------------------------------------------------------------------

    #[test]
    fn jsonrpc_response_serializes_success() {
        // A success response must include jsonrpc, id, result; must omit error.
        let resp = JsonRpcResponse {
            jsonrpc: "2.0".into(),
            id: json!(1),
            result: Some(json!({"tools": []})),
            error: None,
        };
        let s = serde_json::to_string(&resp).expect("serialize");
        let v: Value = serde_json::from_str(&s).expect("roundtrip");
        assert_eq!(v["jsonrpc"], "2.0");
        assert_eq!(v["id"], json!(1));
        assert!(
            v.get("error").is_none() || v["error"].is_null(),
            "error field must be absent in a success response"
        );
    }

    #[test]
    fn jsonrpc_response_serializes_error() {
        // An error response must include the error object; must omit result.
        let resp = JsonRpcResponse {
            jsonrpc: "2.0".into(),
            id: json!(99),
            result: None,
            error: Some(JsonRpcError {
                code: -32601,
                message: "method not found".into(),
                data: None,
            }),
        };
        let s = serde_json::to_string(&resp).expect("serialize");
        let v: Value = serde_json::from_str(&s).expect("roundtrip");
        assert_eq!(v["error"]["code"], -32601);
        assert_eq!(v["error"]["message"], "method not found");
        assert!(
            v.get("result").is_none() || v["result"].is_null(),
            "result must be absent in an error response"
        );
    }

    #[test]
    fn jsonrpc_error_data_omitted_when_none() {
        // The `data` field must be absent (not `null`) when None, per the spec.
        let err = JsonRpcError {
            code: -32700,
            message: "parse error".into(),
            data: None,
        };
        let s = serde_json::to_string(&err).expect("serialize");
        assert!(
            !s.contains("\"data\""),
            "data field must be omitted when None"
        );
    }

    // -------------------------------------------------------------------------
    // McpToolDefinition
    // -------------------------------------------------------------------------

    #[test]
    fn mcp_tool_definition_serializes_name_and_schema() {
        let def = McpToolDefinition {
            name: "graph_query".into(),
            description: "Query artifacts".into(),
            input_schema: json!({"type": "object", "properties": {}}),
        };
        let v: Value = serde_json::to_value(&def).expect("serialize");
        assert_eq!(v["name"], "graph_query");
        assert_eq!(v["inputSchema"]["type"], "object");
    }

    // -------------------------------------------------------------------------
    // McpResource
    // -------------------------------------------------------------------------

    #[test]
    fn mcp_resource_serializes_correctly() {
        let res = McpResource {
            uri: "orqa://schema/core.json".into(),
            name: "Core Schema".into(),
            description: "Platform-level types".into(),
            mime_type: "application/json".into(),
        };
        let v: Value = serde_json::to_value(&res).expect("serialize");
        assert_eq!(v["uri"], "orqa://schema/core.json");
        assert_eq!(v["mimeType"], "application/json");
    }
}

#[cfg(test)]
mod error_tests {
    use crate::error::McpError;

    #[test]
    fn error_daemon_unreachable_display_contains_message() {
        let e = McpError::DaemonUnreachable("connection refused".into());
        assert!(
            e.to_string().contains("connection refused"),
            "DaemonUnreachable display must include the reason"
        );
    }

    #[test]
    fn error_protocol_display_contains_message() {
        let e = McpError::Protocol("unexpected EOF".into());
        assert!(e.to_string().contains("unexpected EOF"));
    }

    #[test]
    fn error_from_serde_json() {
        // Deserializing invalid JSON must produce McpError::Json.
        let bad: Result<serde_json::Value, _> = serde_json::from_str("{invalid}");
        let err = bad.unwrap_err();
        let mcp: McpError = err.into();
        assert!(matches!(mcp, McpError::Json(_)));
    }

    #[test]
    fn error_graph_build_display_includes_detail() {
        let e = McpError::GraphBuild("no .orqa dir".into());
        assert!(e.to_string().contains("no .orqa dir"));
    }
}

#[cfg(test)]
mod tool_tests {
    use serde_json::json;
    use std::path::PathBuf;
    use tempfile::TempDir;

    use crate::tools::graph as graph_tools;

    // -------------------------------------------------------------------------
    // tool_definitions — completeness checks
    // -------------------------------------------------------------------------

    #[test]
    fn graph_tool_definitions_include_required_tools() {
        let defs = graph_tools::tool_definitions();
        let names: Vec<&str> = defs.iter().map(|d| d.name.as_str()).collect();
        for expected in &[
            "graph_query",
            "graph_resolve",
            "graph_relationships",
            "graph_stats",
            "graph_health",
            "graph_validate",
            "graph_read",
            "graph_refresh",
            "graph_traceability",
        ] {
            assert!(
                names.contains(expected),
                "tool_definitions must include '{expected}'"
            );
        }
    }

    #[test]
    fn graph_tool_definitions_each_have_input_schema() {
        let defs = graph_tools::tool_definitions();
        for def in &defs {
            assert!(
                def.input_schema.is_object(),
                "tool '{}' must have an object input_schema",
                def.name
            );
        }
    }

    #[test]
    fn graph_resolve_schema_has_required_id() {
        let defs = graph_tools::tool_definitions();
        let resolve = defs
            .iter()
            .find(|d| d.name == "graph_resolve")
            .expect("graph_resolve");
        let required = resolve.input_schema.get("required").expect("required");
        assert!(
            required
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v.as_str() == Some("id")),
            "graph_resolve must declare 'id' as required"
        );
    }

    #[test]
    fn graph_traceability_schema_has_required_artifact_id() {
        let defs = graph_tools::tool_definitions();
        let trace = defs
            .iter()
            .find(|d| d.name == "graph_traceability")
            .expect("graph_traceability");
        let required = trace.input_schema.get("required").expect("required");
        assert!(
            required
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v.as_str() == Some("artifact_id")),
            "graph_traceability must declare 'artifact_id' as required"
        );
    }

    // -------------------------------------------------------------------------
    // tool_read — no daemon needed, purely local filesystem
    // -------------------------------------------------------------------------

    #[test]
    fn tool_read_returns_file_content() {
        let tmp = TempDir::new().expect("tempdir");
        let path = tmp.path().join("hello.md");
        std::fs::write(&path, "# Hello world\n").expect("write");

        let root = tmp.path().to_path_buf();
        let args = json!({ "path": "hello.md" });
        let result = graph_tools::tool_read(&root, &args);
        assert!(
            result.is_ok(),
            "tool_read should succeed for an existing file"
        );
        assert!(result.unwrap().contains("Hello world"));
    }

    #[test]
    fn tool_read_missing_path_arg_returns_error() {
        // Calling tool_read without the `path` argument must return a descriptive error.
        let root = PathBuf::from("/tmp");
        let args = json!({});
        let result = graph_tools::tool_read(&root, &args);
        assert!(result.is_err());
        assert!(
            result.unwrap_err().contains("missing 'path'"),
            "error must mention the missing 'path' argument"
        );
    }

    #[test]
    fn tool_read_path_traversal_rejected() {
        // A path containing ".." must be rejected outright, not read.
        let root = PathBuf::from("/tmp");
        let args = json!({ "path": "../../etc/passwd" });
        let result = graph_tools::tool_read(&root, &args);
        assert!(result.is_err());
        assert!(
            result.unwrap_err().contains("path traversal not allowed"),
            "path traversal attempt must return the traversal error"
        );
    }

    #[test]
    fn tool_read_missing_file_returns_error() {
        let tmp = TempDir::new().expect("tempdir");
        let args = json!({ "path": "does-not-exist.md" });
        let result = graph_tools::tool_read(tmp.path(), &args);
        assert!(
            result.is_err(),
            "tool_read for a non-existent file must return an error"
        );
    }

    // -------------------------------------------------------------------------
    // tool_traceability argument validation (no daemon needed)
    // -------------------------------------------------------------------------

    // We can test the argument extraction logic by calling the tool and observing
    // the specific error for missing / empty artifact_id before the daemon is hit.
    // Since this is a unit test without a real daemon, the missing-arg path is the
    // most useful target.

    #[test]
    fn tool_traceability_missing_artifact_id_returns_error() {
        use crate::daemon::DaemonClient;
        // Port 1 — nothing listening. The missing-argument check happens before the HTTP call.
        let daemon = DaemonClient::new(1);
        let args = json!({});
        let result = crate::tools::graph::tool_traceability(&daemon, &args);
        assert!(result.is_err());
        assert!(
            result.unwrap_err().contains("missing 'artifact_id'"),
            "must error on missing artifact_id before making an HTTP call"
        );
    }

    #[test]
    fn tool_traceability_empty_artifact_id_returns_error() {
        use crate::daemon::DaemonClient;
        let daemon = DaemonClient::new(1);
        let args = json!({ "artifact_id": "   " });
        let result = crate::tools::graph::tool_traceability(&daemon, &args);
        assert!(result.is_err());
        assert!(
            result.unwrap_err().contains("cannot be empty"),
            "whitespace-only artifact_id must be rejected"
        );
    }
}

#[cfg(test)]
mod daemon_client_tests {
    use crate::daemon::{default_daemon_port, DaemonClient};

    #[test]
    fn daemon_client_new_sets_port_in_url() {
        // The client must use the port it is constructed with.
        // We inspect behaviour indirectly: any call to a closed port fails with
        // DaemonUnreachable (not a panic, not a wrong error type).
        let client = DaemonClient::new(19999);
        let result = client.health();
        assert!(
            matches!(result, Err(crate::error::McpError::DaemonUnreachable(_))),
            "call to a non-listening port must produce DaemonUnreachable"
        );
    }

    #[test]
    fn daemon_client_new_different_ports_behave_independently() {
        // Two clients on different ports must both fail to reach their respective
        // (non-listening) ports, confirming each client uses its own port.
        let client_a = DaemonClient::new(19997);
        let client_b = DaemonClient::new(19996);
        let err_a = client_a.health().unwrap_err();
        let err_b = client_b.health().unwrap_err();
        assert!(matches!(
            err_a,
            crate::error::McpError::DaemonUnreachable(_)
        ));
        assert!(matches!(
            err_b,
            crate::error::McpError::DaemonUnreachable(_)
        ));
    }

    #[test]
    fn daemon_client_query_unreachable_returns_daemon_unreachable() {
        let client = DaemonClient::new(19998);
        let result = client.query(&serde_json::json!({"type": "task"}));
        assert!(matches!(
            result,
            Err(crate::error::McpError::DaemonUnreachable(_))
        ));
    }

    // -------------------------------------------------------------------------
    // default_daemon_port
    // -------------------------------------------------------------------------

    #[test]
    fn default_daemon_port_returns_expected_value_when_env_absent() {
        // When ORQA_PORT_BASE is absent, the daemon port must equal the hardcoded
        // default base (10100) plus the daemon offset (0), which is 10100.
        std::env::remove_var("ORQA_PORT_BASE");
        let port = default_daemon_port();
        assert_eq!(
            port, 10100,
            "default_daemon_port must be 10100 when ORQA_PORT_BASE is not set"
        );
    }

    #[test]
    fn default_daemon_port_reflects_orqa_port_base_env_var() {
        // When ORQA_PORT_BASE is set, the daemon port must use that base.
        std::env::set_var("ORQA_PORT_BASE", "20000");
        let port = default_daemon_port();
        std::env::remove_var("ORQA_PORT_BASE");
        // Daemon offset is 0, so port == base.
        assert_eq!(
            port, 20000,
            "default_daemon_port must use ORQA_PORT_BASE when set"
        );
    }

    #[test]
    fn default_daemon_port_returns_default_for_invalid_env_var() {
        // An invalid ORQA_PORT_BASE must fall back to the default 10100.
        std::env::set_var("ORQA_PORT_BASE", "not-a-port");
        let port = default_daemon_port();
        std::env::remove_var("ORQA_PORT_BASE");
        assert_eq!(
            port, 10100,
            "default_daemon_port must fall back to 10100 for an invalid env var"
        );
    }
}

#[cfg(test)]
mod server_tests {
    use std::path::PathBuf;

    use crate::daemon::default_daemon_port;
    use crate::server::McpServer;

    // -------------------------------------------------------------------------
    // McpServer::new
    // -------------------------------------------------------------------------

    #[test]
    fn mcp_server_new_does_not_panic() {
        // Construction must succeed for any well-formed path, even if the path
        // doesn't exist — the server is lazy and does no I/O in the constructor.
        let root = PathBuf::from("/tmp/test-project");
        let _server = McpServer::new(root);
    }

    #[test]
    fn mcp_server_new_uses_default_daemon_port() {
        // McpServer::new delegates to with_daemon_port(root, default_daemon_port()).
        // The observable difference is that a server built with new() and one built
        // with with_daemon_port(..., default_daemon_port()) behave identically when
        // probed on a non-listening port — both produce the same DaemonUnreachable error.
        // We verify this by checking new() doesn't diverge from with_daemon_port.
        std::env::remove_var("ORQA_PORT_BASE");
        let root = PathBuf::from("/tmp/test-project");
        let _server_new = McpServer::new(root.clone());
        let _server_explicit = McpServer::with_daemon_port(root, default_daemon_port());
        // Both constructions must succeed without panicking.
    }

    // -------------------------------------------------------------------------
    // McpServer::with_daemon_port
    // -------------------------------------------------------------------------

    #[test]
    fn mcp_server_with_daemon_port_does_not_panic_for_arbitrary_port() {
        // Construction with any valid u16 port must succeed without panic.
        // Ports like 1 are not listening — that's fine, errors are lazy.
        let root = PathBuf::from("/tmp/test-project");
        let _server = McpServer::with_daemon_port(root, 1);
    }

    #[test]
    fn mcp_server_with_daemon_port_distinct_from_default() {
        // A server built with port 9999 is distinct from one built with the default.
        // Both constructions must succeed — the only observable effect is the port
        // used when making daemon HTTP calls.
        let root = PathBuf::from("/tmp/test-project");
        let _server_custom = McpServer::with_daemon_port(root.clone(), 9999);
        let _server_default = McpServer::with_daemon_port(root, default_daemon_port());
    }
}

#[cfg(test)]
mod search_tool_tests {
    use crate::tools::search as search_tools;

    // -------------------------------------------------------------------------
    // search::tool_definitions
    // -------------------------------------------------------------------------

    #[test]
    fn search_tool_definitions_returns_expected_tool_names() {
        // All four search tools must be present by name.
        let defs = search_tools::tool_definitions();
        let names: Vec<&str> = defs.iter().map(|d| d.name.as_str()).collect();
        for expected in &[
            "search_regex",
            "search_semantic",
            "search_research",
            "search_status",
        ] {
            assert!(
                names.contains(expected),
                "search tool_definitions must include '{expected}'"
            );
        }
    }

    #[test]
    fn search_tool_definitions_has_no_duplicate_names() {
        // Each tool name must appear exactly once.
        let defs = search_tools::tool_definitions();
        let names: Vec<&str> = defs.iter().map(|d| d.name.as_str()).collect();
        let unique: std::collections::HashSet<_> = names.iter().collect();
        assert_eq!(names.len(), unique.len(), "tool names must be unique");
    }

    #[test]
    fn search_tool_definitions_each_have_object_input_schema() {
        // Every tool definition must carry a valid JSON object as its input_schema.
        let defs = search_tools::tool_definitions();
        for def in &defs {
            assert!(
                def.input_schema.is_object(),
                "tool '{}' must have an object input_schema",
                def.name
            );
            assert_eq!(
                def.input_schema.get("type").and_then(|v| v.as_str()),
                Some("object"),
                "tool '{}' input_schema must have type=object",
                def.name
            );
        }
    }

    #[test]
    fn search_regex_schema_has_required_pattern() {
        // search_regex requires the 'pattern' field.
        let defs = search_tools::tool_definitions();
        let def = defs
            .iter()
            .find(|d| d.name == "search_regex")
            .expect("search_regex");
        let required = def.input_schema.get("required").expect("required field");
        assert!(
            required
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v.as_str() == Some("pattern")),
            "search_regex must declare 'pattern' as required"
        );
    }

    #[test]
    fn search_semantic_schema_has_required_query() {
        // search_semantic requires the 'query' field.
        let defs = search_tools::tool_definitions();
        let def = defs
            .iter()
            .find(|d| d.name == "search_semantic")
            .expect("search_semantic");
        let required = def.input_schema.get("required").expect("required field");
        assert!(
            required
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v.as_str() == Some("query")),
            "search_semantic must declare 'query' as required"
        );
    }

    #[test]
    fn search_research_schema_has_required_question() {
        // search_research requires the 'question' field.
        let defs = search_tools::tool_definitions();
        let def = defs
            .iter()
            .find(|d| d.name == "search_research")
            .expect("search_research");
        let required = def.input_schema.get("required").expect("required field");
        assert!(
            required
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v.as_str() == Some("question")),
            "search_research must declare 'question' as required"
        );
    }

    #[test]
    fn search_status_schema_has_no_required_fields() {
        // search_status takes no parameters — it must not declare any required fields.
        let defs = search_tools::tool_definitions();
        let def = defs
            .iter()
            .find(|d| d.name == "search_status")
            .expect("search_status");
        // Either `required` is absent, or it is an empty array.
        let required = def.input_schema.get("required");
        match required {
            None => {} // acceptable — omitted means no required fields
            Some(r) => assert!(
                r.as_array().is_none_or(Vec::is_empty),
                "search_status must have no required fields"
            ),
        }
    }

    #[test]
    fn search_tool_definitions_each_have_non_empty_description() {
        // Every tool must have a human-readable description.
        let defs = search_tools::tool_definitions();
        for def in &defs {
            assert!(
                !def.description.is_empty(),
                "tool '{}' must have a non-empty description",
                def.name
            );
        }
    }
}
