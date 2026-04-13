---
id: DOC-7f3a2e9c
type: doc
title: "Daemon HTTP API Specification"
summary: "Complete RESTful HTTP API for the OrqaStudio daemon — the sole source of truth for all engine operations"
status: active
created: 2026-04-03
updated: 2026-04-03
category: architecture
version: 1.0.0
relationships:
  - target: DOC-62969bc3
    type: grounded
---

# Daemon HTTP API Specification

The OrqaStudio daemon exposes a RESTful HTTP API that is the **sole interface** to all engine
operations. The Tauri app, CLI, MCP server, and LSP server are all thin HTTP clients — they hold
no engine logic themselves. All artifact management, graph computation, validation, prompt
generation, and plugin operations flow through this API.

**Base URL:** `http://127.0.0.1:{ORQA_PORT_BASE}` (default port 10100)

---

## 1. Conventions

### Request/Response Envelope

All responses are JSON. Successful responses return the resource directly with no wrapper envelope.
Error responses use a consistent shape:

```json
{
  "error": "Human-readable message",
  "code": "ERROR_CODE"
}
```

### HTTP Status Codes

| Code | Meaning |
|------|---------|
| 200  | OK — resource returned |
| 201  | Created — new resource created |
| 204  | No Content — action succeeded, no body |
| 400  | Bad Request — malformed input |
| 404  | Not Found — resource does not exist |
| 422  | Unprocessable Entity — validation/parse error |
| 500  | Internal Server Error — engine failure |
| 503  | Service Unavailable — engine not ready (startup, reloading) |

### Content Types

- Request body: `application/json` (POST/PUT/DELETE with body)
- Response: `application/json` (all endpoints except SSE)
- SSE streams: `text/event-stream`

### Project Path Resolution

Most endpoints require knowing which project to operate on. Two resolution patterns:

1. **Implicit:** The daemon resolves its project root at startup by walking up from CWD until
   `.orqa/` is found. Most endpoints use this cached root.
2. **Explicit:** Some endpoints accept `project_path` in the request body for multi-project or
   organisation mode.

The daemon holds the project root in shared state. No per-request project resolution for the
common case.

---

## 2. Resource Groups

### 2.1 Lifecycle

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/health` | Daemon liveness, uptime, PID, event bus stats, subprocess snapshots | — (daemon internal) |
| POST | `/reload` | Rebuild all cached state (graph, validation context) from disk | `graph`, `validation` |
| POST | `/shutdown` | Graceful daemon shutdown | — (daemon internal) |

#### GET /health

Response:

```json
{
  "status": "ok",
  "uptime_seconds": 3600,
  "pid": 12345,
  "version": "0.1.0",
  "project_root": "/path/to/project",
  "event_bus": {
    "total_published": 1500,
    "active_subscribers": 3,
    "next_id": 1501
  },
  "processes": [
    {
      "name": "lsp",
      "status": "running",
      "pid": 12346,
      "uptime_seconds": 3599
    },
    {
      "name": "mcp",
      "status": "running",
      "pid": 12347,
      "uptime_seconds": 3598
    }
  ]
}
```

#### POST /reload

Request: `{}` (empty body)

Response:

```json
{
  "status": "reloaded",
  "artifacts": 247,
  "rules": 12
}
```

#### POST /shutdown

Request: `{}` (empty body)

Response: `204 No Content` (connection closes after response)

---

### 2.2 Artifacts

Core artifact CRUD and graph operations.

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/artifacts` | Query artifacts with filters | `graph` (build_artifact_graph) |
| GET | `/artifacts/:id` | Get a single artifact by ID | `graph` |
| GET | `/artifacts/:id/content` | Read raw markdown body from disk | `artifact` (extract_frontmatter) |
| POST | `/artifacts` | Create a new artifact file | `artifact` (generate_artifact_id, fs::write) |
| PUT | `/artifacts/:id` | Update an artifact's frontmatter field(s) | `validation` (update_artifact_field) |
| DELETE | `/artifacts/:id` | Delete an artifact file | `artifact` (fs) |
| GET | `/artifacts/:id/traceability` | Full traceability chain for an artifact | `validation` (compute_traceability) |
| GET | `/artifacts/:id/impact` | Parse impact metadata (downstream count, should_warn) | daemon parse logic |

