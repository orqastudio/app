---
id: "TASK-97d5ed5f"
type: "task"
title: "Restructure unfocused documentation and remove stale phase references"
description: "Restructure DOC-9814ec3c (coding-standards) as a principles doc, add purpose to DOC-d9cc1f84 (orchestration), and remove all stale Phase 2a/2b references across 23 documentation files."
status: archived
priority: "P1"
scoring:
  impact: 4
  urgency: 4
  complexity: 2
  dependencies: 3
created: 2026-03-14T00:00:00.000Z
updated: 2026-03-14T00:00:00.000Z
assignee: null
acceptance:
  - "DOC-9814ec3c restructured — leads with principles (why these standards exist), reference material follows"
  - "DOC-d9cc1f84 restructured — leads with purpose and delegation philosophy, procedures follow"
  - "Zero \"Phase 2a\", \"Phase 2b\", or numbered phase references remain in any documentation file"
  - "Phase references replaced with epic names (EPIC-NNN) or removed if context is obsolete"
  - "Clarify or delete DOC-051 (engagement-infrastructure), DOC-9010239f (metrics), DOC-045 (system-artifacts)"
relationships:
  - target: "EPIC-12fba656"
    type: "delivers"
    rationale: "Phase 1 — restructure docs to be fit for graph connection and agent grounding"
  - target: "TASK-0ba4dedd"
    type: "depends-on"
---

## Scope

### Restructure DOC-9814ec3c (coding-standards.md)

Currently: sparse reference material restating rules. Restructure to lead with **why** — what "good code" means in this project, the principles behind the standards, how standards serve the pillars. Reference material (specific rules, lint configs) follows.

### Restructure DOC-d9cc1f84 (orchestration.md)

Currently: 100% procedural. Restructure to lead with **purpose** — why the orchestrator exists, what delegation means, why the orchestrator doing implementation work is a system failure. The delegation reference (TASK-98f928c3) will complement this.

### Remove Phase References

Search all `.orqa/documentation/` files for "Phase 2a", "Phase 2b", "Phase 0", "Phase 1", etc. Replace with epic names or remove if the context is obsolete. Files known to have phase refs: DOC-8cba3805, DOC-bb4d4ae3, DOC-939d8636, DOC-9e1f1ebf, DOC-3d8ed14e (if still exists after merge).

### Clarify or Delete Ambiguous Docs

- DOC-051 (engagement-infrastructure) — unclear scope, read and decide: expand or delete
- DOC-9010239f (metrics) — very short, unclear purpose, read and decide
- DOC-045 (system-artifacts) — scope unclear, read and decide
- DOC-54594c57 (priority-assessment) — vague criteria, read and decide
