---
id: "TASK-c75be77c"
type: task
title: "Resolve AD-2aa4d6db SQLite scoping violation"
description: "Governance scan results should produce research artifacts (.orqa/ files), not SQLite rows. Migrate governance tables out of SQLite."
status: "completed"
created: "2026-03-12"
updated: "2026-03-12"
assignee: "AGENT-1dab5ebe"
acceptance:
  - "governance_analyses and governance_recommendations tables removed from SQLite"
  - "Governance scan results produce research artifacts in .orqa/ that can be translated into epics/tasks"
  - "AD-2aa4d6db updated to reflect the decision"
  - "artifacts table clarified as read-through cache of file-based artifacts"
relationships:
  - target: "EPIC-4726cb3b"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-91bc09f9"
    type: "depended-on-by"
---
## What

Audit found `governance_analyses`, `governance_recommendations`, and `artifacts` tables in SQLite. [AD-2aa4d6db](AD-2aa4d6db) says governance data should be file-based with the node graph as query layer.

## How

1. Remove `governance_analyses` and `governance_recommendations` tables from SQLite
2. Governance scan results should produce research artifacts in `.orqa/delivery/research/` that can be promoted to epics/tasks
3. Update [AD-2aa4d6db](AD-2aa4d6db) to reflect this decision
4. Clarify `artifacts` table as a read-through cache (not source of truth) of file-based artifacts

## Verification

[AD-2aa4d6db](AD-2aa4d6db) accurately describes the actual persistence strategy. No undocumented SQLite tables.