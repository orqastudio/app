---
title: "MCP Host Interface"
category: architecture
tags: []
created: 2026-03-02
updated: 2026-03-04
---

# MCP Host Interface

**Date:** 2026-03-02 | **Status:** Phase 0e specification

How Orqa Studio discovers, connects to, and manages MCP servers. Orqa Studio plays two MCP roles simultaneously: it is an **MCP server** (exposing its native tools to the Agent SDK sidecar) and an **MCP host** (connecting to external user-installed MCP servers and aggregating their tools for the sidecar).

**Architecture References:** AD-007 (sidecar integration), AD-010 (tool implementation as MCP), AD-011 (security model), AD-017 (composability principle)

---

## 1. Architecture Overview

```
                           +---------------------+
                           |    Claude API        |
                           +----------+----------+
                                      |
                           +----------v----------+
                           |  Agent SDK Sidecar   |
                           |  (Bun-compiled TS)   |
                           +----+----------+-----+
                                |          |
                   stdin/stdout |          | MCP (stdio)
                      NDJSON    |          |
                           +----v----------v-----+
                           |   Orqa Studio Rust Core    |
                           |                      |
                           |  +----------------+  |
                           |  | Built-in MCP   |  |
                           |  | Server         |  |
                           |  | (Read, Write,  |  |
                           |  |  Edit, Bash,   |  |
                           |  |  Glob, Grep)   |  |
                           |  +----------------+  |
                           |                      |
                           |  +----------------+  |
                           |  | MCP Host       |  |
                           |  | (connects to   |  |
                           |  |  external MCP  |  |
                           |  |  servers)      |  |
                           |  +----------------+  |
                           +-----------+----------+
                                       |
                          +------------+------------+
                          |            |            |
                     +----v---+  +----v---+  +----v---+
                     | MCP    |  | MCP    |  | MCP    |
                     | Server |  | Server |  | Server |
                     | (stdio)|  | (SSE)  |  | (stdio)|
                     +--------+  +--------+  +--------+
                      e.g.        e.g.        e.g.
                      filesystem  web-search  database
```

**Two distinct roles:**

| Role | Description |
|------|-------------|
| **Built-in MCP Server** | Orqa Studio exposes its native Rust tools (Read, Write, Edit, Bash, Glob, Grep) as an MCP server that the Agent SDK sidecar connects to via `mcpServers` configuration. The sidecar calls these tools on Claude's behalf. |
| **MCP Host** | Orqa Studio connects to external MCP servers (user-installed) and proxies their tool definitions to the sidecar. Tool calls from Claude are routed through Orqa Studio to the appropriate external MCP server. |

---

## 2. Built-in Orqa Studio MCP Server

### Purpose

The built-in MCP server replaces the Agent SDK's default tool implementations. By disabling built-in tools (`tools: []`) and registering a custom MCP server, Orqa Studio gains full control over every file read, write, edit, and shell command. This is essential for the permission model, UI rendering, and audit trail.

### Exposed Tools

| Tool | MCP Tool Name | Description |
|------|---------------|-------------|
| Read | `orqa_read` | Read file contents. Returns content as text with line numbers. |
| Write | `orqa_write` | Write or overwrite a file. |
| Edit | `orqa_edit` | Perform exact string replacement in a file. |
| Bash | `orqa_bash` | Execute a shell command within the project scope. |
| Glob | `orqa_glob` | Find files matching a glob pattern. |
| Grep | `orqa_grep` | Search file contents with regex patterns. |

### Implementation

The built-in MCP server is created in the sidecar using the Agent SDK's `createSdkMcpServer()` function. Each tool definition includes a JSON Schema for its input parameters and maps to a handler that:

1. Serializes the tool call as an NDJSON message on stdout (sidecar to Rust).
2. Waits for the Rust backend to execute the tool natively.
3. Receives the result as an NDJSON message on stdin (Rust to sidecar).
4. Returns the result to the Agent SDK, which sends it back to Claude.

```typescript
// Sidecar: MCP server registration (simplified)
const orqaMcpServer = createSdkMcpServer({
  name: "orqa-studio",
  tools: {
    orqa_read: {
      description: "Read file contents from the project",
      inputSchema: { /* JSON Schema */ },
      handler: async (input) => {
        // Send tool call to Rust backend via stdout NDJSON
        const result = await callRustTool("orqa_read", input);
        return result;
      }
    },
    // ... orqa_write, orqa_edit, orqa_bash, orqa_glob, orqa_grep
  }
});

const agent = new Agent({
  tools: [],  // Disable all built-in tools
  mcpServers: { orqa: orqaMcpServer },
  // ...
});
```

