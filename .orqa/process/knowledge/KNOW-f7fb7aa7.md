---
id: KNOW-f7fb7aa7
type: knowledge
title: "Thinking Mode: Implementation"
description: "The user wants something built, fixed, added, or refactored — hands-on work producing code, artifacts, or configuration changes."
status: active
thinking-mode: implementation
created: 2026-03-21
updated: 2026-03-21
relationships:
  - target: DOC-f7fb7aa7
    type: synchronised-with
tier: "always"
roles:
  - "*"
tags:
  - "thinking-mode"
  - "implementation"
  - "coding"
priority: "P0"
summary: |
  Implementation thinking mode: hands-on work producing code, artifacts, or
  configuration. Requires coding standards, testing patterns, and architecture
  awareness. Triggered by build/fix/add/refactor signals.
---

# Thinking Mode: Implementation

The user wants something built, added, fixed, created, or refactored. This mode produces code, configuration, or artifact changes. The agent does real work — no stubs, no placeholders.

## Example Signals

"build a new component", "add a backend command", "fix the broken store", "refactor the scanner", "create a new plugin", "implement the design", "wire up the integration layer", "add the missing test"

## What the Agent Needs

- Coding standards and end-to-end completeness rules
- Relevant domain knowledge for the area being modified
- Search the codebase for existing implementations before creating new ones
- Verify the full request chain across all layers

## Distinguishing from Similar Modes

- Not **Debugging**: root cause is already known here — work starts immediately
- Not **Planning**: no scoping or design phase — execution is the goal
- Not **Review**: agent produces changes, not verdicts
