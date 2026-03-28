---
id: "TASK-316e5adf"
type: "task"
title: "Add errors.svelte.ts store test"
description: "Write test file for the one untested frontend store."
status: archived
created: 2026-03-12T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
acceptance:
  - "errors.test.ts exists in __tests__/"
  - "Tests cover error creation, dismissal, and clearing"
  - "make test-frontend passes"
relationships:
  - target: "EPIC-2bf6887a"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

`errors.svelte.ts` is the only store without tests. Add coverage.

## How

1. Read the store to understand its API
2. Write tests matching the pattern of existing store tests
3. Cover all exported functions and state transitions

## Verification

`make test-frontend` passes including the new test file.
