---
id: "TASK-7c04e1d8"
type: "task"
title: "Fix db.rs migration error handling (.unwrap_or patterns)"
description: "db.rs lines 55,74,82,103 use .unwrap_or(false) in migration code, silently swallowing query errors when checking column existence."
status: archived
created: 2026-03-12T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
acceptance:
  - "Migration column checks propagate errors instead of swallowing them"
  - "Existing migrations still run correctly on fresh and existing databases"
  - "make test-rust passes"
relationships:
  - target: "EPIC-a1555708"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

db.rs lines 55,74,82,103 use .unwrap_or(false) in migration code, silently swallowing query errors when checking column existence.

## How

To be determined during implementation.

## Verification

- [ ] Migration column checks propagate errors instead of swallowing them
- [ ] Existing migrations still run correctly on fresh and existing databases
- [ ] make test-rust passes
