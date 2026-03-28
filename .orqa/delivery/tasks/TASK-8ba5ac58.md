---
id: "TASK-8ba5ac58"
type: "task"
title: "Replace status transition hook with daemon delegation"
description: "Replace the hardcoded VALID_TRANSITIONS map in app/.githooks/validate-status-transitions.mjs with a thin adapter that calls the daemon /parse endpoint for transition validation. Priority 1 — hardcoded maps WILL drift from config-driven Rust definitions."
status: archived
priority: "P1"
created: 2026-03-24T00:00:00.000Z
updated: 2026-03-24T00:00:00.000Z
acceptance:
  - "app/.githooks/validate-status-transitions.mjs no longer contains hardcoded VALID_TRANSITIONS object"
  - "Hook delegates to daemon /parse or orqa validate for status transition checks"
  - "Pre-commit hook correctly rejects invalid transitions"
  - "Pre-commit hook correctly allows valid transitions"
  - "Follows the validate-artifact.ts pattern (thin adapter, zero business logic)"
relationships:
  - target: "EPIC-0497a1be"
    type: "delivers"
    rationale: "Task delivers work to the deduplication epic"
---

## What

The pre-commit hook `app/.githooks/validate-status-transitions.mjs` contains a hardcoded `VALID_TRANSITIONS` object mapping artifact type prefixes to allowed status transitions. The canonical transition definitions live in the Rust validation crate, driven by plugin `statusTransitions` config. The JS map will silently drift when transitions are added or changed.

## How

1. Follow the pattern of `connectors/claude-code/src/hooks/validate-artifact.ts` — thin adapter calling `POST /parse` on the daemon
2. For each staged `.orqa/**/*.md` file with a status change, send the file to the daemon for validation
3. The daemon already validates status transitions as part of its parse pipeline
4. Remove the entire `VALID_TRANSITIONS` constant and its validation logic
5. Handle daemon-unavailable gracefully (warn but allow commit, or fall back to `orqa validate` binary)

## Files

- `app/.githooks/validate-status-transitions.mjs` — primary target for migration
- `connectors/claude-code/src/hooks/validate-artifact.ts` — reference pattern
- `libs/validation/src/checks/status.rs` — Rust canonical implementation
