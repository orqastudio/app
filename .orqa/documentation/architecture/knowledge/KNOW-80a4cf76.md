---
id: KNOW-80a4cf76
type: knowledge
status: active
title: Key Architecture Decisions
domain: architecture
description: Resolved architectural decisions — reference this before making any design choice to ensure alignment with established resolutions
tier: core
relationships:
  synchronised-with: DOC-80a4cf76
---

# Key Architecture Decisions

| Decision | Resolution |
| ---------- | ----------- |
| Engine structure | Rust library crates per functional domain, consumed by all access layers |
| Language boundary | Rust for all libs/CLI/daemon. TypeScript for frontend only. |
| Daemon purpose | Persistent process: file watchers, MCP/LSP, system tray. Consumes engine crates. |
| Access patterns | App+sidecar, connector, CLI, daemon are peer consumers of engine crates |
| Connector output | Generates to `.claude/` (or equivalent). Watches and regenerates on changes. |
| Methodology vs Workflows | Methodology = overarching approach. Workflows = sub-workflows per stage. |
| Decision levels | Two types: `principle-decision` and `planning-decision` |
| Base roles | 8 fixed roles; task-specific agents generated at runtime, ephemeral |
| Agent artifacts | Removed. Agents are ephemeral. No agent type, no AGENT-*.md files. |
| Core plugin | Unified (learning stage + framework schemas + git hooks). Uninstallable. |
| Wireframes | Own artifact type, visible in planning navigation |
| Resolved workflows | One file per stage, named by stage purpose |
| Source workflows | Stay in plugin directories. NOT copied to `.orqa/`. |
| Storage | Data lives with the process that manages it. Storage traits in engine. |
| Tool executor | Engine-level via MCP for sidecars. Connectors map to native tools. |
| Git integration | Engine concern, not external infrastructure. |
| Workflow inheritance | No inheritance. Plugin owns complete state machine. |
| Guard language | Declarative only. Code hooks for complex cases. |
| Business logic boundary | Engine crates, not MCP/LSP. Protocols are access methods. |
| Backwards compatibility | None during pre-release. `orqa migrate` for breaking changes. |
| Relationship storage | Forward-only. Task stores `delivers: epic`; graph computes inverses. |
| Session state | `.state/` not `tmp/`. Operational data, not disposable. |
| Accuracy over speed | Core product principle for all trade-offs. |
