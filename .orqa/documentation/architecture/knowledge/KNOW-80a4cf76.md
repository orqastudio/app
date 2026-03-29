---
id: KNOW-80a4cf76
type: knowledge
status: active
title: "Key Architecture Decisions Reference"
domain: architecture
description: "Resolved architectural decisions — reference this before making any design choice to ensure alignment with established resolutions"
tier: always
created: 2026-03-28
roles: [orchestrator, implementer, reviewer, planner, governance-steward]
paths: [engine/, daemon/, app/, plugins/, connectors/]
tags: [architecture, decisions, design, reference]
relationships:
  - type: synchronised-with
    target: DOC-80a4cf76
---

# Key Architecture Decisions Reference

Consult this before any design choice. Each row represents a resolved decision that must not be re-opened without explicit rationale.

## Structural Decisions

| Decision | Resolution |
| ---------- | ----------- |
| Engine structure | Rust library crates per functional domain, consumed by all access layers |
| Language boundary | Rust for all libs/CLI/daemon. TypeScript for frontend only. |
| Daemon purpose | Persistent process: file watchers, MCP/LSP, system tray. Consumes engine crates. |
| Access patterns | App+sidecar, connector, CLI, and daemon are peer consumers of engine crates |
| Connector output | Generates to `.claude/` (or equivalent). Watches and regenerates on changes. |
| Business logic boundary | Engine crates, not MCP/LSP. Protocols are access methods. |
| Storage | Data lives with the process that manages it. Storage traits in engine. |
| Tool executor | Engine-level via MCP for sidecars. Connectors map to native tools. |
| Git integration | Engine concern, not external infrastructure. |

## Methodology and Workflow Decisions

| Decision | Resolution |
| ---------- | ----------- |
| Methodology vs Workflows | Methodology = overarching approach. Workflows = self-contained sub-workflows per stage. |
| Workflow inheritance | No inheritance. Plugin owns complete state machine. |
| Guard language | Declarative only. Code hooks for complex cases. |
| Resolved workflows | One file per stage, named by stage purpose. Runtime reads resolved files only. |
| Source workflows | Stay in plugin directories. NOT copied to `.orqa/`. |

## Agent and Artifact Decisions

| Decision | Resolution |
| ---------- | ----------- |
| Decision levels | Two types: `principle-decision` (architectural/wide-reaching) and `planning-decision` (tactical/evolving) |
| Base roles | 8 fixed roles; task-specific agents generated at runtime, ephemeral — not tracked |
| Agent artifacts | Removed. Agents are ephemeral. No agent type, workflow, or AGENT-*.md files. |
| Core plugin | Unified (learning stage + framework schemas + git hooks). `uninstallable: true`. |
| Wireframes | Own artifact type, visible in planning navigation — NOT DOC artifacts |
| Relationship storage | Forward-only. Task stores `delivers: epic`; graph computes inverses. |
| Session state | `.state/` not `tmp/`. Operational data, not disposable. |

## Product Principle Decisions

| Decision | Resolution |
| ---------- | ----------- |
| Accuracy over speed | Core product principle for all trade-offs — prefer correctness over latency |
| Backwards compatibility | None during pre-release. `orqa migrate` for breaking changes. |
| Summary generation | Author writes summaries. `orqa summarize` generates drafts. |

## Critical Anti-Patterns (What NOT to Do)

- Do NOT hardcode governance patterns in engine or frontend — all definitions come from plugins (P1)
- Do NOT create agent artifacts (AGENT-*.md) — agents are ephemeral, generated per-task
- Do NOT put business logic in MCP/LSP — protocols are access methods only
- Do NOT copy source workflows to `.orqa/` — only resolved workflow output lives there
- Do NOT add backwards-compatibility shims — breaking changes go through `orqa migrate`
- Do NOT use `tmp/` for session data — use `.state/` instead
