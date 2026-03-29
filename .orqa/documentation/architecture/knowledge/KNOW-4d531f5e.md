---
id: KNOW-4d531f5e
type: knowledge
status: active
title: Connector Architecture
domain: architecture
description: What a connector does, what the generated plugin contains, and what must NOT be in connector source code — boundary enforcement for connector work
tier: always
relationships:
  synchronised-with: DOC-4d531f5e
---

# Connector Architecture

## What a Connector Is

A special OrqaStudio plugin with two jobs:

1. **Generate** a tool-native plugin from composed methodology, workflows, rules, coding standards
2. **Watch** for changes and **regenerate** in real time

The connector is **not in the runtime path** — it is a live generation pipeline. Once generated, the third-party tool (e.g., Claude Code) interacts with the engine directly via CLI/MCP.

Connector source lives in its own top-level directory. Does NOT live inside `.orqa/`.

## Connector Language Boundary

- **Connector source** is Rust — calls engine crates directly (not via daemon HTTP)
- **Connector output** is in the target tool's native language (e.g., TypeScript/JS for Claude Code)
- Daemon manages the connector's file watcher registrations via manifest declarations
- Code generation happens via direct Rust crate linkage — no HTTP round-trips to daemon

## What the Generated Plugin Contains

| Component | Purpose |
| ----------- | --------- |
| Permission configuration | Role-scoped file access — works WITHOUT bypass permissions |
| Agent definitions | Generated from base roles + workflow context |
| Slash commands | Thin wrappers exposing OrqaStudio actions |
| Hook scripts | Marshal events to engine (via CLI/MCP) — THIN only |
| hooks.json | Generated from plugin hook declarations, not static |
| Validation rules | Generated from engine's artifact validation |

Git hooks and linting configs come from their respective plugins (core, coding-standards, typescript, rust) — NOT from the connector. Those plugins generate to `.orqa/configs/`, not into the connector output.

## What the Connector Source Must NOT Contain

| Anti-Pattern | Where It Belongs |
| ------------- | ----------------- |
| Rule evaluation logic | Engine enforcement crate |
| Knowledge injection algorithms | Engine prompt crate |
| Artifact validation beyond format | Engine enforcement crate |
| Prompt generation/assembly | Engine prompt crate |
| Impact analysis logic | Engine graph crate |
| Departure detection heuristics | Engine enforcement crate |
| Knowledge artifacts | Methodology plugin |
| Custom telemetry endpoints | Engine logging library |

**If connector source has `if/else` trees, scoring algorithms, or domain heuristics — it has exceeded its role.**

## Hooks Must Be Thin

```text
receive event → call daemon/CLI → apply response
```text

No business logic in hooks. Timeout values in seconds (not milliseconds).

## Daemon File Watcher Registry

The daemon watcher is **manifest-driven** — NOT hardcoded. This enforces P1.

- Daemon reads watcher declarations from **generator** plugin manifests at startup (contributor plugins do NOT register watchers)
- Generator watches the full rule path tree — covers all contributor rule files
- When watched files change, daemon invokes the generator, which recomposes from all contributors
- On plugin install/uninstall, generator re-runs immediately; watch registrations update without daemon restart

**What is forbidden:**
- `WATCH_DIRS` or `RULES_DIR` hardcoded in daemon source
- Hardcoded handler dispatch based on file path patterns
- Contributor plugins registering their own watchers (generator owns the watcher)

**Generator manifest declaration:**
```json
{
  "enforcement": { "role": "generator", "tool": "eslint", "output": ".orqa/configs/eslint.config.js" },
  "watchers": [
    { "paths": [".orqa/learning/rules/**/*.md"], "action": "regenerate", "output": ".orqa/configs/eslint.config.js" }
  ]
}
```

## Development Strategy (Target-First)

1. Hand-write the target Claude Code Plugin — the ideal output (test fixture)
2. Build the connector and engine infrastructure to generate that output
3. Validate: turn on generated version, turn off hand-written version, verify no functionality lost
