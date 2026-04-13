---
id: DOC-80a4cf76
type: doc
status: active
title: Key Decisions
domain: architecture
description: Summary table of all key architectural and design decisions with their resolutions and references
created: 2026-03-28T00:00:00.000Z
updated: 2026-03-29
relationships:
  - type: references
    target: DOC-62969bc3
---

# Key Decisions

> This is part of the OrqaStudio Architecture Reference.

---

## 11. Key Decisions

| Decision | Resolution | Reference |
| ---------- | ----------- | ----------- |
| Engine structure | Rust library crates per functional domain, consumed by all access layers | This document |
| Language boundary | Rust for all libs/CLI/daemon. TypeScript for frontend only. | This document |
| Daemon purpose | Persistent process: file watchers, MCP/LSP, system tray. Consumes engine crates. | This document |
| Access patterns | App+sidecar, connector, CLI, and daemon are peer consumers of engine crates | This document |
| Connector output | Generates to `.claude/` (or equivalent). Watches and regenerates on changes. | This document |
| Methodology vs Workflows | Methodology = overarching approach. Workflows = sub-workflows per stage. | This document |
| Decision levels | Two distinct types: `principle-decision` and `planning-decision` | This document |
| Base roles | 8 fixed roles; task-specific agents generated at runtime, ephemeral not tracked | This document |
| Agent artifacts | Removed. Agents are ephemeral. No agent type, workflow, or AGENT-*.md files. | This document |
| Core plugin | Unified (learning stage + framework schemas + git hooks). Uninstallable. | This document |
| Wireframes | Own artifact type, visible in planning navigation | This document |
| Resolved workflows | One file per stage, named by purpose, not per artifact type | This document |
| Source workflows | Stay in plugin directories. NOT copied to .orqa/. | This document |
| Storage | Data lives with the process that manages it. Storage traits in engine. | This document |
| Tool executor | Engine-level via MCP for sidecars. Connectors map to native tools. | This document |
| Git integration | Engine concern, not external infrastructure. | This document |
| Telemetry | Unified logger library. Future split into metrics + logger. | This document |
| Accuracy over speed | Core product principle for all trade-offs | This document |
| Workflow inheritance | No inheritance. Plugin owns complete state machine. | PD-1ef9f57c |
| Guard language | Declarative only. Code hooks for complex cases. | PD-1ef9f57c |
| Business logic boundary | Engine crates, not MCP/LSP. Protocols are access methods. | PD-1ef9f57c |
| Backwards compatibility | None during pre-release. `orqa migrate` for breaking changes. | PD-1ef9f57c |
| Summary generation | Author writes summaries. `orqa summarize` generates drafts. | PD-1ef9f57c |
| Relationship storage | Forward-only. Task stores `delivers: epic`; graph computes inverses. | CLAUDE.md |
| Session state | `.state/` not `tmp/`. Operational data, not disposable. | PD-8727f99a |
| P7 revised — resolved workflow is a record | SurrealDB replaces file-based workflow resolution. Resolved workflow is a deterministic SurrealDB record, exportable via `orqa export`. "Resolved Workflow Is a File on Disk" is retired. Active. 2026-04-12. | This document |
| SurrealDB as source of truth for artifact storage | Markdown-scan into HashMap replaced by SurrealDB with native graph traversals, typed edges, and real graph queries. Enables artifact browser, ancestry traversal, cycle detection, and health metrics. Active. 2026-04-12. | This document |
| rusqlite → SeaORM migration | SeaORM provides type-safe migrations, entity models, and better ergonomics for the SQLite operational data layer. Migration complete; SeaORM is now the ORM for all SQLite data. Complete. 2026-03-19. | This document |
