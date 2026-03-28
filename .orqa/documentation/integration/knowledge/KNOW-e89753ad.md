---
id: KNOW-e89753ad
type: knowledge
title: OrqaStudio CLI Commands
domain: integration/cli
description: "Complete reference for the orqa CLI: every command, its subcommands, and when to use each. Agents must use orqa commands instead of raw cargo/npm/make."
summary: "Complete reference for the orqa CLI: every command, its subcommands, and when to use each. Agents must use orqa commands instead of raw cargo/npm/make."
status: active
created: 2026-03-24
updated: 2026-03-24
category: tooling
version: 1
user-invocable: true
relationships:
  - target: DOC-e89753ad
    type: synchronised-with
    rationale: "User-facing documentation pair for this agent-facing knowledge artifact"
---

# OrqaStudio CLI Commands

The `orqa` CLI is the single developer interface. All commands go through it.

## Core Commands

| Command | Purpose | When to Use |
| --------- | --------- | ------------- |
| `orqa install` | Full setup (prereqs + deps + build + sync) | First-time, after cloning |
| `orqa dev` | Start dev environment (Vite + Tauri) | Daily development |
| `orqa dev restart-tauri` | Restart Tauri only | After Rust changes |
| `orqa check` | All quality checks from plugins | Before every commit |
| `orqa test` | All test suites (rust, app) | After implementation |
| `orqa enforce [path]` | Artifact integrity validation | Schema/relationship checks |
| `orqa enforce --fix` | Auto-fix fixable errors | Quick remediation |
| `orqa audit` | Full governance audit | Session-end health check |

## Graph + Discovery

| Command | Purpose |
| --------- | --------- |
| `orqa graph --type <type>` | Filter artifacts by type |
| `orqa graph --related-to <id>` | Show related artifacts |
| `orqa graph --tree` | Delivery hierarchy view |
| `orqa graph --stats` | Aggregate statistics |

## Plugin Management

| Command | Purpose |
| --------- | --------- |
| `orqa plugin list` | List installed plugins |
| `orqa plugin install <source>` | Install a plugin |
| `orqa plugin refresh [name]` | Re-sync content after edits |
| `orqa plugin diff [name]` | Show content differences |

## Maintenance

| Command | Purpose |
| --------- | --------- |
| `orqa version show/sync/bump` | Version management |
| `orqa id generate/check/migrate` | Artifact ID management |
| `orqa index` | ONNX embedding + search index |
| `orqa repo license/readme` | License and README audits |

## Infrastructure (auto-spawned)

| Command | Purpose |
| --------- | --------- |
| `orqa mcp` | MCP server (stdio, auto-spawned by Claude Code) |
| `orqa lsp` | LSP server (auto-spawned by editor) |
| `orqa daemon start` | Validation daemon (port 10258) |

## FORBIDDEN

- Raw `cargo clippy/test/fmt` or `npm run lint/check` — use `orqa check`/`orqa test`
- `make` targets except `make install` (bootstrap only)
- `--no-verify` on commits
- `orqa dev` when already running — use `orqa dev restart-*`
