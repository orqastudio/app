---
id: KNOW-762facfb
type: knowledge
status: active
title: Codebase Structure
domain: architecture
description: Actual directory layout — where each component lives in the codebase and why, essential when creating or moving files
tier: always
relationships:
  synchronised-with: DOC-762facfb
---

# Codebase Structure

Top-level directories and their purpose:

| Directory | Contents |
| ----------- | ---------- |
| `engine/` | Rust engine crates: types, artifact, graph, workflow, prompt, search, enforcement, plugin, agent, streaming, lesson, project, validation, mcp-server, lsp-server, core (16 crates total) |
| `libs/` | TypeScript libraries only (no Rust crates): sdk, types, logger, graph-visualiser, svelte-components, brand |
| `cli/` | TypeScript CLI at repo root (@orqastudio/cli) |
| `daemon/` | Persistent Rust process (file watchers, health endpoint, system tray) |
| `app/` | Desktop app — `src/` (SvelteKit frontend) + `src-tauri/` (Tauri backend) |
| `connectors/` | Connector plugins — `claude-code/` generates Claude Code Plugin to `.claude/` |
| `plugins/` | OrqaStudio plugins by type: methodology, workflow (discovery/planning/documentation/review/kanban/core), domain knowledge (cli/rust/svelte/tauri/typescript/coding-standards/systems-thinking/plugin-dev/githooks) |
| `sidecars/` | LLM provider sidecar integrations — `claude-agent-sdk/` |
| `templates/` | Plugin scaffold templates — cli-tool, frontend, sidecar, full |
| `models/` | ONNX models for local semantic search |
| `scripts/` | Maintenance scripts |
| `infrastructure/` | Deployment tooling — `orqastudio-git/` (Forgejo setup) |
| `.orqa/` | Governance artifacts (stage-first structure) |
| `.state/` | Session state, runtime metrics (not committed) |
| `.githooks/` | Git hook scripts (committed) |
| `.claude/` | Architecture docs, task lists, agent definitions |
| `targets/` | Hand-written target states (removed after Phase 10) |
| `tools/` | Dev tools (debug dashboard) |

## Engine Crates (Key Functional Domains)

| Crate | Domain |
| ------- | -------- |
| `engine/types` | Shared types, errors, traits, config, paths — foundation |
| `engine/artifact` | Artifact reading, parsing, frontmatter extraction |
| `engine/graph` | Artifact relationships, traceability |
| `engine/workflow` | State machine evaluation, guards, actions |
| `engine/prompt` | Prompt generation pipeline |
| `engine/search` | Semantic search, ONNX embeddings |
| `engine/enforcement` | Rule evaluation, artifact validation, config generation |
| `engine/plugin` | Plugin system, composition, installation |
| `engine/agent` | Base roles, task-specific agent generation |
| `engine/streaming` | Stream loop, tool execution, LLM interaction |
| `engine/lesson` | Lesson store, promotion pipeline |
| `engine/project` | Project scanning, settings, governance counts |
| `engine/validation` | Integrity checks, graph construction |
| `engine/mcp-server` | MCP protocol server (Rust) |
| `engine/lsp-server` | LSP protocol server (Rust) |
| `engine/core` | Thin facade — re-exports all engine crates |