### Tool Execution in Rust

Each tool is implemented natively in Rust within `src-tauri/src/domain/tools/`. No shelling out to external programs (except for `orqa_bash`, which executes user-specified commands under controlled scope).

```
src-tauri/src/domain/tools/
  mod.rs          // Tool dispatcher, MCP protocol handling
  read.rs         // File reading with line numbers
  write.rs        // File writing with backup
  edit.rs         // Exact string replacement
  bash.rs         // Shell command execution (scoped)
  glob.rs         // File pattern matching
  grep.rs         // Regex content search
```

All tool functions return `Result<ToolOutput, ToolError>`. See the [Error Taxonomy](./error-taxonomy.md) for the complete `ToolError` definition.

---

## 3. Tool Call Routing

The full lifecycle of a tool call:

```
1. Claude decides to use a tool (e.g., orqa_read)
     |
2. Agent SDK receives the tool_use content block
     |
3. Agent SDK calls the MCP server handler for "orqa_read"
     |
4. Sidecar handler serializes the call as NDJSON on stdout:
   {"type":"tool_call","id":"tc_abc","tool":"orqa_read","input":{"file_path":"src/main.rs"}}
     |
5. Rust backend reads the NDJSON line from sidecar stdout
     |
6. Rust dispatches to the native tool implementation (tools::read::execute)
     |
7. Tool executes: reads the file, respects security scopes, returns content
     |
8. Rust serializes the result as NDJSON on sidecar stdin:
   {"type":"tool_result","id":"tc_abc","output":{"content":"..."}}
     |
9. Sidecar MCP handler receives the result, returns it to Agent SDK
     |
10. Agent SDK sends the tool_result back to Claude for the next turn
     |
11. Rust simultaneously emits the tool call + result via Channel<T> to the Svelte UI
    for display as a tool call card in the conversation
```

### Latency Budget

| Hop | Expected Latency |
|-----|-----------------|
| Claude API to sidecar | Network-dependent (30-100ms/token) |
| Sidecar to Rust (NDJSON) | ~0.1-0.5ms |
| Rust tool execution | 1-50ms (file ops), 100ms+ (shell commands) |
| Rust to sidecar (NDJSON) | ~0.1-0.5ms |
| Rust to Svelte UI (Channel) | ~0.1ms |

---

## 4. MCP Host: External Server Discovery

### Configuration File

Orqa Studio discovers external MCP servers from a configuration file at the project level or user level. Orqa Studio reads the standard `.claude/mcp-servers.json` format for CLI compatibility, plus its own settings.

**Resolution order (first match wins for duplicate server names):**

1. **Project-level:** `{project_root}/.claude/mcp-servers.json`
2. **User-level:** `~/.config/orqa-studio/mcp-servers.json` (macOS/Linux) or `%APPDATA%/orqa-studio/mcp-servers.json` (Windows)
3. **Settings UI:** Servers configured in Orqa Studio's Settings view are written to the user-level file.

### Configuration Format

```jsonc
// .claude/mcp-servers.json
{
  "mcpServers": {
    "filesystem": {
      "transport": "stdio",
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/path/to/project"],
      "env": {
        "NODE_ENV": "production"
      },
      "trust": "project",
      "enabled": true
    },
    "web-search": {
      "transport": "sse",
      "url": "http://localhost:8080/mcp",
      "headers": {
        "Authorization": "Bearer ${MCP_WEB_SEARCH_KEY}"
      },
      "trust": "user",
      "enabled": true
    },
    "database": {
      "transport": "stdio",
      "command": "uvx",
      "args": ["mcp-server-sqlite", "--db-path", "./data/app.db"],
      "trust": "project",
      "enabled": false
    }
  }
}
```

### Configuration Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `transport` | `"stdio"` or `"sse"` | Yes | Connection transport type. |
| `command` | `string` | stdio only | Executable to spawn. |
| `args` | `string[]` | No | Arguments for the command. |
| `env` | `Record<string, string>` | No | Environment variables. Supports `${VAR}` expansion from system env. |
| `url` | `string` | SSE only | HTTP endpoint for SSE transport. |
| `headers` | `Record<string, string>` | No | HTTP headers for SSE transport. Supports `${VAR}` expansion. |
| `trust` | `"builtin"`, `"user"`, `"project"` | No | Trust level (defaults to `"project"`). See Security section. |
| `enabled` | `boolean` | No | Whether the server is active (defaults to `true`). |

---

## 5. MCP Host: Connection Transports

### 5.1 stdio Transport

For MCP servers distributed as CLI tools (the most common case).

**Lifecycle:**

