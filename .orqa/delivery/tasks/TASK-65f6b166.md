---
id: TASK-65f6b166
title: Fix broken forward-references to non-existent artifacts
description: "verify-links reports 9 broken-link errors for references to artifacts that don't exist: KNOW-bcfeb64e (4 refs), AGENT-1dab5ebe/004/005, VER-NNN (2 refs). Either create the referenced artifacts or remove the forward references."
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - All broken-link errors from verify-links are resolved
  - "Each resolution is either: artifact created, or reference removed with rationale"
  - node tools/verify-links.mjs --check-paths reports zero errors
relationships:
  - target: EPIC-942c7678
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-cea1bc37
    type: depended-on-by
---

## What

Resolve all 9 broken-link errors reported by `verify-links`:

- KNOW-bcfeb64e — 4 references in various artifacts
- AGENT-1dab5ebe — 1 reference
- AGENT-caff7bc1 — 1 reference
- AGENT-fb0ce261 — 1 reference
- VER-NNN — 2 references

## How

1. For each broken reference, determine: does the referenced artifact need to exist (create it) or was the reference premature (remove it)?
2. Create artifacts or remove references accordingly
3. Run `node tools/verify-links.mjs --check-paths` to confirm zero errors

## Verification

- `node tools/verify-links.mjs --check-paths` exits 0 or reports only warnings (no errors)
