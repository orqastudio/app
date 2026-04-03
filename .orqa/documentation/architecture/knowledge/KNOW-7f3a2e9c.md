---
id: KNOW-7f3a2e9c
type: knowledge
title: "Daemon HTTP API Quick Reference"
summary: "Agent-optimized reference for the daemon's RESTful API — endpoint groups, patterns, and key conventions"
status: active
created: 2026-04-03
updated: 2026-04-03
category: architecture
version: 1.0.0
user-invocable: true
relationships:
  - target: DOC-7f3a2e9c
    type: synchronised-with
---

# Daemon HTTP API Quick Reference

## Key Facts

- The daemon is the **sole source of truth** for all engine operations.
- The Tauri app, CLI, MCP server, and LSP server are all thin HTTP clients.
- No engine logic lives outside the daemon — all artifact management, graph computation,
  validation, prompt generation, and plugin operations flow through this API.
- **Base URL:** `http://127.0.0.1:{ORQA_PORT_BASE}` (default port 10100)

## Conventions

- All responses are JSON. Errors use `{ "error": "...", "code": "..." }`.
- GET reads, POST mutates/creates, PUT updates fields, DELETE removes.
- SSE endpoints use `text/event-stream`; all others use `application/json`.
- Project root is resolved at daemon startup (walk up from CWD to find `.orqa/`). Most
  endpoints use this cached root — no per-request resolution.

## Endpoint Groups

| Group | Key Endpoints | Engine Crate |
|---|---|---|
| Lifecycle | `GET /health`, `POST /reload`, `POST /shutdown` | daemon internal |
| Artifacts | `GET /artifacts`, `GET /artifacts/:id`, `POST /artifacts`, `PUT /artifacts/:id`, `DELETE /artifacts/:id` | `artifact`, `graph` |
| Artifact extras | `GET /artifacts/:id/content`, `GET /artifacts/:id/traceability`, `GET /artifacts/:id/impact`, `GET /artifacts/tree` | `artifact`, `validation` |
| Graph | `GET /graph/stats`, `GET /graph/health`, `GET|POST /graph/health/snapshots` | `graph`, `validation` |
| Validation | `POST /validation/scan`, `POST /validation/fix`, `POST /validation/hook` | `validation` |
| Enforcement | `GET /enforcement/rules`, `POST /enforcement/rules/reload`, `GET /enforcement/violations`, `POST /enforcement/scan` | `enforcement` |
| Search | `POST /search/regex`, `POST /search/semantic`, `POST /search/index`, `POST /search/embed`, `GET /search/status` | `search` |
| Workflow | `GET /workflow/transitions`, `POST /workflow/transitions/apply` | `workflow` |
| Prompt | `POST /prompt/generate`, `POST /prompt/knowledge`, `POST /prompt/context`, `POST /prompt/compact-context` | `prompt` |
| Plugins | `GET /plugins`, `GET /plugins/:name`, `POST /plugins/install/local`, `POST /plugins/install/github`, `DELETE /plugins/:name` | `plugin` |
| Agents | `GET /agents/:role`, `GET /agents/behavioral-messages` | `validation` |
| Content | `GET /content/knowledge/:key` | `validation` |
| Lessons | `GET /lessons`, `POST /lessons`, `PUT /lessons/:id/recurrence` | `lesson` |
| Sessions | `GET|POST /sessions`, `GET|PUT|DELETE /sessions/:id`, `POST /sessions/:id/end`, `GET|POST /sessions/:id/messages` | SQLite |
| Projects | `GET /projects`, `GET /projects/active`, `POST /projects/open`, `GET|PUT /projects/settings`, `POST /projects/scan` | `project`, SQLite |
| Session start | `POST /session-start` | daemon internal |
| Settings | `GET /settings`, `PUT /settings/:key` | SQLite |
| Sidecar | `GET /sidecar/status`, `POST /sidecar/restart` | process manager |
| CLI tools | `GET /cli-tools`, `POST /cli-tools/:plugin/:key/run`, `GET /cli-tools/status` | `plugin` |
| Hooks | `GET /hooks`, `POST /hooks/generate` | `plugin` |
| Setup | `GET /setup/status`, `GET /setup/claude-cli`, `GET /setup/claude-auth`, `GET /setup/embedding-model`, `POST /setup/complete` | SQLite, subprocess |
| Events | `GET /events`, `POST /events`, `GET /events/stream` (SSE) | event bus, SQLite |
| DevTools | `POST /devtools/launch`, `GET /devtools/status` | subprocess |
| Git | `GET /git/stashes`, `GET /git/status` | `project` |
| Startup | `GET /startup/status` | startup tracker |

## SSE Endpoints

Three endpoints stream Server-Sent Events:

| Path | Events Emitted | Consumer |
|---|---|---|
| `GET /events/stream` | All `LogEvent` from central bus | DevTools log viewer |
| `GET /artifacts/watch` | `artifact-changed`, `graph-invalidated` | App artifact view refresh |
| `GET /workflow/transitions/stream` | `transition-proposed`, `transition-applied` | App transitions badge |

## Error Handling

All errors return a consistent JSON shape regardless of endpoint:

```json
{ "error": "Human-readable message", "code": "ERROR_CODE" }
```

Status codes: 200 OK, 201 Created, 204 No Content, 400 Bad Request, 404 Not Found,
422 Unprocessable Entity, 500 Internal Server Error, 503 Service Unavailable.

## Common Patterns

- **Query artifacts by type:** `GET /artifacts?type=epic&status=active`
- **Update a single field:** `PUT /artifacts/:id` with `{ "field": "status", "value": "active" }`
- **Trigger a scan:** `POST /validation/scan` with empty body `{}`
- **Reload cached state:** `POST /reload` with empty body `{}`
- **Generate a system prompt:** `POST /prompt/generate` with `{ "message": "...", "role": "implementer" }`
- **Check search index:** `GET /search/status` before running semantic search