1. Orqa Studio spawns the MCP server process using `tauri-plugin-shell` with the configured `command` and `args`.
2. Communication uses stdin/stdout with the MCP JSON-RPC protocol (newline-delimited JSON).
3. Orqa Studio sends `initialize` request, waits for `initialized` notification.
4. Orqa Studio calls `tools/list` to discover available tools.
5. Tool calls are sent as `tools/call` requests; results are returned as responses.
6. On shutdown, Orqa Studio sends a `shutdown` request and then terminates the process.

**Process management:**

- Each stdio MCP server runs as a child process of the Orqa Studio Rust backend.
- Processes are spawned lazily on first tool call (not at app startup) unless configured otherwise.
- If a process exits unexpectedly, Orqa Studio marks it as disconnected and shows a status indicator.
- Orqa Studio attempts one automatic restart. If the restart fails, the server is disabled until manual re-enable.
- All child processes are terminated on app close via process group signals.

### 5.2 SSE Transport (Server-Sent Events)

For MCP servers running as HTTP services (remote or local).

**Lifecycle:**

1. Orqa Studio opens an HTTP connection to the configured `url` with optional `headers`.
2. The SSE stream carries JSON-RPC messages from server to client.
3. Client-to-server messages are sent as HTTP POST requests to the same URL.
4. Tool discovery and invocation follow the same MCP protocol as stdio.

**Connection management:**

- SSE connections include automatic reconnection with exponential backoff (1s, 2s, 4s, max 30s).
- If the server is unreachable at startup, Orqa Studio logs a warning and retries periodically.
- A health check ping is sent every 30 seconds to detect stale connections.

---

## 6. External Tool Aggregation

When the sidecar starts a conversation, Orqa Studio provides it with the complete list of available tools from all connected MCP servers (built-in + external).

### Tool Namespacing

External tools are namespaced by their server name to avoid collisions:

| Source | Tool Name in Agent SDK |
|--------|----------------------|
| Built-in Orqa Studio MCP server | `orqa_read`, `orqa_write`, etc. |
| External "filesystem" server | `mcp__filesystem__read_file`, `mcp__filesystem__write_file`, etc. |
| External "web-search" server | `mcp__web-search__search`, etc. |

The naming convention is `mcp__{server_name}__{tool_name}`.

### Tool List Updates

- Tool lists are cached per MCP server connection session.
- If an external MCP server reconnects, its tool list is re-fetched.
- The sidecar is notified of tool list changes between conversation turns (not mid-turn).

### Routing

When Claude calls a namespaced tool:

1. Rust parses the namespace prefix to identify the target MCP server.
2. The `mcp__` prefix and server name are stripped; the original tool name is sent to the external server.
3. The external server executes the tool and returns the result.
4. Orqa Studio wraps the result in the Agent SDK's expected format and returns it to the sidecar.

---

## 7. User-Facing MCP Management

### Settings View (F-009)

The Settings view in the Explorer Panel includes an "MCP Servers" section:

**Server List:**

| Column | Description |
|--------|-------------|
| Name | Server identifier (from config key) |
| Transport | `stdio` or `sse` |
| Status | Connected / Disconnected / Error / Disabled |
| Tools | Count of tools provided by this server |
| Trust | `builtin` / `user` / `project` |
| Actions | Enable/Disable toggle, Remove, View logs |

**Add Server Dialog:**

- Server name (unique identifier)
- Transport type selection (stdio / SSE)
- For stdio: command, arguments, environment variables
- For SSE: URL, headers
- Trust level selection
- "Test Connection" button: spawns the server, calls `initialize` + `tools/list`, shows results

**Server Detail View (click a server row):**

- Full configuration (read-only for project-level, editable for user-level)
- Tool list with names and descriptions
- Connection log (last 50 messages)
- Restart / reconnect button

### Built-in Server

The built-in Orqa Studio MCP server always appears first in the list with trust level `builtin`. It cannot be disabled or removed. Its tools are not editable.

---

## 8. Security Model

### Trust Levels

MCP servers operate at one of three trust levels, which determine what capabilities they are granted:

| Trust Level | Description | Scope | Example |
|-------------|-------------|-------|---------|
| `builtin` | Orqa Studio's own MCP server. Hardcoded, not user-configurable. | Full access to project files within Tauri security scopes. Shell execution within project root. | The built-in Orqa Studio MCP server. |
| `user` | Installed by the user at the user level. Persists across projects. | Access determined by the server's own capabilities. Orqa Studio does not restrict further (the user explicitly trusted it). | A web search server, a database server. |
| `project` | Defined in `.claude/mcp-servers.json` within a project. May be committed by collaborators. | Most restricted. Tool calls require user approval on first use per session. Environment variable expansion is disabled for security. | Project-specific tooling committed to the repo. |

