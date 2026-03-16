---
id: TASK-247
title: Fix db.rs migration error handling (.unwrap_or patterns)
description: "db.rs lines 55,74,82,103 use .unwrap_or(false) in migration code, silently swallowing query errors when checking column existence."
status: completed
created: 2026-03-12
updated: 2026-03-12
acceptance:
  - Migration column checks propagate errors instead of swallowing them
  - Existing migrations still run correctly on fresh and existing databases
  - make test-rust passes
relationships:
  - target: EPIC-055
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-345
    type: depended-on-by
---

## What

db.rs lines 55,74,82,103 use .unwrap_or(false) in migration code, silently swallowing query errors when checking column existence.

## How

To be determined during implementation.

## Verification

- [ ] Migration column checks propagate errors instead of swallowing them
- [ ] Existing migrations still run correctly on fresh and existing databases
- [ ] make test-rust passes
