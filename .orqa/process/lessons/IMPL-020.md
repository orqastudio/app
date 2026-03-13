---
id: IMPL-020
title: "Plugins from other sidecars should be disabled when not in their sidecar context"
description: "When multiple sidecar-specific plugins are installed, only the plugin matching the active sidecar should be loaded. Plugins from other sidecars should be disabled or scoped to prevent cross-context interference (wrong hooks firing, incompatible tool references)."
status: promoted
created: "2026-03-13"
updated: "2026-03-13"
maturity: understanding
recurrence: 1
relationships:
  - target: IMPL-019
    type: informed-by
    rationale: "This is the enforcement side of the plugin-sidecar pairing observation — once pairing is declared, the system must enforce it"
  - target: IDEA-071
    type: enforces
    rationale: "Context filtering designed in TASK-307, implementation tracked by IDEA-071"
  - target: TASK-307
    type: enforces
    rationale: "TASK-307 designed the plugin context filtering mechanism that addresses this lesson"
  - target: IDEA-071
    type: informs
    rationale: "Lesson informs IDEA-071 — implementation of context-scoped plugin activation"
  - target: IDEA-071
    type: grounded-by
    rationale: "Promoted to this idea for implementation of context-scoped plugin activation"
---

## Pattern

Consider a project with two sidecar-specific plugins installed:

- `orqastudio-claude-plugin` (Claude Code CLI)
- `orqastudio-cursor-plugin` (Cursor, hypothetical)

When working in Claude Code, the Cursor plugin's hooks should not fire, its agents should not be available, and its skills should not be injected. Vice versa in Cursor. Currently, all plugins in `.orqa/plugins/` are treated as active regardless of sidecar context.

This is a broader instance of the plugin-sidecar pairing ([IMPL-019](IMPL-019)) — not just declaring the requirement, but actively scoping plugin activation to the correct runtime context.

## Fix

Load-time filtering (Option A from RES-052). When the system detects the active AI provider, it filters `.orqa/plugins/` by their `requires.ai-providers` field. Non-matching plugins are invisible to hooks, skill loader, and agent resolver. The app UI shows all plugins but greys out non-matching ones for management purposes. User-approved decision.

## Triage

Design completed in [TASK-307](TASK-307) as part of sidecar pairing. Implementation deferred to [IDEA-071](IDEA-071).
