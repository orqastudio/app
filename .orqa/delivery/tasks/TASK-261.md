---
id: TASK-261
title: Set up Rust coverage tooling
description: Configure cargo-tarpaulin or llvm-cov for Rust coverage measurement.
status: completed
created: 2026-03-12
updated: 2026-03-12
assignee: AGENT-002
acceptance:
  - make coverage-rust target exists and produces a coverage report
  - Coverage percentage is visible in terminal output
  - commands.md updated with new target
relationships:
  - target: EPIC-057
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-260
    type: depends-on
  - target: TASK-347
    type: depended-on-by
---

## What

Add Rust test coverage measurement so we can track the 80% target from [RULE-006](RULE-006).

## How

1. Install cargo-tarpaulin (or configure llvm-cov)
2. Add `make coverage-rust` target to Makefile
3. Document in commands.md
4. Run initial report to establish baseline

## Verification

`make coverage-rust` produces a report with per-module coverage percentages.
