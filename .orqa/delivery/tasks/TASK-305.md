---
id: TASK-305
title: "Fix broken forward-references to non-existent artifacts"
description: "verify-links reports 9 broken-link errors for references to artifacts that don't exist: SKILL-045 (4 refs), AGENT-003/004/005, VER-001 (2 refs). Either create the referenced artifacts or remove the forward references."
status: todo
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-059
depends-on: []
acceptance:
  - "All broken-link errors from verify-links are resolved"
  - "Each resolution is either: artifact created, or reference removed with rationale"
  - "node tools/verify-links.mjs --check-paths reports zero errors"
---

## What

Resolve all 9 broken-link errors reported by `verify-links`:

- `[SKILL-045](SKILL-045)` — 4 references in various artifacts
- `[AGENT-003](AGENT-003)` — 1 reference
- `[AGENT-004](AGENT-004)` — 1 reference
- `[AGENT-005](AGENT-005)` — 1 reference
- `[VER-001](VER-001)` — 2 references

## How

1. For each broken reference, determine: does the referenced artifact need to exist (create it) or was the reference premature (remove it)?
2. Create artifacts or remove references accordingly
3. Run `node tools/verify-links.mjs --check-paths` to confirm zero errors

## Verification

- `node tools/verify-links.mjs --check-paths` exits 0 or reports only warnings (no errors)
