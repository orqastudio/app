---
id: "TASK-ae0051a6"
type: "task"
title: "Delete duplicate and stale documentation"
description: "Remove documentation files that duplicate other docs or are entirely outdated. DOC-019 (architecture-overview) duplicates DOC-001, DOC-054 (launch-timeline) is outdated, DOC-032 (process/rules) duplicates RULE-dd5b69e6."
status: archived
priority: "P1"
scoring:
  impact: 3
  urgency: 3
  complexity: 1
  dependencies: 3
created: "2026-03-14"
updated: "2026-03-14"
assignee: null
acceptance:
  - "DOC-019 (architecture-overview.md) deleted — content already in DOC-001"
  - "DOC-054 (launch-timeline.md) deleted — entirely outdated"
  - "DOC-032 (process/rules.md) deleted — duplicates RULE-dd5b69e6"
  - "Any cross-references to deleted docs updated or removed"
  - "No broken links remain after deletion"
relationships:
  - target: "EPIC-12fba656"
    type: "delivers"
    rationale: "Phase 1 — clean up documentation before connecting to graph"
---

## Scope

Delete 3 documentation files identified as duplicates or stale by the documentation audit ([RES-f664f528](RES-f664f528)):

1. **DOC-019** (`development/architecture-overview.md`) — 118-line stub that duplicates DOC-001 (core-architecture.md, 696 lines). No unique content.
2. **DOC-054** (`product/launch-timeline.md`) — References outdated "Phase" numbering for a past launch. No longer relevant.
3. **DOC-032** (`process/rules.md`) — Describes rule structure already defined authoritatively in RULE-dd5b69e6 (skill-enforcement).

After deletion, search all `.orqa/` files for references to these IDs and update or remove them.
