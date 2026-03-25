---
id: TASK-fffd1121
type: task
title: Verify three-tier skill loading
description: "Dry-runs backend, frontend, and governance delegation scenarios to confirm that all three skill tiers load correctly and that no agent definition still carries project-specific skills directly."
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-8e58cd87
acceptance:
  - "Dry-run a backend task delegation — confirm Tier 1, 2, 3 all load"
  - Dry-run a frontend task delegation — confirm correct Tier 2 skills injected
  - Dry-run a governance task delegation — confirm orqa-governance injected
  - "Verify no agent definition still contains orqa-* or chunkhound/orqa-native-search"
relationships:
  - target: EPIC-42a5330b
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-072b9c19
    type: depended-on-by
---

## What

Verify the three-tier skill loading works end-to-end by dry-running several delegation
scenarios and confirming the right skills are loaded in each case.

## Test Scenarios

1. **Backend task** (scope: `backend/src-tauri/src/commands/`) — Expect: agent loads `code-search` +
   `rust-async-patterns` (Tier 1), orchestrator injects `orqa-ipc-patterns` +
   `orqa-error-composition` + `composability` (Tier 2), `code-search` resolves to
   `chunkhound` in CLI (Tier 3)

2. **Frontend task** (scope: `ui/src/lib/stores/`) — Expect: `code-search` +
   `svelte5-best-practices` (Tier 1), `orqa-store-patterns` + `orqa-store-orchestration` +
   `composability` (Tier 2)

3. **Governance task** (scope: `.orqa/`) — Expect: `code-search` + `planning` (Tier 1),
   `orqa-governance` + `composability` (Tier 2)

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.