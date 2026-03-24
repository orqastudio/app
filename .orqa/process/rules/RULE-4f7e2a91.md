---
id: RULE-4f7e2a91
type: rule
title: Real-time Session State Management
description: tmp/session-state.md must be updated in real time during conversations — not just at session end. Every new decision, plan change, scope change, or step completion must be reflected immediately.
status: active
created: 2026-03-21
updated: 2026-03-21
enforcement:
  - mechanism: behavioral
    message: "tmp/session-state.md must be updated in real time during conversations; every new decision, plan change, scope change, or step completion must be reflected immediately"
relationships:
  - target: IMPL-a3f2c1d8
    type: promoted-from
  - target: RULE-1d91e7cb
    type: related
    rationale: "Completion gate depends on session state being current"
---
Session state is a **working document**, not a post-session summary. `tmp/session-state.md` must reflect the current state of the conversation at all times.

## What Must Be Reflected Immediately

Update `tmp/session-state.md` whenever any of the following occur:

- A new step is added to the plan
- A step is completed
- Scope changes (the focus shifts to a different task or epic)
- A new architecture decision is made
- A tangent introduces new work (before pursuing the tangent, not after)
- A blocker is discovered or resolved

## Required Contents

At minimum, session state must contain:

- **Current scope**: active epic and/or task IDs
- **Step checklist**: all planned steps with completion status (`[ ]` / `[x]`)
- **Architecture decisions made**: any ADs reached during the session
- **Lessons captured**: any patterns or issues worth logging

## Tangent Protocol

Agents MUST NOT pursue tangents without first updating the session state checklist.

1. Identify the tangent
2. Add it as a new item in the session state checklist (with its scope relationship clear)
3. Note that the original plan is paused
4. Pursue the tangent
5. Return to the original plan and mark progress

## Stop Hook Behaviour

The stop hook's auto-generated state summary MUST NOT overwrite a richer orchestrator-maintained state. If `tmp/session-state.md` was written by the orchestrator during the session, the stop hook should append a brief summary, not replace the file.

## Enforcement

This rule is enforced by the Claude Code connector's UserPromptSubmit hook (`connectors/claude-code/hooks/scripts/prompt-injector.mjs`):

1. **Constant reminder** — every prompt injection includes a reminder that session state is a working document
2. **Freshness check** — the hook checks `tmp/session-state.md` on every prompt:
   - If missing or auto-generated (no `### Steps` section): injects a reminder to create proper session state
   - If stale (>10 minutes since last update): injects a reminder to update
   - If orchestrator-maintained and fresh: no additional injection
3. **Telemetry** — session state reminder events are logged to `tmp/hook-metrics.json` for audit

The session management protocol in the orchestrator's system prompt (`CLAUDE.md`) references this rule as the authoritative source for session state requirements.

## FORBIDDEN

- Writing `tmp/session-state.md` only at session end
- Pursuing tangents without updating the session state first
- Stop hooks overwriting orchestrator-maintained session state with a shallower summary

## Related Rules

- [RULE-6083347d](RULE-6083347d) (dogfood-mode) — defines restart protocol which depends on session state
