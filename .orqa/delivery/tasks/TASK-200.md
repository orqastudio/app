---
id: TASK-200
title: Implement skill injection in Rust enforcement engine
description: |
  Add inject action handling to the Rust enforcement engine for use in the
  app's native enforcement pipeline.
status: done
created: "2026-03-11"
updated: "2026-03-12"
epic: EPIC-052
depends-on:
  - TASK-193
  - TASK-195
---

## What

The Rust enforcement engine needs to handle `Inject` actions the same way the
plugin does — read SKILL.md files and include their content in the enforcement
result for the stream loop to inject as system context.

## How

1. Add `Inject` variant to the enforcement action enum in `enforcement.rs`
2. Add `skills` field to enforcement entry struct
3. When evaluating an inject entry, read skill files from project directory
4. Return skill content in enforcement result
5. Integrate with WorkflowTracker for deduplication

## Verification

- Rust enforcement engine handles Inject action without errors
- Skill content is returned in enforcement results
- Deduplication works across calls within a session
