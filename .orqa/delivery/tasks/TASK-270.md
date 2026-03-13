---
id: TASK-270
title: "Resolve AD-032 SQLite scoping violation"
description: "Governance scan results should produce research artifacts (.orqa/ files), not SQLite rows. Migrate governance tables out of SQLite."
status: done
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-057
depends-on: []
assignee: AGENT-003
acceptance:
  - "governance_analyses and governance_recommendations tables removed from SQLite"
  - "Governance scan results produce research artifacts in .orqa/ that can be translated into epics/tasks"
  - "AD-032 updated to reflect the decision"
  - "artifacts table clarified as read-through cache of file-based artifacts"
---

## What

Audit found `governance_analyses`, `governance_recommendations`, and `artifacts` tables in SQLite. [AD-032](AD-032) says governance data should be file-based with the node graph as query layer.

## How

1. Remove `governance_analyses` and `governance_recommendations` tables from SQLite
2. Governance scan results should produce research artifacts in `.orqa/planning/research/` that can be promoted to epics/tasks
3. Update AD-032 to reflect this decision
4. Clarify `artifacts` table as a read-through cache (not source of truth) of file-based artifacts

## Verification

AD-032 accurately describes the actual persistence strategy. No undocumented SQLite tables.
