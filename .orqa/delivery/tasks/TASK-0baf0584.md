---
id: TASK-0baf0584
type: task
title: Set up frontend coverage tooling
description: Configure Vitest coverage reporter with threshold enforcement.
status: completed
created: 2026-03-12
updated: 2026-03-12
assignee: AGENT-cc255bc8
acceptance:
  - vitest.config.ts exists with coverage configuration
  - make coverage-frontend target exists and produces a coverage report
  - "Coverage threshold of 80% configured (warn, not fail, initially)"
relationships:
  - target: EPIC-4726cb3b
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-e0cbdfe2
    type: depended-on-by
  - target: TASK-91bc09f9
    type: depended-on-by
---

## What

Add frontend test coverage measurement.

## How

1. Create `vitest.config.ts` with `@vitest/coverage-v8` or `@vitest/coverage-istanbul`
2. Set threshold to 80% (warning mode initially)
3. Add `make coverage-frontend` target
4. Document in commands.md

## Verification

`make coverage-frontend` produces a report with per-file coverage percentages.