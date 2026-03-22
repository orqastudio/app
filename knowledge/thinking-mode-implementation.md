---
id: KNOW-fda0559b
type: knowledge
title: "Thinking Mode: Implementation"
description: "The user wants something built, fixed, added, or refactored — hands-on work producing code, artifacts, or configuration changes."
status: active
created: 2026-03-21
updated: 2026-03-21
relationships:
  - target: DOC-6d410630
    type: synchronised-with
---

# Thinking Mode: Implementation

The user wants something built, added, fixed, created, or refactored. This mode produces code, configuration, or artifact changes. The agent does real work — no stubs, no placeholders.

## Example Signals

"build a new component", "add a Tauri command", "fix the broken store", "refactor the scanner", "create a new plugin", "implement the design", "wire up the IPC layer", "add the missing test"

## What the Agent Needs

- Coding standards (RULE-006) and four-layer completeness rule (RULE-010)
- Relevant domain knowledge: `svelte5-best-practices`, `rust-async-patterns`, `orqa-ipc-patterns`
- Search the codebase for existing implementations before creating new ones
- Verify full request chain: component → store → invoke → Rust command

## Distinguishing from Similar Modes

- Not **Debugging**: root cause is already known here — work starts immediately
- Not **Planning**: no scoping or design phase — execution is the goal
- Not **Review**: agent produces changes, not verdicts