#### GET /artifacts

Query parameters:

- `type` — filter by artifact type (e.g. `epic`, `task`, `rule`)
- `status` — filter by status (e.g. `active`, `draft`, `in-progress`)
- `search` — full-text search across title, description, body
- `project` — filter by child project name (organisation mode)

Response: `[ArtifactNode, ...]`

```json
[
  {
    "id": "EPIC-048",
    "project": null,
    "path": ".orqa/implementation/epics/EPIC-048.md",
    "artifact_type": "epic",
    "title": "My Epic",
    "description": "Description text",
    "status": "active",
    "priority": "P1",
    "frontmatter": { },
    "body": "# Body markdown...",
    "references_out": [
      { "target_id": "TASK-001", "field": "relationships", "source_id": "EPIC-048", "relationship_type": "delivers" }
    ],
    "references_in": [
      { "target_id": "EPIC-048", "field": "relationships", "source_id": "TASK-001", "relationship_type": "delivered-by" }
    ]
  }
]
```

#### GET /artifacts/:id

Response: single `ArtifactNode` or 404.

#### GET /artifacts/:id/content

Response:

```json
{
  "content": "---\nid: EPIC-048\n...\n---\n\n# Body markdown"
}
```

#### POST /artifacts

Request:

```json
{
  "type": "task",
  "title": "New Task",
  "status": "draft",
  "relationships": [
    { "type": "delivers", "target": "EPIC-048" }
  ],
  "body": "# Task description"
}
```

Response (201):

```json
{
  "id": "TASK-1a2b3c4d",
  "path": ".orqa/implementation/tasks/TASK-1a2b3c4d.md"
}
```

#### PUT /artifacts/:id

Request:

```json
{
  "field": "status",
  "value": "active"
}
```

Response:

```json
{
  "id": "EPIC-048",
  "field": "status",
  "old_value": "draft",
  "new_value": "active"
}
```

#### DELETE /artifacts/:id

Response: `204 No Content`

#### GET /artifacts/:id/traceability

Response:

```json
{
  "artifact_id": "EPIC-048",
  "ancestry_chains": [ ],
  "descendants": [ ],
  "siblings": [ ],
  "impact_radius": 12,
  "disconnected": false
}
```

#### GET /artifacts/:id/impact

Response:

```json
{
  "id": "EPIC-048",
  "artifact_type": "epic",
  "high_influence": true,
  "downstream_count": 15,
  "downstream_summary": "TASK-001, TASK-002, TASK-003, and 12 more",
  "should_warn": true
}
```

---

### 2.3 Navigation Tree

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/artifacts/tree` | Scan the navigation tree from schema.composed.json | `artifact` (artifact_entries_from_schema, reader) |

Response:

```json
{
  "groups": [
    {
      "key": "implementation",
      "label": "Implementation",
      "children": [
        { "key": "epic", "label": "Epics", "icon": "target", "path": ".orqa/implementation/epics", "count": 5 },
        { "key": "task", "label": "Tasks", "icon": "check-square", "path": ".orqa/implementation/tasks", "count": 23 }
      ]
    }
  ]
}
```

---

### 2.4 Graph

Graph-level analytics and statistics.

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/graph/stats` | Summary statistics (nodes, edges, orphans, broken refs) | `graph` (graph_stats) |
| GET | `/graph/health` | Extended health metrics (components, density, pillar traceability) | `validation` (compute_health) |
| GET | `/graph/health/snapshots` | Historical health snapshot list | — (SQLite store) |
| POST | `/graph/health/snapshots` | Store a new health snapshot | — (SQLite store) |

#### GET /graph/stats

Response:

```json
{
  "node_count": 247,
  "edge_count": 412,
  "orphan_count": 3,
  "broken_ref_count": 1
}
```

#### GET /graph/health

Response:

```json
{
  "connected_components": 2,
  "orphan_percentage": 1.2,
  "average_degree": 3.34,
  "graph_density": 0.014,
  "pillar_traceability": 0.87,
  "bidirectionality_ratio": 0.95
}
```

#### GET /graph/health/snapshots

Query parameters: `limit` (default 50)

Response:

```json
[
  {
    "id": 1,
    "error_count": 3,
    "warning_count": 7,
    "node_count": 247,
    "edge_count": 412,
    "orphan_count": 3,
    "health_score": 0.87,
    "created_at": "2026-04-01T10:00:00Z"
  }
]
```

