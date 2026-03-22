---
id: "IMPL-a97eccb6"
type: "lesson"
title: "Plugins from other sidecars should be disabled when not in their sidecar context"
description: "When multiple sidecar-specific plugins are installed, only the plugin matching the active sidecar should be loaded. Plugins from other sidecars should be disabled or scoped to prevent cross-context interference (wrong hooks firing, incompatible tool references)."
status: "completed"
created: "2026-03-13"
updated: "2026-03-13"
maturity: "understanding"
recurrence: 1
relationships: []
---
## Pattern

Consider a project with two sidecar-specific plugins installed:

- `orqastudio-claude-plugin` (Claude Code CLI)
- `orqastudio-cursor-plugin` (Cursor, hypothetical)

When working in Claude Code, the Cursor plugin's hooks should not fire, its agents should not be available, and its skills should not be injected. Vice versa in Cursor. Currently, all plugins in `.orqa/plugins/` are treated as active regardless of sidecar context.

This is a broader instance of the plugin-sidecar pairing ([IMPL-e22b63b4](IMPL-e22b63b4)) — not just declaring the requirement, but actively scoping plugin activation to the correct runtime context.

## Fix

Load-time filtering (Option A from RES-cd3d33bf). When the system detects the active AI provider, it filters `.orqa/plugins/` by their `requires.ai-providers` field. Non-matching plugins are invisible to hooks, skill loader, and agent resolver. The app UI shows all plugins but greys out non-matching ones for management purposes. User-approved decision.

## Triage

Design completed in [TASK-53493d31](TASK-53493d31) as part of sidecar pairing. Implementation deferred to [IDEA-1287dd52](IDEA-1287dd52).
