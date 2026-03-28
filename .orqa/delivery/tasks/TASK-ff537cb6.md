---
id: TASK-ff537cb6
type: task
title: "Audit connector hooks for business logic"
status: active
description: "Audit all connector hooks to classify as thin adapter (correct) or business logic (violation V4)"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 2 — Connector Thinning"
  - target: TASK-c298a900
    type: depends-on
    rationale: "Phase 1 cross-references must be validated before connector thinning"
acceptance:
  - "Written audit report listing every hook file in connectors/claude-code/src/hooks/ with classification (thin adapter vs business logic)"
  - "Each business logic hook has a remediation plan specifying which daemon entry types replace it"
  - "Daemon-gate verified functional — session blocks when daemon is not running"
  - "npx tsc --noEmit passes for connectors/claude-code"
---

# TASK-ff537cb6: Audit connector hooks for business logic

## What to do

Audit all connector hooks in `connectors/claude-code/src/hooks/` to identify V4 violations (connector containing business logic):

1. List every `.ts` file in `connectors/claude-code/src/hooks/`
2. For each hook, classify:
   - **Thin adapter (correct):** stdin -> HookContext parsing -> daemon POST /hook -> format output
   - **Business logic (violation):** contains if/else enforcement logic, field value checks, governance decisions, rule evaluation
3. For each violation, document:
   - What logic it contains (field checks, rule enforcement, etc.)
   - Which daemon entry type should replace it (`field-check`, `tool-matcher`, etc.)
   - Estimated effort to move to daemon
4. Verify daemon-gate:
   - Check that `connectors/claude-code/src/` has a gate that blocks session start when daemon is not running
   - Test the gate by checking code flow
5. Also check for standalone enforcement scripts (e.g., `enforce-background-agents.mjs`, `enforce-completion-gate.mjs`) that duplicate daemon logic

## Knowledge needed

- `connectors/claude-code/src/hooks/` — all hook files
- `connectors/claude-code/src/` — daemon-gate implementation
- `.orqa/process/rules/` — rule entry types that should be in daemon
- RES-d6e8ab11 section on connector architecture
- AD-1ef9f57c question 8 — daemon is business logic boundary

## Agent role

Researcher — read-only audit, produces a findings report. No code changes.

## Verification

- Count hook files and confirm each has a classification
- Grep for enforcement patterns in hooks: `if.*status`, `if.*field`, `switch.*type`, direct rule evaluation
- Confirm daemon-gate code path exists
- Run `npx tsc --noEmit` in `connectors/claude-code/`