#### POST /graph/health/snapshots

Request:

```json
{
  "error_count": 3,
  "warning_count": 7,
  "node_count": 247,
  "edge_count": 412,
  "orphan_count": 3,
  "health_score": 0.87
}
```

Response (201): the created snapshot object.

---

### 2.5 Validation

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| POST | `/validation/scan` | Run all integrity checks on the artifact graph | `validation` (validate) |
| POST | `/validation/fix` | Apply auto-fixes for fixable integrity issues | `validation` (auto_fix) |
| POST | `/validation/hook` | Evaluate hook lifecycle rules | `validation` (evaluate_hook) |

#### POST /validation/scan

Request: `{}` (empty body, uses cached graph)

Response:

```json
{
  "checks": [
    {
      "category": "structural",
      "severity": "error",
      "artifact_id": "TASK-001",
      "message": "References non-existent artifact EPIC-999",
      "auto_fixable": false
    }
  ],
  "health": { },
  "enforcement_events": [ ]
}
```

#### POST /validation/fix

Request:

```json
{
  "fix": true
}
```

Response:

```json
{
  "checks": [ ],
  "health": { },
  "fixes_applied": [
    {
      "artifact_id": "TASK-001",
      "field": "status",
      "old_value": "unknown",
      "new_value": "draft",
      "description": "Set status to default 'draft'"
    }
  ],
  "enforcement_events": [ ]
}
```

#### POST /validation/hook

Request:

```json
{
  "event": "PreToolUse",
  "tool_name": "Write",
  "tool_input": { "file_path": ".orqa/rules/RULE-001.md" },
  "file_path": ".orqa/rules/RULE-001.md",
  "user_message": null,
  "agent_type": "implementer"
}
```

Response:

```json
{
  "allowed": true,
  "violations": [],
  "messages": []
}
```

---

### 2.6 Enforcement

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/enforcement/rules` | List all parsed enforcement rules | `enforcement` (parser, store) |
| POST | `/enforcement/rules/reload` | Reload rules from disk | `enforcement` (store) |
| GET | `/enforcement/violations` | List current enforcement violations | `enforcement` (engine) |
| POST | `/enforcement/scan` | Full governance scan (rules + hooks + agents across all areas) | `enforcement` (scanner) |

#### GET /enforcement/rules

Response:

```json
[
  {
    "id": "RULE-abc123",
    "title": "No stubs or placeholders",
    "category": "coding",
    "severity": "error",
    "status": "active",
    "file_path": ".orqa/learning/rules/RULE-abc123.md",
    "patterns": [ ]
  }
]
```

#### POST /enforcement/scan

Response:

```json
{
  "rules": [ ],
  "hooks": [ ],
  "agents": [ ],
  "total_artifacts": 42
}
```

---

### 2.7 Search

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| POST | `/search/index` | Index/re-index the codebase | `search` (SearchEngine::index) |
| POST | `/search/embed` | Generate embeddings for unembedded chunks | `search` (SearchEngine::embed_chunks) |
| POST | `/search/regex` | Regex search over indexed codebase | `search` (SearchEngine::search_regex) |
| POST | `/search/semantic` | Semantic search using ONNX embeddings | `search` (SearchEngine::search_semantic) |
| GET | `/search/status` | Get index status (is_indexed, chunk_count, etc.) | `search` (SearchEngine::get_status) |

#### POST /search/regex

Request:

```json
{
  "pattern": "fn main",
  "path_filter": "*.rs",
  "max_results": 20
}
```

Response:

```json
[
  {
    "file_path": "daemon/src/main.rs",
    "line_number": 371,
    "content": "fn main() {",
    "score": 1.0
  }
]
```

#### POST /search/semantic

Request:

```json
{
  "query": "artifact graph construction",
  "max_results": 10
}
```

Response: same shape as regex search, with cosine similarity scores.

#### GET /search/status

Response:

```json
{
  "is_indexed": true,
  "chunk_count": 4521,
  "file_count": 312,
  "embedded_count": 4521
}
```

---

### 2.8 Workflow

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/workflow/transitions` | Evaluate all proposed status transitions | `workflow` (transitions::evaluate_transitions) |
| POST | `/workflow/transitions/apply` | Apply a single status transition | `workflow` + `validation` (update_artifact_field) |

