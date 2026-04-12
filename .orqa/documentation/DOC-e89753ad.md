---
id: DOC-e89753ad
type: doc
status: active
title: OrqaStudio CLI Commands
description: "Complete reference for the orqa CLI: all commands, subcommands, options, and usage patterns. The single developer interface for setup, quality checks, testing, governance, plugin management, and protocol servers."
category: development
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: KNOW-e89753ad
    type: synchronised-with
    rationale: "Agent-facing knowledge pair for this user-facing documentation page"
  - target: PD-a44384d1
    type: implements
    rationale: "Documents the commands available through the CLI-as-single-interface architecture decision"
---

# OrqaStudio CLI Commands

## Overview

The `orqa` CLI is the single developer interface for OrqaStudio projects. All development operations — code quality, testing, governance, plugin management, search indexing, and protocol servers — are accessed through this one binary. There are no separate servers to manage, no Makefiles to maintain (except `make install` for bootstrap), and no raw `cargo` or `npm` commands to remember.

This command structure was established by [PD-a44384d1](PD-a44384d1).

## Command Categories

### Setup & Dev Environment

| Command | Purpose |
| ------- | ------- |
| `orqa install` | Full dev environment setup (prereqs, deps, build, plugin sync, verify) |
| `orqa install prereqs` | Check prerequisites (node 22+, rust, git) |
| `orqa install deps` | Install npm workspace + cargo dependencies |
| `orqa install build` | Build all libs in dependency order |
| `orqa install publish` | Publish libs to package registry |
| `orqa dev` | Start full dev environment (Vite + Tauri) |
| `orqa dev stop` | Stop gracefully |
| `orqa dev kill` | Force-kill all processes |
| `orqa dev restart` | Restart Vite + Tauri (not the controller) |
| `orqa dev restart-tauri` | Restart Tauri only (after Rust changes) |
| `orqa dev restart-vite` | Restart Vite only |
| `orqa dev status` | Show process status |
| `orqa dev icons` | Generate brand icons from SVG sources |

### Code Quality & Testing

| Command | Purpose |
| ------- | ------- |
| `orqa check` | Run all quality checks from installed plugins |
| `orqa check <tool>` | Run a specific tool (eslint, clippy, svelte-check) |
| `orqa check configure` | Generate linter config files from coding standards |
| `orqa test` | Run all test suites |
| `orqa test rust` | Rust backend tests (cargo test) |
| `orqa test app` | Frontend tests (vitest) |

### Governance & Enforcement

| Command | Purpose |
| ------- | ------- |
| `orqa enforce [path]` | Run all enforcement checks on artifacts |
| `orqa enforce --mechanism <key>` | Run specific mechanism (e.g. json-schema) |
| `orqa enforce --rule <id>` | Run all mechanisms for a specific rule |
| `orqa enforce --file <path>` | Run all mechanisms for a specific file |
| `orqa enforce --report` | Enforcement coverage report |
| `orqa enforce --fix` | Auto-fix fixable errors |
| `orqa enforce schema` | Validate project.json and plugin manifests |
| `orqa audit` | Full governance audit (integrity, version, license, readme) |
| `orqa audit --fix` | Full audit with auto-fix |
| `orqa audit escalation` | Scan lessons for escalation candidates |
| `orqa audit escalation --create-tasks` | Create CRITICAL tasks for escalation findings |

### Artifact Graph

| Command | Purpose |
| ------- | ------- |
| `orqa graph` | List all artifacts |
| `orqa graph --type <type>` | Filter by artifact type |
| `orqa graph --status <status>` | Filter by status |
| `orqa graph --related-to <id>` | Show related artifacts |
| `orqa graph --id <id>` | Show details for one artifact |
| `orqa graph --stats` | Aggregate statistics |
| `orqa graph --tree` | Delivery tree hierarchy |
| `orqa graph --json` | JSON output |

### Plugin Management

| Command | Purpose |
| ------- | ------- |
| `orqa plugin list` | List installed plugins |
| `orqa plugin install <source>` | Install a plugin (owner/repo or local path) |
| `orqa plugin uninstall <name>` | Remove a plugin |
| `orqa plugin update [name]` | Update one or all plugins |
| `orqa plugin enable <name>` | Enable a plugin (copy content to .orqa/) |
| `orqa plugin disable <name>` | Disable a plugin (remove content) |
| `orqa plugin refresh [name]` | Re-sync content for enabled plugins |
| `orqa plugin diff [name]` | Show content differences |
| `orqa plugin registry` | Browse the plugin registry |
| `orqa plugin create` | Scaffold a new plugin |

### Version & ID Management

| Command | Purpose |
| ------- | ------- |
| `orqa version show` | Show current canonical version |
| `orqa version sync` | Sync VERSION file to all manifests |
| `orqa version bump <ver>` | Set new version and sync |
| `orqa version check` | Check for version drift |
| `orqa id generate <TYPE>` | Generate a new hex ID |
| `orqa id check` | Scan for duplicate IDs |
| `orqa id check --fix` | Auto-regenerate duplicates |
| `orqa id migrate <old> <new>` | Rename an ID across the graph |

### Git Operations

| Command | Purpose |
| ------- | ------- |
| `orqa git status` | Component-aware change status |
| `orqa git pr` | Create a pull request on local git server |
| `orqa git sync` | Push to all remotes |
| `orqa git audit` | Check git infrastructure health |

### Protocol Servers

| Command | Purpose |
| ------- | ------- |
| `orqa mcp [project-path]` | Start MCP server (stdio) for Claude Code |
| `orqa lsp [project-path]` | Start LSP server (stdio) for editor diagnostics |
| `orqa daemon start` | Start validation daemon (port 10258) |
| `orqa daemon stop` | Stop the daemon |
| `orqa daemon status` | Show daemon health |

### Search & Indexing

| Command | Purpose |
| ------- | ------- |
| `orqa index [project-path]` | Download model, index codebase, generate embeddings |
| `orqa index --download-only` | Download ONNX model only |
| `orqa index --skip-download` | Index using existing model |

### Maintenance

| Command | Purpose |
| ------- | ------- |
| `orqa repo license` | License audit across all packages |
| `orqa repo readme` | README completeness audit |
| `orqa debug [command]` | Run the debug tool |
| `orqa hosting up/down/setup/status/push` | Local git server management |

## Configuration

### Claude Code (MCP)

In `.mcp.json`:

```json
{
  "mcpServers": {
    "orqastudio": {
      "command": "orqa",
      "args": ["mcp"]
    }
  }
}
```text

### VS Code (LSP)

Configure the LSP client to spawn `orqa lsp` as the server command.

### Pre-Commit Hook

The pre-commit hook (`.githooks/pre-commit`) automatically calls `orqa check` and `orqa enforce` on staged files. It cannot be bypassed with `--no-verify`.

## What Replaces Make

The `orqa` CLI replaces all `make` targets except `make install` (bootstrap only):

| Old (make) | New (orqa) |
| ---------- | ---------- |
| `make dev` | `orqa dev` |
| `make check` | `orqa check` |
| `make test` | `orqa test` |
| `make lint` | `orqa check` |
| `make format` | `orqa check configure` + tool-specific format |
| `make build` | `orqa install build` |
| `make restart-tauri` | `orqa dev restart-tauri` |

## Related Documents

- [KNOW-e89753ad](KNOW-e89753ad) — Agent-facing knowledge pair for this documentation page
- [PD-a44384d1](PD-a44384d1) — Architecture decision: CLI as single interface
- [DOC-22783288](DOC-22783288) — CLI Architecture (protocol modes and daemon)
