---
id: TASK-036
title: Verify three-tier skill loading
status: done
epic: EPIC-042
description: Dry-runs backend, frontend, and governance delegation scenarios to confirm that all three skill tiers load correctly and that no agent definition still carries project-specific skills directly.
created: 2026-03-09
updated: 2026-03-09
assignee: qa-tester
skills: [orqa-governance, orqa-testing]
scope: [.orqa/team/agents/, .orqa/team/skills/, .orqa/governance/rules/skill-enforcement.md]
acceptance:
  - "Dry-run a backend task delegation — confirm Tier 1, 2, 3 all load"
  - "Dry-run a frontend task delegation — confirm correct Tier 2 skills injected"
  - "Dry-run a governance task delegation — confirm orqa-governance injected"
  - "Verify no agent definition still contains orqa-* or chunkhound/orqa-native-search"
---

## What

Verify the three-tier skill loading works end-to-end by dry-running several delegation
scenarios and confirming the right skills are loaded in each case.

## Test Scenarios

1. **Backend task** (scope: `src-tauri/src/commands/`) — Expect: agent loads `code-search` +
   `rust-async-patterns` (Tier 1), orchestrator injects `orqa-ipc-patterns` +
   `orqa-error-composition` + `orqa-composability` (Tier 2), `code-search` resolves to
   `chunkhound` in CLI (Tier 3)

2. **Frontend task** (scope: `ui/lib/stores/`) — Expect: `code-search` +
   `svelte5-best-practices` (Tier 1), `orqa-store-patterns` + `orqa-store-orchestration` +
   `orqa-composability` (Tier 2)

3. **Governance task** (scope: `.orqa/`) — Expect: `code-search` + `planning` (Tier 1),
   `orqa-governance` + `orqa-composability` (Tier 2)