#### GET /workflow/transitions

Response:

```json
[
  {
    "artifact_id": "TASK-001",
    "artifact_path": ".orqa/implementation/tasks/TASK-001.md",
    "current_status": "blocked",
    "proposed_status": "active",
    "reason": "All blocking dependencies resolved",
    "auto_apply": true
  }
]
```

#### POST /workflow/transitions/apply

Request:

```json
{
  "artifact_id": "TASK-001",
  "proposed_status": "active"
}
```

Response:

```json
{
  "artifact_id": "TASK-001",
  "old_status": "blocked",
  "new_status": "active"
}
```

---

### 2.9 Prompt

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| POST | `/prompt/generate` | Classify message and generate system prompt | `prompt` (build_system_prompt) |
| POST | `/prompt/knowledge` | Knowledge injection (declared + semantic) | `prompt` (knowledge), `search` (embedder) |
| POST | `/prompt/context` | Active rules and workflows for connector CLAUDE.md generation | daemon context logic |
| POST | `/prompt/compact-context` | Governance context document for pre-compaction preservation | daemon compact_context logic |

#### POST /prompt/generate

Request:

```json
{
  "message": "implement the login flow",
  "role": "implementer",
  "project_path": "/path/to/project"
}
```

Response:

```json
{
  "prompt": "<system prompt text>",
  "prompt_type": "implementation",
  "method": "keyword",
  "tokens": 2340,
  "budget": 2800,
  "sections": [
    { "name": "system-prompt[role=implementer,stage=implement]", "tokens": 2340 }
  ]
}
```

#### POST /prompt/knowledge

Request:

```json
{
  "agent_prompt": "You are an Implementer. Fix the login bug in...",
  "project_path": "/path/to/project"
}
```

Response:

```json
{
  "entries": [
    {
      "id": "KNOW-abc123",
      "title": "Rust error handling patterns",
      "path": "/path/to/KNOW-abc123.md",
      "source": "declared",
      "score": null
    },
    {
      "id": "KNOW-def456",
      "title": "Authentication module",
      "path": "/path/to/KNOW-def456.md",
      "source": "semantic",
      "score": 0.82
    }
  ]
}
```

#### POST /prompt/context

Request:

```json
{
  "project_path": "/path/to/project"
}
```

Response:

```json
{
  "rule_titles": ["No stubs or placeholders", "Always test before commit"],
  "workflow_names": ["agent", "task", "epic", "review"]
}
```

#### POST /prompt/compact-context

Request:

```json
{
  "project_path": "/path/to/project"
}
```

Response:

```json
{
  "context_document": "# Governance Context (saved before compaction)\n...",
  "summary": "GOVERNANCE CONTEXT PRESERVED: Active epics: EPIC-048..."
}
```

---

### 2.10 Plugins

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/plugins` | List installed plugins | `plugin` (discovery::scan_plugins) |
| GET | `/plugins/:name` | Get a plugin's manifest | `plugin` (manifest::read_manifest) |
| GET | `/plugins/:name/path` | Get the filesystem path to a plugin | `plugin` (discovery) |
| POST | `/plugins/install/local` | Install a plugin from a local path | `plugin` (installer::install_local) |
| POST | `/plugins/install/github` | Install a plugin from GitHub | `plugin` (installer::install_github) |
| DELETE | `/plugins/:name` | Uninstall a plugin | `plugin` (installer::uninstall) |
| GET | `/plugins/registry` | Browse the plugin registry (official/community) | `plugin` (registry) |
| GET | `/plugins/updates` | Check for available plugin updates | `plugin` (lockfile + registry) |

#### GET /plugins

Response:

```json
[
  {
    "name": "software",
    "path": "/path/to/plugins/methodology/software",
    "version": "1.0.0",
    "taxonomy": "methodology",
    "enabled": true
  }
]
```

#### POST /plugins/install/local

Request:

```json
{
  "path": "/path/to/local/plugin"
}
```

Response (201):

```json
{
  "name": "my-plugin",
  "version": "1.0.0",
  "installed_at": "/path/to/plugins/methodology/my-plugin"
}
```

#### POST /plugins/install/github

Request:

```json
{
  "repo": "orqastudio/plugin-software",
  "version": "1.0.0"
}
```

Response (201): same shape as local install.

---

### 2.11 Agents

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/agents/:role` | Load agent preamble/definition by role | `validation` (content::find_agent) |
| GET | `/agents/behavioral-messages` | Extract behavioral messages from all agents | `validation` (content::extract_behavioral_messages) |

