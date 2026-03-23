---
id: "TASK-54fae8bf"
type: task
title: "Set up Rust coverage tooling"
description: "Configure cargo-tarpaulin or llvm-cov for Rust coverage measurement."
status: "completed"
created: "2026-03-12"
updated: "2026-03-12"
assignee: "AGENT-cc255bc8"
acceptance:
  - "make coverage-rust target exists and produces a coverage report"
  - "Coverage percentage is visible in terminal output"
  - "commands.md updated with new target"
relationships:
  - target: "EPIC-4726cb3b"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-efaf25d7"
    type: "depends-on"
  - target: "TASK-91bc09f9"
    type: "depended-on-by"
---
## What

Add Rust test coverage measurement so we can track the 80% target from [RULE-b49142be](RULE-b49142be).

## How

1. Install cargo-tarpaulin (or configure llvm-cov)
2. Add `make coverage-rust` target to Makefile
3. Document in commands.md
4. Run initial report to establish baseline

## Verification

`make coverage-rust` produces a report with per-module coverage percentages.