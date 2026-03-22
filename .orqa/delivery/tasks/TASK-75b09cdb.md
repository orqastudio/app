---
id: "TASK-75b09cdb"
title: "Update RULE-303c1cc8: loop-closure + scope verification requirements"
description: "Update RULE-303c1cc8 to require loop-closure phases in enforcement epics and explicit user approval for out-of-scope sections"
status: "completed"
created: "2026-03-13"
updated: "2026-03-13"
acceptance:
  - "RULE-303c1cc8 requires epics producing enforcement tooling to include a loop-closure phase and out-of-scope sections to have user approval"
relationships:
  - target: "EPIC-4e6e9eae"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-247b6ce4"
    type: "depends-on"
  - target: "TASK-7cf80542"
    type: "depended-on-by"
---
## What

Update [RULE-303c1cc8](RULE-303c1cc8) plan-mode-compliance with loop-closure and scope verification requirements.

## How

Add requirements to [RULE-303c1cc8](RULE-303c1cc8) that any epic producing enforcement or audit tooling includes a loop-closure phase, and that out-of-scope sections are presented to the user for explicit approval before being committed.

## Verification

Completed as part of [EPIC-4e6e9eae](EPIC-4e6e9eae) Phase 8.

## Lessons

No new lessons.