#### GET /agents/:role

Response:

```json
{
  "role": "orchestrator",
  "preamble": "You are the orchestrator...",
  "file_path": ".orqa/documentation/architecture/DOC-b951327c.md"
}
```

---

### 2.12 Content

Knowledge artifact loading for the prompt pipeline.

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/content/knowledge/:key` | Load a specific knowledge artifact | `validation` (content::find_knowledge) |

#### GET /content/knowledge/:key

Response:

```json
{
  "key": "domain-services",
  "content": "# Domain Services\n\n...",
  "file_path": ".orqa/documentation/knowledge/KNOW-abc123.md"
}
```

---

### 2.13 Lessons

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/lessons` | List all lessons | `lesson` (store::FileLessonStore) |
| POST | `/lessons` | Create a new lesson | `lesson` (render_lesson) |
| PUT | `/lessons/:id/recurrence` | Increment recurrence count | `lesson` (parse_lesson + render_lesson) |

#### GET /lessons

Response:

```json
[
  {
    "id": "IMPL-001",
    "title": "Agent forgot to load skills",
    "category": "process",
    "recurrence": 2,
    "status": "active",
    "promoted_to": null,
    "created": "2026-03-05",
    "updated": "2026-03-05",
    "body": "## Description\nTest body.",
    "file_path": ".orqa/learning/lessons/IMPL-001.md"
  }
]
```

#### POST /lessons

Request:

```json
{
  "title": "New Lesson",
  "category": "process",
  "body": "## Description\nWhat happened."
}
```

Response (201): the created `Lesson` object.

---

### 2.14 Sessions

App-level session management (chat sessions with sidecars).

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| POST | `/sessions` | Create a new session | — (SQLite) |
| GET | `/sessions` | List sessions for a project | — (SQLite) |
| GET | `/sessions/:id` | Get a single session | — (SQLite) |
| PUT | `/sessions/:id` | Update session (title, status) | — (SQLite) |
| DELETE | `/sessions/:id` | Delete a session | — (SQLite) |
| POST | `/sessions/:id/end` | End an active session | — (SQLite) |
| GET | `/sessions/:id/messages` | List messages in a session | — (SQLite) |
| POST | `/sessions/:id/messages` | Send a message in a session (stream response) | `streaming`, `prompt` |

Query parameters for `GET /sessions`: `project_id`, `status`

---

### 2.15 Projects

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/projects` | List known projects | — (SQLite) |
| GET | `/projects/active` | Get the active project | — (SQLite) |
| POST | `/projects/open` | Open/activate a project by path | — (SQLite) |
| GET | `/projects/settings` | Read project.json settings | `project` (settings, store) |
| PUT | `/projects/settings` | Write project.json settings | `project` (store) |
| POST | `/projects/scan` | Scan project filesystem (tech stack, governance counts) | `project` (scanner) |
| POST | `/projects/icon` | Upload project icon | — (filesystem) |
| GET | `/projects/icon` | Read project icon | — (filesystem) |

#### POST /projects/scan

Response:

```json
{
  "languages": ["rust", "typescript", "svelte"],
  "frameworks": ["tauri", "sveltekit"],
  "governance_counts": {
    "rules": 12,
    "agents": 8,
    "lessons": 5
  }
}
```

---

### 2.16 Session Start

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| POST | `/session-start` | Structured startup checks for connector sessions | daemon session_start logic |

Response:

```json
{
  "checks": [
    { "name": "installation", "passed": true, "message": "connector installation verified" },
    { "name": "daemon", "passed": true, "message": "daemon is running" },
    { "name": "graph_integrity", "passed": true, "message": "graph artifacts are well-formed" },
    { "name": "git_state", "passed": true, "message": "git state clean" }
  ],
  "warnings": [],
  "session_state": "# Previous session...",
  "migration_context": null,
  "governance_context": null,
  "checklist": [
    "Read context above",
    "Set scope: which epic/task is the focus?"
  ]
}
```

---

### 2.17 App Settings

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/settings` | Get all app settings (optionally by scope) | — (SQLite) |
| PUT | `/settings/:key` | Set a single setting | — (SQLite) |

