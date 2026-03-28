---
id: "PILLAR-c9e0a695"
type: "pillar"
title: "Clarity Through Structure"
description: "Making thinking, standards, and decisions visible and structured."
status: active
created: 2026-03-09T00:00:00.000Z
updated: 2026-03-09T00:00:00.000Z
gate:
  - "Does this make governance artifacts visible and manageable?"
  - "Does it produce structured knowledge (plans, decisions, rules)?"
  - "Does it enforce a workflow that ensures understanding precedes action?"
  - "Does it surface what would otherwise be hidden in files, terminal output, or people's heads?"
  - "Does the system mechanically enforce its own structural rules?"
relationships:
  - target: "VISION-4893db55"
    type: "upholds"

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

Pillars are equal in importance. When this pillar appears to conflict with Pillar 2 (Learning Through Reflection), the conflict should be flagged to the user for resolution rather than one pillar automatically winning. Agents do not prioritise one pillar over another unilaterally.
