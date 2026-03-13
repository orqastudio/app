---
id: TASK-265
title: "Add errors.svelte.ts store test"
description: "Write test file for the one untested frontend store."
status: done
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-057
depends-on: []
assignee: AGENT-002
acceptance:
  - "errors.test.ts exists in __tests__/"
  - "Tests cover error creation, dismissal, and clearing"
  - "make test-frontend passes"
---

## What

`errors.svelte.ts` is the only store without tests. Add coverage.

## How

1. Read the store to understand its API
2. Write tests matching the pattern of existing store tests
3. Cover all exported functions and state transitions

## Verification

`make test-frontend` passes including the new test file.