---

### 2.18 Sidecar

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/sidecar/status` | Get sidecar process status | — (process manager) |
| POST | `/sidecar/restart` | Restart the sidecar process | — (process manager) |

---

### 2.19 CLI Tools

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/cli-tools` | List registered CLI tools from plugins | `plugin` (cli_runner) |
| POST | `/cli-tools/:plugin/:key/run` | Execute a plugin CLI tool | `plugin` (cli_runner) |
| GET | `/cli-tools/status` | Get status of registered tools | `plugin` (cli_runner) |

---

### 2.20 Hooks

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/hooks` | List registered hooks from plugins | `plugin` (hooks) |
| POST | `/hooks/generate` | Generate hook dispatcher files | `plugin` (hooks) |

---

### 2.21 Setup

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/setup/status` | Get setup wizard status | — (SQLite) |
| GET | `/setup/claude-cli` | Check Claude CLI availability | — (subprocess) |
| GET | `/setup/claude-auth` | Check Claude authentication | — (subprocess) |
| POST | `/setup/claude-reauth` | Re-authenticate Claude CLI | — (subprocess) |
| GET | `/setup/embedding-model` | Check embedding model availability | `search` (embedder) |
| POST | `/setup/complete` | Mark setup as complete | — (SQLite) |

---

### 2.22 Events

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/events` | Query stored events with filters | — (SQLite event store) |
| POST | `/events` | Ingest events from external sources | — (event bus) |
| GET | `/events/stream` | SSE stream of all live events | — (event bus broadcast) |

#### GET /events

Query parameters:

- `level` — filter by severity (debug, info, warn, error, perf)
- `source` — filter by source (daemon, app, frontend, mcp, lsp, etc.)
- `category` — filter by category
- `since` — Unix timestamp (ms) — events after this time
- `until` — Unix timestamp (ms) — events before this time
- `limit` — max results (default 100)
- `session_id` — filter by agent session

Response:

```json
{
  "events": [
    {
      "id": 42,
      "timestamp": 1711929600000,
      "level": "info",
      "source": "daemon",
      "category": "health",
      "message": "daemon started",
      "metadata": {},
      "session_id": null
    }
  ],
  "count": 1
}
```

#### POST /events

Request (batch):

```json
[
  {
    "level": "info",
    "source": "frontend",
    "category": "navigation",
    "message": "User opened artifacts view",
    "timestamp": 1711929600000,
    "session_id": null
  }
]
```

Response: `204 No Content`

#### GET /events/stream (SSE)

Each event is delivered as an SSE `data` field containing a JSON `LogEvent`:

```text
data: {"id":42,"timestamp":1711929600000,"level":"info","source":"daemon","category":"health","message":"graph reloaded","metadata":{},"session_id":null}

data: {"id":43,...}
```

The stream runs until the client disconnects. Clients reconnect on network failure.

---

### 2.23 DevTools

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| POST | `/devtools/launch` | Launch the OrqaDev devtools window | — (subprocess) |
| GET | `/devtools/status` | Check if devtools is running | — (process check) |

---

### 2.24 Git

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/git/stashes` | List git stashes | `project` (git::stash_list) |
| GET | `/git/status` | Get uncommitted file status | `project` (git::uncommitted_status) |

---

### 2.25 Startup

| Method | Path | Description | Engine Crate |
|--------|------|-------------|--------------|
| GET | `/startup/status` | Get status of all startup tasks | — (startup tracker) |

---

## 3. SSE Streaming Endpoints

Three endpoints emit Server-Sent Events for real-time updates:

| Path | Events | Use Case |
|------|--------|----------|
| `GET /events/stream` | All `LogEvent` from the central bus | DevTools log viewer, real-time monitoring |
| `GET /artifacts/watch` | `artifact-changed`, `graph-invalidated` | App refreshes artifact views on .orqa/ changes |
| `GET /workflow/transitions/stream` | `transition-proposed`, `transition-applied` | App shows pending transitions badge |

### GET /artifacts/watch

Emits events when the file watcher detects changes in `.orqa/`:

