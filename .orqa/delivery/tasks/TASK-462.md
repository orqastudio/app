---
id: TASK-462
title: Restructure unfocused documentation and remove stale phase references
description: "Restructure DOC-021 (coding-standards) as a principles doc, add purpose to DOC-030 (orchestration), and remove all stale Phase 2a/2b references across 23 documentation files."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 4
  complexity: 2
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - "DOC-021 restructured — leads with principles (why these standards exist), reference material follows"
  - "DOC-030 restructured — leads with purpose and delegation philosophy, procedures follow"
  - "Zero \"Phase 2a\", \"Phase 2b\", or numbered phase references remain in any documentation file"
  - Phase references replaced with epic names (EPIC-NNN) or removed if context is obsolete
  - "Clarify or delete DOC-051 (engagement-infrastructure), DOC-029 (metrics), DOC-045 (system-artifacts)"
relationships:
  - target: EPIC-064
    type: delivers
    rationale: Phase 1 — restructure docs to be fit for graph connection and agent grounding
  - target: TASK-461
    type: depends-on
  - target: TASK-463
    type: depended-on-by
---
## Scope

### Restructure DOC-021 (coding-standards.md)

Currently: sparse reference material restating rules. Restructure to lead with **why** — what "good code" means in this project, the principles behind the standards, how standards serve the pillars. Reference material (specific rules, lint configs) follows.

### Restructure DOC-030 (orchestration.md)

Currently: 100% procedural. Restructure to lead with **purpose** — why the orchestrator exists, what delegation means, why the orchestrator doing implementation work is a system failure. The delegation reference (TASK-464) will complement this.

### Remove Phase References

Search all `.orqa/documentation/` files for "Phase 2a", "Phase 2b", "Phase 0", "Phase 1", etc. Replace with epic names or remove if the context is obsolete. Files known to have phase refs: DOC-004, DOC-012, DOC-037, DOC-052, DOC-081 (if still exists after merge).

### Clarify or Delete Ambiguous Docs

- DOC-051 (engagement-infrastructure) — unclear scope, read and decide: expand or delete
- DOC-029 (metrics) — very short, unclear purpose, read and decide
- DOC-045 (system-artifacts) — scope unclear, read and decide
- DOC-062 (priority-assessment) — vague criteria, read and decide
