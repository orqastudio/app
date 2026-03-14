---
id: TASK-409
title: "Research document relationship schema update and data backfill"
description: "Add relationships array to research schema. Backfill existing research documents with relationship data. Backfill epic horizon fields."
status: done
priority: P2
created: "2026-03-13"
updated: "2026-03-14"
epic: EPIC-063
depends-on: []
assignee: null
skills:
  - orqa-governance
  - orqa-schema-compliance
  - migration-tooling
acceptance:
  - "Research schema includes relationships array matching the standard relationship pattern"
  - "All research documents have relationship data backfilled (at minimum, informs edges to epics that reference them)"
  - "Bidirectional consistency verified — epic research-refs have matching research→epic relationships"
  - "All epics have horizon field populated (active/next/later/someday/null)"
  - "verify-links passes clean after backfill"
relationships:
  - target: EPIC-063
    type: delivers
    rationale: "Theme G — data integrity backfill from UAT"
---

## Scope

### Research Schema Update (Finding #21)
- **Current**: `.orqa/delivery/research/schema.json` has NO `relationships` field
- **Fix**: Add `relationships` array matching the pattern used in rules, skills, ideas, etc.
- **Backfill**: For each RES-NNN, add at minimum the `informs` relationship to any epic that lists it in `research-refs`

### Epic Horizon Backfill (Finding #25)
- **Current**: `horizon` field exists in epic schema but zero epics have it populated
- **Fix**: Populate all epics with appropriate horizon value based on milestone status