```text
event: artifact-changed
data: {"path":".orqa/implementation/tasks/TASK-001.md","event_type":"modified"}

event: graph-invalidated
data: {"reason":"artifact-changed","timestamp":1711929600000}
```

### GET /workflow/transitions/stream

Emits events when auto-transitions are applied or manual transitions are proposed:

```text
event: transition-applied
data: {"artifact_id":"TASK-001","old_status":"blocked","new_status":"active","reason":"dependencies resolved"}

event: transition-proposed
data: {"artifact_id":"EPIC-048","current_status":"active","proposed_status":"completed","reason":"all tasks completed"}
```

---

## 4. Engine Crate Coverage

| Engine Crate | Endpoints |
|---|---|
| `artifact` | `/artifacts` (POST, DELETE), `/artifacts/:id/content`, `/artifacts/tree` |
| `graph` | `GET /artifacts`, `GET /artifacts/:id`, `/graph/stats` |
| `validation` | `/artifacts/:id/traceability`, `PUT /artifacts/:id`, `/graph/health`, `/validation/*`, `/agents/*`, `/content/*` |
| `enforcement` | `/enforcement/*` |
| `search` | `/search/*`, `/setup/embedding-model` |
| `workflow` | `/workflow/*` |
| `prompt` | `/prompt/*` |
| `plugin` | `/plugins/*`, `/cli-tools/*`, `/hooks/*` |
| `lesson` | `/lessons/*` |
| `project` | `/projects/settings`, `/projects/scan`, `/git/*` |
| SQLite only | `/sessions/*`, `/projects` (list/open/active), `/settings`, `/setup/*`, `/startup/*` |
| daemon internal | `GET /health`, `POST /reload`, `POST /shutdown`, `/sidecar/*`, `/devtools/*`, `/events/*` |

---

## 5. Migration Mapping: Tauri IPC Commands

The Tauri app's IPC commands are replaced by direct HTTP calls to the daemon:

| Tauri IPC Command | New Daemon Endpoint |
|-------------------|---------------------|
| `daemon_health` | `GET /health` |
| `sidecar_status` | `GET /sidecar/status` |
| `sidecar_restart` | `POST /sidecar/restart` |
| `stream_send_message` | `POST /sessions/:id/messages` |
| `stream_stop` | `POST /sessions/:id/messages/stop` |
| `stream_tool_approval_respond` | `POST /sessions/:id/messages/tool-approval` |
| `project_open` | `POST /projects/open` |
| `project_get_active` | `GET /projects/active` |
| `project_list` | `GET /projects` |
| `session_create` | `POST /sessions` |
| `session_list` | `GET /sessions?project_id=N` |
| `session_get` | `GET /sessions/:id` |
| `session_update_title` | `PUT /sessions/:id` |
| `session_end` | `POST /sessions/:id/end` |
| `session_delete` | `DELETE /sessions/:id` |
| `message_list` | `GET /sessions/:id/messages` |
| `artifact_scan_tree` | `GET /artifacts/tree` |
| `artifact_watch_start` | `GET /artifacts/watch` (SSE) |
| `project_settings_read` | `GET /projects/settings` |
| `project_settings_write` | `PUT /projects/settings` |
| `project_scan` | `POST /projects/scan` |
| `project_icon_upload` | `POST /projects/icon` |
| `project_icon_read` | `GET /projects/icon` |
| `settings_set` | `PUT /settings/:key` |
| `settings_get_all` | `GET /settings` |
| `get_startup_status` | `GET /startup/status` |
| `get_setup_status` | `GET /setup/status` |
| `check_claude_cli` | `GET /setup/claude-cli` |
| `check_claude_auth` | `GET /setup/claude-auth` |
| `check_embedding_model` | `GET /setup/embedding-model` |
| `complete_setup` | `POST /setup/complete` |
| `reauthenticate_claude` | `POST /setup/claude-reauth` |
| `lessons_list` | `GET /lessons` |
| `lessons_create` | `POST /lessons` |
| `lesson_increment_recurrence` | `PUT /lessons/:id/recurrence` |
| `enforcement_rules_list` | `GET /enforcement/rules` |
| `enforcement_rules_reload` | `POST /enforcement/rules/reload` |
| `enforcement_violations_list` | `GET /enforcement/violations` |
| `governance_scan` | `POST /enforcement/scan` |
| `get_artifacts_by_type` | `GET /artifacts?type=X` |
| `read_artifact_content` | `GET /artifacts/:id/content` |
| `get_graph_stats` | `GET /graph/stats` |
| `get_graph_health` | `GET /graph/health` |
| `get_artifact_traceability` | `GET /artifacts/:id/traceability` |
| `refresh_artifact_graph` | `POST /reload` |
| `run_integrity_scan` | `POST /validation/scan` |
| `apply_auto_fixes` | `POST /validation/fix` |
| `store_health_snapshot` | `POST /graph/health/snapshots` |
| `get_health_snapshots` | `GET /graph/health/snapshots` |
| `update_artifact_field` | `PUT /artifacts/:id` |
| `evaluate_status_transitions` | `GET /workflow/transitions` |
| `apply_status_transition` | `POST /workflow/transitions/apply` |
| `get_registered_cli_tools` | `GET /cli-tools` |
| `run_cli_tool` | `POST /cli-tools/:plugin/:key/run` |
| `cli_tool_status` | `GET /cli-tools/status` |
| `plugin_list_installed` | `GET /plugins` |
| `plugin_registry_list` | `GET /plugins/registry` |
| `plugin_install_local` | `POST /plugins/install/local` |
| `plugin_install_github` | `POST /plugins/install/github` |
| `plugin_uninstall` | `DELETE /plugins/:name` |
| `plugin_check_updates` | `GET /plugins/updates` |
| `plugin_get_path` | `GET /plugins/:name/path` |
| `plugin_get_manifest` | `GET /plugins/:name` |
| `get_registered_hooks` | `GET /hooks` |
| `generate_hook_dispatchers` | `POST /hooks/generate` |
| `launch_devtools` | `POST /devtools/launch` |
| `is_devtools_running` | `GET /devtools/status` |

