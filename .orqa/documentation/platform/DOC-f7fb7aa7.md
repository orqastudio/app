---
id: DOC-f7fb7aa7
type: doc
title: "Thinking Mode: Implementation"
description: "The user wants something built, fixed, added, or refactored — hands-on work producing code, artifacts, or configuration changes."
category: platform
status: active
created: 2026-03-21
updated: 2026-03-21
relationships:
  - target: KNOW-f7fb7aa7
    type: synchronised-with
---

## What This Mode Is

Implementation Mode is active when the user wants something built, added, fixed, created, or refactored. The result of this mode is always a concrete deliverable: code, an artifact, a configuration change, or a migration. The agent does real work — no stubs, no placeholders, no deferred deliverables.

This is the most common active-work mode. When a user says "build X" or "fix Y", the thinking framework routes to this mode and injects the appropriate domain knowledge for the task.

---

## When It Activates

The orchestrator routes here when the user's request has an execution target — something that will be different after the conversation ends.

Typical signals:
- "build a new component"
- "add a Tauri command for X"
- "fix the broken store"
- "refactor the scanner module"
- "create a new plugin"
- "implement the design from the spec"
- "wire up the IPC layer"
- "add the missing test coverage"

---

## What the Agent Needs

The orchestrator injects knowledge appropriate to the implementation domain:

| Domain | Injected Knowledge |
|--------|-------------------|
| Svelte/frontend | `svelte5-best-practices`, `orqa-frontend-best-practices` |
| Rust/backend | `rust-async-patterns`, `orqa-backend-best-practices` |
| IPC boundary | `orqa-ipc-patterns`, `orqa-error-composition` |
| Stores | `orqa-store-patterns`, `orqa-store-orchestration` |

The implementer must also verify the **four-layer completeness rule** (RULE-010): every feature requires a Rust command, IPC types, a Svelte component, and a store binding — all committed together.

---

## How It Connects to the Thinking Framework

Implementation Mode is the execution endpoint for several other modes:

- **Planning Mode** scopes work and produces task artifacts → Implementation executes them
- **Debugging Mode** diagnoses a root cause → Implementation fixes it
- **Review Mode** produces a FAIL verdict → Implementation addresses the violations

Implementation Mode never self-certifies. After the implementer finishes, the orchestrator routes to a reviewer to verify quality.

---

## Governance

- RULE-006 (coding standards) applies to all implementation work
- RULE-010 (end-to-end completeness) applies to any feature touching the IPC boundary
- RULE-020 (no stubs) applies universally — real implementations only
