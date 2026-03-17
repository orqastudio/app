---
id: TASK-460
title: Delete duplicate and stale documentation
description: "Remove documentation files that duplicate other docs or are entirely outdated. DOC-019 (architecture-overview) duplicates DOC-001, DOC-054 (launch-timeline) is outdated, DOC-032 (process/rules) duplicates RULE-026."
status: completed
priority: P1
scoring:
  impact: 3
  urgency: 3
  complexity: 1
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - DOC-019 (architecture-overview.md) deleted — content already in DOC-001
  - DOC-054 (launch-timeline.md) deleted — entirely outdated
  - DOC-032 (process/rules.md) deleted — duplicates RULE-026
  - Any cross-references to deleted docs updated or removed
  - No broken links remain after deletion
relationships:
  - target: EPIC-064
    type: delivers
    rationale: Phase 1 — clean up documentation before connecting to graph
  - target: RES-062
    type: informed-by
    rationale: Documentation audit identified these duplicates
  - target: TASK-461
    type: informs
    rationale: "Auto-generated inverse of informs relationship from TASK-461"
  - target: SKILL-037
    type: informed-by
  - target: SKILL-011
    type: informed-by
  - target: TASK-461
    type: depended-on-by
  - target: SKILL-011
    type: informed-by
  - target: SKILL-037
    type: informed-by
---
## Scope

Delete 3 documentation files identified as duplicates or stale by the documentation audit ([RES-062](RES-062)):

1. **DOC-019** (`development/architecture-overview.md`) — 118-line stub that duplicates DOC-001 (core-architecture.md, 696 lines). No unique content.
2. **DOC-054** (`product/launch-timeline.md`) — References outdated "Phase" numbering for a past launch. No longer relevant.
3. **DOC-032** (`process/rules.md`) — Describes rule structure already defined authoritatively in RULE-026 (skill-enforcement).

After deletion, search all `.orqa/` files for references to these IDs and update or remove them.