---

## 6. Migration Mapping: Validation Daemon Endpoints

The validation daemon (`engine/validation/src/daemon.rs`) ran as a separate `tiny_http` server.
Its endpoints are consolidated into the main daemon. **The validation daemon binary is eliminated.**
All functionality moves into the main daemon's shared state.

| Old Validation Daemon | New Daemon Endpoint | Notes |
|-----------------------|---------------------|-------|
| `GET /health` | `GET /health` | Merged — main daemon health includes artifact/rule counts |
| `POST /parse` | `GET /artifacts/:id` or `GET /artifacts/:id/impact` | Replaced by resource-oriented artifact endpoints |
| `POST /query` | `GET /artifacts?type=X&status=Y&search=Q` | Query parameters replace POST body |
| `POST /hook` | `POST /validation/hook` | Path changed for RESTful consistency |
| `POST /content/agent` | `GET /agents/:role` | Resource-oriented path |
| `POST /content/knowledge` | `GET /content/knowledge/:key` | Resource-oriented path |
| `POST /content/behavioral` | `GET /agents/behavioral-messages` | Resource-oriented path |
| `POST /validate` | `POST /validation/scan` or `POST /validation/fix` | Split into separate scan and fix endpoints |
| `POST /traceability` | `GET /artifacts/:id/traceability` | Nested under artifact resource |
| `POST /reload` | `POST /reload` | Same |

---

## 7. Migration Mapping: Current Daemon Endpoints

Endpoints from the current daemon that are kept, changed, or removed:

| Current Daemon Endpoint | Status | New Path |
|-------------------------|--------|----------|
| `GET /health` | Kept (enhanced) | `GET /health` — adds version, project_root fields |
| `POST /parse` | Changed | `GET /artifacts/:id/impact` — RESTful, uses graph cache |
| `POST /context` | Kept | `POST /prompt/context` — moved under prompt group |
| `POST /compact-context` | Kept | `POST /prompt/compact-context` — moved under prompt group |
| `POST /knowledge` | Kept | `POST /prompt/knowledge` — moved under prompt group |
| `POST /prompt` | Kept | `POST /prompt/generate` — more specific path |
| `POST /session-start` | Kept | `POST /session-start` — unchanged |
| `GET /events` | Kept | `GET /events` — unchanged |
| `POST /events` | Kept | `POST /events` — unchanged |
| `GET /events/stream` | Kept | `GET /events/stream` — unchanged |
