---
id: KNOW-762facfb
type: knowledge
status: active
title: Proposed Codebase Structure
domain: architecture
description: Target directory layout — where each component lives in the codebase and why, essential when creating or moving files
tier: core
relationships:
  synchronised-with: DOC-762facfb
---

# Proposed Codebase Structure

Top-level directories and their purpose:

| Directory | Contents |
| ----------- | ---------- |
| `engine/` | Rust engine crates: types, artifact, graph, workflow, prompt, search, enforcement, plugin, agent, streaming, lesson, project, validation, core |
| `libs/` | TypeScript libraries: cli, types, sdk, logger, graph-visualiser, svelte-components, mcp-server, lsp-server, brand |
| `daemon/` | Persistent Rust process (file watchers, health endpoint, system tray) |
| `app/` | Desktop app — `src/` (SvelteKit frontend) + `src-tauri/` (Tauri backend) |
| `connectors/` | Connector plugins — `claude-code/` generates Claude Code Plugin to `.claude/` |
| `plugins/` | OrqaStudio plugins by type: methodology, workflow (discovery/planning/documentation/review/kanban/core), domain knowledge (cli/rust/svelte/tauri/typescript/coding-standards/systems-thinking/plugin-dev/githooks) |
| `integrations/` | LLM provider integrations — `claude-agent-sdk/` |
| `models/` | ONNX models for local semantic search |
| `scripts/` | Maintenance scripts |
| `infrastructure/` | Deployment tooling (Forgejo setup) |
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
| `engine/graph` | Artifact relationships, traceability |
| `engine/workflow` | State machine evaluation, guards, actions |
| `engine/prompt` | Prompt generation pipeline |
| `engine/search` | Semantic search, ONNX embeddings |
| `engine/enforcement` | Rule evaluation, artifact validation, config generation |
| `engine/plugin` | Plugin system, composition, installation |
| `engine/agent` | Base roles, task-specific agent generation |
| `engine/core` | Thin facade — re-exports all engine crates |
