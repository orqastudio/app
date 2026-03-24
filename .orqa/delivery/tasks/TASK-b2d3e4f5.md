---
id: TASK-b2d3e4f5
type: task
title: "Test coverage gap analysis"
description: "Identify Rust modules and frontend code below the 80% test coverage threshold. Produce a prioritised list of modules needing test improvements."
status: ready
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - "Coverage report generated for all Rust modules"
  - "Coverage report generated for frontend code (Vitest)"
  - "Modules below 80% threshold are listed with current coverage percentage"
  - "Priority ranking based on criticality (domain logic > commands > utilities)"
  - "At least one test improvement task is created for the most critical gap"
relationships:
  - target: EPIC-b2f0399e
    type: delivers
---

## What

Measure test coverage across the entire codebase and identify modules below the 80% threshold defined in RULE-b49142be (coding-standards).

## How

### Rust
1. Run `cargo tarpaulin` (or equivalent coverage tool) with per-module reporting
2. Parse output to identify modules below 80%
3. Categorise by criticality: domain > commands > repo > utilities

### Frontend
1. Run `npx vitest --coverage` to generate coverage report
2. Identify files and directories below 80%
3. Categorise by criticality: stores > components > utilities

## Verification

1. Coverage reports exist for both Rust and frontend
2. Every module below 80% is listed with its current coverage percentage
3. Findings are triaged: immediate fixes vs follow-up tasks
