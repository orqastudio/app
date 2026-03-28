---
id: DOC-4d531f5e
type: doc
status: active
title: Connector Architecture
domain: architecture
description: What a connector is, what the generated plugin should contain, anti-patterns to avoid, and the development strategy
created: 2026-03-28T00:00:00.000Z
---

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

### 8.2 What the Generated Plugin Should Contain

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

### 8.3 What the Connector Source Should NOT Contain

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

### 8.4 Development Strategy

The connector was built using a target-first approach:

1. **Disconnected Claude Code** from the development process to break the circular dependency of building OrqaStudio with OrqaStudio while OrqaStudio was still being defined.
2. **Hand-wrote the target Claude Code Plugin** — the ideal output that the connector should generate. This serves as a test fixture.
3. **Work backwards** — build the connector and engine infrastructure that generates the ideal plugin.
4. **Test for completion:** turn on the generated version, turn off the hand-written one, verify no functionality is lost.

The same target-first approach applies to git hooks, linting configs, and validation rules: define the target output, then build the generation pipeline to produce it.
