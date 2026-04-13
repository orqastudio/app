---
id: DOC-4d531f5e
type: doc
status: active
title: Connector Architecture
domain: architecture
description: What a connector is, what the generated plugin should contain, anti-patterns to avoid, and the development strategy
created: 2026-03-28T00:00:00.000Z
updated: 2026-03-29
relationships:
  - type: references
    target: DOC-41ccf7c4
  - type: references
    target: DOC-62969bc3
---

> **Post-MVP scope:** Connector generation is post-MVP per PLAN-mvp.md. The connector framework architecture is built in the MVP; generating output for third-party tools (Claude Code plugin, Cursor rules, Copilot instructions) comes after the first public beta.

# Connector Architecture

> This is part of the OrqaStudio Architecture Reference.

---

## 8. Connector Architecture

### 8.1 What a Connector Is

A connector is a special OrqaStudio plugin with two responsibilities:

1. **Generate** a tool-native plugin from the composed methodology, workflows, rules, and coding standards
2. **Watch** for changes to plugins, rules, and composition and **regenerate** in real time

The generated output goes directly where the third-party tool expects it (e.g., `.claude/` at the project root for Claude Code). Once generated, the third-party tool interacts with the engine directly (via CLI/MCP). The connector is not in the runtime path — it is a live generation pipeline.

The connector source lives in its own top-level directory alongside app, daemon, plugins, etc. It does NOT live inside `.orqa/`.

### 8.2 Connector Language Boundary

The connector source is **Rust** — it calls engine crates directly (not via daemon HTTP). The connector generates a plugin in the **target tool's native language** — for Claude Code, this is TypeScript/JS (`.claude/` directory contents). The language of the output is determined by what the target tool requires, not by OrqaStudio internals.

**Connector source calls engine crates directly.** It does not make HTTP calls to the daemon. The daemon runs the connector's file watcher registration, but code generation itself happens via direct crate linkage.

### 8.3 What the Generated Plugin Should Contain

| Component | Purpose |
| ----------- | --------- |
| Permission configuration | Role-scoped file access — works WITHOUT bypass permissions |
| Agent definitions | Generated from base roles + workflow context, in the tool's native format |
| Slash commands | Thin wrappers exposing OrqaStudio actions |
| Hook scripts | Marshal events to the engine (via CLI/MCP), apply responses — THIN |
| hooks.json | Generated from plugin hook declarations, not static |
| Validation rules | Generated from engine's artifact validation |

Git hooks and linting configs are NOT part of the generated tool-native plugin. Those come from their respective OrqaStudio plugins (core, coding-standards, typescript, rust) which install enforcement infrastructure directly.

The generated plugin enforces workflow constraints and agent permissions. Agents get scoped permissions matching their role — preventing them from modifying files outside their artifact scope.

### 8.4 What the Connector Source Should NOT Contain

| Anti-Pattern | Why It's Wrong | Where It Belongs |
| ------------- | --------------- | ----------------- |
| Rule evaluation logic | Business logic | Engine enforcement crate |
| Knowledge injection algorithms | Business logic | Engine prompt crate |
| Artifact validation beyond format | Business logic | Engine enforcement crate |
| Prompt generation/assembly | Business logic | Engine prompt crate |
| Impact analysis logic | Business logic | Engine graph crate |
| Departure detection heuristics | Business logic | Engine enforcement crate |
| Knowledge artifacts | Workflow knowledge | Methodology plugin |
| Custom telemetry endpoints | Should use unified logging | Engine logging library |

The connector's code should be generation, translation, and file-watching logic only. If it contains `if/else` trees, scoring algorithms, or domain-specific heuristics, it has exceeded its role. The generated hooks should be thin: receive event -> call engine (via CLI/MCP) -> apply response.

### 8.5 Daemon File Watcher Registry

The daemon manages file watchers on behalf of all installed plugins. The watcher system is **manifest-driven** — the daemon does NOT hardcode which paths to watch or what to do when they change. This is a P1 constraint: plugins provide definitions, the engine provides capabilities.

**How the registry works:**

1. At daemon startup, read watcher declarations from all installed **generator** plugin manifests
2. Set up OS file watches for each declared path pattern (these paths cover rules installed by all contributors)
3. When a watched file changes, invoke the generator plugin's tool — it reads rules from ALL contributors and recomposes the output
4. On plugin install/uninstall, update watch registrations without restarting the daemon

The key point: the **generator** plugin owns the watcher, not the contributor plugins. A contributor plugin installing new rules does not need to register its own watcher — the generator already watches the rule path tree. When a contributor is installed or uninstalled, the generator re-runs immediately to recompose from current contributors.

**Generator manifest declaration example:**

```json
{
  "enforcement": { "role": "generator", "tool": "eslint", "output": ".orqa/configs/eslint.config.js" },
  "watchers": [
    {
      "paths": [".orqa/learning/rules/**/*.md"],
      "action": "regenerate",
      "output": ".orqa/configs/eslint.config.js"
    }
  ]
}
```

**What is NOT allowed:**

- Hardcoded `WATCH_DIRS` constants in daemon source — all watch paths come from manifests
- Hardcoded `RULES_DIR` constants — rule paths come from plugin manifests
- Hardcoded handler dispatch (e.g., `if changed_path.starts_with(".orqa/") { rebuild_graph() }`) — handlers come from manifest declarations

The daemon watcher exists to keep generated outputs in sync with their source data. It does not contain business logic — it reads declarations and invokes the right generator when inputs change.

### 8.6 Development Strategy

The connector was built using a target-first approach:

1. **Disconnected Claude Code** from the development process to break the circular dependency of building OrqaStudio with OrqaStudio while OrqaStudio was still being defined.
2. **Hand-wrote the target Claude Code Plugin** — the ideal output that the connector should generate. This serves as a test fixture.
3. **Work backwards** — build the connector and engine infrastructure that generates the ideal plugin.
4. **Test for completion:** turn on the generated version, turn off the hand-written one, verify no functionality is lost.

The same target-first approach applies to git hooks, linting configs, and validation rules: define the target output, then build the generation pipeline to produce it.
