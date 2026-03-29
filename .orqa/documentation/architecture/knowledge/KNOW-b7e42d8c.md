---
id: KNOW-b7e42d8c
type: knowledge
status: active
title: "Access Layer Taxonomy: Daemon, App, CLI, Connector"
description: "Four peer access layers that consume engine crates — their purpose, boundaries, and how they differ from each other"
tier: always
created: 2026-03-29
roles: [implementer, reviewer, planner, orchestrator]
paths: [daemon/, app/, cli/, connectors/]
tags: [architecture, access-layers, daemon, app, cli, connector]
relationships:
  - type: synchronised-with
    target: DOC-62969bc3
---

# Access Layer Taxonomy: Daemon, App, CLI, Connector

## The Four Access Layers

All four are peer consumers of the engine crate library. None IS the engine — they consume it.

### Daemon

A persistent Rust process. The only long-running service in the OrqaStudio development environment.

**Responsibilities:**

- Manifest-driven file watchers (paths declared by generator plugins — never hardcoded)
- MCP server (exposes engine to LLM tools like Claude Code)
- LSP server (exposes validation to editors)
- System tray icon and context menu

**Boundaries:**

- Outlives the app
- File watcher registrations come from plugin manifests at startup + plugin install
- Contains no business logic — reads declarations, invokes generators, returns engine results via protocols

### App + Sidecar

The Tauri desktop application (SvelteKit frontend + Rust backend). Empty shell without plugins.

**Responsibilities:**

- Custom UI for interacting with the engine
- Composes governance data from the engine for display
- Routes LLM inference through sidecar plugins

**Key constraint:** The app has no built-in LLM inference. At least one sidecar plugin (LLM provider integration) must be installed.

**Enforcement UI rule (P1):** Enforcement plugin pages dynamically render Run and Fix buttons from the installed plugin registry. No tool is hardcoded in the frontend. The Rust backend dispatches to `orqa enforce --<engine>`. Installing a new enforcement plugin adds its controls automatically.

### CLI (`orqa`)

A thin Rust binary wrapping the engine crates. An access method, not business logic.

**Key commands:**

- `orqa install` — sync plugin content, run composition pipeline
- `orqa enforce` — universal enforcement entry point (dynamic flags from installed plugins)
- `orqa enforce --staged` — called by pre-commit git hook
- `orqa graph` — query artifact relationships
- `orqa search` — semantic search over governance artifacts

**Dynamic flags:** `orqa enforce --eslint`, `--clippy`, etc. are generated at runtime from installed plugin manifests. None are hardcoded in CLI source. Installing a plugin makes its flag valid; uninstalling removes it.

### Connector

A Rust binary that generates a tool-native plugin from the composed methodology. Also watches for changes and regenerates.

**Responsibilities:**

- Generate tool-native plugin output (e.g., `.claude/` for Claude Code) from composed methodology, workflows, rules, and coding standards
- Watch for changes (via daemon file watcher registrations) and regenerate in real time

**Not in the runtime path:** once generated, the third-party tool (e.g., Claude Code) interacts with the engine directly via CLI/MCP. The connector is a generation pipeline, not a relay.

**Language boundary:** connector source is Rust; connector output is in the target tool's native language (e.g., TypeScript/JS for Claude Code).

## Sidecars vs Connectors (Critical Distinction)

| | Sidecar | Connector |
| --- | --------- | ----------- |
| **Direction** | Provides LLM inference TO the app | Generates config FOR a tool that already has inference |
| **Required?** | At least one (app is useless without inference) | Optional (only for third-party tool users) |
| **Runtime role** | Active — routes inference requests | Generation-time — generates then watches |

## App vs Connector (Peer Relationship)

App and connector are peers — both interfaces to the same underlying engine. A project is ready for dogfooding when the same methodology and principles apply identically whether working via the app or via the connector-generated plugin. Neither is "more correct" than the other.