### Capability Scoping

**Built-in tools** operate within Tauri's file system security scopes (AD-011):

- Base scope: `$HOME/**`
- Project scope: dynamically expanded via `app_handle.fs_scope().allow_directory()` when a project is opened
- Deny list: `.ssh`, `.gnupg`, `.aws/credentials`, and other sensitive paths
- Shell commands (`orqa_bash`): executed with `cwd` set to the project root; `PATH` is inherited but the command is run within the Tauri shell plugin's pre-declared command validators

**External stdio servers:**

- Spawned with `cwd` set to the project root
- Inherit a filtered environment (sensitive variables like `AWS_SECRET_ACCESS_KEY` are excluded unless explicitly listed in `env`)
- The spawned process has the same filesystem access as Orqa Studio itself (OS-level); Orqa Studio does not sandbox external processes beyond trust-level approval gating

**External SSE servers:**

- Network access only; no local filesystem access unless the server itself provides it
- Headers may contain secrets via `${VAR}` expansion; these are resolved at connection time and never logged

### First-Use Approval for Project-Level Servers

When a project-level MCP server is encountered for the first time:

1. Orqa Studio shows a dialog: "This project includes an MCP server '{name}' ({transport}). Allow it to run?"
2. Options: "Allow for this session" / "Always allow for this project" / "Deny"
3. "Always allow" is persisted in the user-level config (not in the project file)
4. Denied servers are marked `enabled: false` in the user-level override

### Environment Variable Handling

- `${VAR}` syntax in `env` and `headers` fields is expanded from the system environment at connection time.
- For `project`-level servers, `${VAR}` expansion is disabled by default (security). The user must explicitly approve each variable reference.
- Secrets should use OS keychain storage (via `tauri-plugin-keyring`) and be referenced by variable name.

---

## 9. Server Lifecycle Events

### Startup Sequence

```
App Launch
  |
  v
Load user-level mcp-servers.json
  |
  v
Open project
  |
  v
Load project-level .claude/mcp-servers.json
  |
  v
Merge configurations (project overrides user for same server name)
  |
  v
Check trust/approval status for each server
  |
  v
Start enabled servers (lazy: deferred until first tool call)
  |                       OR
  v                     (eager: for servers marked "always connect")
Sidecar spawned with tool list from all connected servers
```

### Shutdown Sequence

```
App Close / Project Switch
  |
  v
Send shutdown to all active MCP server connections
  |
  v
Wait up to 5 seconds for graceful shutdown per server
  |
  v
Force-kill any remaining processes
  |
  v
Clear connection state
```

### Error Recovery

| Event | Behavior |
|-------|----------|
| Server fails to start | Log error, mark as "Error" in UI, surface toast notification |
| Server crashes during session | Mark as disconnected, attempt one auto-restart, notify user via toast |
| Server returns invalid JSON-RPC | Log the malformed message, skip it, continue processing |
| Tool call times out (30s default) | Return a timeout error to Claude, let it decide how to proceed |
| SSE connection drops | Automatic reconnect with exponential backoff |

---

## 10. Configuration File Examples

### Minimal Project Configuration

```json
{
  "mcpServers": {
    "github": {
      "transport": "stdio",
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"]
    }
  }
}
```

### Full User-Level Configuration

```json
{
  "mcpServers": {
    "web-search": {
      "transport": "sse",
      "url": "http://localhost:3001/mcp",
      "trust": "user",
      "enabled": true
    },
    "postgres": {
      "transport": "stdio",
      "command": "uvx",
      "args": ["mcp-server-postgres", "--connection-string", "${DATABASE_URL}"],
      "env": {
        "DATABASE_URL": "${DATABASE_URL}"
      },
      "trust": "user",
      "enabled": true
    }
  },
  "projectOverrides": {
    "/home/user/projects/myapp": {
      "github": {
        "approved": true,
        "approvedAt": "2026-03-01T10:00:00Z"
      }
    }
  }
}
```

The `projectOverrides` section stores per-project trust decisions for project-level MCP servers, so the user is not re-prompted on every project open.

---

## Related Documents

- [Architecture Decisions](./decisions.md) -- AD-007, AD-010, AD-011, AD-017
- [Error Taxonomy](./error-taxonomy.md) -- Error types for MCP/sidecar/tool failures
- [MVP Specification](/product/mvp-specification) -- F-002 (sidecar lifecycle), F-012 (MCP tool server)
- [Interaction Patterns](/ui/interaction-patterns) -- Tool call display, error states
