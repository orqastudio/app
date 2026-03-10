---
id: PILLAR-001
title: Clarity Through Structure
description: Making thinking, standards, and decisions visible and structured.
status: active
created: "2026-03-09"
updated: "2026-03-09"
gate:
  - Does this make governance artifacts visible and manageable?
  - Does it produce structured knowledge (plans, decisions, rules)?
  - Does it enforce a workflow that ensures understanding precedes action?
  - Does it surface what would otherwise be hidden in files, terminal output, or people's heads?
---
## What This Pillar Means

Clarity Through Structure is the principle that thinking, standards, and decisions must be visible and structured — not hidden in people's heads, buried in terminal output, or scattered across incompatible files.

This pillar governs features that:

- **Make governance tangible** — Rules, agents, skills, and hooks are browsable, editable documents, not invisible config files
- **Produce structured knowledge** — Plans, decisions, and research are first-class artifacts with frontmatter, connections, and lifecycle states
- **Enforce understanding before action** — Documentation-first workflow, plan approval gates, definition of ready
- **Surface hidden information** — AI transparency (system prompts, context injection, thinking), scanner dashboards, compliance indicators

## Examples of Work That Serves This Pillar

- Artifact browser that renders `.orqa/` content as navigable documents
- Rule editor that lets users view and modify enforcement rules in-app
- System prompt transparency showing what context the AI receives
- Scanner dashboard displaying pass/fail trends and violation details
- Architecture decision records that capture why the system is built this way

## Anti-Patterns

- Features that add capability without making governance more visible
- Tools that work silently without surfacing what they do
- Hiding complexity behind automation without providing an inspection layer
- Adding features that don't produce or organize structured knowledge

## Conflict Resolution

When this pillar conflicts with Pillar 2 (Learning Through Reflection), this pillar takes priority. You cannot improve a process that isn't visible and structured. Governance must be solid before the learning loop can meaningfully operate on it.
