---
id: "TASK-c79077be"
type: "task"
title: "Audit product docs for vision alignment"
description: "Audit all product documentation pages against the updated vision. Fix references to .claude/ as source of truth, Claude-specific identity language, and missing three-layer architecture concepts."
status: "completed"
created: 2026-03-08T00:00:00.000Z
updated: 2026-03-08T00:00:00.000Z
assignee: "AGENT-bbad3d30"
acceptance:
  - "Every product doc checked against 5 audit criteria from plan"
  - "No .claude/ referenced as source of truth (only as optional symlink layer)"
  - "No Claude-as-product-identity language (provider-agnostic framing)"
  - "Three-layer architecture (canon/project/plugin) referenced where governance discussed"
  - "Pillar names consistent (Clarity Through Structure"
  - "Learning Through Reflection)"
  - "Hardcoded artifact type lists note configurability via project.json"
relationships:
  - target: "EPIC-57dd7d4c"
    type: "delivers"
    rationale: "Task belongs to this epic"
---
## What

Read each product doc listed in scope. For each one, check the 5 audit criteria
defined in the plan. Fix any misalignment in-place.

## Audit Criteria

1. `.claude/` references → `.orqa/` as source of truth
2. Claude-specific language → provider-agnostic
3. Three-layer (canon/project/plugin) distinction present where relevant
4. Pillar names consistent
5. Artifact types noted as configurable

## Skip

- `product/vision.md` — already updated
- `product/governance.md` — already updated

## Deliverable

Updated markdown files with a summary of changes made per file.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.