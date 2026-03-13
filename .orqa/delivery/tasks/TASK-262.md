---
id: TASK-262
title: "Set up frontend coverage tooling"
description: "Configure Vitest coverage reporter with threshold enforcement."
status: done
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-057
depends-on: []
assignee: AGENT-002
acceptance:
  - "vitest.config.ts exists with coverage configuration"
  - "make coverage-frontend target exists and produces a coverage report"
  - "Coverage threshold of 80% configured (warn, not fail, initially)"
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
